
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SvData {
    interfaces: Vec<SvInterfaceDeclaration>,
    modules: Vec<SvModuleDeclaration>,
    packages: Vec<SvPackageDeclaration>,
}

#[derive(Debug, Serialize)]
struct SvInterfaceDeclaration {
    parameters: Vec<SvParameter>,
    ports: Vec<SvPort>,
    signals: Vec<SvSignal>,
    modports: Vec<SvModport>,
}

#[derive(Debug, Serialize)]
struct SvModuleDeclaration {
    parameters: Vec<SvParameter>,
    ports: Vec<SvPort>,
    signals: Vec<SvSignal>,
}

#[derive(Debug, Serialize)]
struct SvPackageDeclaration {
    parameters: Vec<SvParameter>,
}

#[derive(Debug, Serialize)]
struct SvModport {
    foo: u8,
}

#[derive(Debug, Serialize)]
struct SvParameter {
    identifier: String,
    datatype: String,
    is_twostate: bool,
    is_signed: bool,
    is_packed: bool,
}

#[derive(Debug, Serialize)]
struct SvPort {
    foo: u8,
}

#[derive(Debug, Serialize)]
struct SvSignal {
    foo: u8,
}

