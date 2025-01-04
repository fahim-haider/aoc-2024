use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>
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
                .parse::<u64>().unwrap_or(0);
        let operands = iter.next().unwrap()
                .split_whitespace()
                .map(|opr| opr.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
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
                Multiply => result *= operands[i+1],
                Concat => {
                    let b_digits: u32 = ((operands[i+1]+1) as f64).log10().ceil() as u32;
                    let power: u64 = 10_u64.pow(b_digits);
                    result = result*power + operands[i+1];
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

// TODO:
fn is_valid_formula_recursive (eqn: &Equation) -> bool {
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
                Multiply => result *= operands[i+1],
                Concat => {
                    let b_digits: u32 = ((operands[i+1]+1) as f64).log10().ceil() as u32;
                    let power: u64 = 10_u64.pow(b_digits);
                    result = result*power + operands[i+1];
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
    let sum: u64 = test.iter()
                            .filter(|&eqn| is_valid_formula(&eqn.clone()))
                            .map(|e| e.result).sum();
    let duration = start_time.elapsed();
    println!("Part 2: {:?}", sum);
    println!("Time taken for Part 2 = {:?}", duration);
}
/* 



RESULTS:
Part 1: 289.8ms (including input parsing!!)
Part 2: ~16s
*/