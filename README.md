# roslyn_python_core_parser
Roslyn like Python Core Parser written in Rust

This will be a library containing a fully working Python 3.13 parser and tokenizer that produces a syntax tree with tokens and trivia just like the Roslyn parser for C# from Microsoft. 
It is fully written in Rust and will be extended with utility functions to manipulate the abstract syntax tree. Enjoy.

## Building and run tests:

- ``` cargo build --release  ``` Building the release dll library
- ``` cargo test  ``` Building and execute all unit tests and integration tests.
-  ``` ls -la target/release ``` To view resulting files. On windows use *dir*


  
