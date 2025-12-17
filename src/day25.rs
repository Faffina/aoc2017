pub fn first() {
    let mut turing_machine: Turing<400_000> = Turing::new();

    for _ in 0..12386363{
        turing_machine.step();
    }

    println!("number of ones: {}", turing_machine.ones());
}

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Turing<const TAPE_SIZE: usize> {
    tape: [u64; TAPE_SIZE],
    positon: usize,
    state: State,
}

impl<const TAPE_SIZE: usize> Turing<TAPE_SIZE> {
    fn new() -> Self {
        let tape = [0u64; TAPE_SIZE];
        let positon = TAPE_SIZE * 64 / 2;
        let state = State::A;
        Self {
            tape,
            positon,
            state,
        }
    }

    fn set(&mut self) {
        assert!(self.positon < TAPE_SIZE * 64);
        let word = self.positon / 64;
        let bit = self.positon % 64;
        self.tape[word] |= 1u64 << bit;
    }

    fn clear(&mut self) {
        assert!(self.positon < TAPE_SIZE * 64);
        let word = self.positon / 64;
        let bit = self.positon % 64;
        self.tape[word] &= !(1u64 << bit);
    }

    fn is_set(&self) -> bool {
        assert!(self.positon < TAPE_SIZE * 64);
        let word = self.positon / 64;
        let bit = self.positon % 64;
        (self.tape[word] & (1u64 << bit)) > 0
    }
        
    fn ones(&self) -> usize {
        self.tape.iter().map(|x| x.count_ones() as usize).sum()
    }

    fn step(&mut self) {
        match self.state {
            State::A => {
                if self.is_set() {
                    self.clear();
                    self.positon -= 1;
                    self.state = State::E;
                } else {
                    self.set();
                    self.positon += 1;
                    self.state = State::B;
                }
            }

            State::B => {
                if self.is_set() {
                    self.clear();
                    self.positon += 1;
                    self.state = State::A;
                } else {
                    self.set();
                    self.positon -= 1;
                    self.state = State::C;
                }
            }

            State::C => {
                if self.is_set() {
                    self.clear();
                    self.positon += 1;
                    self.state = State::C;
                } else {
                    self.set();
                    self.positon -= 1;
                    self.state = State::D;
                }
            }

            State::D => {
                if self.is_set() {
                    self.clear();
                    self.positon -= 1;
                    self.state = State::F;
                } else {
                    self.set();
                    self.positon -= 1;
                    self.state = State::E;
                }
            }

            State::E => {
                if self.is_set() {
                    self.set();
                    self.positon -= 1;
                    self.state = State::C;
                } else {
                    self.set();
                    self.positon -= 1;
                    self.state = State::A;
                }
            }

            State::F => {
                if self.is_set() {
                    self.set();
                    self.positon += 1;
                    self.state = State::A;
                } else {
                    self.set();
                    self.positon -= 1;
                    self.state = State::E
                }
            }
        }
    }
}

