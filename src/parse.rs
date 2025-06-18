use tree_sitter::{Language, Parser, Node};
use crate::symbols::{SymbolTable, UsageTable};

extern "C" { fn tree_sitter_python() -> Language; }

pub fn parse_python_source(
    source_code: &str,
    filename: &str,
    symbols: &mut SymbolTable,
    usages: &mut UsageTable,
) {
    let mut parser = Parser::new();
    parser.set_language(unsafe { tree_sitter_python() }).expect("Failed to load Python grammar");

    if let Some(tree) = parser.parse(source_code, None) {
        let root = tree.root_node();
        visit_node(root, source_code, filename, symbols, usages);
    }
}

fn visit_node(
    node: Node,
    source: &str,
    filename: &str,
    symbols: &mut SymbolTable,
    usages: &mut UsageTable,
) {
    let kind = node.kind();

    match kind {
        "function_definition" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                let line = name_node.start_position().row + 1;
                symbols.add_declaration(&name, (filename.to_string(), line));
            }
        }
        "class_definition" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                let line = name_node.start_position().row + 1;
                symbols.add_declaration(&name, (filename.to_string(), line));
            }
        }
        "import_statement" | "import_from_statement" => {
            // Extract imported names (simplified)
            for child in node.named_children(&mut node.walk()) {
                if child.kind() == "identifier" {
                    let name = child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                    let line = child.start_position().row + 1;
                    symbols.add_declaration(&name, (filename.to_string(), line));
                }
            }
        }
        "identifier" => {
            // Consider this a usage of a symbol
            let name = node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            let line = node.start_position().row + 1;
            usages.add_usage(&name, (filename.to_string(), line));
        }
        _ => {}
    }

    for child in node.named_children(&mut node.walk()) {
        visit_node(child, source, filename, symbols, usages);
    }
}
