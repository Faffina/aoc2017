use std::{collections::VecDeque, fs::read_to_string};

pub fn first() {
    let mut p0 = Computer::new(0);
    let mut p1 = Computer::new(1);
    
    loop {
        p0.run(&mut p1.data_in);
        p1.run(&mut p0.data_in);

        if p0.halt && p1.halt {
            println!("{}", p1.send);
            break;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(usize),
    Value(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Send(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Recover(Operand),
    JumpGraterZero(Operand, Operand),
}

struct Computer {
    pc: i64,
    registers: [i64; 26],
    program: Vec<Instruction>,
    data_in: VecDeque<i64>,
    halt: bool,
    send: i64,
}

impl Operand {
    fn new(op: &str) -> Operand {
        if op.len() == 1 && ('a'..='z').contains(&op.chars().nth(0).unwrap()) {
            let register = (op.bytes().nth(0).unwrap() - b'a') as usize;
            Operand::Register(register)
        } else {
            let value = op.parse::<i64>().unwrap();
            Operand::Value(value)
        }
    }
}

impl Computer {
    fn value(&self, op: Operand) -> i64 {
        match op {
            Operand::Value(v) => v,
            Operand::Register(i) => self.registers[i],
        }
    }

    fn get_register(&mut self, op: Operand) -> &mut i64 {
        match op {
            Operand::Value(_) => panic!("cant set a value"),
            Operand::Register(i) => &mut self.registers[i],
        }
    }

    fn run(&mut self, data_out: &mut VecDeque<i64>) {
        if self.pc >= 0 && self.pc < self.program.len() as i64 {
            let cmd = self.program[self.pc as usize];
            match cmd {
                Instruction::Send(op) => {
                    data_out.push_back(self.value(op));
                    self.send += 1;
                }
                Instruction::Set(a, b) => *self.get_register(a) = self.value(b),
                Instruction::Add(a, b) => *self.get_register(a) += self.value(b),
                Instruction::Mul(a, b) => *self.get_register(a) *= self.value(b),
                Instruction::Mod(a, b) => *self.get_register(a) %= self.value(b),
                Instruction::Recover(a) => {
                    if let Some(value) = self.data_in.pop_front() {
                        *self.get_register(a) = value;
                        self.halt = false;
                    } else {
                        self.halt = true;
                        return;
                    }
                }
                Instruction::JumpGraterZero(a, b) => {
                    if self.value(a) > 0 {
                        self.pc += self.value(b);
                        return;
                    }
                }
            }
            self.pc += 1;
        } else {
            self.halt = true;
        }
    }

    fn new(id: i64) -> Computer {
        let v: Vec<Instruction> = read_to_string("data/18")
            .unwrap()
            .lines()
            .filter_map(|x| {
                let parts: Vec<_> = x.split_whitespace().collect();
                match parts.as_slice() {
                    ["snd", opa] => Some(Instruction::Send(Operand::new(opa))),
                    ["rcv", opa] => Some(Instruction::Recover(Operand::new(opa))),
                    ["set", opa, opb] => {
                        Some(Instruction::Set(Operand::new(opa), Operand::new(opb)))
                    }
                    ["add", opa, opb] => {
                        Some(Instruction::Add(Operand::new(opa), Operand::new(opb)))
                    }
                    ["mul", opa, opb] => {
                        Some(Instruction::Mul(Operand::new(opa), Operand::new(opb)))
                    }
                    ["mod", opa, opb] => {
                        Some(Instruction::Mod(Operand::new(opa), Operand::new(opb)))
                    }
                    ["jgz", opa, opb] => Some(Instruction::JumpGraterZero(
                        Operand::new(opa),
                        Operand::new(opb),
                    )),
                    _ => None,
                }
            })
            .collect();
        let mut pc = Computer {
            pc: 0,
            registers: [0i64; 26],
            data_in: VecDeque::new(),
            program: v,
            halt: false,
            send: 0,
        };
        *pc.get_register(Operand::new("p")) = id;
        pc
    }
}
