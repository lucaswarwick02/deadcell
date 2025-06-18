# deadcell

⚡️ **deadcell** is a fast, Rust-powered CLI tool for detecting dead Python code — unused variables, functions, classes, methods, and imports — across `.py` files and Jupyter notebooks.

It helps data scientists and developers clean up legacy codebases, reduce bloat, and spot forgotten code hiding in notebooks.


## Features

- 🚀 Blazing fast thanks to Tree-sitter
- 🧠 Understands `.py` and `.ipynb` files
- 📎 Tracks references across files and notebooks
- 📉 Reports unused:
  - Variables
  - Functions and methods
  - Classes
  - Imports


## Installation

You can install `deadcell` using `pip` (once published):

``pip install deadcell``

Or directly via Cargo:

``cargo install deadcell``


## Usage

Run `deadcell` on a folder:

``deadcell src/ notebooks/``

Specify output format:

``deadcell . --format json``

Ignore paths:

``deadcell . --ignore tests/ scratch/``

## Example Output (JSON)

``{"unused": [
  {
    "name": "unused_var",
    "type": "variable",
    "file": "analysis.ipynb",
    "cell": 2
  },
  {
    "name": "helper_fn",
    "type": "function",
    "file": "utils/helpers.py",
    "line": 45
  }
]}``


## Roadmap

- [x] Parse `.py` and `.ipynb`
- [x] Track symbols and references
- [x] Detect unused code
- [ ] Cross-module import resolution
- [ ] Config file support (`deadcell.toml`)
- [ ] Graph export of code relationships


## Contributing

Contributions welcome! Open an issue or submit a PR.

--

## License

MIT
