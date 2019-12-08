use std::convert::TryFrom;
use std::error::Error;
use std::io;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn get_program<R: BufRead>(read: R) -> Result<Vec<i64>> {
    let mut inputs = Vec::new();
    for value in read.split(b',') {
        let value = value?;
        let value = std::str::from_utf8(&value)?;
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        let number = value.parse::<i64>()?;
        inputs.push(number)
    }

    Ok(inputs)
}

#[derive(Debug)]
enum Instruction {
    Halt,
    Add { a: Param, b: Param, c: usize },
    Mul { a: Param, b: Param, c: usize },
    Input(usize),
    Output(Param),
    JumpIfTrue { cond: Param, next: Param },
    JumpIfFalse { cond: Param, next: Param },
    LessThan { a: Param, b: Param, c: usize },
    Eq { a: Param, b: Param, c: usize },
}

impl Instruction {
    fn from_slice(slice: &[i64]) -> Option<Instruction> {
        let command = slice.first().and_then(|x| u64::try_from(*x).ok())?;
        let op = (command % 100) as u64;
        let get_param = |idx: usize| {
            let mode = (command / 10_u64.pow(idx as u32 + 2)) % 10;
            let param = *slice.get(idx + 1)?;
            let res = match mode {
                0 => Param::Position(usize::try_from(param).ok()?),
                1 => Param::Immediate(param),
                _ => return None,
            };
            Some(res)
        };

        let res = match op {
            99 => Instruction::Halt,
            1 => Instruction::Add {
                a: get_param(0)?,
                b: get_param(1)?,
                c: get_param(2)?.position()?,
            },
            2 => Instruction::Mul {
                a: get_param(0)?,
                b: get_param(1)?,
                c: get_param(2)?.position()?,
            },
            3 => Instruction::Input(get_param(0)?.position()?),
            4 => Instruction::Output(get_param(0)?),
            5 => Instruction::JumpIfTrue {
                cond: get_param(0)?,
                next: get_param(1)?,
            },
            6 => Instruction::JumpIfFalse {
                cond: get_param(0)?,
                next: get_param(1)?,
            },
            7 => Instruction::LessThan {
                a: get_param(0)?,
                b: get_param(1)?,
                c: get_param(2)?.position()?,
            },
            8 => Instruction::Eq {
                a: get_param(0)?,
                b: get_param(1)?,
                c: get_param(2)?.position()?,
            },
            _ => return None,
        };
        Some(res)
    }
}

#[derive(Debug)]
enum Param {
    Position(usize),
    Immediate(i64),
}

impl Param {
    fn position(&self) -> Option<usize> {
        match self {
            Param::Position(addr) => Some(*addr),
            Param::Immediate(_) => None,
        }
    }

    fn get(&self, memory: &[i64]) -> Option<i64> {
        match self {
            Param::Position(addr) => memory.get(*addr).cloned(),
            Param::Immediate(x) => Some(*x),
        }
    }
}

enum ExecutionOutput {
    SuspendInput(Vec<i64>),
    Halt(Vec<i64>),
}

struct VM {
    memory: Vec<i64>,
    input: Vec<i64>,
    pc: usize,
}

impl VM {
    fn new(memory: Vec<i64>) -> VM {
        VM {
            memory,
            input: Vec::new(),
            pc: 0,
        }
    }

    fn feed_inputs(&mut self, inputs: &[i64]) {
        self.input.extend_from_slice(inputs)
    }

    fn resume(&mut self) -> Option<ExecutionOutput> {
        let VM { memory, input, pc } = self;
        let mut outputs = Vec::new();
        loop {
            let slice = memory.get(*pc..)?;
            let inst = Instruction::from_slice(slice)?;

            match &inst {
                Instruction::Halt => return Some(ExecutionOutput::Halt(outputs)),
                Instruction::Add { a, b, c } => {
                    *memory.get_mut(*c)? = a.get(memory)? + b.get(memory)?;
                    *pc += 4;
                }
                Instruction::Mul { a, b, c } => {
                    *memory.get_mut(*c)? = a.get(memory)? * b.get(memory)?;
                    *pc += 4;
                }
                Instruction::Input(pos) => {
                    if let Some(&value) = input.get(0) {
                        input.drain(0..1);
                        *memory.get_mut(*pos)? = value;
                        *pc += 2;
                    } else {
                        return Some(ExecutionOutput::SuspendInput(outputs));
                    }
                }
                Instruction::Output(param) => {
                    outputs.push(param.get(memory)?);
                    *pc += 2;
                }
                Instruction::JumpIfTrue { cond, next } => {
                    if cond.get(memory)? != 0 {
                        *pc = usize::try_from(next.get(memory)?).ok()?;
                    } else {
                        *pc += 3;
                    }
                }
                Instruction::JumpIfFalse { cond, next } => {
                    if cond.get(memory)? == 0 {
                        *pc = usize::try_from(next.get(memory)?).ok()?;
                    } else {
                        *pc += 3;
                    }
                }
                Instruction::LessThan { a, b, c } => {
                    memory[*c] = if a.get(memory) < b.get(memory) { 1 } else { 0 };
                    *pc += 4;
                }
                Instruction::Eq { a, b, c } => {
                    memory[*c] = if a.get(memory) == b.get(memory) { 1 } else { 0 };
                    *pc += 4;
                }
            }
        }
    }
}

fn thruster_output(memory: Vec<i64>, phase_settings: &[i64]) -> Option<i64> {
    let mut vms = Vec::new();
    for &phase in phase_settings {
        let mut vm = VM::new(memory.clone());
        vm.feed_inputs(&[phase]);
        vms.push(vm);
    }
    let mut queue = std::collections::vec_deque::VecDeque::new();
    for vm in vms.into_iter() {
        queue.push_back(vm);
    }

    let mut input = vec![0];
    while let Some(mut vm) = queue.pop_front() {
        vm.feed_inputs(input.as_slice());
        match vm.resume()? {
            ExecutionOutput::SuspendInput(output) => {
                input = output;
                queue.push_back(vm)
            }
            ExecutionOutput::Halt(output) => {
                input = output;
            }
        }
    }
    Some(input[0])
}

fn decode_phase(phase: i64) -> Option<Vec<i64>> {
    let mut result = Vec::new();
    for i in 0..5 {
        result.push((phase / 5_i64.pow(i)) % 5 + 5)
    }
    let mut count = [0; 5];
    for x in &result {
        count[*x as usize - 5] += 1;
    }
    if count.iter().all(|x| *x == 1) {
        Some(result)
    } else {
        None
    }
}

fn find_maximum_thruster_output(memory: Vec<i64>) -> Option<i64> {
    let mut max = 0;
    for phase_encoded in 0..5 * 5 * 5 * 5 * 5 {
        let phase = if let Some(phase) = decode_phase(phase_encoded) {
            phase
        } else {
            continue;
        };
        let output = thruster_output(memory.clone(), phase.as_slice());
        if let Some(output) = output {
            max = max.max(output);
        }
    }
    Some(max)
}

fn run<R: BufRead>(read: R, input: i64) -> Result<i64> {
    let program = get_program(read)?;
    let maximum = find_maximum_thruster_output(program).ok_or("program error")?;
    Ok(maximum)
}

const INPUT: i64 = 1;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let max = run(stdin.lock(), INPUT)?;
    println!("maximum: {}", max);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/07");
    assert_eq!(run(&input[..], INPUT).unwrap(), 19539216);
}
