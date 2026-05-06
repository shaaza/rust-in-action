mod fixed_point_decimal;

struct CPU {
    registers: [u8; 2],
    memory: [u8; 0x1000],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

type Operation = (u8, u8, u8, u8);

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Add { x: usize, y: usize },
    Call { address: usize },
    Return,
}

fn decode_opcode(opcode: u16) -> Operation {
    let c = ((opcode & 0xF000) >> 12) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;
    let d = (opcode & 0x000F) as u8;

    (c, x, y, d)
}

fn decode_instruction(operation: Operation) -> Instruction {
    match operation {
        (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
        (0x2, x, y, d) => Instruction::Call {
            address: ((x as usize) << 8) | ((y as usize) << 4) | d as usize,
        },
        (0x8, x, y, 0x4) => Instruction::Add {
            x: x as usize,
            y: y as usize,
        },
        _ => todo!(),
    }
}

impl CPU {
    fn load(&mut self, opcodes: &[u16], start: usize) {
        for (i, opcode) in opcodes.iter().enumerate() {
            let memory_index = start + (i * 2);
            self.memory[memory_index] = (opcode >> 8) as u8;
            self.memory[memory_index + 1] = (opcode & 0x00FF) as u8;
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();

            if !self.dispatch_instruction(opcode) {
                return;
            }

            self.program_counter += 2;
        }
    }

    fn dispatch_instruction(&mut self, opcode: u16) -> bool {
        if opcode == 0x0000 {
            return false;
        }

        let operation = decode_opcode(opcode);
        let instruction = decode_instruction(operation);

        match instruction {
            Instruction::Add { x, y } => {
                self.registers[x] = self.registers[x].wrapping_add(self.registers[y]);
            }
            Instruction::Call { address } => self.call(address),
            Instruction::Return => self.ret(),
        }

        true
    }

    fn call(&mut self, address: usize) {
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = address - 2;
    }

    fn ret(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 2],
        memory: [0; 0x1000],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    println!("{} + {} =", cpu.registers[0], cpu.registers[1],);

    cpu.run();

    println!("{}", cpu.registers[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cpu_for_add(x: u8, y: u8) -> CPU {
        // Build a CPU with one ADD instruction loaded at the start of memory.
        let mut cpu = CPU {
            registers: [x, y],
            memory: [0; 0x1000],
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };

        cpu.load(&[0x8014], 0);

        cpu
    }

    #[test]
    fn decode_opcode_into_four_nibbles() {
        // CHIP-8 opcodes are four nibbles: 0x8, 0x0, 0x1, and 0x4.
        assert_eq!(decode_opcode(0x8014), (0x8, 0x0, 0x1, 0x4));
    }

    #[test]
    fn decode_opcode_preserves_each_nibble_position() {
        // Each hex digit is decoded from its own position in the u16 opcode.
        assert_eq!(decode_opcode(0xABCD), (0xA, 0xB, 0xC, 0xD));
    }

    #[test]
    fn decode_operation_into_add_instruction() {
        // The 8xy4 opcode means "add register y into register x".
        assert_eq!(
            decode_instruction((0x8, 0x0, 0x1, 0x4)),
            Instruction::Add { x: 0, y: 1 }
        );
    }

    #[test]
    fn decode_operation_into_call_instruction() {
        // 0x2006 calls the function loaded at memory address 0x006, decimal 6.
        assert_eq!(
            decode_instruction((0x2, 0x0, 0x0, 0x6)),
            Instruction::Call { address: 0x006 }
        );
    }

    #[test]
    fn decode_operation_into_return_instruction() {
        // 0x00EE returns from the current function.
        assert_eq!(decode_instruction((0x0, 0x0, 0xE, 0xE)), Instruction::Return);
    }

    #[test]
    fn add_two_registers() {
        // Running 0x8014 adds register 1 into register 0.
        let mut cpu = cpu_for_add(5, 10);

        cpu.run();

        assert_eq!(cpu.registers, [15, 10]);
    }

    #[test]
    fn add_zero_to_zero() {
        // Adding zero to zero leaves both registers unchanged.
        let mut cpu = cpu_for_add(0, 0);

        cpu.run();

        assert_eq!(cpu.registers, [0, 0]);
    }

    #[test]
    fn add_zero_to_max_value() {
        // Adding zero to the largest u8 value should not change it.
        let mut cpu = cpu_for_add(u8::MAX, 0);

        cpu.run();

        assert_eq!(cpu.registers, [u8::MAX, 0]);
    }

    #[test]
    fn add_wraps_on_overflow() {
        // u8 addition wraps: 255 + 1 becomes 0.
        let mut cpu = cpu_for_add(u8::MAX, 1);

        cpu.run();

        assert_eq!(cpu.registers, [0, 1]);
    }

    #[test]
    fn executes_several_instructions_in_sequence() {
        let mut cpu = cpu_for_add(5, 10);
        // The CPU reads instructions as two-byte opcodes.
        // This creates a second 0x8014 ADD instruction at memory[2..4].
        cpu.load(&[0x8014], 2);

        cpu.run();

        // Register 0 starts at 5. Each ADD adds register 1, which is 10.
        // Two ADD instructions means 5 + 10 + 10 = 25.
        assert_eq!(cpu.registers, [25, 10]);
    }

    #[test]
    fn read_opcode_from_memory() {
        // Two neighboring bytes in memory become one 16-bit opcode.
        let cpu = cpu_for_add(5, 10);

        assert_eq!(cpu.read_opcode(), 0x8014);
    }

    #[test]
    fn calls_function_twice_and_returns_to_calling_location() {
        let mut cpu = CPU {
            registers: [
                5,  // decimal 5
                10, // decimal 10
            ],
            memory: [0; 0x1000],
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };

        // Main program:
        // 0x2006 calls the function at 0x006, decimal 6.
        // 0x2006 calls the function at 0x006, decimal 6.
        cpu.load(&[0x2006, 0x2006], 0);

        // add_twice:
        // 0x8014 adds register 1, decimal 10, into register 0.
        // 0x8014 adds register 1, decimal 10, into register 0.
        // 0x00EE returns to the previous CALL instruction.
        cpu.load(&[0x8014, 0x8014, 0x00EE], 0x006);

        cpu.run();

        // 5 + (10 * 2) + (10 * 2) = 45
        assert_eq!(cpu.registers, [45, 10]);
        assert_eq!(cpu.stack_pointer, 0);
    }
}
