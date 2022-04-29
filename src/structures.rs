use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SvData {
    pub modules: Vec<SvModuleDeclaration>,
    pub packages: Vec<SvPackageDeclaration>,
}

#[derive(Debug, Serialize)]
pub struct SvModuleDeclaration {
    pub parameters: Vec<SvParameter>,
    pub ports: Vec<SvPort>,
    pub filepath: String,
    pub declaration_type: String, // "ANSI"/"NONANSI"
}

#[derive(Debug, Serialize)]
pub struct SvPackageDeclaration {
    pub parameters: Vec<SvParameter>,
}

#[derive(Debug, Serialize)]
pub struct SvParameter {
    pub identifier: String,
    pub datatype: String,
}

// "IMPLICIT" is only used for NON-ANSI since in ANSI it will either be explicit or the default (and for both we would be able to immediately know the explicit category)
// In case of an NON-ANSI declaration then IMPLICIT means default except if it is explicitly defined through an internal data object later in the script (default is replaced by explicit)  
// "IMPLICIT" should never be left in the end of a full parse (1st phase) - For Non-Ansi models and during phase 1 an "IMPLICIT handler function will be responsible for placing the default
// entries based on what is left IMPLICIT and what is not

#[derive(Debug, Serialize)]
pub enum SvPortDirection {
    Inout,
    Input,
    Output,
    Ref,
    IMPLICIT,
}

#[derive(Debug, Serialize)]
pub enum SvDataKind {
    Net,
    Variable,
    IMPLICIT,
}

#[derive(Debug, Serialize)]
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
    Class,
    IMPLICIT,
}

#[derive(Debug, Serialize)]
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
    NA, // In not SVDataKind != Net
    IMPLICIT,
}

#[derive(Debug, Serialize)]
pub enum SvSignedness {
    Signed,
    Unsigned,
    IMPLICIT,
}

#[derive(Debug, Serialize)]
pub struct SvUnpackedDimensions {
    pub dimensions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SvPackedDimensions {
    pub dimensions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SvPort {
    pub identifier: String,
    pub direction: SvPortDirection,
    pub port_expression: String, // Identifier of the internal object connected to the port (allows e.g: .a(i))
    pub datakind: SvDataKind,
    pub datatype: SvDataType,
    pub nettype: SvNetType,
    pub signedness: SvSignedness,
    pub unpacked_dim: SvUnpackedDimensions,
    pub packed_dim: SvPackedDimensions,
}
