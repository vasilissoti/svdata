use crate::structures::{SvModuleDeclaration, SvParamType, SvPort};
use crate::sv_misc::identifier;
use crate::sv_port::{port_declaration_ansi, port_parameter_declaration_ansi};
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
                let param_type = unwrap_node!(
                    p,
                    ParameterDeclarationParam,
                    LocalParameterDeclarationParam,
                    ParameterPortDeclarationParamList
                );

                let common_data =
                    unwrap_node!(param_type.clone().unwrap(), DataType, DataTypeOrImplicit)
                        .unwrap();
                let a = unwrap_node!(param_type.clone().unwrap(), ListOfParamAssignments);

                let param_type = match param_type {
                    Some(RefNode::LocalParameterDeclarationParam(_)) => SvParamType::LocalParam,
                    Some(RefNode::ParameterDeclarationParam(_))
                    | Some(RefNode::ParameterPortDeclarationParamList(_)) => SvParamType::Parameter,
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
