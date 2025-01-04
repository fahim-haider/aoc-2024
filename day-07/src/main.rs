use std::fs;
use std::ops::Mul;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>
}

#[derive(Clone, Debug)]
enum Operators {
    Add,
    Multiply,
    Concat
}

use Operators::*;

const OPERATIONS: [Operators; 3] = [Add,Multiply,Concat];

fn parse_eqns (file: &str) -> Vec<Equation>{
    let contents = fs::read_to_string(file).unwrap();
    let mut equations: Vec<Equation> = Vec::new();
    for line in contents.lines() {
        let mut iter = line.split(':');
        let result = iter.next().unwrap()
                .parse::<i64>().unwrap_or(0);
        let operands = iter.next().unwrap()
                .split_whitespace()
                .map(|opr| opr.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        equations.push(Equation {result, operands});
    }
    equations
}

// Creates all possible operator combinations from the parameter's operands field
fn create_combinations (combinations: &mut Vec<Operators>, 
                length: usize, 
                formulas: &mut Vec<Vec<Operators>>) {
    if combinations.len() == length {
        formulas.push(combinations.to_vec());
        return;
    }
    for c in OPERATIONS {
        combinations.push(c);
        create_combinations(combinations, length, formulas);
        combinations.pop();
    }
}

fn is_valid_formula (eqn: &Equation) -> bool {
    let target = eqn.result;
    let operands = &eqn.operands;
    let mut combinations: Vec<Operators> = Vec::new();
    let mut formulas: Vec<Vec<Operators>> = Vec::new();
    create_combinations(&mut combinations, operands.len()-1, &mut formulas);
    //println!("Possible formulas for {:?}: {:?}", target, formulas);
    let mut result = 0;
    for formula in formulas.clone() {
        result = operands[0];
        for i in 0..formula.len() {
            match formula[i] {
                Add => result += operands[i+1],
                Multiply => result = result * operands[i+1],
                Concat => {
                    let concatenated = format!("{}{}",result,operands[i+1]);
                    result = concatenated.parse::<i64>().unwrap();
                },
            };
        }
        //println!("Result for {:?}: {:?}", formula, result);
        if result == target {
            //println!("VALID WHEN {:?}", formula);
            return true;
        }
    }
    result == target
}

fn main() {
    let start_time = Instant::now();
    let test = parse_eqns("src/input.in");
    let sum: i64 = test.iter()
                            .filter(|eqn| is_valid_formula((&eqn).clone()))
                            .map(|e| e.result).sum();
    let duration = start_time.elapsed();
    println!("Part 2: {:?}", sum);
    println!("Time taken for Part 2 = {:?}", duration);
}
