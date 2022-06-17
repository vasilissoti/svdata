use crate::structures::{
    SvDataKind, SvDataType, SvNetType, SvPackedDimension, SvParamStatus, SvParamType, SvParameter,
    SvPort, SvPortDirection, SvSignedness, SvUnpackedDimension,
};
use crate::sv_misc::{get_string, identifier, keyword, symbol};
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
            packed_dimensions: port_packeddim_ansi(RefNode::AnsiPortDeclaration(p), syntax_tree),
            unpacked_dimensions: port_unpackeddim_ansi(
                RefNode::AnsiPortDeclaration(p),
                syntax_tree,
            ),
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
            packed_dimensions: prev_port.clone().unwrap().packed_dimensions,
            unpacked_dimensions: port_unpackeddim_ansi(
                RefNode::AnsiPortDeclaration(p),
                syntax_tree,
            ),
        };
    }

    return ret;
}

pub fn port_parameter_declaration_ansi(
    p: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
    common_data: RefNode,
    param_type: &SvParamType,
) -> SvParameter {
    let found_assignment = port_parameter_check_default_ansi(p);
    let (param_datatype, param_datatype_status) =
        port_parameter_datatype_ansi(common_data.clone(), p, syntax_tree, found_assignment);
    let (param_signedness, param_signedness_status) = port_parameter_signedness_ansi(
        common_data.clone(),
        p,
        &param_datatype,
        found_assignment,
        param_datatype_status.clone(),
        syntax_tree,
    );

    let ret = SvParameter {
        identifier: port_parameter_identifier_ansi(p, syntax_tree),
        paramtype: param_type.clone(),
        datatype: param_datatype.clone(),
        datatype_status: param_datatype_status.clone(),
        classid: port_parameter_classid_ansi(common_data.clone(), &param_datatype, syntax_tree),
        signedness: param_signedness.clone(),
        signedness_status: param_signedness_status,
        packed_dimensions: port_packeddim_ansi(common_data, syntax_tree),
        unpacked_dimensions: port_unpackeddim_ansi(RefNode::ParamAssignment(p), syntax_tree),
        value: port_parameter_value_ansi(p, syntax_tree, found_assignment),
    };

    port_parameter_syntax_ansi(&ret.datatype, &ret.signedness, &ret.packed_dimensions);

    ret
}

fn port_parameter_check_default_ansi(node: &sv_parser::ParamAssignment) -> bool {
    let expression = unwrap_node!(node, ConstantParamExpression);
    match expression {
        Some(RefNode::ConstantParamExpression(_)) => true,
        _ => false,
    }
}

fn port_parameter_syntax_ansi(
    datatype: &Option<SvDataType>,
    signedness: &Option<SvSignedness>,
    packed_dimensions: &Vec<SvPackedDimension>,
) {
    if !packed_dimensions.is_empty() {
        match datatype {
            Some(SvDataType::Integer) => {
                panic!("Cannot combine packed dimensions with an integer!")
            }
            Some(SvDataType::Real) => panic!("Cannot combine packed dimensions with a real!"),
            Some(SvDataType::String) => panic!("Cannot combine packed dimensions with a string!"),
            Some(SvDataType::Time) => panic!("Cannot combine packed dimensions with time!"),
            _ => (),
        }
    }

    match signedness {
        Some(SvSignedness::Signed) | Some(SvSignedness::Unsigned) => match datatype {
            Some(SvDataType::Real) => panic!("Reals cannot have signedness!"),
            Some(SvDataType::String) => panic!("Strings cannot have signedness!"),
            Some(SvDataType::Time) => panic!("Time cannot have signedness!"),
            _ => (),
        },

        _ => (),
    }
}

fn parameter_resolver_needed_ansi(node: &sv_parser::ParamAssignment) -> bool {
    let expression = unwrap_node!(
        node,
        ConstantFunctionCall,
        BinaryOperator,
        ConstantConcatenation,
        ConditionalExpression
    );
    match expression {
        Some(_) => true,
        _ => false,
    }
}

fn parameter_datatype_resolver_ansi(node: &sv_parser::ParamAssignment) -> SvDataType {
    let datatype = unwrap_node!(
        node,
        Number,
        TimeLiteral,
        UnbasedUnsizedLiteral,
        StringLiteral
    );
    match datatype {
        Some(RefNode::Number(sv_parser::Number::IntegralNumber(_))) => {
            let subtype = unwrap_node!(node, RealNumber);
            match subtype {
                Some(_) => return SvDataType::Real,
                _ => return SvDataType::Integer,
            }
        }

        Some(RefNode::Number(sv_parser::Number::RealNumber(_))) => {
            return SvDataType::Real;
        }
        Some(RefNode::TimeLiteral(_)) => {
            let subtype = unwrap_node!(node, RealNumber, IntegralNumber);
            match subtype {
                Some(RefNode::RealNumber(_)) => return SvDataType::Real,
                Some(RefNode::IntegralNumber(_)) => return SvDataType::Integer,
                _ => return SvDataType::Time,
            }
        }
        Some(RefNode::UnbasedUnsizedLiteral(_)) => {
            let subtype = unwrap_node!(node, RealNumber, IntegralNumber);
            match subtype {
                Some(RefNode::RealNumber(_)) => return SvDataType::Real,
                Some(RefNode::IntegralNumber(_)) => return SvDataType::Integer,
                _ => return SvDataType::Bit,
            }
        }
        Some(RefNode::StringLiteral(_)) => {
            return SvDataType::String;
        }
        _ => unreachable!(),
    }
}

fn parameter_signedness_resolver_ansi(
    node: &sv_parser::ParamAssignment,
    datatype: &Option<SvDataType>,
    syntax_tree: &SyntaxTree,
) -> Option<SvSignedness> {
    match datatype {
        Some(SvDataType::String) => return None,
        _ => (),
    }

    for sub_node in node {
        match sub_node {
            RefNode::Number(sv_parser::Number::IntegralNumber(_)) => {
                let integral_type = unwrap_node!(sub_node, BinaryNumber, HexNumber, OctalNumber);
                match integral_type {
                    Some(RefNode::BinaryNumber(_))
                    | Some(RefNode::HexNumber(_))
                    | Some(RefNode::OctalNumber(_)) => {
                        let base =
                            unwrap_node!(integral_type.unwrap(), BinaryBase, HexBase, OctalBase);
                        let base_token = get_string(base.clone().unwrap(), syntax_tree).unwrap();

                        match base {
                            Some(RefNode::BinaryBase(_)) => {
                                if base_token != "'sb" {
                                    return Some(SvSignedness::Unsigned);
                                }
                            }

                            Some(RefNode::HexBase(_)) => {
                                if base_token != "'sh" {
                                    return Some(SvSignedness::Unsigned);
                                }
                            }

                            Some(RefNode::OctalBase(_)) => {
                                if base_token != "'so" {
                                    return Some(SvSignedness::Unsigned);
                                }
                            }

                            _ => unreachable!(),
                        }
                    }

                    _ => (),
                }
            }

            RefNode::Number(sv_parser::Number::RealNumber(_)) => {
                return None;
            }

            RefNode::TimeLiteral(_) => {
                return Some(SvSignedness::Unsigned);
            }

            RefNode::BinaryOperator(_) => {
                let symbol_token = symbol(sub_node, syntax_tree).unwrap();
                if symbol_token == ">>" {
                    return Some(SvSignedness::Unsigned);
                }
            }

            _ => (),
        }
    }

    Some(SvSignedness::Signed)
}

fn port_parameter_identifier_ansi(
    node: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
) -> String {
    let id = unwrap_node!(node, ParameterIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

fn port_parameter_value_ansi(
    node: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
    found_assignment: bool,
) -> Option<String> {
    if !found_assignment {
        return None;
    } else {
        let expression = unwrap_node!(node, ConstantExpression);
        get_string(expression.unwrap(), syntax_tree)
    }
}

fn port_parameter_datatype_ansi(
    common_data: RefNode,
    p: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
    found_assignment: bool,
) -> (Option<SvDataType>, SvParamStatus) {
    let datatype = unwrap_node!(
        common_data.clone(),
        IntegerVectorType,
        IntegerAtomType,
        NonIntegerType,
        ClassType,
        TypeReference
    );
    match datatype {
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Logic(_))) => {
            (Some(SvDataType::Logic), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Reg(_))) => {
            (Some(SvDataType::Reg), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Bit(_))) => {
            (Some(SvDataType::Bit), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Byte(_))) => {
            (Some(SvDataType::Byte), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Shortint(_))) => {
            (Some(SvDataType::Shortint), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Int(_))) => {
            (Some(SvDataType::Int), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Longint(_))) => {
            (Some(SvDataType::Longint), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Integer(_))) => {
            (Some(SvDataType::Integer), SvParamStatus::Fixed)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Time(_))) => {
            (Some(SvDataType::Time), SvParamStatus::Fixed)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Shortreal(_))) => {
            (Some(SvDataType::Shortreal), SvParamStatus::Fixed)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Realtime(_))) => {
            (Some(SvDataType::Realtime), SvParamStatus::Fixed)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Real(_))) => {
            (Some(SvDataType::Real), SvParamStatus::Fixed)
        }
        Some(RefNode::ClassType(_)) => (Some(SvDataType::Class), SvParamStatus::Fixed),
        Some(RefNode::TypeReference(_)) => (Some(SvDataType::TypeRef), SvParamStatus::Fixed),
        _ => match unwrap_node!(common_data.clone(), DataType) {
            Some(x) => match keyword(x, syntax_tree) {
                Some(x) => {
                    if x == "string" {
                        return (Some(SvDataType::String), SvParamStatus::Fixed);
                    } else {
                        println!("{}", x);
                        unreachable!();
                    }
                }

                _ => unreachable!(),
            },
            _ => {
                if found_assignment {
                    if parameter_resolver_needed_ansi(p) {
                        match unwrap_node!(p, BinaryOperator) {
                            Some(_) => {
                                return (
                                    Some(parameter_datatype_resolver_ansi(p)),
                                    SvParamStatus::Overridable,
                                )
                            }
                            _ => {
                                return (Some(SvDataType::Unsupported), SvParamStatus::Overridable)
                            }
                        }
                    } else {
                        let implicit_type = unwrap_node!(
                            p,
                            Number,
                            TimeLiteral,
                            UnbasedUnsizedLiteral,
                            StringLiteral
                        );
                        match implicit_type {
                            Some(RefNode::Number(sv_parser::Number::IntegralNumber(_))) => {
                                return (Some(SvDataType::Integer), SvParamStatus::Overridable)
                            }
                            Some(RefNode::Number(sv_parser::Number::RealNumber(_))) => {
                                return (Some(SvDataType::Real), SvParamStatus::Overridable)
                            }
                            Some(RefNode::TimeLiteral(_)) => {
                                return (Some(SvDataType::Time), SvParamStatus::Overridable)
                            }
                            Some(RefNode::UnbasedUnsizedLiteral(_)) => {
                                (Some(SvDataType::Bit), SvParamStatus::Overridable)
                            }
                            Some(RefNode::StringLiteral(_)) => {
                                (Some(SvDataType::String), SvParamStatus::Overridable)
                            }
                            _ => unreachable!(),
                        }
                    }
                } else {
                    return (None, SvParamStatus::Overridable);
                }
            }
        },
    }
}

fn port_parameter_signedness_ansi(
    m: RefNode,
    p: &sv_parser::ParamAssignment,
    datatype: &Option<SvDataType>,
    found_assignment: bool,
    datatype_status: SvParamStatus,
    syntax_tree: &SyntaxTree,
) -> (Option<SvSignedness>, SvParamStatus) {
    let signedness = unwrap_node!(m.clone(), Signing);
    match signedness {
        Some(RefNode::Signing(sv_parser::Signing::Signed(_))) => {
            return (Some(SvSignedness::Signed), SvParamStatus::Fixed)
        }
        Some(RefNode::Signing(sv_parser::Signing::Unsigned(_))) => {
            return (Some(SvSignedness::Unsigned), SvParamStatus::Fixed)
        }
        _ => (),
    }

    match datatype {
        Some(SvDataType::Class)
        | Some(SvDataType::String)
        | Some(SvDataType::Real)
        | Some(SvDataType::Time) => match datatype_status {
            SvParamStatus::Overridable => return (None, SvParamStatus::Overridable),
            SvParamStatus::Fixed => return (None, SvParamStatus::Fixed),
        },

        Some(SvDataType::Shortint)
        | Some(SvDataType::Int)
        | Some(SvDataType::Longint)
        | Some(SvDataType::Byte) => (Some(SvSignedness::Signed), SvParamStatus::Overridable),

        Some(SvDataType::Integer) => {
            if !found_assignment {
                return (Some(SvSignedness::Signed), SvParamStatus::Overridable);
            } else if parameter_resolver_needed_ansi(p) {
                match unwrap_node!(p, BinaryOperator) {
                    Some(_) => {
                        return (
                            parameter_signedness_resolver_ansi(p, datatype, syntax_tree),
                            SvParamStatus::Overridable,
                        )
                    }
                    _ => return (Some(SvSignedness::Unsupported), SvParamStatus::Overridable),
                }
            } else {
                let integral_type =
                    unwrap_node!(p, DecimalNumber, BinaryNumber, HexNumber, OctalNumber);
                match integral_type {
                    Some(RefNode::DecimalNumber(_)) => {
                        return (Some(SvSignedness::Signed), SvParamStatus::Overridable)
                    }
                    _ => {
                        let base =
                            unwrap_node!(integral_type.unwrap(), BinaryBase, HexBase, OctalBase);
                        let base_token = get_string(base.clone().unwrap(), syntax_tree).unwrap();

                        match base {
                            Some(RefNode::BinaryBase(_)) => {
                                if base_token == "'sb" {
                                    return (
                                        Some(SvSignedness::Signed),
                                        SvParamStatus::Overridable,
                                    );
                                } else {
                                    return (
                                        Some(SvSignedness::Unsigned),
                                        SvParamStatus::Overridable,
                                    );
                                }
                            }

                            Some(RefNode::HexBase(_)) => {
                                if base_token == "'sh" {
                                    return (
                                        Some(SvSignedness::Signed),
                                        SvParamStatus::Overridable,
                                    );
                                } else {
                                    return (
                                        Some(SvSignedness::Unsigned),
                                        SvParamStatus::Overridable,
                                    );
                                }
                            }

                            Some(RefNode::OctalBase(_)) => {
                                if base_token == "'so" {
                                    return (
                                        Some(SvSignedness::Signed),
                                        SvParamStatus::Overridable,
                                    );
                                } else {
                                    return (
                                        Some(SvSignedness::Unsigned),
                                        SvParamStatus::Overridable,
                                    );
                                }
                            }

                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        _ => match datatype {
            Some(SvDataType::Unsupported) => {
                (Some(SvSignedness::Unsupported), SvParamStatus::Overridable)
            }
            None => (None, SvParamStatus::Overridable),
            _ => (Some(SvSignedness::Unsigned), SvParamStatus::Overridable),
        },
    }
}

fn port_parameter_classid_ansi(
    m: RefNode,
    datatype: &Option<SvDataType>,
    syntax_tree: &SyntaxTree,
) -> Option<String> {
    match datatype {
        Some(SvDataType::Class) => {
            let id = unwrap_node!(m, ClassIdentifier).unwrap();
            Some(identifier(id, &syntax_tree).unwrap())
        }

        _ => None,
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
    let objecttype = unwrap_node!(m, AnsiPortDeclarationVariable, AnsiPortDeclarationNet);
    match objecttype {
        Some(RefNode::AnsiPortDeclarationVariable(_)) => {
            match unwrap_node!(m, PortDirection, DataType, Signing, PackedDimension) {
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
        SvDataType::Class | SvDataType::String | SvDataType::Real | SvDataType::Time => None,
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

            match datatype {
                SvDataType::Shortint
                | SvDataType::Int
                | SvDataType::Longint
                | SvDataType::Byte
                | SvDataType::Integer => Some(SvSignedness::Signed),
                _ => Some(SvSignedness::Unsigned),
            }
        }
    }
}

fn port_packeddim_ansi(m: RefNode, syntax_tree: &SyntaxTree) -> Vec<SvPackedDimension> {
    let mut ret: Vec<SvPackedDimension> = Vec::new();

    for node in m {
        match node {
            RefNode::PackedDimensionRange(x) => {
                let range = unwrap_node!(x, ConstantRange);
                match range {
                    Some(RefNode::ConstantRange(sv_parser::ConstantRange { nodes })) => {
                        let (l, _, r) = nodes;
                        let left =
                            get_string(RefNode::ConstantExpression(&l), syntax_tree).unwrap();
                        let right =
                            get_string(RefNode::ConstantExpression(&r), syntax_tree).unwrap();

                        ret.push((left.clone(), right.clone()));
                    }

                    _ => (),
                }
            }

            _ => (),
        }
    }

    ret
}

fn port_unpackeddim_ansi(m: RefNode, syntax_tree: &SyntaxTree) -> Vec<SvUnpackedDimension> {
    let mut ret: Vec<SvUnpackedDimension> = Vec::new();

    for node in m {
        match node {
            RefNode::UnpackedDimensionRange(x) => {
                let range = unwrap_node!(x, ConstantRange);
                match range {
                    Some(RefNode::ConstantRange(sv_parser::ConstantRange { nodes })) => {
                        let (l, _, r) = nodes;
                        let left = get_string(RefNode::ConstantExpression(l), syntax_tree).unwrap();
                        let right =
                            get_string(RefNode::ConstantExpression(r), syntax_tree).unwrap();

                        ret.push((left.clone(), Some(right.clone())));
                    }

                    _ => (),
                }
            }

            RefNode::UnpackedDimensionExpression(x) => {
                let range = unwrap_node!(x, ConstantExpression).unwrap();
                let left = get_string(range, syntax_tree).unwrap();

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
    let datatype = unwrap_node!(
        m,
        DataType,
        Signing,
        NetType,
        VarDataType,
        PortDirection,
        PackedDimension
    );

    match prev_port {
        Some(_) => match datatype {
            Some(_) => false,
            _ => true,
        },
        None => false,
    }
}
