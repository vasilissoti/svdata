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
    pub declaration_type: SvModuleDeclarationType,
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
pub enum SvModuleDeclarationType {
    Ansi,
    NonAnsi,
}

// "IMPLICIT" is only used for NON-ANSI since in ANSI it will either be explicit or the default (and for both we would be able to immediately know the explicit category)
// In case of an NON-ANSI declaration then IMPLICIT means default except if it is explicitly defined through an internal data object later in the script (default is replaced by explicit)  
// "IMPLICIT" should never be left in the end of a full parse (1st phase) - For Non-Ansi models and during phase 1 an "IMPLICIT handler function will be responsible for placing the default
// entries based on what is left IMPLICIT and what is not

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
    Array, // 
    Enum, // Class?
    Struct, //Class?
    Union, // Class?
    Class,
    TypeRef, // VNotes: That means whatever the datatype of reference is
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
    NA, // In not SVDataKind != Net
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub enum SvSignedness {
    Signed,
    Unsigned,
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub struct SvUnpackedDimensions {
    pub dimensions: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SvPackedDimensions {
    pub dimensions: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
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

// VNotes: Packages, and Port_Parameters are not yet supported

impl fmt::Display for SvData{
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        for module in self.modules.clone(){
            write!(f, "{}", module)?;
        }

        write!(f, "")
    }
}

impl fmt::Display for SvModuleDeclaration{
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        
        writeln!(f, "Module:")?; // VNotes: In the future that will be implemented within the display of SvData (similar to SvPort and "Port")
        writeln!(f, "  Identifier: {}", self.identifier)?;
        writeln!(f, "  Type: {:?}", self.declaration_type)?;
        writeln!(f, "  Filepath: {}", self.filepath)?;

        for port in self.ports.clone(){
            write!(f, "{}", port)?;
        }

        writeln!(f, "")
    }
}

impl fmt::Display for SvPort{
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        
        writeln!(f, "  Port: ")?;
        writeln!(f, "    Identifier: {}", self.identifier)?;
        writeln!(f, "    Direction: {:?}", self.direction)?;
        writeln!(f, "    Expression: {}", self.port_expression)?;
        writeln!(f, "    DataKind: {:?}", self.datakind)?;
        writeln!(f, "    DataType: {:?}", self.datatype)?;
        writeln!(f, "    NetType: {:?}", self.nettype)?;
        writeln!(f, "    Signedness: {:?}", self.signedness)?;
        writeln!(f, "    Unpacked Dim: {}", self.unpacked_dim)?;
        writeln!(f, "    Packed Dim: {}", self.packed_dim) 

    }
}

impl fmt::Display for SvPackedDimensions{
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {

        for dim in self.dimensions.clone(){
            write!(f, "{}", dim)?;
        }

        write!(f, "")
    }
}

impl fmt::Display for SvUnpackedDimensions{
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {

        for dim in self.dimensions.clone(){
            write!(f, "{}", dim)?;
        }

        write!(f, "")
    }
}

