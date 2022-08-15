use crate::structures::SvInstance;
use sv_parser::{unwrap_node, RefNode, SyntaxTree};
use crate::sv_misc::{identifier, get_string};

pub fn module_instance(
    p: &sv_parser::ModuleInstantiation,
    syntax_tree: &SyntaxTree,
) -> SvInstance {
    let ret = SvInstance {
        module_identifier: inst_module_identifier(p, syntax_tree),
        hierarchical_instance: inst_hierarchical_instance(p, syntax_tree),
        hierarchy: inst_hierarchy(p, syntax_tree),
        connections: inst_connections(p, syntax_tree),
    };

    return ret;
}

// Find module name for the instantiation (mother module)
fn inst_module_identifier(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> String {
    let mut ret: String = String::new();
    ret
}

// Find name of the instantiation (daughter module)
fn inst_hierarchical_instance(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> String {
    let mut ret: String = String::new();
    ret
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
