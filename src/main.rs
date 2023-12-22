use std::collections::HashMap;
mod expr_element;
use expr_element::expr_element::ExprElement;
use expr_element::expr_element::choose_argument;
use expr_element::expr_element::choose_from_list_by_weight_table;
use expr_element::expr_element::choose_function;
use expr_element::expr_element::choose_start;
use rand::random;

fn develop_expression(
    el: &ExprElement,
    start_weight_table: &HashMap<ExprElement, usize>,
    arg_weight_table: &HashMap<ExprElement, usize>,
    func_weight_table: &HashMap<usize, usize>,
) -> Option<Vec<ExprElement>> {
    match *el {
        ExprElement::Operation(depth) => Some(vec![
            ExprElement::Argument(depth),
            ExprElement::Operator,
            ExprElement::Argument(depth),
        ]),
        ExprElement::Argument(depth) => Some(choose_argument(arg_weight_table, depth)),
        ExprElement::FunctionName(_) => None,
        ExprElement::Function(depth) => Some(choose_function(func_weight_table, depth)),
        ExprElement::Arguments(depth, size) => {
            let mut vector: Vec<ExprElement> = Vec::with_capacity(2 * size - 1);
            for _ in 1..size {
                vector.push(ExprElement::Argument(depth));
                vector.push(ExprElement::Comma);
            }
            vector.push(ExprElement::Argument(depth));
            Some(vector)
        }
        ExprElement::Parantheses(depth) => Some(vec![
            ExprElement::OpenParantheses,
            ExprElement::Operation(depth - 1),
            ExprElement::CloseParantheses,
        ]),
        ExprElement::FuncParantheses(depth, size) => Some(vec![
            ExprElement::OpenParantheses,
            ExprElement::Arguments(depth - 1, size),
            ExprElement::CloseParantheses,
        ]),
        ExprElement::Start(depth) => Some(choose_start(start_weight_table, depth)),
        _ => None,
    }
}

fn generate_expression(
    seed: ExprElement,
    start_weight_table: &HashMap<ExprElement, usize>,
    arg_weight_table: &HashMap<ExprElement, usize>,
    func_weight_table: &HashMap<usize, usize>,
    func_name_weight_table: &HashMap<&str, usize>,
    operator_weight_table: &HashMap<char, usize>,
) -> String {
    let mut vector: Vec<ExprElement> = vec![seed];
    let mut flag = true;
    while flag {
        flag = false;
        let mut new_vector: Vec<ExprElement> = Vec::new();
        for el in vector.iter() {
            let result = develop_expression(
                el,
                &start_weight_table,
                &arg_weight_table,
                &func_weight_table,
            );
            match result {
                Some(els) => {
                    for el in els.iter() {
                        new_vector.push(*el);
                    }
                    flag = true;
                }
                None => {
                    new_vector.push(*el);
                }
            }
        }
        vector = new_vector;
    }
    let mut expression_str = String::new();
    let number_limit = 100;
    for el in vector.iter() {
        match el {
            &ExprElement::FunctionName(args) => {
                if args > 1 {
                    expression_str.push_str("func");
                    expression_str.push_str(&args.to_string());
                }else{
                    let mut vec: Vec<&str> = Vec::new();
                    for (k, _) in func_name_weight_table {
                        vec.push(k);
                    }
                    let func_str = choose_from_list_by_weight_table(func_name_weight_table, vec.as_slice());
                    expression_str.push_str(func_str);
                }
            },
            &ExprElement::CloseParantheses => expression_str.push_str(")"),
            &ExprElement::Comma => expression_str.push_str(", "),
            &ExprElement::Number => expression_str.push_str(&(random::<usize>() % number_limit).to_string()),
            &ExprElement::OpenParantheses => expression_str.push_str("("),
            &ExprElement::Operator => {
                let mut buf = [0u8; 1];
                let mut vec: Vec<char> = Vec::new();
                for (k, _) in operator_weight_table.iter() {
                    vec.push(*k);
                }
                let ch = choose_from_list_by_weight_table(operator_weight_table, vec.as_slice());
                expression_str.push_str(" ");
                expression_str.push_str(ch.encode_utf8(&mut buf));
                expression_str.push_str(" ");
            },
            &ExprElement::Variable => expression_str.push_str("x"),
            _ => (),
        };
    }
    expression_str
}

fn main() {
    let start_weight_table = HashMap::from([
        (ExprElement::Function(0), 1),
        (ExprElement::Operation(0), 1),
    ]);
    let arg_weight_table: HashMap<ExprElement, usize> = HashMap::from([
        (ExprElement::Variable, 1),
        (ExprElement::Number, 1),
        (ExprElement::Function(0), 1),
        (ExprElement::Operation(0), 1),
        (ExprElement::Parantheses(0), 1),
    ]);
    let func_weight_table: HashMap<usize, usize> = HashMap::from([(1, 1)]);
    let func_name_weight_table: HashMap<&str, usize> = HashMap::from([
        ("sin", 1),
        ("cos", 1),
        ("tan", 1),
    ]);
    let operator_weight_table: HashMap<char, usize> = HashMap::from([
        ('+', 1),
        ('-', 1),
        ('/', 1),
        ('*', 1),
        ('^', 1),
    ]);

    let seed = ExprElement::Start(2);
    // for _ in 0..100 
    {
        let expression_str = generate_expression(
            seed,
            &start_weight_table,
            &arg_weight_table,
            &func_weight_table,
            &func_name_weight_table,
            &operator_weight_table,
        );
        println!("{}", expression_str);
    }
}
