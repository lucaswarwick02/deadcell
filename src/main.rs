mod parse;
mod symbols;
mod walk;

use std::env;
use std::path::PathBuf;
use crate::symbols::{SymbolTable, UsageTable};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: deadcell <path1> [<path2> ...]");
        std::process::exit(1);
    }

    let mut symbols = SymbolTable::new();
    let mut usages = UsageTable::new();

    for arg in args {
        let root_path = PathBuf::from(arg);
        for file_path in walk::collect_files(&root_path) {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                parse::parse_python_source(&content, &file_path.to_string_lossy(), &mut symbols, &mut usages);
            }
        }
    }

    for (sym, decls) in symbols.all_symbols() {
        if let Some(uses) = usages.get_usages(sym) {
            println!("Symbol `{}` declared at {:?} is used at {:?}", sym, decls, uses);
        } else {
            println!("Unused symbol `{}` declared at {:?}", sym, decls);
        }
    }
}
