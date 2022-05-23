use anyhow::Error;
use clap::Parser;
use enquote;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, process};
use sv_parser::{parse_sv, unwrap_node, Define, DefineText, NodeEvent, RefNode, SyntaxTree};
use svdata::structures::{
    SvData, SvDataKind, SvDataType, SvModuleDeclaration, SvNetType, SvParameter, SvPort,
    SvPortDirection, SvSignedness,
};
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
        Ok(Some(_)) => 0,
        Ok(None) => 1,
        Err(_) => 2,
    };

    process::exit(exit_code);
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt(opt: &Opt) -> Result<Option<SvData>, Error> {
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
    let mut svdata = SvData {
        modules: Vec::new(),
        packages: Vec::new(),
    };

    for path in &files {
        let mut pass = true;
        match parse_sv(&path, &defines, &includes, opt.ignore_include, false) {
            Ok((syntax_tree, new_defines)) => {
                sv_to_structure(
                    &syntax_tree,
                    &path.to_string_lossy().into_owned(),
                    &mut svdata,
                );
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

    let ret: Option<SvData> = if all_pass { Some(svdata) } else { None };
    Ok(ret)
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

fn sv_to_structure(syntax_tree: &SyntaxTree, filepath: &str, svdata: &mut SvData) -> () {
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

fn keyword(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = match unwrap_node!(parent, Keyword) {
        Some(RefNode::Keyword(x)) => Some(x.nodes.0),

        _ => None,
    };

    match id {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

fn module_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = unwrap_node!(node, ModuleIdentifier).unwrap();
    identifier(id, &syntax_tree)
}

fn parse_module_declaration_ansi(
    m: RefNode,
    syntax_tree: &SyntaxTree,
    filepath: &str,
) -> SvModuleDeclaration {
    let mut ret = SvModuleDeclaration {
        identifier: module_identifier(m.clone(), syntax_tree).unwrap(),
        parameters: Vec::new(),
        ports: Vec::new(),
        filepath: String::from(filepath),
    };

    let mut prev_port: Option<SvPort> = None;

    for node in m {
        match node {
            RefNode::ParameterDeclarationParam(p) => ret
                .parameters
                .push(parse_module_declaration_parameter(p, syntax_tree)),
            RefNode::AnsiPortDeclaration(p) => {
                let parsed_port: SvPort =
                    parse_module_declaration_port_ansi(p, syntax_tree, &prev_port.clone());
                ret.ports.push(parsed_port.clone());
                prev_port = Some(parsed_port.clone());
            }
            _ => (),
        }
    }
    ret
}

fn parse_module_declaration_nonansi(
    _m: RefNode,
    _syntax_tree: &SyntaxTree,
    _filepath: &str,
) -> SvModuleDeclaration {
    let ret = SvModuleDeclaration {
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
) -> SvParameter {
    println!("parameter={:?}", p);
    SvParameter {
        identifier: String::from("foo"),
        datatype: String::from("bar"),
    }
}

fn port_identifier(node: &sv_parser::AnsiPortDeclaration, syntax_tree: &SyntaxTree) -> String {
    let id = unwrap_node!(node, PortIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

fn port_direction_ansi(
    node: &sv_parser::AnsiPortDeclaration,
    prev_port: &Option<SvPort>,
) -> SvPortDirection {
    let dir = unwrap_node!(node, PortDirection);
    match dir {
        Some(RefNode::PortDirection(sv_parser::PortDirection::Inout(_))) => SvPortDirection::Inout,
        Some(RefNode::PortDirection(sv_parser::PortDirection::Input(_))) => SvPortDirection::Input,
        Some(RefNode::PortDirection(sv_parser::PortDirection::Output(_))) => {
            SvPortDirection::Output
        }
        Some(RefNode::PortDirection(sv_parser::PortDirection::Ref(_))) => SvPortDirection::Ref,
        _ => match prev_port {
            Some(_) => prev_port.clone().unwrap().direction,
            None => SvPortDirection::Inout,
        },
    }
}

fn port_datakind_ansi(nettype: &Option<SvNetType>) -> SvDataKind {
    match nettype {
        None => SvDataKind::Variable,

        Some(_) => SvDataKind::Net,
    }
}

fn port_datatype_ansi(
    node: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> SvDataType {
    let datatype = unwrap_node!(
        node,
        IntegerVectorType,
        IntegerAtomType,
        NonIntegerType,
        ClassType,
        TypeReference
    );
    match datatype {
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Logic(_))) => {
            SvDataType::Logic
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Reg(_))) => SvDataType::Reg,
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Bit(_))) => SvDataType::Bit,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Byte(_))) => SvDataType::Byte,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Shortint(_))) => {
            SvDataType::Shortint
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Int(_))) => SvDataType::Int,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Longint(_))) => {
            SvDataType::Longint
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Integer(_))) => {
            SvDataType::Integer
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Time(_))) => SvDataType::Time,
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Shortreal(_))) => {
            SvDataType::Shortreal
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Realtime(_))) => {
            SvDataType::Realtime
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Real(_))) => SvDataType::Real,
        Some(RefNode::ClassType(_)) => SvDataType::Class,
        Some(RefNode::TypeReference(_)) => SvDataType::TypeRef,
        _ => match unwrap_node!(node, DataType) {
            Some(x) => match keyword(x, syntax_tree) {
                Some(x) => {
                    if x == "string" {
                        return SvDataType::String;
                    } else {
                        println!("{}", x);
                        unreachable!();
                    }
                }

                _ => unreachable!(),
            },
            _ => return SvDataType::Logic,
        },
    }
}

fn port_nettype_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    direction: &SvPortDirection,
) -> Option<SvNetType> {
    let nettype = unwrap_node!(m, AnsiPortDeclarationVariable, AnsiPortDeclarationNet);
    match nettype {
        Some(RefNode::AnsiPortDeclarationVariable(_)) => {
            match unwrap_node!(m, PortDirection, DataType, Signing) {
                Some(_) => return None,
                _ => return Some(SvNetType::Wire),
            }
        }

        Some(RefNode::AnsiPortDeclarationNet(x)) => {
            let dir = unwrap_node!(x, NetType);

            match dir {
                // "Var" token was not found
                Some(RefNode::NetType(sv_parser::NetType::Supply0(_))) => {
                    return Some(SvNetType::Supply0)
                }
                Some(RefNode::NetType(sv_parser::NetType::Supply1(_))) => {
                    return Some(SvNetType::Supply1)
                }
                Some(RefNode::NetType(sv_parser::NetType::Triand(_))) => {
                    return Some(SvNetType::Triand)
                }
                Some(RefNode::NetType(sv_parser::NetType::Trior(_))) => {
                    return Some(SvNetType::Trior)
                }
                Some(RefNode::NetType(sv_parser::NetType::Trireg(_))) => {
                    return Some(SvNetType::Trireg)
                }
                Some(RefNode::NetType(sv_parser::NetType::Tri0(_))) => {
                    return Some(SvNetType::Tri0)
                }
                Some(RefNode::NetType(sv_parser::NetType::Tri1(_))) => {
                    return Some(SvNetType::Tri1)
                }
                Some(RefNode::NetType(sv_parser::NetType::Tri(_))) => return Some(SvNetType::Tri),
                Some(RefNode::NetType(sv_parser::NetType::Uwire(_))) => {
                    return Some(SvNetType::Uwire)
                }
                Some(RefNode::NetType(sv_parser::NetType::Wire(_))) => {
                    return Some(SvNetType::Wire)
                }
                Some(RefNode::NetType(sv_parser::NetType::Wand(_))) => {
                    return Some(SvNetType::Wand)
                }
                Some(RefNode::NetType(sv_parser::NetType::Wor(_))) => return Some(SvNetType::Wor),

                _ => match direction {
                    SvPortDirection::Inout | SvPortDirection::Input => {
                        return Some(SvNetType::Wire);
                    }
                    SvPortDirection::Output => match unwrap_node!(m, DataType) {
                        Some(_) => return None,
                        _ => return Some(SvNetType::Wire),
                    },

                    SvPortDirection::Ref => {
                        return None;
                    }

                    _ => unreachable!(),
                },
            }
        }

        _ => unreachable!(),
    }
}

fn port_signedness_ansi(m: &sv_parser::AnsiPortDeclaration) -> SvSignedness {
    let signedness = unwrap_node!(m, Signing);
    match signedness {
        Some(RefNode::Signing(sv_parser::Signing::Signed(_))) => SvSignedness::Signed,
        Some(RefNode::Signing(sv_parser::Signing::Unsigned(_))) => SvSignedness::Unsigned,
        _ => SvSignedness::Unsigned,
    }
}

fn port_check_inheritance_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    prev_port: &Option<SvPort>,
) -> bool {
    let datatype = unwrap_node!(m, DataType, Signing, NetType, VarDataType, PortDirection);

    match prev_port {
        Some(_) => match datatype {
            Some(_) => false,
            _ => true,
        },
        None => false,
    }
}

fn parse_module_declaration_port_ansi(
    p: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
    prev_port: &Option<SvPort>,
) -> SvPort {
    let inherit = port_check_inheritance_ansi(p, prev_port);
    let ret: SvPort;

    if inherit == false {
        ret = SvPort {
            identifier: port_identifier(p, syntax_tree),
            direction: port_direction_ansi(p, prev_port),
            nettype: port_nettype_ansi(p, &port_direction_ansi(p, prev_port)),
            datakind: port_datakind_ansi(&port_nettype_ansi(p, &port_direction_ansi(p, prev_port))),
            datatype: port_datatype_ansi(p, syntax_tree),
            signedness: port_signedness_ansi(p),
        }
    } else {
        ret = SvPort {
            identifier: port_identifier(p, syntax_tree),
            direction: prev_port.clone().unwrap().direction,
            nettype: prev_port.clone().unwrap().nettype,
            datakind: prev_port.clone().unwrap().datakind,
            datatype: prev_port.clone().unwrap().datatype,
            signedness: prev_port.clone().unwrap().signedness,
        };
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use serde_yaml;
    use std::fs;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read, Write};

    fn check_outputs(name: &str) {
        let out_dir = env::var("OUT_DIR").unwrap();

        let sv_path = format!("testcases/sv/{}.sv", name);
        let args = vec!["svdata", &sv_path];
        let opt = Opt::parse_from(args.iter());
        let svdata = run_opt(&opt).unwrap();

        let e = format!("testcases/display/{}.txt", name);
        let e = File::open(e).unwrap();
        let mut e = BufReader::new(e);
        let mut expected_string: String = String::new();
        e.read_to_string(&mut expected_string).unwrap();

        let actual_string: String = format!("{}", svdata.clone().unwrap());

        // Write actual display to file for manual inspection.
        fs::create_dir_all(Path::new(&out_dir).join("testcases/display")).unwrap();
        let a = Path::new(&out_dir).join(format!("testcases/display/{}.txt", name));
        let a = File::create(a);
        let mut a = BufWriter::new(a.unwrap());
        write!(a, "{}", actual_string).unwrap();

        assert_eq!(expected_string, actual_string);

        let e = format!("testcases/json/{}.json", name);
        let e = File::open(e).unwrap();
        let e = BufReader::new(e);
        let expected_json_value: serde_json::Value = serde_json::from_reader(e).unwrap();

        let s: String = serde_json::to_string_pretty(&svdata.clone().unwrap()).unwrap();
        let actual_json_value: serde_json::Value = serde_json::from_str(&s).unwrap();

        // Write actual JSON to file for manual inspection.
        fs::create_dir_all(Path::new(&out_dir).join("testcases/json")).unwrap();
        let a = Path::new(&out_dir).join(format!("testcases/json/{}.json", name));
        let a = File::create(a);
        let mut a = BufWriter::new(a.unwrap());
        write!(a, "{}", s).unwrap();

        assert_eq!(expected_json_value, actual_json_value);

        let e = format!("testcases/yaml/{}.yaml", name);
        let e = File::open(e).unwrap();
        let e = BufReader::new(e);
        let expected_yaml_value: serde_yaml::Value = serde_yaml::from_reader(e).unwrap();

        let s: String = serde_yaml::to_string(&svdata.clone().unwrap()).unwrap();
        let actual_yaml_value: serde_yaml::Value = serde_yaml::from_str(&s).unwrap();

        // Write actual YAML to file for manual inspection.
        fs::create_dir_all(Path::new(&out_dir).join("testcases/yaml")).unwrap();
        let a = Path::new(&out_dir).join(format!("testcases/yaml/{}.yaml", name));
        let a = File::create(a);
        let mut a = BufWriter::new(a.unwrap());
        write!(a, "{}", s).unwrap();

        assert_eq!(expected_yaml_value, actual_yaml_value);
    }
    include!(concat!(env!("OUT_DIR"), "/tests.rs"));
}
