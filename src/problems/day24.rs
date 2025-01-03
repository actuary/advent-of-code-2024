use regex::Regex;
use std::collections::HashMap;

enum Operator {
    And,
    Or,
    Xor,
}

struct Equation {
    left: String,
    right: String,
    op: Operator,
    destination: String,
}

struct System {
    values: HashMap<String, i64>,
    equations: Vec<Equation>,
}

fn solve_reference<'a>(
    reference: &'a str,
    equations: &Vec<Equation>,
    values: &mut HashMap<String, i64>,
) -> Option<i64> {
    if values.contains_key(reference) {
        return Some(*values.get(reference).unwrap());
    }

    for eq in equations {
        if eq.destination == reference {
            let Some(left_value) = solve_reference(eq.left.as_str(), equations, values) else {
                panic!("Failed to solve {}", eq.left);
            };

            let Some(right_value) = solve_reference(eq.right.as_str(), equations, values) else {
                panic!("Failed to solve {}", eq.right);
            };

            let result = match eq.op {
                Operator::And => Some(left_value & right_value),
                Operator::Or => Some(left_value | right_value),
                Operator::Xor => Some(left_value ^ right_value),
            };

            values.insert(reference.to_string(), result.unwrap());
            return result;
        }
    }

    None
}

fn solve_reference_x_y<'a>(
    reference: &'a str,
    equations: &Vec<Equation>,
    values: &mut HashMap<String, i64>,
) -> Option<String> {
    if values.contains_key(reference) {
        return Some(reference.to_string());
    }

    for eq in equations {
        if eq.destination == reference {
            let Some(left_value) = solve_reference_x_y(eq.left.as_str(), equations, values) else {
                panic!("Failed to solve {}", eq.left);
            };

            let Some(right_value) = solve_reference_x_y(eq.right.as_str(), equations, values)
            else {
                panic!("Failed to solve {}", eq.right);
            };

            let result = match eq.op {
                Operator::And => format!("({left_value} & {right_value})"),
                Operator::Or => format!("({left_value} | {right_value})"),
                Operator::Xor => format!("({left_value} ^ {right_value})"),
            };

            return Some(result);
        }
    }

    None
}

fn parse(data: &str) -> System {
    let re =
        Regex::new(r"(?P<left>\w+) (?P<op>XOR|OR|AND) (?P<right>\w+) -> (?P<result>\w+)").unwrap();

    let (initial_values, equations) = data.trim().split_once("\n\n").unwrap();

    let initial_values: HashMap<String, i64> = initial_values
        .split("\n")
        .map(|line| {
            let (reference, value) = line.split_once(": ").unwrap();
            (reference.to_string(), value.parse().unwrap())
        })
        .collect();

    let equations: Vec<Equation> = equations
        .split("\n")
        .map(|line| {
            let Some(caps) = re.captures(line) else {
                panic!("malformed equation {line}");
            };

            Equation {
                left: caps["left"].to_string(),
                right: caps["right"].to_string(),
                op: match &caps["op"] {
                    "AND" => Operator::And,
                    "OR" => Operator::Or,
                    "XOR" => Operator::Xor,
                    _ => panic!("malformed operator {}", &caps["op"]),
                },
                destination: caps["result"].to_string(),
            }
        })
        .collect();

    System {
        values: initial_values,
        equations,
    }
}

fn calculate_bits(starts_with: &str, values: &HashMap<String, i64>) -> i64 {
    let mut number: i64 = 0;
    for (destination, result) in values {
        if destination.starts_with(starts_with) {
            let shift: i64 = destination[1..].parse().unwrap();
            number += result << shift;
        }
    }

    number
}

pub fn part1(data: &str) -> i64 {
    let mut system = parse(data);

    for equation in &system.equations {
        solve_reference(
            &equation.destination.as_str(),
            &system.equations,
            &mut system.values,
        );
    }

    let x = calculate_bits("x", &system.values);
    let y = calculate_bits("y", &system.values);
    let z = calculate_bits("z", &system.values);

    println!("{x} + {y} = {z} ({})", x + y);

    z
}

pub fn part2(data: &str) -> &str {
    // solve this by parsing the equations as x and ys only, 
    // and then looking for the ones that didn't fit the pattern.
    let mut system = parse(data);

    let mut results: Vec<(String, String)> = system
        .equations
        .iter()
        .filter(|eq| eq.destination.starts_with('z'))
        .map(|eq| {
            let Some(x) = solve_reference_x_y(
                &eq.destination.as_str(),
                &system.equations,
                &mut system.values,
            ) else {
                panic!("Bad shit");
            };

            (eq.destination.to_string(), x)
        })
        .collect();

    results.sort_by_key(|k| k.0.to_string());

    for (dest, result) in results {
        // check these equations to find the oddities
        println!("{}: {}", dest, result[1..result.len()-1].to_string());
    }

    for equation in &system.equations {
        solve_reference(
            &equation.destination.as_str(),
            &system.equations,
            &mut system.values,
        );
    }

    let x = calculate_bits("x", &system.values);
    let y = calculate_bits("y", &system.values);
    let z = calculate_bits("z", &system.values);

    println!("{x} + {y} = {z} ({})", x + y); // with adjusted file, ties back
    "cpm,ghp,gpr,krs,nks,z10,z21,z33"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!(part1(data), 4);
    }

    #[test]
    fn part1_works_larger_example() {
        let data = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!(part1(data), 2024);
    }

    #[test]
    fn part2_works() {
        let data = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
        assert_eq!(part2(data), "z00,z01,z02,z05");
    }
}
