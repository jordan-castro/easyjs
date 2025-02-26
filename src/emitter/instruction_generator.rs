use wasm_encoder::{Instruction, ValType};

pub fn new_ptr<'a>() -> Vec<Instruction<'a>> {
    vec![Instruction::I32Const(0)]
}

pub fn allocate_space<'a>() -> Vec<Instruction<'a>> {
    vec![Instruction::I32Const(0)]
}

pub fn add_i32<'a>(numbers : Vec<i32>) -> Vec<Instruction<'a>> {
    let mut instructions = Vec::new();
    for number in numbers {
        instructions.push(Instruction::I32Const(number));
    }

    instructions.push(Instruction::I32Add);

    instructions
}

pub fn add_f32<'a>(numbers : Vec<f32>) -> Vec<Instruction<'a>> {
    let mut instructions = Vec::new();
    for number in numbers {
        instructions.push(Instruction::F32Const(number));
    }

    instructions.push(Instruction::F32Add);

    instructions
}
