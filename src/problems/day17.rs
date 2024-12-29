#[derive(Debug, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug)]
enum Operand {
    Literal(u8),
    Combo(u8),
}

#[derive(Debug)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Opcode {
    fn from_u8(n: u8) -> Opcode {
        match n {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("Unknown opcode {n}"),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    registers: Registers,
    instruction_pointer: u64,
    program: Vec<u8>,
    output: Vec<u8>,
}

#[derive(PartialEq, Eq)]
enum Status {
    Finished,
    Ready,
}

impl State {
    fn run(&mut self) -> () {
        while let Status::Ready = self.tick() {}
    }

    fn clear(&mut self) -> () {
        self.registers = Registers { a: 0, b: 0, c: 0 };
        self.instruction_pointer = 0;
        self.output.clear();
    }
    fn tick(&mut self) -> Status {
        if self.instruction_pointer >= self.program.len() as u64 {
            return Status::Finished;
        }

        let opcode = Opcode::from_u8(self.program[self.instruction_pointer as usize]);
        let operand = self.program[self.instruction_pointer as usize + 1] as u8;

        self.instruction_pointer += 2;
        match opcode {
            Opcode::Adv => {
                let operand_value = self.get_value(Operand::Combo(operand));
                self.registers.a = self.registers.a / (2u64.pow(operand_value as u32));
            }
            Opcode::Bxl => {
                let operand_value = self.get_value(Operand::Literal(operand));
                self.registers.b = self.registers.b ^ (operand_value);
            }
            Opcode::Bst => {
                let operand_value = self.get_value(Operand::Combo(operand));
                self.registers.b = operand_value % 8;
            }
            Opcode::Jnz => {
                let operand_value = self.get_value(Operand::Literal(operand));
                if self.registers.a != 0 {
                    self.instruction_pointer = operand_value;
                }
            }
            Opcode::Bxc => {
                self.registers.b = self.registers.b ^ self.registers.c;
            }
            Opcode::Out => {
                let operand_value = self.get_value(Operand::Combo(operand));
                self.output.push((operand_value % 8) as u8);
            }
            Opcode::Bdv => {
                let operand_value = self.get_value(Operand::Combo(operand));
                self.registers.b = self.registers.a / (2u64.pow(operand_value as u32));
            }
            Opcode::Cdv => {
                let operand_value = self.get_value(Operand::Combo(operand));
                self.registers.c = self.registers.a / (2u64.pow(operand_value as u32));
            }
        }

        Status::Ready
    }

    fn get_value(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Literal(n) => n as u64,
            Operand::Combo(n) => {
                if n < 4 {
                    return n as u64;
                }

                if n == 4 {
                    self.registers.a
                } else if n == 5 {
                    self.registers.b
                } else if n == 6 {
                    self.registers.c
                } else if n == 7 {
                    panic!("Reserved {n}");
                } else {
                    panic!("Invalid operand {n}.");
                }
            }
        }
    }
}

fn parse(data: &str) -> State {
    let lines: Vec<&str> = data.split("\n").collect();
    assert!(lines.len() >= 5);

    let reg_a = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let reg_b = lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let reg_c = lines[2].split_once(": ").unwrap().1.parse().unwrap();
    let program = lines[4]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    State {
        registers: Registers {
            a: reg_a,
            b: reg_b,
            c: reg_c,
        },
        instruction_pointer: 0,
        program,
        output: Vec::new(),
    }
}

pub fn part1(data: &str) -> String {
    let mut state = parse(data);

    state.run();

    let result: String = state
        .output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",");

    result
}

fn solve(state: &mut State, a: u64, i: usize) -> Option<u64> {
    if i == state.program.len() {
        return Some(a);
    }

    for x in 0..8 {
        let a_tmp = x | (a << 3);
        state.clear();
        state.registers.a = a_tmp;
        state.run();

        if state.output.len() == i + 1
            && state.output[0] == state.program[state.program.len() - 1 - i]
        {
            if let Some(b) = solve(state, a_tmp, i + 1) {
                return Some(b)
            };
        }
    }

    None
}

pub fn part2(data: &str) -> u64 {
    let original_state = parse(data);
    let mut state = original_state.clone();

    let Some(a) = solve(&mut state, 0, 0) else {
        panic!("Failed to solve.");
    };

    // YESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part1(data), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_works() {
        let data = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part2(data), 117440);
    }
}
