use std::{
    fs::read_to_string,
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread,
};

pub fn first() {
    let (main_tx, p_rx) = mpsc::channel::<i64>();
    let (p_tx, main_rx) = mpsc::channel::<i64>();
    let mut p = Computer::new(p_tx, p_rx);
    let pc_handle = thread::spawn(move || p.run());
    while !pc_handle.is_finished() {
        match main_rx.try_recv() {
            Ok(value) => {
                main_tx.send(value).unwrap();
            },
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => break,
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
    tx: Sender<i64>,
    rx: Receiver<i64>,
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

    fn run(&mut self) {
        while self.pc >= 0 && self.pc < self.program.len() as i64 {
            let cmd = self.program[self.pc as usize];
            match cmd {
                Instruction::Send(op) => self.tx.send(self.value(op)).unwrap(),
                Instruction::Set(a, b) => *self.get_register(a) = self.value(b),
                Instruction::Add(a, b) => *self.get_register(a) += self.value(b),
                Instruction::Mul(a, b) => *self.get_register(a) *= self.value(b),
                Instruction::Mod(a, b) => *self.get_register(a) %= self.value(b),
                Instruction::Recover(a) => {
                    *self.get_register(a) = self.rx.try_iter().last().unwrap();
                    println!("{}", self.value(a));
                    return;
                }
                Instruction::JumpGraterZero(a, b) => {
                    if self.value(a) > 0 {
                        self.pc += self.value(b);
                        continue;
                    }
                }
            }
            self.pc += 1;
        }
    }

    fn new(tx: Sender<i64>, rx: Receiver<i64>) -> Computer {
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
        Computer {
            pc: 0,
            registers: [0i64; 26],
            rx,
            tx,
            program: v,
        }
    }
}
