# REFACTOR

Right now, the Typst element syntax tree is converted into a KaTeX node syntax tree, all inside Rust. But, it's much better to represent Typst elements inside Typescript, to then build the KaTeX node tree in Javascript directly. This refactor plans to substitute the older logic by this one, which is easier to understand, and produces clearer code.
