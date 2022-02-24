
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

#[derive(Debug, Serialize)]
pub enum SvPortDirection {
    Inout,
    Input,
    Output,
    Ref,
    IMPLICIT,
}

#[derive(Debug, Serialize)]
pub enum SvPortDatakind {
    Net,
    Variable,
    IMPLICIT,
}

#[derive(Debug, Serialize)]
pub struct SvPort {
    pub identifier: String,
    pub direction: SvPortDirection,
    pub datakind: SvPortDatakind,
    pub datatype: String,
}

