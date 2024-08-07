use thiserror::Error;

#[derive(Debug, Error)]
pub enum D19Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid input")]
    InvalidInput,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    LT,
    GT,
}

pub enum Instruction {
    Conditional(String, Operator, u32),
    Unconditional,
}

pub fn parse_instruction(line: String) -> Result<(String, Vec<(Instruction, String)>), D19Error> {
    let Some((wkey, rest)) = line.split_once('{') else {
        return Err(D19Error::InvalidInput);
    };
    let key = wkey.to_string();
    let rest = rest.trim_end_matches('}').to_string();

    let mut instructions = vec![];
    for s in rest.clone().split(',') {
        if let Some((cond, dest)) = s.split_once(':') {
            if let Some((key, val)) = cond.split_once('<') {
                instructions.push((
                    Instruction::Conditional(key.to_string(), Operator::LT, val.parse()?),
                    dest.to_string(),
                ));
            } else if let Some((key, val)) = cond.split_once('>') {
                instructions.push((
                    Instruction::Conditional(key.to_string(), Operator::GT, val.parse()?),
                    dest.to_string(),
                ));
            } else {
                return Err(D19Error::InvalidInput);
            }
        } else {
            instructions.push((Instruction::Unconditional, s.to_string()));
        }
    }
    Ok((key, instructions))
}
