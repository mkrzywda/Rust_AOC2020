use std::fs::{read_to_string};

fn load_input() -> Vec<String> {
    let contents = read_to_string("in/day18").expect("could not read input file");
    contents
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

fn solve(input: &[String], splitter: SplitterFn) -> i64 {
    input
        .iter()
        .map(|line| Expression::parse(line, splitter).evaluate())
        .sum()
}

pub fn day18() {
    let input = load_input();

    let p1 = solve(&input, split_equal_precedence);
    let p2 = solve(&input, split_multiply_first);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(Debug)]
enum Expression {
    Value(i64),
    SubExpr(Box<Expression>, Operator, Box<Expression>),
}

type SplitterFn = fn(&str) -> Option<(&str, &str, &str)>;

impl Expression {
    fn parse(line: &str, splitter: SplitterFn) -> Self {
        if let Some(parts) = splitter(line) {
            let left = Self::parse(parts.0, splitter);
            let op = Operator::parse(parts.1);
            let right = Self::parse(parts.2, splitter);

            return Self::SubExpr(Box::new(left), op, Box::new(right));
        }

        if line.starts_with('(') && line.ends_with(')') {
            return Self::parse(&line[1..(line.len() - 1)], splitter);
        }

        Self::Value(line.trim().parse().unwrap())
    }

    fn evaluate(&self) -> i64 {
        match self {
            &Self::Value(val) => val,
            Self::SubExpr(left, op, right) => match op {
                Operator::Add => left.evaluate() + right.evaluate(),
                Operator::Multiply => left.evaluate() * right.evaluate(),
            },
        }
    }
}

fn split_at_operator<'a, 'b>(
    line: &'a str,
    operators: &'b [char],
) -> Option<(&'a str, &'a str, &'a str)> {
    let mut depth = 0;

    for (i, c) in line.char_indices().rev() {
        match c {
            '(' => {
                depth -= 1;
            }
            ')' => {
                depth += 1;
            }
            _ if operators.iter().any(|&op| op == c) => {
                if depth == 0 {
                    if i == 0 {
                        panic!("cannot parse {}", i);
                    }
                    return Some((&line[0..(i - 1)], &line[i..(i + 1)], &line[(i + 2)..]));
                }
            }
            _ => {}
        };
    }

    None
}

fn split_equal_precedence(line: &str) -> Option<(&str, &str, &str)> {
    split_at_operator(line, &['+', '*'])
}

fn split_multiply_first(line: &str) -> Option<(&str, &str, &str)> {
    if let Some(parts) = split_at_operator(line, &['*']) {
        return Some(parts);
    }

    split_at_operator(line, &['+'])
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(raw_op: &str) -> Self {
        let op = raw_op.trim();
        if op.len() != 1 {
            panic!("cannot parse {} as an operator", op);
        }

        match op.chars().next().unwrap() {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => panic!("cannot parse {} as an operator", op),
        }
    }
}
