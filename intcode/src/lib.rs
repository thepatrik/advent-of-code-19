use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Intcode {
    mem: HashMap<u64, i64>,
    pointer: i64,
    pub input: VecDeque<i64>,
    pub output: i64,
    pub halted: bool,
    pub rel_base: i64,
}

impl Intcode {
    pub fn init(&mut self, input: &str, init_input: i64) {
        self.load(input);
        self.write_to_buff(init_input);
    }

    fn get_2_params(&self, loc: i64) -> (i64, i64) {
        (self.read(loc + 1), self.read(loc + 2))
    }

    fn get_3_params(&self, loc: i64) -> (i64, i64, i64) {
        (self.read(loc + 1), self.read(loc + 2), self.read(loc + 3))
    }

    pub fn read(&self, loc: i64) -> i64 {
        match self.mem.get(&(loc as u64)) {
            Some(v) => *v,
            None => 0,
        }
        // *self.mem.get(&(loc as u64)).unwrap()
    }

    pub fn write(&mut self, loc: i64, val: i64) {
        *self.mem.entry(loc as u64).or_insert(val) = val;
    }

    pub fn write_to_buff(&mut self, v: i64) {
        self.input.push_front(v);
    }

    fn write_dest(&self, d: i64, param_mode: i64) -> i64 {
        if param_mode == 0 {
            return d;
        }
        d + self.rel_base
    }

    fn val(&self, p: i64, param_mode: i64) -> i64 {
        match param_mode {
            0 => self.read(p),
            1 => p,
            2 => self.read(p + self.rel_base),
            _ => panic!("Illegal parameter mode :o"),
        }
    }

    pub fn run(&mut self) -> i64 {
        loop {
            let instr = self.read(self.pointer);
            let opcode = instr - instr / 100 * 100;
            let m_1 = (instr - instr / 1000 * 1000) / 100 % 3;
            let m_2 = (instr - instr / 10000 * 10000) / 1000 % 3;
            let m_3 = instr / 10000 % 3;

            match opcode {
                1 => {
                    let (a, b, d) = self.get_3_params(self.pointer);
                    self.write(self.write_dest(d, m_3), self.val(a, m_1) + self.val(b, m_2));
                    self.pointer += 4;
                }
                2 => {
                    let (a, b, d) = self.get_3_params(self.pointer);
                    self.write(self.write_dest(d, m_3), self.val(a, m_1) * self.val(b, m_2));
                    self.pointer += 4;
                }
                3 => {
                    let mut d = self.read(self.pointer + 1);
                    if m_1 == 2 {
                        d += self.rel_base;
                    }

                    match self.input.pop_back() {
                        Some(v) => self.write(d, v),
                        None => panic!("Attempted to read from empty input buffer!"),
                    }
                    self.pointer += 2;
                }
                4 => {
                    let a = self.read(self.pointer + 1);
                    self.output = self.val(a, m_1);
                    self.pointer += 2;
                    break;
                }
                5 => {
                    let (a, j) = self.get_2_params(self.pointer);
                    if self.val(a, m_1) != 0 {
                        self.pointer = self.val(j, m_2);
                    } else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    let (a, j) = self.get_2_params(self.pointer);
                    if self.val(a, m_1) == 0 {
                        self.pointer = self.val(j, m_2);
                    } else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    let (a, b, d) = self.get_3_params(self.pointer);
                    if self.val(a, m_1) < self.val(b, m_2) {
                        self.write(self.write_dest(d, m_3), 1);
                    } else {
                        self.write(self.write_dest(d, m_3), 0);
                    }
                    self.pointer += 4;
                }
                8 => {
                    let (a, b, d) = self.get_3_params(self.pointer);
                    if self.val(a, m_1) == self.val(b, m_2) {
                        self.write(self.write_dest(d, m_3), 1);
                    } else {
                        self.write(self.write_dest(d, m_3), 0);
                    }
                    self.pointer += 4;
                }
                9 => {
                    let a = self.read(self.pointer + 1);
                    self.rel_base += self.val(a, m_1);
                    self.pointer += 2;
                }
                99 => {
                    self.halted = true;
                    break;
                }
                _ => panic!("Unexpected opcode {}", opcode),
            }
        }
        self.output
    }

    pub fn new() -> Intcode {
        Intcode {
            pointer: 0,
            mem: HashMap::new(),
            input: VecDeque::new(),
            output: 0,
            halted: false,
            rel_base: 0,
        }
    }

    pub fn load(&mut self, prog_txt: &str) {
        let arr: Vec<i64> = prog_txt
            .split(",")
            .map(|a| a.parse::<i64>().unwrap())
            .collect();
        for loc in 0..arr.len() {
            self.mem.insert(loc as u64, arr[loc]);
        }

        self.pointer = 0;
        self.halted = false;
    }
}
