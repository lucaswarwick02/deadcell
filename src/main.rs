mod walk;
mod parse;

use clap::Parser;
use std::path::PathBuf;
use walk::visit_paths;

#[derive(Parser, Debug)]
#[command(name = "deadcell")]
#[command(about = "Dead code finder for Python projects")]
struct Cli {
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    for path in &cli.paths {
        visit_paths(path);
    }

    parse::parse_python_source(r#"
import os

class Thing:
    def __init__(self):
        self.x = 1

def func(): pass
"#);

}