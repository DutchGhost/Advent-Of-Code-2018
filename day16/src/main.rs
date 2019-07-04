use aoc::aoc;

macro_rules! assert_len {
    ($e:expr) => {
        match $e.len() {
            4 => {}
            len => {
                dbg!($e);
                panic!("Expected a slice with a len of 4, got len of {}", len);
            }
        }
    };
}

macro_rules! opcode_impl_ops {
    ([$fn_register:ident, $fn_immidiate:ident] -> $operation:tt $(,[$fn_registers:ident, $fn_immidiates:ident] -> $operations:tt)*) => {
        fn $fn_register(instruction: &[usize], registers: &[usize]) -> [usize; 4] {
            assert_len!(instruction);
            assert_len!(registers);

            let lhs = instruction[1];
            let rhs = instruction[2];
            let store = instruction[3];

            let mut output = copy_register(registers);

            output[store] = registers[lhs] $operation registers[rhs];
            output
        }

        pub fn $fn_immidiate(instruction: &[usize], registers: &[usize]) -> [usize; 4] {
            assert_len!(instruction);
            assert_len!(registers);

            let lhs = instruction[1];
            let rhs = instruction[2];
            let store = instruction[3];

            let mut output = copy_register(registers);

            output[store] = registers[lhs] $operation rhs;
            output
        }

        opcode_impl_ops!($([$fn_registers, $fn_immidiates] -> $operations),*);
    };
    () => {}
}

opcode_impl_ops!(
    [addr, addi] -> +,
    [mulr, muli] -> *,
    [banr, bani] -> &,
    [borr, bori] -> |
);

pub fn setr(instruction: &[usize], registers: &[usize]) -> [usize; 4] {
    assert_len!(instruction);
    assert_len!(registers);

    let src = instruction[1];
    let dst = instruction[3];

    let mut output = copy_register(registers);

    output[dst] = output[src];
    output
}

pub fn seti(instruction: &[usize], registers: &[usize]) -> [usize; 4] {
    assert_len!(instruction);
    assert_len!(registers);

    let src = instruction[1];
    let dst = instruction[3];

    let mut output = copy_register(registers);

    output[dst] = src;
    output
}

macro_rules! opcode_impl_eqs {
    ([$fn_val_reg:ident, $fn_reg_val:ident, $fn_reg_reg:ident] -> $operation:tt $(,[$fn_val_regs:ident, $fn_reg_vals:ident, $fn_reg_regs:ident] -> $operations:tt)*) => (
        fn $fn_val_reg(instruction: &[usize], registers: &[usize]) -> [usize; 4] {
            assert_len!(instruction);
            assert_len!(registers);

            let lhs = instruction[1];
            let rhs = instruction[2];
            let store = instruction[3];

            let mut output = copy_register(registers);

            if lhs $operation registers[rhs] {
                output[store] = 1;
            } else {
                output[store] = 0;
            }

            output
        }

        fn $fn_reg_val(instruction: &[usize], registers: &[usize]) -> [usize; 4] {
            assert_len!(instruction);
            assert_len!(registers);

            let lhs = instruction[1];
            let rhs = instruction[2];
            let store = instruction[3];

            let mut output = copy_register(registers);

            if registers[lhs] $operation rhs {
                output[store] = 1;
            } else {
                output[store] = 0;
            }

            output
        }

        fn $fn_reg_reg(instruction: &[usize], registers: &[usize]) -> [usize; 4] {
            assert_len!(instruction);
            assert_len!(registers);

            let lhs = instruction[1];
            let rhs = instruction[2];
            let store = instruction[3];

            let mut output = copy_register(registers);

            if registers[lhs] $operation registers[rhs] {
                output[store] = 1
            } else {
                output[store] = 0;
            }

            output
        }

        opcode_impl_eqs!($([$fn_val_regs, $fn_reg_vals, $fn_reg_regs] -> $operations),*);
    );

    () => {}

}

opcode_impl_eqs!(
    [gtir, gtri, gtrr] -> >,
    [eqir, eqri, eqrr] -> ==
);

fn copy_register(registers: &[usize]) -> [usize; 4] {
    assert_len!(registers);

    let mut output = [0; 4];
    output.copy_from_slice(registers);
    output
}

fn parse(input: &str) -> Vec<&str> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .collect()
}

fn clamp<'a>(
    iter: impl Iterator<Item = &'a [&'a str]> + 'a,
) -> impl Iterator<Item = [[usize; 4]; 3]> + 'a {
    iter.map(move |chunk| match chunk {
        [before, instruction, after] => {
            let mut before_array = [0; 4];

            for (src, dst) in before[9..before.len() - 1]
                .split(", ")
                .zip(before_array.iter_mut())
            {
                *dst = src.parse().unwrap();
            }

            let mut instruction_array = [0; 4];

            for (src, dst) in instruction.split(' ').zip(instruction_array.iter_mut()) {
                *dst = src.parse().unwrap();
            }

            let mut after_array = [0; 4];
            for (src, dst) in after[9..after.len() - 1]
                .split(", ")
                .zip(after_array.iter_mut())
            {
                *dst = src.parse().unwrap();
            }

            [before_array, instruction_array, after_array]
        }
        _ => unreachable!(),
    })
}

#[aoc(2018, 16, 1)]
fn main(input: &str) -> usize {
    let opcodes = [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    clamp(
        parse(input)
            .chunks(3)
            .take_while(|chunk| chunk[0].starts_with("Before")),
    )
    .filter(|line| match line {
        [before, instruction, after] => {
            opcodes
                .iter()
                .filter(|opcode| opcode(instruction, before)[..] == after[..])
                .count()
                >= 3
        }
    })
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr() {
        let instruction = [9, 1, 2, 3];
        let registers = [10, 20, 30, 40];

        let output = addr(&instruction, &registers);
        assert_eq!(output, [10, 20, 30, 50]);
    }

    #[test]
    fn test_addi() {
        let instruction = [9, 1, 2, 3];
        let registers = [10, 20, 30, 40];

        let output = addi(&instruction, &registers);
        assert_eq!(output, [10, 20, 30, 22]);
    }
}
