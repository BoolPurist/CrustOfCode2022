type CpuProgram = Vec<CpuInst>;
use core::fmt::Debug;
#[derive(Debug)]
enum CpuInst {
    Noop,
    Addx(i64),
}

struct Cpu {
    register: i64,
    program_counter: usize,
    program: CpuProgram,
    inst_countdown: usize,
    terminated: bool,
}

pub fn get_signal_strength_up_to(
    input: &str,
    up_to_cycle: usize,
    cycle_steps: usize,
    cycle_offset: usize,
) -> i64 {
    let program = parse_input(input);
    process_program(program, up_to_cycle, cycle_steps, cycle_offset)
}

pub fn get_drawing(input: &str, height: usize, width: usize) -> String {
    let program = parse_input(input);
    draw_according_to_program(program, height, width)
}

fn draw_according_to_program(program: CpuProgram, height: usize, width: usize) -> String {
    let pixels = height * width;
    let mut cpu = Cpu::new(program);
    let mut screen = String::with_capacity(pixels);
    let mut cursor = 0;
    for current_cycle in 1..=pixels {
        cpu.next_cycle();
        let current_x = cpu.get_reg_v();
        let min_x = (current_x - 1) as usize;
        let max_x = (current_x + 1) as usize;

        if cursor >= min_x && cursor <= max_x {
            screen.push('#');
        } else {
            screen.push('.');
        }

        cursor += 1;
        if (current_cycle % width) == 0 {
            cursor = 0;
            screen.push('\n');
        }
    }

    screen
}

fn process_program(
    program: CpuProgram,
    up_to_cycle: usize,
    cycle_steps: usize,
    cycle_offset: usize,
) -> i64 {
    let mut signals: Vec<i64> = Default::default();
    let mut cpu = Cpu::new(program);
    let mut signal_counter = cycle_offset;
    for current_cycle in 0..=up_to_cycle {
        if (signal_counter % cycle_steps) == 0 {
            dbg!(current_cycle);
            let new_signal = (current_cycle as i64) * cpu.get_reg_v();
            signals.push(new_signal);
        }

        cpu.next_cycle();

        signal_counter += 1;
    }

    dbg!(&signals);

    signals
        .into_iter()
        .fold(i64::default(), |akk, signal| akk + signal)
}

fn parse_input(input: &str) -> CpuProgram {
    input
        .lines()
        .map(|line| {
            let mut inst_splitted = line.split(" ");

            match (inst_splitted.next(), inst_splitted.next()) {
                (Some("addx"), Some(amount)) => CpuInst::Addx(
                    amount
                        .parse()
                        .expect("adding not  parsable to singed integer"),
                ),
                (Some("noop"), None) => CpuInst::Noop,
                _ => panic!("line not parseable: {}", line),
            }
        })
        .collect()
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Cpu))
            .field("X", &self.register)
            .field("Instruction countdown", &self.inst_countdown)
            .field("Current program line", &self.program_counter)
            .field("Terminated", &self.terminated)
            .finish()
    }
}

impl Cpu {
    fn new(program: CpuProgram) -> Self {
        let mut new_self = Self {
            program,
            terminated: false,
            register: 1,
            program_counter: 0,
            inst_countdown: 0,
        };

        new_self._adjust_inst_countdown();

        new_self
    }

    fn next_cycle(&mut self) {
        if self.terminated {
            return;
        } else if self.inst_countdown != 0 {
            self.inst_countdown -= 1;
        } else {
            let to_execute = &self.program[self.program_counter];

            match to_execute {
                CpuInst::Noop => (),
                CpuInst::Addx(to_add) => {
                    self.register += to_add;
                }
            };

            self.program_counter += 1;
            self.terminated = self.program_counter >= self.program.len();
            if self.terminated {
                return;
            }

            self._adjust_inst_countdown();
            self.inst_countdown -= 1;
        }
    }

    fn _adjust_inst_countdown(&mut self) {
        let next_instruction = &self.program[self.program_counter];

        self.inst_countdown = match next_instruction {
            CpuInst::Noop => 1,
            CpuInst::Addx(_) => 2,
        }
    }

    fn get_reg_v(&self) -> i64 {
        self.register
    }
}
