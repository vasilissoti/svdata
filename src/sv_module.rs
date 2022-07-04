use crate::structures::{SvModuleDeclaration, SvParamType, SvPort};
use crate::sv_misc::identifier;
use crate::sv_port::{port_declaration_ansi, port_parameter_declaration_ansi};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

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
            RefNode::ParameterPortList(p) => {
                let mut common_scope_found: bool = false;
                let mut param_type: RefNode = node.clone();

                for sub_node in p.into_iter().event() {
                    match sub_node {
                        NodeEvent::Enter(RefNode::ParameterDeclarationParam(x)) => {
                            common_scope_found = true;
                            param_type = RefNode::ParameterDeclarationParam(x);
                        }

                        NodeEvent::Enter(RefNode::LocalParameterDeclarationParam(x)) => {
                            common_scope_found = true;
                            param_type = RefNode::LocalParameterDeclarationParam(x);
                        }

                        NodeEvent::Enter(RefNode::ParameterPortDeclarationParamList(x)) => {
                            common_scope_found = true;
                            param_type = RefNode::ParameterPortDeclarationParamList(x);
                        }

                        NodeEvent::Leave(RefNode::LocalParameterDeclarationParam(_))
                        | NodeEvent::Leave(RefNode::ParameterDeclarationParam(_))
                        | NodeEvent::Leave(RefNode::ParameterPortDeclarationParamList(_)) => {
                            common_scope_found = false;
                        }

                        NodeEvent::Enter(RefNode::ListOfParamAssignments(a)) => {
                            if !common_scope_found {
                                let common_data = None;
                                let param_type = SvParamType::Parameter;

                                for param in a {
                                    match param {
                                        RefNode::ParamAssignment(x) => {
                                            ret.parameters.push(port_parameter_declaration_ansi(
                                                x,
                                                syntax_tree,
                                                common_data.clone(),
                                                &param_type,
                                            ))
                                        }
                                        _ => (),
                                    }
                                }
                            } else {
                                let common_data =
                                    unwrap_node!(param_type.clone(), DataType, DataTypeOrImplicit);
                                let a = unwrap_node!(param_type.clone(), ListOfParamAssignments);

                                let param_type = match param_type {
                                    RefNode::LocalParameterDeclarationParam(_) => {
                                        SvParamType::LocalParam
                                    }
                                    RefNode::ParameterDeclarationParam(_)
                                    | RefNode::ParameterPortDeclarationParamList(_) => {
                                        SvParamType::Parameter
                                    }
                                    _ => unreachable!(),
                                };

                                for param in a.unwrap() {
                                    match param {
                                        RefNode::ParamAssignment(x) => {
                                            ret.parameters.push(port_parameter_declaration_ansi(
                                                x,
                                                syntax_tree,
                                                common_data.clone(),
                                                &param_type,
                                            ))
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }

                        _ => (),
                    }
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
