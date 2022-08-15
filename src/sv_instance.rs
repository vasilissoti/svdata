use crate::structures::SvInstance;
use sv_parser::SyntaxTree;

pub fn module_instance(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> SvInstance {
    let ret = SvInstance {
        module_identifier: inst_module_identifier(p, syntax_tree),
        hierarchical_instance: inst_hierarchical_instance(p, syntax_tree),
        hierarchy: inst_hierarchy(p, syntax_tree),
        connections: inst_connections(p, syntax_tree),
    };

    ret
}

// Find module identifier for the instantiation (parent module)
fn inst_module_identifier(
    _p: &sv_parser::ModuleInstantiation,
    _syntax_tree: &SyntaxTree,
) -> String {
    String::new()
}

// Find hierarchical instance for the instantiation (child module)
fn inst_hierarchical_instance(
    _p: &sv_parser::ModuleInstantiation,
    _syntax_tree: &SyntaxTree,
) -> String {
    String::new()
}

// Find hierarchy for the instantiation
fn inst_hierarchy(_p: &sv_parser::ModuleInstantiation, _syntax_tree: &SyntaxTree) -> Vec<String> {
    Vec::new()
}

// Finding connections for the instantiation
fn inst_connections(
    _p: &sv_parser::ModuleInstantiation,
    _syntax_tree: &SyntaxTree,
) -> Vec<Vec<String>> {
    Vec::new()
}
