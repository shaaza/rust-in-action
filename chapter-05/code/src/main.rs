mod fixed_point_decimal;

struct CPU {
    registers: [u8; 2],
    current_operation: u16,
}

type Operation = (u8, u8, u8, u8);

fn decode_opcode(opcode: u16) -> Operation {
    let c = ((opcode & 0xF000) >> 12) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;
    let d = (opcode & 0x000F) as u8;

    (c, x, y, d)
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();
        let operation = decode_opcode(opcode);

        self.dispatch_operation(operation);
    }

    fn dispatch_operation(&mut self, operation: Operation) {
        match operation {
            (0x8, 0x0, 0x1, 0x4) => {
                self.registers[0] = self.registers[0].wrapping_add(self.registers[1]);
            }
            _ => todo!(),
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 2],
        current_operation: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.current_operation = 0x8014;
    println!("{} + {} =", cpu.registers[0], cpu.registers[1],);

    cpu.run();

    println!("{}", cpu.registers[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cpu_for_add(x: u8, y: u8) -> CPU {
        CPU {
            registers: [x, y],
            current_operation: 0x8014,
        }
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
    fn add_two_registers() {
        let mut cpu = cpu_for_add(5, 10);

        cpu.run();

        assert_eq!(cpu.registers, [15, 10]);
    }

    #[test]
    fn add_zero_to_zero() {
        let mut cpu = cpu_for_add(0, 0);

        cpu.run();

        assert_eq!(cpu.registers, [0, 0]);
    }

    #[test]
    fn add_zero_to_max_value() {
        let mut cpu = cpu_for_add(u8::MAX, 0);

        cpu.run();

        assert_eq!(cpu.registers, [u8::MAX, 0]);
    }

    #[test]
    fn add_wraps_on_overflow() {
        let mut cpu = cpu_for_add(u8::MAX, 1);

        cpu.run();

        assert_eq!(cpu.registers, [0, 1]);
    }
}
