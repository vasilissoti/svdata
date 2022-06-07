use crate::structures::{SvModuleDeclaration, SvParamType, SvParameter, SvPort, SvSignedness};
use crate::sv_misc::identifier;
use crate::sv_port::port_declaration_ansi;
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub fn module_declaration_ansi(
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
            RefNode::ParameterPortDeclaration(p) => {
                let param_type =
                    unwrap_node!(p, ParameterDeclaration, ParameterPortDeclarationParamList);
                match param_type {
                    Some(RefNode::ParameterDeclaration(x)) => ret
                        .parameters
                        .push(module_declaration_parameter(x, syntax_tree)),

                    Some(RefNode::ParameterPortDeclarationParamList(x)) => {
                        let common_data = unwrap_node!(x, DataType).unwrap();
                        let a = unwrap_node!(x, ListOfParamAssignments);

                        for param in a.unwrap() {
                            match param {
                                RefNode::ParamAssignment(x) => {
                                    ret.parameters.push(module_declaration_parameter_list(
                                        x,
                                        syntax_tree,
                                        common_data.clone(),
                                    ))
                                }
                                _ => (),
                            }
                        }
                    }

                    _ => unreachable!(),
                }
            }
            RefNode::AnsiPortDeclaration(p) => {
                let parsed_port: SvPort = port_declaration_ansi(p, syntax_tree, &prev_port.clone());
                ret.ports.push(parsed_port.clone());
                prev_port = Some(parsed_port.clone());
            }
            _ => (),
        }
    }
    ret
}

pub fn module_declaration_nonansi(
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

fn module_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = unwrap_node!(node, ModuleIdentifier).unwrap();
    identifier(id, &syntax_tree)
}

fn module_declaration_parameter(
    p: &sv_parser::ParameterDeclaration,
    _syntax_tree: &SyntaxTree,
) -> SvParameter {
    println!("parameter={:?}", p);
    // TODO
    SvParameter {
        identifier: String::from("foo"),
        paramtype: SvParamType::Parameter,
        datatype: String::from("bar"),
        signedness: Some(SvSignedness::Unsigned),
    }
}

fn module_declaration_parameter_list(
    p: &sv_parser::ParamAssignment,
    _syntax_tree: &SyntaxTree,
    _common_data: RefNode,
) -> SvParameter {
    println!("parameter={:?}", p);
    // TODO
    SvParameter {
        identifier: String::from("foo"),
        paramtype: SvParamType::Parameter,
        datatype: String::from("bar"),
        signedness: Some(SvSignedness::Unsigned),
    }
}
