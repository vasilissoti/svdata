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
    pub datatype: String,
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
    NA,
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub struct SvPort {
    pub identifier: String,
    pub direction: SvPortDirection,
    pub datakind: SvDataKind,
    pub datatype: SvDataType,
    pub nettype: SvNetType,
    pub signedness: SvSignedness,
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
        writeln!(f, "    NetType: {:?}", self.nettype)?;
        writeln!(f, "    Signedness: {:?}", self.signedness)
    }
}
