use core::hash::Hash;
use rand::random;
use std::collections::HashMap;
use std::marker::Copy;
use std::fmt::Debug;
use dyn_fmt::AsStrFormatExt;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum ExprElement {
    FunctionName(usize),
    Variable,
    Number,
    OpenParantheses,
    CloseParantheses,
    Operation(usize),
    Operator,
    Argument(usize),
    Function(usize),
    Arguments(usize, usize),
    Comma,
    Parantheses(usize),
    FuncParantheses(usize, usize),
}

fn choose_from_list_by_weight_table<A>(weight_table: &HashMap<A, usize>, choices: &[A]) -> A
where
    A: Eq,
    A: Hash,
    A: Copy,
    A: Debug,
{
    let mut total_weights = 0;
    let mut vector: Vec<usize> = Vec::with_capacity(choices.len());
    for choice in choices.iter() {
        vector.push(total_weights);
        total_weights += match weight_table.get(choice) {
            Some(val) => val,
            None => &0,
        };
    }

    let ran = random::<usize>() % total_weights;

    for i in 0..choices.len() {
        if ran <= vector[i] {
            return choices[i].clone();
        }
    }

    todo!();
}

fn choose_argument(weight_table: &HashMap<ExprElement, usize>, depth: usize) -> Vec<ExprElement> {
    let argument_subjects: [ExprElement; 5] = [
        ExprElement::Variable,
        ExprElement::Number,
        ExprElement::Function(depth),
        ExprElement::Operation(depth),
        ExprElement::Parantheses(depth),
    ];
    vec![choose_from_list_by_weight_table(
        weight_table,
        &argument_subjects,
    )]
}

fn choose_function(weight_table: &HashMap<usize, usize>, depth: usize) -> Vec<ExprElement> {
    let function_subjects: [usize; 2] = [1, 2];
    let choice = choose_from_list_by_weight_table(weight_table, &function_subjects);
    vec![
        ExprElement::FunctionName(choice),
        ExprElement::FuncParantheses(depth, choice),
    ]
}

fn develop_expression(
    el: &ExprElement,
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
            ExprElement::Operation(depth + 1),
            ExprElement::CloseParantheses,
        ]),
        ExprElement::FuncParantheses(depth, size) => Some(vec![
            ExprElement::OpenParantheses,
            ExprElement::Arguments(depth + 1, size),
            ExprElement::CloseParantheses,
        ]),
        _ => None,
    }
}

fn main() {
    let mut vector: Vec<ExprElement> = vec![ExprElement::Argument(0)];
    let mut flag = true;
    let arg_weight_table: HashMap<ExprElement, usize> = HashMap::from([
        (ExprElement::Variable, 1),
        (ExprElement::Number, 1),
        (ExprElement::Function(0), 1),
        (ExprElement::Operation(0), 1),
        (ExprElement::Parantheses(0), 1),
    ]);
    let func_weight_table: HashMap<usize, usize> = HashMap::from([
        (1, 2),
        (2, 1),
    ]);
    while flag {
        flag = false;
        let mut new_vector: Vec<ExprElement> = Vec::new();
        for el in vector.iter() {
            let result = develop_expression(el, &arg_weight_table, &func_weight_table);
            match result {
                Some(els) => {
                    for el in els.iter() {
                        new_vector.push(*el);
                    }
                    flag = true;
                },
                None => {
                    new_vector.push(*el);
                },
            }
        }
        vector = new_vector;
    }
    let mut expression_str = String::new();
    for el in vector.iter() {
        expression_str.push_str(
            match el {
                &ExprElement::FunctionName(args) => {
                    let str = String::from("func") + &args.to_string();
                    str.as_str()
                },
                &ExprElement::CloseParantheses => ")",
                &ExprElement::Comma => ",",
                &ExprElement::Number => "1",
                &ExprElement::OpenParantheses => "(",
                &ExprElement::Operator => "+",
                &ExprElement::Variable => "x",
                _ => "",
            }
        )
    }
}
