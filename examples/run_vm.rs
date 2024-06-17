use self_vm::{
    utils::{bytes_from_32, bytes_from_64, Number},
    vm::Vm,
};

fn main() {
    let mut instructions: Vec<u8> = vec![0x01, 0x01];
    instructions.extend_from_slice(&bytes_from_64(Number::I64(14)));
    instructions.push(0x01);
    instructions.push(0x02);
    instructions.extend_from_slice(&bytes_from_32(Number::U32(20)));
    instructions.push(0x02);

    let mut vm = Vm::new(instructions);
    vm.run();
}
