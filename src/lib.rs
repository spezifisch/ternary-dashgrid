use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TernaryCPU {
    memory: [u8; 10],
    pointer: usize,
    program_counter: usize,
    opcodes: Vec<u8>,
}

#[wasm_bindgen]
impl TernaryCPU {
    pub fn new(opcodes: Vec<u8>) -> TernaryCPU {
        TernaryCPU {
            memory: [0; 10],
            pointer: 0,
            program_counter: 0,
            opcodes,
        }
    }

    pub fn run(&mut self) -> String {
        let mut output = String::new();
        while self.program_counter < self.opcodes.len() {
            let opcode = self.opcodes[self.program_counter];
            self.program_counter += 1; // Move to the next instruction
            match opcode {
                11 => self.increment(),
                12 => self.decrement(),
                20 => {
                    // Jump to label 1
                    if let Some(pos) = self.opcodes.iter().position(|&x| x == 1) {
                        self.program_counter = pos + 1;
                        output.push_str(&format!("Jumped to label 1\n"));
                    }
                }
                1..=9 => {
                    // Skip over label, don't treat it as an instruction
                    output.push_str(&format!("At label: {}\n", opcode));
                    continue;
                }
                0 => break, // Halt
                _ => output.push_str("Unknown opcode\n"),
            }
            output.push_str(&format!("Memory: {:?}\n", self.memory));
        }
        output
    }

    pub fn get_memory(&self) -> Vec<u8> {
        self.memory.to_vec()
    }

    fn increment(&mut self) {
        self.memory[self.pointer] = (self.memory[self.pointer] + 1) % 3;
    }

    fn decrement(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.memory[self.pointer] = 2;
        } else {
            self.memory[self.pointer] -= 1;
        }
    }
}

