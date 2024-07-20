# REFACTOR

Right now, the Typst element syntax tree is converted into a KaTeX node syntax tree, all inside Rust. But, it's much better to represent Typst elements inside Typescript, to then build the KaTeX node tree in Javascript directly. This refactor plans to substitute the older logic by this one, which is easier to understand, and produces clearer code.

Also, the functionalities to be implemented:

- [ ] Functional minimal demo to render `x + y = z`, in this refactored form.
- [ ] Package download.
