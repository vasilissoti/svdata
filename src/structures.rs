use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize, Clone)]
pub struct SvData {
    pub modules: Vec<SvModuleDeclaration>,
    pub packages: Vec<SvPackageDeclaration>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SvModuleDeclaration {
    pub identifier: String,
    pub parameters: Vec<SvParameter>,
    pub ports: Vec<SvPort>,
    pub filepath: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SvPackageDeclaration {
    pub parameters: Vec<SvParameter>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SvParameter {
    pub identifier: String,
    pub paramtype: SvParamType,
    pub datatype: Option<String>,
    pub signedness: Option<SvSignedness>,
    // pub packed_dimensions: Vec<SvPackedDimension>,
    // pub unpacked_dimensions: Vec<SvUnpackedDimension>,
}

#[derive(Debug, Serialize, Clone)]
pub enum SvParamType {
    Parameter,
    LocalParam,
}

#[derive(Debug, Serialize, Clone)]
pub enum SvPortDirection {
    Inout,
    Input,
    Output,
    Ref,
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub enum SvDataKind {
    Net,
    Variable,
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub enum SvSignedness {
    Signed,
    Unsigned,
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub enum SvDataType {
    Logic,
    Reg,
    Bit,
    Byte,
    Integer,
    Int,
    Shortint,
    Longint,
    Time,
    Real,
    Shortreal,
    Realtime,
    Array,
    Enum,
    Struct,
    Union,
    Class,
    TypeRef,
    String,
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub enum SvNetType {
    Wire,
    Uwire,
    Tri,
    Wor,
    Wand,
    Triand,
    Trior,
    Trireg,
    Tri0,
    Tri1,
    Supply0,
    Supply1,
    IMPLICIT,
}

pub type SvPackedDimension = (String, String);
pub type SvUnpackedDimension = (String, Option<String>);

#[derive(Debug, Serialize, Clone)]
pub struct SvPort {
    pub identifier: String,
    pub direction: SvPortDirection,
    pub datakind: SvDataKind,
    pub datatype: SvDataType,
    pub classid: Option<String>,
    pub nettype: Option<SvNetType>,
    pub signedness: Option<SvSignedness>,
    pub packed_dimensions: Vec<SvPackedDimension>,
    pub unpacked_dimensions: Vec<SvUnpackedDimension>,
}

impl fmt::Display for SvData {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        for module in self.modules.clone() {
            write!(f, "{}", module)?;
        }

        write!(f, "")
    }
}

impl fmt::Display for SvModuleDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Module:")?;
        writeln!(f, "  Identifier: {}", self.identifier)?;
        writeln!(f, "  Filepath: {}", self.filepath)?;

        for port in self.ports.clone() {
            write!(f, "{}", port)?;
        }

        // for param in self.parameters.clone() {
        //     write!(f, "{}", param)?;
        // }

        writeln!(f, "")
    }
}

impl fmt::Display for SvPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  Port: ")?;
        writeln!(f, "    Identifier: {}", self.identifier)?;
        writeln!(f, "    Direction: {:?}", self.direction)?;
        writeln!(f, "    DataKind: {:?}", self.datakind)?;
        writeln!(f, "    DataType: {:?}", self.datatype)?;
        match self.classid.clone() {
            None => {
                writeln!(f, "    ClassIdentifier: None")?;
            }
            Some(x) => {
                writeln!(f, "    ClassIdentifier: {}", x)?;
            }
        }
        match self.nettype.clone() {
            None => {
                writeln!(f, "    NetType: None")?;
            }
            Some(x) => {
                writeln!(f, "    NetType: {:?}", x)?;
            }
        }
        match self.signedness.clone() {
            None => {
                writeln!(f, "    Signedness: None")?;
            }
            Some(x) => {
                writeln!(f, "    Signedness: {:?}", x)?;
            }
        }

        writeln!(f, "    PackedDimensions: {:?}", self.packed_dimensions)?;
        let mut unpackeddim_display: Vec<(String, String)> = Vec::new();

        for (u, l) in self.unpacked_dimensions.clone() {
            match l {
                Some(x) => unpackeddim_display.push((u.clone(), x.clone())),
                None => unpackeddim_display.push((u.clone(), String::from("None"))),
            }
        }
        writeln!(f, "    UnpackedDimensions: {:?}", unpackeddim_display)?;

        write!(f, "")
    }
}

impl fmt::Display for SvParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  Parameter: ")?;
        writeln!(f, "    Identifier: {}", self.identifier)?;
        writeln!(f, "    ParameterType: {:?}", self.paramtype)?;

        match self.datatype.clone() {
            None => {
                writeln!(f, "    DataType: None")?;
            }
            Some(x) => {
                writeln!(f, "    DataType: {:?}", x)?;
            }
        }
        match self.signedness.clone() {
            None => {
                writeln!(f, "    Signedness: None")?;
            }
            Some(x) => {
                writeln!(f, "    Signedness: {:?}", x)?;
            }
        }

        write!(f, "")
    }
}
