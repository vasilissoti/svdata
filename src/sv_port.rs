use crate::structures::{
    SvDataKind, SvDataType, SvNetType, SvPort, SvPortDirection, SvSignedness, SvUnpackedDimension,
};
use crate::sv_misc::{identifier, keyword, number, symbol};
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub fn port_declaration_ansi(
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
            classid: port_classid_ansi(p, &port_datatype_ansi(p, syntax_tree), syntax_tree),
            signedness: port_signedness_ansi(p, &port_datatype_ansi(p, syntax_tree)),
            unpacked_dimensions: port_unpackeddim_ansi(p, syntax_tree),
        }
    } else {
        ret = SvPort {
            identifier: port_identifier(p, syntax_tree),
            direction: prev_port.clone().unwrap().direction,
            nettype: prev_port.clone().unwrap().nettype,
            datakind: prev_port.clone().unwrap().datakind,
            datatype: prev_port.clone().unwrap().datatype,
            classid: prev_port.clone().unwrap().classid,
            signedness: prev_port.clone().unwrap().signedness,
            unpacked_dimensions: port_unpackeddim_ansi(p, syntax_tree),
        };
    }

    return ret;
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
    let objecttype = unwrap_node!(m, AnsiPortDeclarationVariable, AnsiPortDeclarationNet);
    match objecttype {
        Some(RefNode::AnsiPortDeclarationVariable(_)) => {
            match unwrap_node!(m, PortDirection, DataType, Signing) {
                Some(_) => return None,
                _ => return Some(SvNetType::Wire),
            }
        }

        Some(RefNode::AnsiPortDeclarationNet(x)) => {
            let nettype = unwrap_node!(x, NetType);

            match nettype {
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

fn port_signedness_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    datatype: &SvDataType,
) -> Option<SvSignedness> {
    match datatype {
        SvDataType::Class => None,
        _ => {
            let signedness = unwrap_node!(m, Signing);
            match signedness {
                Some(RefNode::Signing(sv_parser::Signing::Signed(_))) => {
                    return Some(SvSignedness::Signed)
                }
                Some(RefNode::Signing(sv_parser::Signing::Unsigned(_))) => {
                    return Some(SvSignedness::Unsigned)
                }
                _ => (),
            }

            Some(SvSignedness::Unsigned)
        }
    }
}

fn port_unpackeddim_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> Vec<SvUnpackedDimension> {
    let mut ret: Vec<SvUnpackedDimension> = Vec::new();

    for node in m {
        match node {
            RefNode::UnpackedDimensionRange(x) => {
                let mut left = String::new();
                let mut right = String::new();

                let range = unwrap_node!(x, ConstantRange);
                match range {
                    Some(RefNode::ConstantRange(sv_parser::ConstantRange { nodes })) => {
                        let (u, _, l) = nodes;
                        for sub_node in u {
                            match sub_node {
                                RefNode::BinaryOperator(_) => {
                                    left.push_str(&symbol(sub_node, syntax_tree).unwrap())
                                }
                                RefNode::Identifier(_) => {
                                    left.push_str(&identifier(sub_node, syntax_tree).unwrap())
                                }
                                RefNode::Number(_) => {
                                    left.push_str(&number(sub_node, syntax_tree).unwrap())
                                }
                                _ => (),
                            }
                        }
                        for sub_node in l {
                            match sub_node {
                                RefNode::BinaryOperator(_) => {
                                    right.push_str(&symbol(sub_node, syntax_tree).unwrap())
                                }
                                RefNode::Identifier(_) => {
                                    right.push_str(&identifier(sub_node, syntax_tree).unwrap())
                                }
                                RefNode::Number(_) => {
                                    right.push_str(&number(sub_node, syntax_tree).unwrap())
                                }
                                _ => (),
                            }
                        }

                        ret.push((left.clone(), Some(right.clone())));
                    }

                    _ => (),
                }
            }

            RefNode::UnpackedDimensionExpression(u) => {
                let mut left = String::new();
                for sub_node in u {
                    match sub_node {
                        RefNode::BinaryOperator(_) => {
                            left.push_str(&symbol(sub_node, syntax_tree).unwrap())
                        }
                        RefNode::Identifier(_) => {
                            left.push_str(&identifier(sub_node, syntax_tree).unwrap())
                        }
                        RefNode::Number(_) => {
                            left.push_str(&number(sub_node, syntax_tree).unwrap())
                        }
                        _ => (),
                    }
                }

                ret.push((left.clone(), None));
            }

            _ => (),
        }
    }

    ret
}

fn port_classid_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    datatype: &SvDataType,
    syntax_tree: &SyntaxTree,
) -> Option<String> {
    match datatype {
        SvDataType::Class => {
            let id = unwrap_node!(m, ClassIdentifier).unwrap();
            Some(identifier(id, &syntax_tree).unwrap())
        }

        _ => None,
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
