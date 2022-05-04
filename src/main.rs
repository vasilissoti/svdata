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
                    let id = module_identifier(node.clone(), &syntax_tree).unwrap(); // VNotes: To be removed
                    println!("ENTER ANSI module: {}", id);

                    let d = parse_module_declaration_ansi(node, x, &syntax_tree, filepath);
                    //println!("  {:?}", d);
                    println!("{}", d); // VNotes: Used for debugging deplay trait

                }
                RefNode::ModuleDeclarationNonansi(x) => {
                    let id = module_identifier(node.clone(), &syntax_tree).unwrap(); // VNotes: To be removed
                    println!("ENTER non-ANSI module: {}", id);

                    let d = parse_module_declaration_nonansi(node, x, &syntax_tree);
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

// VNotes: For future implementations
fn keyword(parent: RefNode, syntax_tree: &SyntaxTree) -> String {
    let id = match unwrap_node!(parent, Keyword){
        Some(RefNode::Keyword(x)) => { // VNotes Question ?
            Some(x.nodes.0)
        },

        _ => unreachable!(),
    };

    match id {
        Some(x) => syntax_tree.get_str(&x).unwrap().to_string(),
        _ => unreachable!(),
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

// VNotes: Used for debugging
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn module_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = unwrap_node!(node, ModuleIdentifier).unwrap();
    identifier(id, &syntax_tree)
}

// XXX: `ref` is unsupported.
// FIXME: `ref` is unsupported, it's a bug.
// TODO: `ref` is unsupported, but will be later.

// This is the core of the parsed data into structures for the ansi models

fn parse_module_declaration_ansi(
    node: RefNode,
    m: &sv_parser::ModuleDeclarationAnsi,
    syntax_tree: &SyntaxTree, filepath: &str // VNotes
) -> structures::SvModuleDeclaration {
    let mut ret = structures::SvModuleDeclaration {
        identifier: module_identifier(node, syntax_tree).unwrap(),
        parameters: Vec::new(),
        ports: Vec::new(),
        filepath: String::from(filepath), // VNotes
        declaration_type: String::from("ANSI"), // VNotes
    };

    let mut prev_port: Option<structures::SvPort> = None;
    
    for node in m {
        match node {
            RefNode::ParameterDeclarationParam(p) =>
                ret.parameters.push(parse_module_declaration_ansi_parameter(p, syntax_tree)),
            RefNode::AnsiPortDeclaration(p) => {
                let parsed_port: structures::SvPort = parse_module_declaration_ansi_port(p, syntax_tree, &prev_port.clone());
                ret.ports.push(parsed_port.clone());
                prev_port = Some(parsed_port.clone());
            },
            _ => (),
        }
    }
    ret
}

fn parse_module_declaration_nonansi(
    _node: RefNode,
    _m: &sv_parser::ModuleDeclarationNonansi,
    _syntax_tree: &SyntaxTree,
) -> structures::SvModuleDeclaration {
    let ret = structures::SvModuleDeclaration {
        identifier: module_identifier(_node, _syntax_tree).unwrap(),
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
    syntax_tree: &SyntaxTree
) -> String {
    let id = unwrap_node!(node, PortIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

fn port_direction_ansi( // VNotes
    node: &sv_parser::AnsiPortDeclaration, prev_port: &Option<structures::SvPort>
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
            match prev_port{
                Some(_) => prev_port.clone().unwrap().direction, // If not the first port, take the previous port's direction
                None => structures::SvPortDirection::Inout, // VNotes: Default case
            }
    }
}

fn port_datakind_ansi( // VNotes
    nettype: &structures::SvNetType
) -> structures::SvDataKind {

    match nettype{
        structures::SvNetType::NA => structures::SvDataKind::Variable,

        _ => structures::SvDataKind::Net,
    }
}

fn port_datatype_ansi( // VNotes
    node: &sv_parser::AnsiPortDeclaration
) -> structures::SvDataType {

    let dir = unwrap_node!(node, IntegerVectorType, IntegerAtomType, NonIntegerType, ClassType, TypeReference);
    match dir {
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Logic(_))) =>
            structures::SvDataType::Logic,
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Reg(_))) =>
            structures::SvDataType::Reg,
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Bit(_))) =>
            structures::SvDataType::Bit,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Byte(_))) =>
            structures::SvDataType::Byte,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Shortint(_))) =>
            structures::SvDataType::Shortint,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Int(_))) =>
            structures::SvDataType::Int,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Longint(_))) =>
            structures::SvDataType::Longint,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Integer(_))) =>
            structures::SvDataType::Integer,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Time(_))) =>
            structures::SvDataType::Time,
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Shortreal(_))) =>
            structures::SvDataType::Shortreal,
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Realtime(_))) =>
            structures::SvDataType::Realtime,
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Real(_))) =>
            structures::SvDataType::Real,
        Some(RefNode::ClassType(_)) =>
            structures::SvDataType::Class,
        Some(RefNode::TypeReference(_)) =>
            structures::SvDataType::TypeRef,
        _ =>
            structures::SvDataType::Logic, // VNotes: Default = Logic
    }

}

fn port_nettype_ansi(
    m: &sv_parser::AnsiPortDeclaration, direction: &structures::SvPortDirection, prev_port: &Option<structures::SvPort>) -> structures::SvNetType {

    let dir = unwrap_node!(m, AnsiPortDeclarationVariable, AnsiPortDeclarationNet);
    match dir{
        Some(RefNode::AnsiPortDeclarationVariable(_)) =>
            return structures::SvNetType::NA, // "Var" token was found
        
        Some(RefNode::AnsiPortDeclarationNet(x)) => {
            let dir = unwrap_node!(x, NetType);

            match dir{ // "Var" token was not found
                Some(RefNode::NetType(sv_parser::NetType::Supply0(_))) =>
                    return structures::SvNetType::Supply0,
                Some(RefNode::NetType(sv_parser::NetType::Supply1(_))) =>
                    return structures::SvNetType::Supply1,
                Some(RefNode::NetType(sv_parser::NetType::Triand(_))) =>
                    return structures::SvNetType::Triand,
                Some(RefNode::NetType(sv_parser::NetType::Trior(_))) =>
                    return structures::SvNetType::Trior,
                Some(RefNode::NetType(sv_parser::NetType::Trireg(_))) =>
                    return structures::SvNetType::Trireg,
                Some(RefNode::NetType(sv_parser::NetType::Tri0(_))) =>
                    return structures::SvNetType::Tri0,
                Some(RefNode::NetType(sv_parser::NetType::Tri1(_))) =>
                    return structures::SvNetType::Tri1,
                Some(RefNode::NetType(sv_parser::NetType::Tri(_))) =>
                    return structures::SvNetType::Tri,
                Some(RefNode::NetType(sv_parser::NetType::Uwire(_))) =>
                    return structures::SvNetType::Uwire,
                Some(RefNode::NetType(sv_parser::NetType::Wire(_))) =>
                    return structures::SvNetType::Wire,
                Some(RefNode::NetType(sv_parser::NetType::Wand(_))) =>
                    return structures::SvNetType::Wand,
                Some(RefNode::NetType(sv_parser::NetType::Wor(_))) =>
                    return structures::SvNetType::Wor,
                
                _ => match direction{ // Explicit net type was not found

                    structures::SvPortDirection::Inout | structures::SvPortDirection::Input => {
                        let dir = unwrap_node!(m, PortDirection);
                        match dir {
                            Some(_) => return structures::SvNetType::Wire, // For input/inout direction, if explicit direction found, no hierarchy for net type and data kind, default: net of net type wire
                            _ => return prev_port.clone().unwrap().nettype, // Else inherit from previous port
                        }
                    },
                    structures::SvPortDirection::Output => {
                        match unwrap_node!(m,  IntegerVectorType, IntegerAtomType, NonIntegerType, ClassType, TypeReference) {  // VNotes Add array enum, struct, class!
                            Some(_) => return structures::SvNetType::NA, // For output with explicit data type, default: variable
                            _ => {
                                let dir = unwrap_node!(m, PortDirection);
                                match dir{
                                    Some(_) => return structures::SvNetType::Wire, // For output with no explicit data type, default: net of net type wire
                                    _ => return prev_port.clone().unwrap().nettype, // Else inherit from previous port
                                }
                            }
                        }
                    },

                    structures::SvPortDirection::Ref => {
                        return structures::SvNetType::NA; // For ref, default/always: variable
                    },

                    _  => unreachable!() // Should never get here - IMPLICIT should never be used by ANSI

                }
            }       
        },

        _ => unreachable!(), // VNotes: Should never get here - Always one of the two must be available
    }
}


fn port_signedness_ansi(
    m: &sv_parser::AnsiPortDeclaration) -> structures::SvSignedness {
    
    let dir = unwrap_node!(m, Signing);
    match dir {
        Some(RefNode::Signing(sv_parser::Signing::Signed(_))) =>
            structures::SvSignedness::Signed,
        Some(RefNode::Signing(sv_parser::Signing::Unsigned(_))) =>
            structures::SvSignedness::Unsigned,
        _ =>
            structures::SvSignedness::Unsigned, // VNotes: The default is signed
    }

}




fn port_check_inheritance_ansi(
    m: &sv_parser::AnsiPortDeclaration) -> bool {
    let dir = unwrap_node!(m, DataType, Signing, NetType, VarDataType, PortDirection);

    match dir{
        Some(_) => false, // Do not inherit signedness, data_type, data_kind and direction from last port
        _ => true, // Inherit them
    }
    
}



fn parse_module_declaration_ansi_port(
    p: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree, prev_port: &Option<structures::SvPort>
) -> structures::SvPort {
    //println!("port={:?}", p);

    let vet1 = structures::SvUnpackedDimensions{ // VNotes {TEMP}
        dimensions: vec![String::from("Not supported yet")],
    };

    let vet2 = structures::SvPackedDimensions{ // VNotes {TEMP}
        dimensions: vec![String::from("Not supported yet")],
    };

    // VNotes complete inheritance

    let inherit = port_check_inheritance_ansi(p);
    let ret: structures::SvPort;

    if inherit == false{
        ret = structures::SvPort { // VNotes: Attention order of compilation in the following lines matters!
            identifier: port_identifier(p, syntax_tree),
            direction: port_direction_ansi(p, prev_port),
            nettype: port_nettype_ansi(p, &port_direction_ansi(p, prev_port), prev_port),
            datakind: port_datakind_ansi(&port_nettype_ansi(p, &port_direction_ansi(p, prev_port), prev_port)),
            datatype: port_datatype_ansi(p),
            signedness: port_signedness_ansi(p),
            unpacked_dim: vet1,
            packed_dim: vet2,
            port_expression: String::from("Same"),

        };
    }

    else {
        ret = structures::SvPort{
            identifier: port_identifier(p, syntax_tree),
            direction: prev_port.clone().unwrap().direction,
            nettype: prev_port.clone().unwrap().nettype,
            datakind: prev_port.clone().unwrap().datakind,
            datatype: prev_port.clone().unwrap().datatype,
            signedness: prev_port.clone().unwrap().signedness,
            unpacked_dim: vet1,
            packed_dim: vet2,
            port_expression: String::from("Same"),
        };
    }

    //println!("{:?}", ret); // VNotes: Used for debugging

    return ret;
}

/*
fn parse_package_declaration() -> structures::SvPackageDeclaration {
}

fn parse_package_declaration_parameter() -> structures::SvParameter {
}
*/

