From test_script.py
- unused_var (global variable)
- unused_function (function)
- UnusedClass (class)
- unused_local (local variable inside used_function)
- unused_method_local (local variable inside UsedClass.method)
- math (import)
- js (alias import for json)

From test_notebook.ipynb
- unused_notebook_var (global variable in notebook)
- os_alias (import alias, imported but never used)

Notes
- used_local is used and should not be flagged
- used_function, UsedClass, UsedOnlyInNotebook and their usages should not be flagged
- operating_system (import alias in script) is used in notebook and should not be flagged