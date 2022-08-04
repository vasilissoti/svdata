use crate::structures::{SvPackageDeclaration, SvParamType};
use crate::sv_misc::identifier;
use crate::sv_port::port_parameter_declaration_ansi;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

pub fn package_declaration(
    m: RefNode,
    syntax_tree: &SyntaxTree,
    filepath: &str,
) -> SvPackageDeclaration {
    let mut ret = SvPackageDeclaration {
        identifier: package_identifier(m.clone(), syntax_tree).unwrap(),
        parameters: Vec::new(),
        filepath: String::from(filepath),
    };
    let mut param_type: RefNode = m.clone();

    for node in m.into_iter().event() {
        match node {
            NodeEvent::Enter(RefNode::ParameterDeclarationParam(x)) => {
                param_type = RefNode::ParameterDeclarationParam(x);
            }

            NodeEvent::Enter(RefNode::LocalParameterDeclarationParam(x)) => {
                param_type = RefNode::LocalParameterDeclarationParam(x);
            }

            NodeEvent::Enter(RefNode::ListOfParamAssignments(a)) => {
                let common_data = unwrap_node!(param_type.clone(), DataType, DataTypeOrImplicit);

                let param_type = match param_type {
                    RefNode::LocalParameterDeclarationParam(_) => SvParamType::LocalParam,
                    RefNode::ParameterDeclarationParam(_) => SvParamType::Parameter,
                    _ => unreachable!(),
                };

                for param in a {
                    match param {
                        RefNode::ParamAssignment(x) => {
                            let mut parameter = port_parameter_declaration_ansi(
                                x,
                                syntax_tree,
                                common_data.clone(),
                                &SvParamType::LocalParam,
                            );

                            parameter.paramtype = param_type.clone();
                            ret.parameters.push(parameter);
                        }
                        _ => (),
                    }
                }
            }

            _ => (),
        }
    }

    ret
}

fn package_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = unwrap_node!(node, PackageIdentifier).unwrap();
    identifier(id, &syntax_tree)
}
