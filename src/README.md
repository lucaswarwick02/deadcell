### 1. Add import statements as declarations
Parse both import_statement and import_from_statement nodes.

Extract each imported symbol or alias and add it to SymbolTable.

Track usages of those imported names in code.

### 2. Handle variable assignments at module level
Detect variables like unused_var, unused_imported_var assigned at module scope.

Add them as declared symbols.

Track their usages if any.

### 3. Distinguish between local variables and globals
Avoid tracking locals inside functions/methods as global symbols (unless you want to track local usage as well).

Only track function parameters, locals if you want fine-grained analysis.

### 4. Track class method declarations and usages properly
You seem to handle class methods, but verify method calls are correctly recognized as usages of method symbols.

### 5. Mark symbols used only in notebooks if relevant
If you have notebook support, mark those classes/functions used only in notebooks as special cases or track notebook files separately.

### 6. Improve usage detection granularity
Your current usage detection may be imprecise (e.g., a symbol appears “used” at its own declaration line). Refine detection so it excludes declaration line usage or clearly distinguishes declaration vs usage.

### 7. Fix symbol naming for aliases
For imports with aliases (e.g., import json as js), track the alias js as the symbol, not the original json.

### 8. Report unused symbols explicitly
After building symbol and usage tables, print only symbols with no usage locations besides declaration.