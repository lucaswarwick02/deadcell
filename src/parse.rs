use tree_sitter::{Language, Parser, TreeCursor};

extern "C" { fn tree_sitter_python() -> Language; }

pub fn parse_python_source(source_code: &str) {
    let mut parser = Parser::new();
    parser.set_language(unsafe { tree_sitter_python() }).expect("Error loading Python grammar");

    if let Some(tree) = parser.parse(source_code, None) {
        let root = tree.root_node();
        let cursor = root.walk();

        walk_and_print(cursor, source_code);
    } else {
        println!("Failed to parse source.");
    }
}

fn walk_and_print(cursor: TreeCursor, source: &str) {
    visit_node(cursor.node(), source);
}

fn visit_node(node: tree_sitter::Node, source: &str) {
    if node.kind() == "function_definition"
        || node.kind() == "class_definition"
        || node.kind() == "import_statement"
        || node.kind() == "import_from_statement"
    {
        let text = node.utf8_text(source.as_bytes()).unwrap_or("<err>");
        let first_line = text.lines().next().unwrap_or(text);
        println!(
            "[{}] @ line {}: {}",
            node.kind(),
            node.start_position().row + 1,
            first_line
        );
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            visit_node(child, source);
        }
    }
}
