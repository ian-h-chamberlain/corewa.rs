use std::{error, fmt, str::FromStr};

use itertools::Itertools;
use pest::{
    error::{Error as PestError, ErrorVariant, LineColLocation},
    iterators::{Pair, Pairs},
};

use crate::load_file::{AddressMode, Core, Field, Instruction, Modifier, Opcode, Value};

mod grammar;

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.details)
    }
}

impl Error {
    pub fn no_input() -> Error {
        Error {
            details: "No input found".to_owned(),
        }
    }
}

impl From<PestError<grammar::Rule>> for Error {
    fn from(pest_error: PestError<grammar::Rule>) -> Error {
        Error {
            details: format!(
                "Parse error: {} {}",
                match pest_error.variant {
                    ErrorVariant::ParsingError {
                        positives,
                        negatives,
                    } => format!("expected one of {:?}, none of {:?}", positives, negatives),
                    ErrorVariant::CustomError { message } => message,
                },
                match pest_error.line_col {
                    LineColLocation::Pos((line, col)) => format!("at line {} column {}", line, col),
                    LineColLocation::Span((start_line, start_col), (end_line, end_col)) => format!(
                        "from line {} column {} to line {} column {}",
                        start_line, start_col, end_line, end_col
                    ),
                }
            ),
        }
    }
}

impl From<String> for Error {
    fn from(details: String) -> Error {
        Error { details }
    }
}

pub fn parse(file_contents: &str) -> Result<Core, Error> {
    if file_contents.is_empty() {
        return Err(Error::no_input());
    }

    let mut core = Core::default();

    let parse_result = grammar::parse(grammar::Rule::File, file_contents)?
        .next()
        .ok_or_else(Error::no_input)?;

    let mut i = 0;
    for mut line_pair in parse_result
        .into_inner()
        .map(Pair::into_inner)
        .filter(|line_pair| line_pair.peek().is_some())
    {
        let label_pairs = line_pair
            .take_while_ref(|pair| pair.as_rule() == grammar::Rule::Label)
            .map(|pair| pair.as_str().to_owned());

        for label in label_pairs {
            core.add_label(i, label.to_string())?;
        }

        if line_pair.peek().is_some() {
            core.set(i, parse_instruction(line_pair));
            i += 1;
        }
    }

    Ok(core)
}

fn parse_instruction(mut instruction_pairs: Pairs<grammar::Rule>) -> Instruction {
    let mut operation_pairs = instruction_pairs
        .next()
        .expect("Operation must be first pair after Label in Instruction")
        .into_inner();

    let opcode = parse_opcode(
        &operation_pairs
            .next()
            .expect("Opcode must be first pair in Operation"),
    );

    let maybe_modifier = operation_pairs
        .peek()
        .filter(|pair| pair.as_rule() == grammar::Rule::Modifier)
        .map(|pair| parse_modifier(&pair));

    let field_a = parse_field(
        instruction_pairs
            .next()
            .expect("Field must appear after Opcode"),
    );

    let field_b = instruction_pairs
        .next()
        .filter(|pair| pair.as_rule() == grammar::Rule::Field)
        .map_or_else(Field::default, parse_field);

    let modifier = maybe_modifier.unwrap_or_else(|| {
        Modifier::default_88_to_94(opcode, field_a.address_mode, field_b.address_mode)
    });

    Instruction {
        opcode,
        modifier,
        field_a,
        field_b,
    }
}

fn parse_modifier(modifier_pair: &Pair<grammar::Rule>) -> Modifier {
    Modifier::from_str(modifier_pair.as_str().to_uppercase().as_ref()).unwrap()
}

fn parse_opcode(opcode_pair: &Pair<grammar::Rule>) -> Opcode {
    Opcode::from_str(opcode_pair.as_str().to_uppercase().as_ref()).unwrap()
}

fn parse_field(field_pair: Pair<grammar::Rule>) -> Field {
    let field_pairs = field_pair.into_inner();

    let address_mode = field_pairs
        .peek()
        .filter(|pair| pair.as_rule() == grammar::Rule::AddressMode)
        .map_or(AddressMode::default(), |pair| {
            AddressMode::from_str(pair.as_str()).expect("Invalid AddressMode")
        });

    let value = parse_value(
        field_pairs
            .skip_while(|pair| pair.as_rule() != grammar::Rule::Expr)
            .next()
            .expect("No Expr in Field"),
    );

    Field {
        address_mode,
        value,
    }
}

fn parse_value(value_pair: Pair<grammar::Rule>) -> Value {
    let expr_inner = value_pair
        .into_inner()
        .next()
        .expect("Expr must have inner value");

    match expr_inner.as_rule() {
        grammar::Rule::Number => Value::Literal(
            i32::from_str_radix(expr_inner.as_str(), 10)
                .expect("Number type must be decimal integer"),
        ),
        grammar::Rule::Label => Value::Label(expr_inner.as_str().to_owned()),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let result = parse("");
        assert!(result.is_err());

        assert_eq!(result.unwrap_err().details, "No input found");
    }

    #[test]
    fn duplicate_labels() {
        let simple_input = "
            label1  dat 0,0
            label1  dat 0,0
        ";

        parse(simple_input).expect_err("Should fail for duplicate label");
    }

    #[test]
    fn parse_simple_file() {
        let simple_input = "
            preload
            begin:  mov 1, 3 ; make sure comments parse out
                    mov 100, #12
            loop:
            main    dat #0, #0
                    jmp +123, #45
                    jmp begin
                    jmp -1
        ";

        let mut expected_core = Core::default();

        expected_core.set(
            0,
            Instruction::new(Opcode::Mov, Field::direct(1), Field::direct(3)),
        );
        expected_core.set(
            1,
            Instruction::new(Opcode::Mov, Field::direct(100), Field::immediate(12)),
        );
        expected_core.set(
            2,
            Instruction::new(Opcode::Dat, Field::immediate(0), Field::immediate(0)),
        );
        expected_core.set(
            3,
            Instruction::new(Opcode::Jmp, Field::direct(123), Field::immediate(45)),
        );
        expected_core.set(
            4,
            Instruction::new(Opcode::Jmp, Field::direct(-4), Field::immediate(0)),
        );
        expected_core.set(
            5,
            Instruction::new(Opcode::Jmp, Field::direct(-1), Field::immediate(0)),
        );

        expected_core
            .resolve()
            .expect("Should resolve a core with no labels");

        let mut parsed = parse(simple_input).expect("Should parse simple file");
        parsed.resolve().expect("Parsed file should resolve");

        assert_eq!(parsed, expected_core);
    }
}
