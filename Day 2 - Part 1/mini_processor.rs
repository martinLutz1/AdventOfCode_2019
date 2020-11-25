fn add(ram: &mut [i64], counter_position: u64) {
    let input_1_position = ram[(counter_position + 1) as usize];
    let input_2_position = ram[(counter_position + 2) as usize];
    let output_position = ram[(counter_position + 3) as usize];

    ram[output_position as usize] = ram[input_1_position as usize] + ram[input_2_position as usize];
}

fn multiply(ram: &mut [i64], counter_position: u64) {
    let input_1_position = ram[(counter_position + 1) as usize];
    let input_2_position = ram[(counter_position + 2) as usize];
    let output_position = ram[(counter_position + 3) as usize];

    ram[output_position as usize] = ram[input_1_position as usize] * ram[input_2_position as usize];
}

fn print(ram: &mut [i64]) {
    let mut ram_as_string = String::from("[");
    for i in 0..ram.len() {
        ram_as_string.push_str(&ram[i as usize].to_string());
        if i < (ram.len() - 1) {
            ram_as_string.push_str(", ");
        }
    }
    ram_as_string.push_str("]");

    println!("{}", ram_as_string);
}

fn run(ram: &mut [i64]) {
    let mut instruction_counter: u64 = 0;
    let mut num_of_instructions: u64 = 0;

    loop {
        let op_code = ram[instruction_counter as usize];

        match op_code {
            1 => {
                add(ram, instruction_counter);

                instruction_counter += 4;
                num_of_instructions += 1;
            }
            2 => {
                multiply(ram, instruction_counter);

                instruction_counter += 4;
                num_of_instructions += 1;
            }
            99 => {
                //instruction_counter += 1;
                num_of_instructions += 1;

                print(ram);
                println!("Terminated normally. Executed {} instructions.", num_of_instructions);
                break
            }
            _ => {
                panic!("Crash due to unknown opcode '{}' at position {}.", op_code, instruction_counter);
            }
        }
    }
}

fn main() {
    let mut ram: Vec<i64> = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,2,19,9,23,1,23,5,27,2,6,27,31,1,31,5,35,1,35,5,39,2,39,6,43,2,43,10,47,1,47,6,51,1,51,6,55,2,55,6,59,1,10,59,63,1,5,63,67,2,10,67,71,1,6,71,75,1,5,75,79,1,10,79,83,2,83,10,87,1,87,9,91,1,91,10,95,2,6,95,99,1,5,99,103,1,103,13,107,1,107,10,111,2,9,111,115,1,115,6,119,2,13,119,123,1,123,6,127,1,5,127,131,2,6,131,135,2,6,135,139,1,139,5,143,1,143,10,147,1,147,2,151,1,151,13,0,99,2,0,14,0];
    run(&mut ram);
}
