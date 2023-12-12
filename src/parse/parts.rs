use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{eof, map, map_res, opt},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::ast::{CharacterIdType, LineContainer, ShellDescriptLine};

pub(super) fn newline_body<'a>(input: &'a str) -> IResult<&'a str, &'a str, ShellParseError> {
    alt((tag("\r\n"), tag("\r"), tag("\n")))(input)
}

pub(super) fn parse_line_func<'a, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, LineContainer, ShellParseError>
where
    F: FnMut(&'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError>,
{
    map(terminated(f, alt((newline_body, eof))), |v| {
        LineContainer::Body(v)
    })
}

pub(super) fn empty_line<'a>(input: &'a str) -> IResult<&'a str, LineContainer, ShellParseError> {
    map(newline_body, |_| LineContainer::EmptyLine)(input)
}

pub(super) fn digit<'a, T>(input: &'a str) -> IResult<&'a str, T, ShellParseError>
where
    T: FromStr,
{
    map_res(digit1, |v: &str| v.parse())(input)
}

pub(super) fn digit_neg<'a, T>(input: &'a str) -> IResult<&'a str, T, ShellParseError>
where
    T: FromStr + std::ops::Neg<Output = T>,
{
    map(tuple((opt(tag("-")), digit::<T>)), |(sign, v)| {
        if sign.is_some() {
            -v
        } else {
            v
        }
    })(input)
}

pub(super) fn char_id<'a>(input: &'a str) -> IResult<&'a str, CharacterIdType, ShellParseError> {
    preceded(tag("char"), digit)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod newline_body {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "\r\n";
            let (remain, result) = newline_body(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, "\r\n");

            let case = "\r\r";
            let (remain, result) = newline_body(case).unwrap();
            assert_eq!(remain, "\r");
            assert_eq!(result, "\r");

            let case = "\n\n";
            let (remain, result) = newline_body(case).unwrap();
            assert_eq!(remain, "\n");
            assert_eq!(result, "\n");
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "abc";
            assert!(newline_body(case).is_err());
        }
    }

    mod parse_line_func {
        use shell_parser_common_rs::charset::Charset;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_t_func = map(tag("abc"), |_| ShellDescriptLine::Charset(Charset::ASCII));
            let mut case_func = parse_line_func(case_t_func);

            let case = "abc\r\n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                LineContainer::Body(ShellDescriptLine::Charset(Charset::ASCII))
            );

            let case = "abc";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                LineContainer::Body(ShellDescriptLine::Charset(Charset::ASCII))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case_t_func = map(tag("abc"), |_| ShellDescriptLine::Charset(Charset::ASCII));
            let mut case_func = parse_line_func(case_t_func);

            let case = "abcdef";
            assert!(case_func(case).is_err());
        }
    }

    mod empty_line {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "\r\nabc";
            let (remain, result) = empty_line(case).unwrap();
            assert_eq!(remain, "abc");
            assert_eq!(result, LineContainer::EmptyLine);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "abc\r\n";
            assert!(empty_line(case).is_err());
        }
    }

    mod digit {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "123a";
            let (remain, result) = digit::<i32>(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, 123);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "XYZ";
            assert!(digit::<i32>(case).is_err());
        }
    }

    mod digit_neg {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "1234\r\n";
            let (remain, result) = digit_neg::<i16>(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, 1234);

            let case = "-1234\r\n";
            let (remain, result) = digit_neg::<i16>(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, -1234);

            let case = "0";
            let (remain, result) = digit_neg::<i8>(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, 0);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "abc";
            assert!(digit_neg::<i8>(case).is_err());
        }
    }

    mod char_id {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char0.";
            let (remain, result) = char_id(case).unwrap();
            assert_eq!(remain, ".");
            assert_eq!(result, 0);

            let case = "char256.";
            let (remain, result) = char_id(case).unwrap();
            assert_eq!(remain, ".");
            assert_eq!(result, 256);
        }
    }
}
