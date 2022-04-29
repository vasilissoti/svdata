use anyhow::Error;
use clap::Parser;
use enquote;
use std::collections::HashMap;
use std::{env, process};
use std::path::{Path, PathBuf};
use sv_parser::{parse_sv, unwrap_node, Define, DefineText, NodeEvent, RefNode, SyntaxTree};
use svdata::structures;
use verilog_filelist_parser;


// Clap is used for accepting arguments through command prompt

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
    let opt = Parser::parse(); // This is from clap
    let exit_code = match run_opt(&opt) {
        Ok(pass) => {
            if pass {
                0
            } else {
                1
            }
        }
        Err(_) => {
            2
        }
    };

    process::exit(exit_code);
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt(opt: &Opt) -> Result<bool, Error> { // VNotes: The run opt will return [Err] if something didn't go well or otherwise will return [Ok]
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

    for path in &files {
        
        println!("");
        println!("The current path is: {}", path.to_string_lossy().into_owned()); // VNotes
        println!("");
        
        let mut pass = true;
        match parse_sv(&path, &defines, &includes, opt.ignore_include, false) {
            Ok((syntax_tree, new_defines)) => {
                sv_to_structure(&syntax_tree, &path.to_string_lossy().into_owned()); // VNotes
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

    Ok(all_pass)
}

// In case that the system verilog files are given in the format of a filelist

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

// Take it for granted up to here
// The following function is responsible for storing the data to the corresponding structs

fn sv_to_structure(syntax_tree: &SyntaxTree, filepath: &str) -> () { // VNotes
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
                RefNode::ModuleDeclarationAnsi(x) => {
                    let id = module_identifier(node, &syntax_tree).unwrap();
                    println!("ENTER ANSI module: {}", id);

                    let d = parse_module_declaration_ansi(x, &syntax_tree, filepath);
                    println!("  {:?}", d);

                }
                RefNode::ModuleDeclarationNonansi(x) => {
                    let id = module_identifier(node, &syntax_tree).unwrap();
                    println!("ENTER non-ANSI module: {}", id);

                    let d = parse_module_declaration_nonansi(x, &syntax_tree);
                    println!("  {:?}", d);

                }
                _ => (),
            }
        } else {
            match node {
                RefNode::ModuleDeclarationAnsi(_) |
                RefNode::ModuleDeclarationNonansi(_) => {
                    let id = module_identifier(node, &syntax_tree).unwrap();
                    println!("LEAVE module: {}", id);

                }
                _ => (),
            }
        }
    }
}

fn identifier(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = match unwrap_node!(parent, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            Some(x.nodes.0)
        },
        Some(RefNode::EscapedIdentifier(x)) => {
            Some(x.nodes.0)
        },
        _ => None,
    };


    match id {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

fn datatype(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
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
        _ => None
    };

    /*
    match t {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
    */
    t
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn module_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = unwrap_node!(node, ModuleIdentifier).unwrap();
    identifier(id, &syntax_tree)
}


// This is the core of the parsed data into structures for the ansi models

fn parse_module_declaration_ansi(
    m: &sv_parser::ModuleDeclarationAnsi,
    syntax_tree: &SyntaxTree, filepath: &str // VNotes
) -> structures::SvModuleDeclaration {
    let mut ret = structures::SvModuleDeclaration {
        parameters: Vec::new(),
        ports: Vec::new(),
        filepath: String::from(filepath), // VNotes
        declaration_type: String::from("ANSI"), // VNotes
    };
    for node in m {
        match node {
            RefNode::ParameterDeclarationParam(p) =>
                ret.parameters.push(parse_module_declaration_ansi_parameter(p, syntax_tree)),
            RefNode::AnsiPortDeclaration(p) => {
                ret.ports.push(parse_module_declaration_ansi_port(p, syntax_tree))
            },
            _ => (),
        }
    }
    ret
}

fn parse_module_declaration_nonansi(
    _m: &sv_parser::ModuleDeclarationNonansi,
    _syntax_tree: &SyntaxTree,
) -> structures::SvModuleDeclaration {
    let ret = structures::SvModuleDeclaration {
        parameters: Vec::new(),
        ports: Vec::new(),
        filepath: String::new(), // VNotes
        declaration_type: String::new(), // VNotes
    };
    // TODO
    ret
}

fn parse_module_declaration_ansi_parameter(
    p: &sv_parser::ParameterDeclarationParam,
    _syntax_tree: &SyntaxTree,
) -> structures::SvParameter {
    println!("parameter={:?}", p);
    structures::SvParameter {
        identifier: String::from("foo"),
        datatype: String::from("bar"),
    }
}

fn port_identifier(
    node: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> String {
    let id = unwrap_node!(node, PortIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

fn port_direction_ansi( // VNotes
    node: &sv_parser::AnsiPortDeclaration,
) -> structures::SvPortDirection {
    let dir = unwrap_node!(node, PortDirection);
    match dir {
        Some(RefNode::PortDirection(sv_parser::PortDirection::Inout(_))) =>
            structures::SvPortDirection::Inout,
        Some(RefNode::PortDirection(sv_parser::PortDirection::Input(_))) =>
            structures::SvPortDirection::Input,
        Some(RefNode::PortDirection(sv_parser::PortDirection::Output(_))) =>
            structures::SvPortDirection::Output,
        Some(RefNode::PortDirection(sv_parser::PortDirection::Ref(_))) =>
            structures::SvPortDirection::Ref,
        _ =>
            structures::SvPortDirection::IMPLICIT,
    }
}

fn port_datakind_ansi( // VNotes
    node: &sv_parser::AnsiPortDeclaration,
) -> structures::SvDatakind {
    match node {
        sv_parser::AnsiPortDeclaration::Net(_) =>
            structures::SvDatakind::Net, // VNotes
        sv_parser::AnsiPortDeclaration::Variable(_) =>
            structures::SvDatakind::Variable, // VNotes
        sv_parser::AnsiPortDeclaration::Paren(_) =>
            structures::SvDatakind::IMPLICIT,
    }
}

fn port_datatype_ansi( // VNotes
    node: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> String {
    match node {
        sv_parser::AnsiPortDeclaration::Net(p) => {
            println!("Net Detected"); //VNotes
            println!("The name of that port is: {}", port_identifier(node, syntax_tree)); // VNotes

            match &p.nodes.0 {
                Some(x) => syntax_tree.get_str_trim(x).unwrap().to_string(),
                None => String::from("IMPLICIT"),
            }},
        sv_parser::AnsiPortDeclaration::Variable(p) => {
            println!("Var Detected"); //VNotes
            println!("The name of that port is: {}", port_identifier(node, syntax_tree)); // VNotes

            match &p.nodes.0 {
                Some(x) => {
                    //let t = datatype(x, syntax_tree);
                    let t = Some(String::from("TODO"));
                    match t {
                        Some(x) => x,
                        _ => String::from("IMPLICIT"),
                    }
                },
                None => String::from("IMPLICIT"),
            }},
        sv_parser::AnsiPortDeclaration::Paren(_) =>
            String::from("IMPLICIT"),
    }
}

fn implicit_handler_ansi(s: &structures::SvPort){
    //println!("nothing");
}

fn parse_module_declaration_ansi_port(
    p: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> structures::SvPort {
    //println!("port={:?}", p);

    let vet1 = structures::SvUnpackedDimensions{ // VNotes
        dimensions: Vec::new(),
    };

    let vet2 = structures::SvPackedDimensions{ // VNotes
        dimensions: Vec::new(),
    };

    let ret = structures::SvPort { // VNotes: Attention order of compilation in the following lines matters!
        identifier: port_identifier(p, syntax_tree),
        direction: port_direction_ansi(p),
        nettype: None,
        datakind: port_datakind_ansi(p),
        datatype: port_datatype_ansi(p, syntax_tree),
        signedness: structures::SvSignedness::Signed,
        unpacked_dim: vet1,
        packed_dim: vet2,
        port_expression: String::from("Same"),

    };

    implicit_handler_ansi(&ret);

    return ret;
}

/*
fn parse_package_declaration() -> structures::SvPackageDeclaration {
}

fn parse_package_declaration_parameter() -> structures::SvParameter {
}
*/

