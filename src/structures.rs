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
pub enum SvPortDatakind {
    Net,
    Variable,
    IMPLICIT,
}

#[derive(Debug, Serialize, Clone)]
pub struct SvPort {
    pub identifier: String,
    pub direction: SvPortDirection,
    pub datakind: SvPortDatakind,
    pub datatype: String,
}

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
        writeln!(f, "    DataKind: {:?}", self.datakind)?;
        writeln!(f, "    DataType: {:?}", self.datatype) 

    }
}
