use std::process::Output;

type Register = u64;
type Program = Vec<u8>;

fn parse_program_spec(input: &str) -> (Register, Register, Register, Program) {
    let mut lines = input.lines();

    // read register A
    let mut err = "no definition for register A found!";
    let register_a = lines
        .next()
        .expect(err)
        .strip_prefix("Register A: ")
        .expect(err)
        .parse::<u64>()
        .expect(err);

    // read register B
    err = "no definition for register B found!";
    let register_b = lines
        .next()
        .expect(err)
        .strip_prefix("Register B: ")
        .expect(err)
        .parse::<u64>()
        .expect(err);

    // read register C
    err = "no definition for register C found!";
    let register_c = lines
        .next()
        .expect(err)
        .strip_prefix("Register C: ")
        .expect(err)
        .parse::<u64>()
        .expect(err);

    // read program
    err = "no definition for program!";
    lines.next().expect(err);
    let program = lines
        .next()
        .expect(err)
        .strip_prefix("Program: ")
        .expect(err)
        .split(',')
        .map(|s| s.parse::<u8>().expect(err))
        .collect::<Vec<u8>>();

    (register_a, register_b, register_c, program)
}

fn as_combo_operand(
    operand: u8,
    register_a: &Register,
    register_b: &Register,
    register_c: &Register,
) -> u64 {
    match operand {
        0..4 => operand as u64,
        4 => *register_a,
        5 => *register_b,
        6 => *register_c,
        7 => panic!("combo operand 7 is reserved and will not appear in valid programs"),
        _ => panic!("not a 3-bit number: {operand}"),
    }
}

fn as_literal_operand(operand: u8) -> u8 {
    if (0..=7).contains(&operand) {
        operand
    } else {
        panic!("not a 3-bit number: {operand}")
    }
}

fn run_program(
    register_a: &mut Register,
    register_b: &mut Register,
    register_c: &mut Register,
    program: &Vec<u8>,
    output: &mut Vec<u8>,
) -> usize {
    let mut instruction_ptr = 0;

    let program_length = program.len();

    let a_initial = *register_a;

    loop {
        //let l = output.len();
        //if program[0..l] != output[..] {
        //println!("{:?} != {:?}", *output, *program);
        //    return false;
        //}
        if instruction_ptr == program_length {
            break;
        }
        if instruction_ptr > program_length {
            panic!("illegal machine-state: instruction_ptr is {instruction_ptr}, but program-size is only {program_length}");
        }

        let opcode = program[instruction_ptr];
        let operand = program[instruction_ptr + 1];

        match opcode {
            0 =>
            // adv
            {
                let numerator = *register_a;
                let operand = as_combo_operand(operand, register_a, register_b, register_c);
                let denomitator = 2_u64.pow(operand.try_into().unwrap());
                *register_a = numerator / denomitator;
                instruction_ptr += 2;
            }

            1 =>
            // bxl
            {
                *register_b ^= as_literal_operand(operand) as u64;
                instruction_ptr += 2;
            }

            2 =>
            // bst
            {
                let operand = as_combo_operand(operand, register_a, register_b, register_c);
                *register_b = (operand % 8) & 7;
                instruction_ptr += 2;
            }

            3 =>
            // jnz
            {
                if *register_a == 0 {
                    instruction_ptr += 2;
                } else {
                    instruction_ptr = as_literal_operand(operand) as usize;
                }
            }

            4 =>
            // bxc
            {
                *register_b ^= *register_c;
                instruction_ptr += 2;
            }

            5 =>
            // out
            {
                let operand = as_combo_operand(operand, register_a, register_b, register_c);
                output.push((operand % 8) as u8);
                instruction_ptr += 2;
            }

            6 =>
            // bdv
            {
                let numerator = *register_a;
                let operand = as_combo_operand(operand, register_a, register_b, register_c);
                let denomitator = 2_u64.pow(operand.try_into().unwrap());
                *register_b = numerator / denomitator;
                instruction_ptr += 2;
            }

            7 =>
            // cdv
            {
                let numerator = *register_a;
                let operand = as_combo_operand(operand, register_a, register_b, register_c);
                let denomitator = 2_u64.pow(operand.try_into().unwrap());
                *register_c = numerator / denomitator;
                instruction_ptr += 2;
            }

            _ => panic!("not a 3-bit number: {opcode}"),
        }
    }
    /*if output.len() > program_length {
        println!("offset {a_initial} and/or step too large");
        return false;
    }
    //println!("{}", output.len());
    if *output == *program {
        true
    } else {
        //println!("{:?} != {:?}", *output, *program);
        false
    }*/
    output.len()
}

pub fn part2(input: &str, brute_force_offset: u64, step: u64) -> u64 {
    let (_, register_b, register_c, program) = parse_program_spec(input);

    let mut register_a_iter = brute_force_offset;
    loop {
        let mut cur_register_a = register_a_iter;
        let mut cur_register_b = register_b;
        let mut cur_register_c = register_c;
        let mut cur_output = Vec::new();
        let l = run_program(
            &mut cur_register_a,
            &mut cur_register_b,
            &mut cur_register_c,
            &program,
            &mut cur_output,
        );

        if program == cur_output {
            return register_a_iter;
        }

        if l < program.len() {
            println!("output too short ({l}), adjust your offset");
            return 0;
        }

        if l > program.len() {
            println!("output too long ({l}), adjust your offset and/or stepsize");
            return 0;
        }

        //let mut correct_digits_from_end = 0;
        //while program[l - correct_digits_from_end - 1]
        //    == cur_output[l - correct_digits_from_end - 1]
        //{
        //    correct_digits_from_end += 1;
        //}

        //println!("correct_digits_from_end (A was {register_a_iter}): {correct_digits_from_end}",);

        register_a_iter += step;
    }
    register_a_iter
}
