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

#[derive(Debug)]
struct Machine {
    initial_b: u64,
    initial_c: u64,
    program: Vec<u8>,
}

#[derive(PartialEq)]
enum SimpleCompare {
    Lower,
    Equal,
    Higher,
}

impl Machine {
    pub fn from_input(input: &str) -> Self {
        let (_, initial_b, initial_c, program) = parse_program_spec(input);

        Self {
            initial_b,
            initial_c,
            program,
        }
    }

    pub fn run_for_output(&self, register_a: u64) -> Vec<u8> {
        let mut init_register_a = register_a;
        let mut init_register_b = self.initial_b;
        let mut init_register_c = self.initial_c;
        let mut output = Vec::new();

        run_program(
            &mut init_register_a,
            &mut init_register_b,
            &mut init_register_c,
            &self.program,
            &mut output,
        );

        output
    }

    pub fn run_for_correct_output_len(&self, register_a: u64) -> SimpleCompare {
        let output_len = self.run_for_output(register_a).len();
        let program_len = self.program.len();

        if output_len < program_len {
            SimpleCompare::Lower
        } else if output_len > program_len {
            SimpleCompare::Higher
        } else {
            SimpleCompare::Equal
        }
    }

    pub fn run_for_error(&self, register_a: u64) -> Option<u64> {
        let output = self.run_for_output(register_a);

        if output.len() != self.program.len() {
            return None;
        }

        let mut error = output.len() as u64;
        for (k, val) in output.iter().enumerate().rev() {
            if *val != self.program[k] {
                break;
            }
            error -= 1;
        }
        Some(error)
    }
}

#[derive(Debug)]
struct Optimizer {
    machine: Machine,
    max_too_low: u64,
    min_too_high: u64,
}

impl Optimizer {
    fn determine_outer_bounds(machine: &Machine) -> (u64, u64) {
        // find range where output-length is correct
        if matches!(
            machine.run_for_correct_output_len(1),
            SimpleCompare::Higher | SimpleCompare::Equal
        ) {
            todo!();
        }

        let mut max_too_low = 1;
        let mut min_too_high = None;

        let mut base = max_too_low;
        while min_too_high.is_none() {
            let n = 2 * base;
            match machine.run_for_correct_output_len(n) {
                SimpleCompare::Lower => {
                    max_too_low = n;
                    base = n;
                }
                SimpleCompare::Higher => {
                    min_too_high = Some(n);
                }
                SimpleCompare::Equal => {
                    base = n;
                }
            }
        }
        let mut min_too_high = min_too_high.unwrap();

        // refine max_too_low
        let mut center = max_too_low + (min_too_high - max_too_low) / 2;
        loop {
            match machine.run_for_correct_output_len(center) {
                SimpleCompare::Lower => {
                    max_too_low = center;
                    center = max_too_low + (min_too_high - max_too_low) / 2;
                }
                SimpleCompare::Higher => {
                    min_too_high = center;
                    center = max_too_low + (min_too_high - max_too_low) / 2;
                }
                SimpleCompare::Equal => {
                    if center == max_too_low + 1 {
                        break;
                    }
                    center = max_too_low + (center - max_too_low) / 2;
                }
            }
        }

        // refine min_too_high
        let mut center = max_too_low + (min_too_high - max_too_low) / 2;
        loop {
            match machine.run_for_correct_output_len(center) {
                SimpleCompare::Lower => {
                    panic!("value should be Equal");
                }
                SimpleCompare::Higher => {
                    min_too_high = center;
                    center = max_too_low + (min_too_high - max_too_low) / 2;
                }
                SimpleCompare::Equal => {
                    if center == min_too_high - 1 {
                        break;
                    }
                    center = center + (min_too_high - center) / 2;
                }
            }
        }

        (max_too_low, min_too_high)
    }

    pub fn new(machine: Machine) -> Self {
        let bounds = Optimizer::determine_outer_bounds(&machine);

        Optimizer {
            machine,
            max_too_low: bounds.0,
            min_too_high: bounds.1,
        }
    }

    pub fn find_register_a(&self) -> Option<u64> {
        let err_lower = self.machine.run_for_error(self.max_too_low + 1).unwrap();
        let err_upper = self.machine.run_for_error(self.min_too_high - 1).unwrap();

        if let Some(mut res) =
            self.seek_in_interval(self.max_too_low, self.min_too_high, err_lower, err_upper)
        {
            while let Some(0) = self.machine.run_for_error(res) {
                res -= 1;
            }
            Some(res + 1)
        } else {
            None
        }
    }

    fn seek_in_interval(
        &self,
        lower: u64,
        upper: u64,
        err_lower: u64,
        err_upper: u64,
    ) -> Option<u64> {
        let center = lower + (upper - lower) / 2;
        if center == lower || center == upper {
            return None;
        }
        let err_center = self.machine.run_for_error(center).unwrap();
        println!("======");
        println!("err at {lower}: {err_lower}");
        println!("err at {center}: {err_center}");
        println!("err at {upper}: {err_upper}");
        println!("======");
        if err_lower == 0 {
            return Some(center);
        }
        if let Some(res) = self.seek_in_interval(center, upper, err_center, err_upper) {
            return Some(res);
        }
        if let Some(res) = self.seek_in_interval(lower, center, err_lower, err_center) {
            return Some(res);
        }
        None
    }
}

pub fn part2(input: &str) -> u64 {
    let machine = Machine::from_input(input);

    let optimizer = Optimizer::new(machine);
    println!("{:?}", optimizer);

    optimizer.find_register_a().unwrap()
}
