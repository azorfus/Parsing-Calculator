mod lexer;
mod parser;


#[macro_export]
macro_rules! read_input {
    () => {{
        use std::io::{stdin, stdout, Write};

        let mut input = String::new();
        print!(">> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()  // returns a String, not &str
    }};
}


fn main() {
    let mut expr_index = 1;
    let mut input = String::new();

    println!("Type \"quit\" to exit. You can enter multiple expressions in a single input. Ex: 3 + 3; 3 + 2; 3 + 1\n");

    loop {
        input = read_input!(); // take fresh input each time

        if input == "quit" {
            break;
        }

        let mut pos = 0; // reset for each new input

        while pos < input.len() {
            if input.chars().last() != Some(';') {
                input.push(';');
            }
            match parser::parse(&input, &mut pos) {
                Some(ast) => {
                    println!("\nExpression {} AST:", expr_index);
                    pretty_print(&ast, "", true);
                    expr_index += 1;
                    match eval(&ast) {
                        Some(result) => println!("\nResult: {}\n", result),
                        None => println!("\nEvaluation failed (e.g., division by zero)\n"),
                    }
                }
                None => {
                    println!("Failed to parse expression at position {}", pos);
                    break;
                }
            }
        }
        expr_index = 1;
    }
}

fn eval(node: &parser::ASTNode) -> Option<f64> {
    match node {
        parser::ASTNode::Number(n) => Some(*n),
        parser::ASTNode::Op { op, left, right } => {
            let left_val = eval(left)?;
            let right_val = eval(right)?;

            match op.as_str() {
                "+" => Some(left_val + right_val),
                "-" => Some(left_val - right_val),
                "*" => Some(left_val * right_val),
                "/" => {
                    if right_val != 0.0 {
                        Some(left_val / right_val)
                    } else {
                        None // prevent division by zero
                    }
                }
                _ => None,
            }
        }
    }
}

fn pretty_print(node: &parser::ASTNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    print!("{}", prefix);
    print!("{}", connector);

    match node {
        parser::ASTNode::Number(n) => {
            println!("Number({})", n);
        }
        parser::ASTNode::Op { op, left, right } => {
            println!("Operator('{}')", op);

            let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            pretty_print(left, &child_prefix, false);
            pretty_print(right, &child_prefix, true);
        }
    }
}
