use crate::structures::SvInstance;
use sv_parser::{unwrap_node, RefNode, SyntaxTree};
use crate::sv_misc::{identifier, get_string};

pub fn module_instance(
    p: &sv_parser::ModuleInstantiation,
    syntax_tree: &SyntaxTree,
) -> SvInstance {
    let ret = SvInstance {
        module_name: inst_module_name(p, syntax_tree),
        instance_name: inst_instance_name(p, syntax_tree),
        hierarchy: inst_hierarchy(p, syntax_tree),
        connections: inst_connections(p, syntax_tree),
    };

    return ret;
}

// Find module name for the instantiation (mother module)
fn inst_module_name(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> String {
    let id = unwrap_node!(p, ModuleIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

// Find name of the instantiation (daughter module)
fn inst_instance_name(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> String {
    let id = unwrap_node!(p, InstanceIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

// Find hierarchy for the instantiation (only finds label for the time being)
fn inst_hierarchy(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    ret
}

// Finding connections for the instantiation
fn inst_connections(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();

    ret
}
