use crate::structures::{
    SvDataKind, SvDataType, SvNetType, SvPackedDimension, SvParamType, SvParameter, SvPort,
    SvPortDirection, SvSignedness, SvUnpackedDimension,
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
    let (param_datatype, param_datatype_overridable) =
        port_parameter_datatype_ansi(common_data.clone(), p, syntax_tree, found_assignment);
    let (param_signedness, param_signedness_overridable) = port_parameter_signedness_ansi(
        common_data.clone(),
        p,
        &param_datatype,
        found_assignment,
        param_datatype_overridable.clone(),
        syntax_tree,
    );

    let ret = SvParameter {
        identifier: port_parameter_identifier_ansi(p, syntax_tree),
        paramtype: param_type.clone(),
        datatype: param_datatype.clone(),
        datatype_overridable: param_datatype_overridable.clone(),
        classid: port_parameter_classid_ansi(common_data.clone(), &param_datatype, syntax_tree),
        signedness: param_signedness.clone(),
        signedness_overridable: param_signedness_overridable,
        packed_dimensions: port_packeddim_ansi(common_data, syntax_tree),
        unpacked_dimensions: port_unpackeddim_ansi(RefNode::ParamAssignment(p), syntax_tree),
        expression: port_parameter_value_ansi(p, syntax_tree, found_assignment),
        bit_num: port_parameter_bits_ansi(
            port_packeddim_ansi(common_data.clone(), syntax_tree).clone(),
            p,
            &param_datatype,
            param_datatype_status,
            found_assignment,
            &port_parameter_value_ansi(p, syntax_tree, found_assignment),
            syntax_tree,
        ),
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
                _ => return SvDataType::Logic,
            }
        }

        Some(RefNode::Number(sv_parser::Number::RealNumber(_))) => {
            return SvDataType::Real;
        }
        Some(RefNode::TimeLiteral(_)) => {
            let subtype = unwrap_node!(node, RealNumber, IntegralNumber);
            match subtype {
                Some(RefNode::RealNumber(_)) => return SvDataType::Real,
                Some(RefNode::IntegralNumber(_)) => return SvDataType::Logic,
                _ => return SvDataType::Time,
            }
        }
        Some(RefNode::UnbasedUnsizedLiteral(_)) => {
            let subtype = unwrap_node!(node, RealNumber, IntegralNumber);
            match subtype {
                Some(RefNode::RealNumber(_)) => return SvDataType::Real,
                Some(RefNode::IntegralNumber(_)) => return SvDataType::Logic,
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

            RefNode::UnbasedUnsizedLiteral(_) => {
                return Some(SvSignedness::Unsigned);
            }

            RefNode::BinaryOperator(_) => {
                let symbol_token = symbol(sub_node, syntax_tree).unwrap();
                match symbol_token.as_str() {
                    "&" | "~&" | "|" | "~|" | "^" | "~^" | "<" | "<=" | ">" | ">=" | "=="
                    | "=!" => return Some(SvSignedness::Unsigned),
                    _ => (),
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
) -> (Option<SvDataType>, bool) {
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
            (Some(SvDataType::Logic), false)
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Reg(_))) => {
            (Some(SvDataType::Reg), false)
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Bit(_))) => {
            (Some(SvDataType::Bit), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Byte(_))) => {
            (Some(SvDataType::Byte), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Shortint(_))) => {
            (Some(SvDataType::Shortint), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Int(_))) => {
            (Some(SvDataType::Int), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Longint(_))) => {
            (Some(SvDataType::Longint), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Integer(_))) => {
            (Some(SvDataType::Integer), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Time(_))) => {
            (Some(SvDataType::Time), false)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Shortreal(_))) => {
            (Some(SvDataType::Shortreal), false)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Realtime(_))) => {
            (Some(SvDataType::Realtime), false)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Real(_))) => {
            (Some(SvDataType::Real), false)
        }
        Some(RefNode::ClassType(_)) => (Some(SvDataType::Class), false),
        Some(RefNode::TypeReference(_)) => (Some(SvDataType::TypeRef), false),
        _ => match unwrap_node!(common_data.clone(), DataType) {
            Some(x) => match keyword(x, syntax_tree) {
                Some(x) => {
                    if x == "string" {
                        return (Some(SvDataType::String), false);
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
                            Some(_) => return (Some(parameter_datatype_resolver_ansi(p)), true),
                            _ => return (Some(SvDataType::Unsupported), true),
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
                                return (Some(SvDataType::Logic), true)
                            }
                            Some(RefNode::Number(sv_parser::Number::RealNumber(_))) => {
                                return (Some(SvDataType::Real), true)
                            }
                            Some(RefNode::TimeLiteral(_)) => return (Some(SvDataType::Time), true),
                            Some(RefNode::UnbasedUnsizedLiteral(_)) => {
                                (Some(SvDataType::Bit), true)
                            }
                            Some(RefNode::StringLiteral(_)) => (Some(SvDataType::String), true),
                            _ => unreachable!(),
                        }
                    }
                } else {
                    return (None, true);
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
    datatype_status: bool,
    syntax_tree: &SyntaxTree,
) -> (Option<SvSignedness>, bool) {
    let signedness = unwrap_node!(m.clone(), Signing);
    match signedness {
        Some(RefNode::Signing(sv_parser::Signing::Signed(_))) => {
            return (Some(SvSignedness::Signed), false)
        }
        Some(RefNode::Signing(sv_parser::Signing::Unsigned(_))) => {
            return (Some(SvSignedness::Unsigned), false)
        }
        _ => (),
    }

    match datatype {
        Some(SvDataType::Class)
        | Some(SvDataType::String)
        | Some(SvDataType::Real)
        | Some(SvDataType::Time) => match datatype_status {
            true => return (None, true),
            false => return (None, false),
        },

        Some(SvDataType::Shortint)
        | Some(SvDataType::Int)
        | Some(SvDataType::Longint)
        | Some(SvDataType::Byte)
        | Some(SvDataType::Integer) => (Some(SvSignedness::Signed), true),

        Some(SvDataType::Logic) => {
            if !found_assignment {
                return (Some(SvSignedness::Unsigned), true);
            } else {
                if parameter_resolver_needed_ansi(p) {
                    match unwrap_node!(p, BinaryOperator) {
                        Some(_) => {
                            return (
                                parameter_signedness_resolver_ansi(p, datatype, syntax_tree),
                                true,
                            )
                        }
                        _ => return (Some(SvSignedness::Unsupported), true),
                    }
                } else {
                    let integral_type =
                        unwrap_node!(p, DecimalNumber, BinaryNumber, HexNumber, OctalNumber);
                    match integral_type {
                        Some(RefNode::DecimalNumber(_)) => {
                            return (Some(SvSignedness::Signed), true)
                        }
                        _ => {
                            let base = unwrap_node!(
                                integral_type.unwrap(),
                                BinaryBase,
                                HexBase,
                                OctalBase
                            );
                            let base_token =
                                get_string(base.clone().unwrap(), syntax_tree).unwrap();

                            match base {
                                Some(RefNode::BinaryBase(_)) => {
                                    if base_token == "'sb" {
                                        return (Some(SvSignedness::Signed), true);
                                    } else {
                                        return (Some(SvSignedness::Unsigned), true);
                                    }
                                }

                                Some(RefNode::HexBase(_)) => {
                                    if base_token == "'sh" {
                                        return (Some(SvSignedness::Signed), true);
                                    } else {
                                        return (Some(SvSignedness::Unsigned), true);
                                    }
                                }

                                Some(RefNode::OctalBase(_)) => {
                                    if base_token == "'so" {
                                        return (Some(SvSignedness::Signed), true);
                                    } else {
                                        return (Some(SvSignedness::Unsigned), true);
                                    }
                                }

                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
        }

        _ => match datatype {
            Some(SvDataType::Unsupported) => (Some(SvSignedness::Unsupported), true),
            None => (None, true),
            _ => (Some(SvSignedness::Unsigned), true),
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

fn port_parameter_bits_ansi(
    mut packed_dimensions: Vec<SvPackedDimension>,
    p: &sv_parser::ParamAssignment,
    datatype: &Option<SvDataType>,
    _datatype_overridable: bool,
    found_assignment: bool,
    expression: &Option<String>,
    syntax_tree: &SyntaxTree,
) -> Option<u64> {
    if !packed_dimensions.is_empty() {
        let mut nu_bits: u64 = 0;
        packed_dimensions.reverse();

        for dim in packed_dimensions {
            let (left, right) = dim;
            let left_num: u64 = left.as_str().parse().unwrap();
            let right_num: u64 = right.as_str().parse().unwrap();

            if nu_bits == 0 {
                nu_bits = left_num - right_num + 1;
            } else {
                nu_bits = nu_bits * (left_num - right_num + 1);
            }
        }

        Some(nu_bits)
    } else {
        if parameter_resolver_needed_ansi(p) {
            return Some(404); // TODO
        } else {
            match datatype {
                Some(SvDataType::Class) => return None,

                Some(SvDataType::Bit) => return Some(1),

                Some(SvDataType::Byte) => return Some(8),

                Some(SvDataType::Integer) | Some(SvDataType::Int) | Some(SvDataType::Shortreal) => {
                    return Some(32)
                }

                Some(SvDataType::Shortint) => Some(16),

                Some(SvDataType::Longint)
                | Some(SvDataType::Time)
                | Some(SvDataType::Real)
                | Some(SvDataType::Realtime) => return Some(64),

                Some(SvDataType::String) => {
                    if !found_assignment {
                        return None;
                    } else {
                        return Some((expression.clone().unwrap().len() as u64 - 2) * 8);
                    }
                }

                Some(SvDataType::Reg) | Some(SvDataType::Logic) => {
                    if !found_assignment {
                        return Some(0);
                    } else {
                        let fixed_size = unwrap_node!(p, Size);

                        match fixed_size {
                            Some(_) => {
                                let ret: u64;
                                ret = get_string(fixed_size.clone().unwrap(), syntax_tree)
                                    .unwrap()
                                    .as_str()
                                    .parse()
                                    .unwrap();
                                return Some(ret);
                            }

                            _ => return Some(32),
                        }
                    }
                }

                _ => unreachable!(),
            }
        }
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
