use calculator::expression::Expression;
use std::io;

fn main() {
    loop {
        let mut string_expression = String::new();
        io::stdin()
            .read_line(&mut string_expression)
            .expect("Failed to read line");
        let str_expression = string_expression.trim_end_matches(is_newline);
        let mut expr = Expression::new(&str_expression);
        println!("{} With tokens: {:?}", expr.tokenise(), expr.infix_token);
        println!("{} With tokens: {:?}", expr.postfix(), expr.postfix_token);
    }
}

fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}
