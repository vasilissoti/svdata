use anyhow::Error;
use clap::Parser;
use enquote;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, process};
use sv_parser::{parse_sv, unwrap_node, Define, DefineText, NodeEvent, RefNode, SyntaxTree};
use svdata::structures;
use verilog_filelist_parser;

#[derive(Debug, Parser)]
#[clap(name = "svdata")]
#[clap(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
pub struct Opt {
    #[clap(required_unless_present_any = &["filelist"])]
    pub files: Vec<PathBuf>,

    #[clap(short = 'f', long = "filelist", conflicts_with = "files")]
    pub filelist: Vec<PathBuf>,

    #[clap(
        short = 'd',
        long = "define",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub defines: Vec<String>,

    #[clap(
        short = 'i',
        long = "include",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub includes: Vec<PathBuf>,

    #[clap(long = "ignore-include")]
    pub ignore_include: bool,
}

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let opt = Parser::parse();
    let exit_code = match run_opt(&opt) {
        Ok(pass) => {
            if pass {
                0
            } else {
                1
            }
        }
        Err(_) => 2,
    };

    process::exit(exit_code);
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt(opt: &Opt) -> Result<bool, Error> {
    let mut defines = HashMap::new();
    for define in &opt.defines {
        let mut define = define.splitn(2, '=');
        let ident = String::from(define.next().unwrap());
        let text = if let Some(x) = define.next() {
            let x = enquote::unescape(x, None)?;
            Some(DefineText::new(x, None))
        } else {
            None
        };
        let define = Define::new(ident.clone(), vec![], text);
        defines.insert(ident, Some(define));
    }

    let (files, includes) = if !opt.filelist.is_empty() {
        let mut files = opt.files.clone();
        let mut includes = opt.includes.clone();

        for filelist in &opt.filelist {
            let (mut f, mut i, d) = parse_filelist(filelist)?;
            files.append(&mut f);
            includes.append(&mut i);
            for (k, v) in d {
                defines.insert(k, v);
            }
        }

        (files, includes)
    } else {
        (opt.files.clone(), opt.includes.clone())
    };

    let mut all_pass = true;
    let mut svdata = structures::SvData {
        modules: Vec::new(),
        packages: Vec::new(),
    };

    for path in &files {
        let mut pass = true;
        match parse_sv(&path, &defines, &includes, opt.ignore_include, false) {
            Ok((syntax_tree, new_defines)) => {
                sv_to_structure(&syntax_tree, &path.to_string_lossy().into_owned(), &mut svdata);
                defines = new_defines;
            }
            Err(_) => {
                println!("Parse failed");
                pass = false;
            }
        }

        if !pass {
            all_pass = false;
        }
    }

    println!("{}", svdata);

    Ok(all_pass)
}

#[cfg_attr(tarpaulin, skip)]
fn parse_filelist(
    path: &Path,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>, HashMap<String, Option<Define>>), Error> {
    let filelist = match verilog_filelist_parser::parse_file(path) {
        Ok(f) => f,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "failed to open '{}'",
                path.to_string_lossy()
            ))
        }
    };
    let mut defines = HashMap::new();
    for (d, t) in filelist.defines {
        match t {
            Some(t) => {
                let define_text = DefineText::new(String::from(&t[1..]), None);
                let define = Define::new(String::from(&d), vec![], Some(define_text));
                defines.insert(String::from(&d), Some(define));
            }
            None => {
                defines.insert(String::from(&d), None);
            }
        }
    }

    Ok((filelist.files, filelist.incdirs, defines))
}

fn sv_to_structure(syntax_tree: &SyntaxTree, filepath: &str, svdata: &mut structures::SvData) -> () {
    for event in syntax_tree.into_iter().event() {
        let enter_not_leave = match event {
            NodeEvent::Enter(_) => true,
            NodeEvent::Leave(_) => false,
        };
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(x) => x,
        };

        if enter_not_leave {
            match node {
                RefNode::ModuleDeclarationAnsi(_) => {
                    let d = parse_module_declaration_ansi(node, &syntax_tree, filepath);
                    svdata.modules.push(d.clone());
                }
                RefNode::ModuleDeclarationNonansi(_) => {
                    let _d = parse_module_declaration_nonansi(node, &syntax_tree, filepath);
                }
                _ => (),
            }
        }
    }
}

fn identifier(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = match unwrap_node!(parent, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    };

    match id {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

fn _datatype(parent: RefNode, _syntax_tree: &SyntaxTree) -> Option<String> {
    let t = match unwrap_node!(parent, DataType) {
        /*
        Some(RefNode::DataType(x)) => {
            println!("HERE x={:?}", x);
            Some(String::from("TODO"))
        }
        */
        Some(x) => {
            println!("HERE x={:?}", x);
            Some(String::from("TODO"))
        }
        _ => None,
    };

    /*
    match t {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
    */
    t
}

fn module_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = unwrap_node!(node, ModuleIdentifier).unwrap();
    identifier(id, &syntax_tree)
}

fn parse_module_declaration_ansi(
    m: RefNode,
    syntax_tree: &SyntaxTree,
    filepath: &str,
) -> structures::SvModuleDeclaration {
    let mut ret = structures::SvModuleDeclaration {
        identifier: module_identifier(m.clone(), syntax_tree).unwrap(),
        parameters: Vec::new(),
        ports: Vec::new(),
        filepath: String::from(filepath),
    };

    for node in m {
        match node {
            RefNode::ParameterDeclarationParam(p) => ret
                .parameters
                .push(parse_module_declaration_parameter(p, syntax_tree)),
            RefNode::AnsiPortDeclaration(p) => ret
                .ports
                .push(parse_module_declaration_port(p, syntax_tree)),
            _ => (),
        }
    }
    ret
}

fn parse_module_declaration_nonansi(
    _m: RefNode,
    _syntax_tree: &SyntaxTree,
    _filepath: &str,
) -> structures::SvModuleDeclaration {
    let ret = structures::SvModuleDeclaration {
        identifier: module_identifier(_m, _syntax_tree).unwrap(),
        parameters: Vec::new(),
        ports: Vec::new(),
        filepath: String::from(_filepath),
    };
    // TODO
    ret
}

fn parse_module_declaration_parameter(
    p: &sv_parser::ParameterDeclarationParam,
    _syntax_tree: &SyntaxTree,
) -> structures::SvParameter {
    println!("parameter={:?}", p);
    structures::SvParameter {
        identifier: String::from("foo"),
        datatype: String::from("bar"),
    }
}

fn port_identifier(node: &sv_parser::AnsiPortDeclaration, syntax_tree: &SyntaxTree) -> String {
    let id = unwrap_node!(node, PortIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

fn port_direction(node: &sv_parser::AnsiPortDeclaration) -> structures::SvPortDirection {
    let dir = unwrap_node!(node, PortDirection);
    match dir {
        Some(RefNode::PortDirection(sv_parser::PortDirection::Inout(_))) => {
            structures::SvPortDirection::Inout
        }
        Some(RefNode::PortDirection(sv_parser::PortDirection::Input(_))) => {
            structures::SvPortDirection::Input
        }
        Some(RefNode::PortDirection(sv_parser::PortDirection::Output(_))) => {
            structures::SvPortDirection::Output
        }
        Some(RefNode::PortDirection(sv_parser::PortDirection::Ref(_))) => {
            structures::SvPortDirection::Ref
        }
        _ => structures::SvPortDirection::IMPLICIT,
    }
}

fn port_datakind(node: &sv_parser::AnsiPortDeclaration) -> structures::SvPortDatakind {
    match node {
        sv_parser::AnsiPortDeclaration::Net(_) => structures::SvPortDatakind::Net,
        sv_parser::AnsiPortDeclaration::Variable(_) => structures::SvPortDatakind::Variable,
        sv_parser::AnsiPortDeclaration::Paren(_) => structures::SvPortDatakind::IMPLICIT,
    }
}

fn port_datatype(node: &sv_parser::AnsiPortDeclaration, syntax_tree: &SyntaxTree) -> String {
    match node {
        sv_parser::AnsiPortDeclaration::Net(p) => match &p.nodes.0 {
            Some(x) => syntax_tree.get_str_trim(x).unwrap().to_string(),
            None => String::from("IMPLICIT"),
        },
        sv_parser::AnsiPortDeclaration::Variable(p) => match &p.nodes.0 {
            Some(_x) => {
                //let t = datatype(x, syntax_tree);
                let t = Some(String::from("TODO"));
                match t {
                    Some(x) => x,
                    _ => String::from("IMPLICIT"),
                }
            }
            None => String::from("IMPLICIT"),
        },
        sv_parser::AnsiPortDeclaration::Paren(_) => String::from("IMPLICIT"),
    }
}

fn parse_module_declaration_port(
    p: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> structures::SvPort {
    structures::SvPort {
        identifier: port_identifier(p, syntax_tree),
        direction: port_direction(p),
        datakind: port_datakind(p),
        datatype: port_datatype(p, syntax_tree),
    }
}

/*
fn parse_package_declaration() -> structures::SvPackageDeclaration {
}

fn parse_package_declaration_parameter() -> structures::SvParameter {
}
*/
