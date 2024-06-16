mod development;
mod instructions;
mod types;
mod vm;

use std::env;

use development::*;
use vm::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "test" {
        println!("Running test bytecode: \n");
        test::run_bytecode();
    }
}
