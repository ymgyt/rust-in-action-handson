const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const ADD_XY: u8 = 0x4;

struct CPU {
    current_operation: u16,
    registers: [u8;2],
}

impl CPU {
    fn run(&mut self) {
        let raw_op = self.current_operation;

        let op_major = ((raw_op & 0xF000) >> 12 ) as u8;
        let x = ((raw_op & 0x0F00) >> 8) as u8;
        let y = ((raw_op & 0x00F0) >> 4) as u8;
        let op_minor = (raw_op & 0x000F) as u8;

        match (op_major, op_minor) {
            (ARITHMETIC_AND_LOGIC, ADD_XY) => self.add_xy(x,y),
            _ => unimplemented!(),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}


fn main() {
    let mut cpu = CPU {
        current_operation: 0x8014,
        registers:[0;2],
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);
}
