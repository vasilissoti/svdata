
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SvData {
    modules: Vec<SvModuleDeclaration>,
    packages: Vec<SvPackageDeclaration>,
}

#[derive(Debug, Serialize)]
pub struct SvModuleDeclaration {
    parameters: Vec<SvParameter>,
    ports: Vec<SvPort>,
}

#[derive(Debug, Serialize)]
pub struct SvPackageDeclaration {
    parameters: Vec<SvParameter>,
}

#[derive(Debug, Serialize)]
pub struct SvParameter {
    identifier: String,
    datatype: String,
}

#[derive(Debug, Serialize)]
pub enum SvPortDirection {
    Input,
    Output,
    Inout,
    Ref,
    Interface,
}

#[derive(Debug, Serialize)]
pub struct SvPort {
    identifier: String,
    datatype: String,
    direction: Option<SvPortDirection>,
}

