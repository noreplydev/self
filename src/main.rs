mod development;
mod instructions;
mod types;
mod vm;

use development::*;
use vm::*;

fn main() {
    let mut instructions: Vec<u8> = vec![0x01, 0x01];
    instructions.extend_from_slice(&u64_to_bytes(14));
    instructions.push(0x01);
    instructions.push(0x01);
    instructions.extend_from_slice(&u64_to_bytes(20));
    instructions.push(0x02);

    let mut vm = Vm::new(instructions);
    vm.run();
}
