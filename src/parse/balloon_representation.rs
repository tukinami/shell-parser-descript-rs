use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::preceded, IResult};
use shell_parser_common_rs::ShellParseError;

use crate::ast::ShellDescriptLine;

use super::parts::digit;

pub(super) fn balloon_representation<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((
        sakura_balloon_offsetx,
        sakura_balloon_offsety,
        kero_balloon_offsetx,
        kero_balloon_offsety,
    ))(input)
}

fn sakura_balloon_offsetx<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("sakura.balloon.offsetx,"), digit), |v| {
        ShellDescriptLine::SakuraBalloonOffsetx(v)
    })(input)
}

fn sakura_balloon_offsety<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("sakura.balloon.offsety,"), digit), |v| {
        ShellDescriptLine::SakuraBalloonOffsety(v)
    })(input)
}

fn kero_balloon_offsetx<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("kero.balloon.offsetx,"), digit), |v| {
        ShellDescriptLine::KeroBalloonOffsetx(v)
    })(input)
}

fn kero_balloon_offsety<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("kero.balloon.offsety,"), digit), |v| {
        ShellDescriptLine::KeroBalloonOffsety(v)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sakura_balloon_offsetx {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.balloon.offsetx,50";
            let (remain, result) = sakura_balloon_offsetx(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraBalloonOffsetx(50));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.balloon.offsetx,";
            assert!(sakura_balloon_offsetx(case).is_err());
        }
    }

    mod sakura_balloon_offsety {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.balloon.offsety,50";
            let (remain, result) = sakura_balloon_offsety(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraBalloonOffsety(50));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.balloon.offsety,";
            assert!(sakura_balloon_offsety(case).is_err());
        }
    }

    mod kero_balloon_offsetx {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.balloon.offsetx,50";
            let (remain, result) = kero_balloon_offsetx(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroBalloonOffsetx(50));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.balloon.offsetx,";
            assert!(kero_balloon_offsetx(case).is_err());
        }
    }

    mod kero_balloon_offsety {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.balloon.offsety,50";
            let (remain, result) = kero_balloon_offsety(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroBalloonOffsety(50));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.balloon.offsety,";
            assert!(kero_balloon_offsety(case).is_err());
        }
    }
}
