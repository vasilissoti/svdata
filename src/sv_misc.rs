use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub fn identifier(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = match unwrap_node!(parent, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    };

    match id {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

pub fn keyword(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let kwd = match unwrap_node!(parent, Keyword) {
        Some(RefNode::Keyword(x)) => Some(x.nodes.0),

        _ => None,
    };

    match kwd {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

pub fn number(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let nu = match unwrap_node!(parent, UnsignedNumber) {
        Some(RefNode::UnsignedNumber(x)) => Some(x.nodes.0),

        _ => None,
    };

    match nu {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

pub fn symbol(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let symbol = match unwrap_node!(parent, Symbol) {
        Some(RefNode::Symbol(x)) => Some(x.nodes.0),

        _ => None,
    };

    match symbol {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}
