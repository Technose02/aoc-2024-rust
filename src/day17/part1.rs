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
    program: Vec<u8>,
    output: &mut Vec<u8>,
) {
    let mut instruction_ptr = 0;

    let program_length = program.len();

    loop {
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
}

pub fn part1(input: &str) -> String {
    let (mut register_a, mut register_b, mut register_c, program) = parse_program_spec(input);
    let mut output = Vec::new();
    run_program(
        &mut register_a,
        &mut register_b,
        &mut register_c,
        program,
        &mut output,
    );

    output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {

    use super::*;

    const PROGRAM_SPEC_1_STR: &str = r#"Register A: 729
Register B: 67
Register C: 6780

Program: 0,1,5,4,3,0"#;

    #[test]
    fn parse_program_spec_works() {
        assert_eq!(
            parse_program_spec(PROGRAM_SPEC_1_STR),
            (
                729 as Register,
                67 as Register,
                6780 as Register,
                vec![0, 1, 5, 4, 3, 0]
            )
        )
    }

    #[test]
    fn run_program_works_1() {
        let mut register_a = 0_u64;
        let mut register_b = 0_u64;
        let mut register_c = 9_u64;
        let mut output = Vec::new();

        run_program(
            &mut register_a,
            &mut register_b,
            &mut register_c,
            vec![2, 6],
            &mut output,
        );
        assert_eq!(register_b, 1);
    }

    #[test]
    fn run_program_works_2() {
        let mut register_a = 10_u64;
        let mut register_b = 0_u64;
        let mut register_c = 0_u64;
        let mut output = Vec::new();

        run_program(
            &mut register_a,
            &mut register_b,
            &mut register_c,
            vec![5, 0, 5, 1, 5, 4],
            &mut output,
        );
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn run_program_works_3() {
        let mut register_a = 2024_u64;
        let mut register_b = 0_u64;
        let mut register_c = 0_u64;
        let mut output = Vec::new();

        run_program(
            &mut register_a,
            &mut register_b,
            &mut register_c,
            vec![0, 1, 5, 4, 3, 0],
            &mut output,
        );
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(register_a, 0);
    }

    #[test]
    fn run_program_works_4() {
        let mut register_a = 0_u64;
        let mut register_b = 29_u64;
        let mut register_c = 0_u64;
        let mut output = Vec::new();

        run_program(
            &mut register_a,
            &mut register_b,
            &mut register_c,
            vec![1, 7],
            &mut output,
        );
        assert_eq!(register_b, 26);
    }

    #[test]
    fn run_program_works_5() {
        let mut register_a = 0_u64;
        let mut register_b = 2024_u64;
        let mut register_c = 43690_u64;
        let mut output = Vec::new();

        run_program(
            &mut register_a,
            &mut register_b,
            &mut register_c,
            vec![4, 0],
            &mut output,
        );
        assert_eq!(register_b, 44354);
    }
}
