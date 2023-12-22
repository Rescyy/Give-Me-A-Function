pub mod expr_element {

    use core::hash::Hasher;
    use core::hash::Hash;
    use std::marker::Copy;
    use std::fmt::Debug;
    use rand::random;
    use std::collections::HashMap;
    
    
    pub fn choose_from_list_by_weight_table<A>(weight_table: &HashMap<A, usize>, choices: &[A]) -> A
    where
        A: Eq,
        A: Hash,
        A: Copy,
        A: Debug,
    {
        let mut total_weights = 0;
        let mut vector: Vec<usize> = Vec::with_capacity(choices.len());
        vector.push(0);
        for choice in choices.iter() {
            let choice_weight = weight_table.get(choice);
            
            
            total_weights += match choice_weight {
                Some(val) => val,
                None => &0,
            };
            vector.push(total_weights);
        }
        
        let ran = random::<usize>() % total_weights;
        
        for i in 0..choices.len() {
            if ran >= vector[i] && ran < vector[i+1] {
                return choices[i].clone();
            }
        }
        
        todo!();
    }
    
    pub fn choose_argument(weight_table: &HashMap<ExprElement, usize>, depth: usize) -> Vec<ExprElement> {
        let argument_subjects = [
            ExprElement::Variable,
            ExprElement::Number,
            ExprElement::Function(depth),
            ExprElement::Operation(depth),
            ExprElement::Parantheses(depth),
        ];
        if depth > 0 {
            vec![choose_from_list_by_weight_table(
                weight_table,
                &argument_subjects,
            )]
        } else {
            vec![choose_from_list_by_weight_table(
                weight_table,
                &argument_subjects[0..2],
            )]
        }
    }
    
    pub fn choose_function(weight_table: &HashMap<usize, usize>, depth: usize) -> Vec<ExprElement> {
        let mut function_subjects: Vec<usize> = Vec::new();
        for (k, _) in weight_table.iter() {
            function_subjects.push(*k);
        }
        let choice = choose_from_list_by_weight_table(weight_table, function_subjects.as_slice());
        vec![
            ExprElement::FunctionName(choice),
            ExprElement::FuncParantheses(depth, choice),
        ]
    }
    
    pub fn choose_start(weight_table: &HashMap<ExprElement, usize>, depth: usize) -> Vec<ExprElement> {
        let start_subjects = [ExprElement::Operation(depth), ExprElement::Function(depth)];
        vec![choose_from_list_by_weight_table(
            weight_table,
            &start_subjects,
        )]
    }
    #[derive(Debug, Eq, Clone, Copy)]
    pub enum ExprElement {
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
        Start(usize),
    }
    
    impl ExprElement {
        fn get_index(&self) -> usize {
            match self {
                &ExprElement::FunctionName(_) => 0,
                &ExprElement::Variable => 1,
                &ExprElement::Number => 2,
                &ExprElement::OpenParantheses => 3,
                &ExprElement::CloseParantheses => 4,
                &ExprElement::Operation(_) => 5,
                &ExprElement::Operator => 6,
                &ExprElement::Argument(_) => 7,
                &ExprElement::Function(_) => 8,
                &ExprElement::Arguments(_, _) => 9,
                &ExprElement::Comma => 10,
                &ExprElement::Parantheses(_) => 11,
                &ExprElement::FuncParantheses(_, _) => 12,
                &ExprElement::Start(_) => 13,
            }
        }
    }
    
    impl PartialEq for ExprElement {
        fn eq(&self, other: &Self) -> bool {
            self.get_index() == other.get_index()
        }
    }
    
    impl Hash for ExprElement {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.get_index().hash(state);
        }
    }
}