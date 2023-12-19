use core::hash::Hash;
use rand::random;
use std::collections::HashMap;
use std::marker::Copy;

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

#[derive(Debug)]
struct Production<'a> {
    expr: ExprElement,
    block: &'a [ExprElement],
}

fn choose_from_list_by_weight_table<A>(weight_table: HashMap<A, usize>, choices: &[A]) -> A
where
    A: Eq,
    A: Hash,
    A: Copy,
{
    let mut total_weights = 0;
    let mut vector: Vec<usize> = Vec::with_capacity(choices.len());
    for choice in choices {
        vector.push(total_weights);
        total_weights += weight_table.get(choice).unwrap();
    }

    let ran = random::<usize>() % total_weights;

    for i in 0..choices.len() {
        if ran > vector[i] {
            return choices[i].clone();
        }
    }

    todo!();
}

fn choose_argument(weight_table: HashMap<ExprElement, usize>, depth: usize) -> Vec<ExprElement> {
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

fn choose_function(weight_table: HashMap<usize, usize>, depth: usize) -> Vec<ExprElement> {
    let function_subjects: [usize; 2] = [1, 2];
    let choice = choose_from_list_by_weight_table(weight_table, &function_subjects);
    vec![
        ExprElement::FunctionName(choice),
        ExprElement::FuncParantheses(depth, choice),
    ]
}

fn develop_expression(
    el: &ExprElement,
    arg_weight_table: HashMap<ExprElement, usize>,
    func_weight_table: HashMap<usize, usize>,
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
            Some(vector)
        }
        ExprElement::Parantheses(depth) => Some(vec![
            ExprElement::OpenParantheses,
            ExprElement::Argument(depth + 1),
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
    
}
