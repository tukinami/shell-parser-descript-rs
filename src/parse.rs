use nom::{
    branch::alt, combinator::eof, combinator::map, multi::many0, sequence::terminated, IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::ast::{LineContainer, ShellDescript, ShellDescriptLine};

use self::parts::{empty_line, parse_line_func};

mod balloon_representation;
mod base;
mod parts;
mod shell_representation;

pub fn parse<'a>(input: &'a str) -> Result<ShellDescript, nom::Err<ShellParseError>> {
    shell_descript(input).map(|(_, v)| v)
}

fn shell_descript<'a>(input: &'a str) -> IResult<&'a str, ShellDescript, ShellParseError> {
    map(terminated(parse_lines, eof), |v| ShellDescript::new(v))(input)
}

fn parse_lines<'a>(input: &'a str) -> IResult<&'a str, Vec<LineContainer>, ShellParseError> {
    many0(parse_line)(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, LineContainer, ShellParseError> {
    alt((parse_line_func(shell_descript_line), empty_line))(input)
}

fn shell_descript_line<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((base::base, shell_representation::shell_representation))(input)
}
