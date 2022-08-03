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

    for node in syntax_tree {
        match node {
            RefNode::GenerateBlock(x) => {
                for instance in x {
                    match instance {
                        RefNode::ModuleInstantiation(y) => {
                            if y == p {
                                let label = unwrap_node!(node.clone(), GenerateBlockIdentifier).unwrap();
                                let label = identifier(label, &syntax_tree).unwrap();
                                ret.push(label);
                            }
                        }
                        _ => (),
                }
                }
            }
            _ => (),
        }
    }

    ret
}

// Finding connections for the instantiation
fn inst_connections(p: &sv_parser::ModuleInstantiation, syntax_tree: &SyntaxTree) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();

    for node in p {
        match node {
            RefNode::NamedPortConnection(x) => {            // Port connection by name
                // Connection in mother module
                let left = unwrap_node!(node.clone(), PortIdentifier).unwrap();
                let left = identifier(left, &syntax_tree).unwrap();
                // Connection in daughter module
                let right_node = unwrap_node!(node.clone(), HierarchicalIdentifier).unwrap();
                let right_name = identifier(right_node, &syntax_tree).unwrap();
                let mut right_index = String::new();
                for select_node in x {
                    match select_node {
                        RefNode::Select(y) => {
                            for expression_node in y {
                                match expression_node {
                                    RefNode::HierarchicalIdentifier(_) => { // Indexing a variabel
                                        let right_node = unwrap_node!(expression_node.clone(), Identifier).unwrap();
                                        right_index = identifier(right_node, &syntax_tree).unwrap(); 
                                    },
                                    RefNode::IntegralNumber(_) => { // Indexing a number
                                        let right_node = unwrap_node!(select_node.clone(), DecimalNumber).unwrap();
                                        right_index = get_string(right_node, &syntax_tree).unwrap();                   
                                    },
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                // Push connection to ret
                if right_index == "" {      // If no indexing
                    ret.push([left, right_name].to_vec());
                }
                else {                      // If there is indexing
                    let right = format!("{}[{}]", right_name, right_index);
                    ret.push([left, right].to_vec());
                }
            }
            RefNode::OrderedPortConnection(x) => {          // Port connection by order
                let right_node = unwrap_node!(node.clone(), HierarchicalIdentifier).unwrap();
                let right_name = identifier(right_node, &syntax_tree).unwrap();
                let mut right_index = String::new();
                for select_node in x {
                    match select_node {
                        RefNode::Select(y) => {
                            for expression_node in y {
                                match expression_node {
                                    RefNode::HierarchicalIdentifier(_) => { // Indexing a variabel
                                        let right_node = unwrap_node!(expression_node.clone(), Identifier).unwrap();
                                        right_index = identifier(right_node, &syntax_tree).unwrap(); 
                                    },
                                    RefNode::IntegralNumber(_) => { // Indexng a number
                                        let right_node = unwrap_node!(expression_node.clone(), DecimalNumber).unwrap();
                                        right_index = get_string(right_node, &syntax_tree).unwrap();                   
                                    },
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                // Push connection to ret
                if right_index == "" {      // If no indexing
                    ret.push([right_name].to_vec());
                }
                else {                      // If there is indexing
                    let right = format!("{}[{}]", right_name, right_index);
                    ret.push([right].to_vec());
                }
            }
            _ => (),
        }
    }

    ret
}
