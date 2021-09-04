fn main() {
    use rustpython_parser::parser;

    let python_source = std::fs::read_to_string("example.py").unwrap();
    let python_ast = parser::parse_program(&python_source).unwrap();
    // println!("{:#?}", python_ast);
    for (i, statement) in python_ast.statements.iter().enumerate() {
        println!("statemenet {}", i);
        println!("{:#?}", statement.node);
    }
}
