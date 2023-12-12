use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::ast::{BalloonPosition, FlagType, ShellDescriptLine};

use super::parts::{char_id, digit, digit_neg};

pub(super) fn balloon_representation<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((
        sakura_balloon_offsetx,
        sakura_balloon_offsety,
        kero_balloon_offsetx,
        kero_balloon_offsety,
        sakura_balloon_alignment,
        kero_balloon_alignment,
        sakura_balloon_dontmove,
        kero_balloon_dontmove,
        char_balloon_dontmove,
    ))(input)
}

fn sakura_balloon_offsetx<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("sakura.balloon.offsetx,"), digit_neg), |v| {
        ShellDescriptLine::SakuraBalloonOffsetx(v)
    })(input)
}

fn sakura_balloon_offsety<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("sakura.balloon.offsety,"), digit_neg), |v| {
        ShellDescriptLine::SakuraBalloonOffsety(v)
    })(input)
}

fn kero_balloon_offsetx<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("kero.balloon.offsetx,"), digit_neg), |v| {
        ShellDescriptLine::KeroBalloonOffsetx(v)
    })(input)
}

fn kero_balloon_offsety<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("kero.balloon.offsety,"), digit_neg), |v| {
        ShellDescriptLine::KeroBalloonOffsety(v)
    })(input)
}

fn sakura_balloon_alignment<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("sakura.balloon.alignment,"), balloon_position),
        |v| ShellDescriptLine::SakuraBalloonAlignment(v),
    )(input)
}

fn kero_balloon_alignment<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("kero.balloon.alignment,"), balloon_position),
        |v| ShellDescriptLine::KeroBalloonAlignment(v),
    )(input)
}

fn sakura_balloon_dontmove<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("sakura.balloon.dontmove,"), balloon_dontmove),
        |v| ShellDescriptLine::SakuraBalloonDontmove(v),
    )(input)
}

fn kero_balloon_dontmove<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("kero.balloon.dontmove,"), balloon_dontmove),
        |v| ShellDescriptLine::KeroBalloonDontmove(v),
    )(input)
}

fn char_balloon_dontmove<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            char_id,
            preceded(tag(".balloon.dontmove,"), balloon_dontmove),
        )),
        |(id, v)| ShellDescriptLine::CharBalloonDontmove(id, v),
    )(input)
}

fn balloon_position<'a>(input: &'a str) -> IResult<&'a str, BalloonPosition, ShellParseError> {
    alt((
        map(tag("none"), |_| BalloonPosition::None),
        map(tag("left"), |_| BalloonPosition::Left),
        map(tag("right"), |_| BalloonPosition::Right),
    ))(input)
}

fn balloon_dontmove<'a>(input: &'a str) -> IResult<&'a str, FlagType, ShellParseError> {
    alt((digit, map(tag("true"), |_| 1)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod balloon_representation {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.balloon.offsetx,50";
            let (remain, result) = balloon_representation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraBalloonOffsetx(50));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.balloon.offsetx,";
            assert!(balloon_representation(case).is_err());
        }
    }

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

    mod sakura_balloon_alignment {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.balloon.alignment,left";
            let (remain, result) = sakura_balloon_alignment(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraBalloonAlignment(BalloonPosition::Left)
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.balloon.alignment,";
            assert!(sakura_balloon_alignment(case).is_err());
        }
    }

    mod kero_balloon_alignment {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.balloon.alignment,left";
            let (remain, result) = kero_balloon_alignment(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroBalloonAlignment(BalloonPosition::Left)
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.balloon.alignment,";
            assert!(kero_balloon_alignment(case).is_err());
        }
    }

    mod sakura_balloon_dontmove {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.balloon.dontmove,1";
            let (remain, result) = sakura_balloon_dontmove(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraBalloonDontmove(1));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.balloon.dontmove,";
            assert!(sakura_balloon_dontmove(case).is_err());
        }
    }

    mod kero_balloon_dontmove {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.balloon.dontmove,1";
            let (remain, result) = kero_balloon_dontmove(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroBalloonDontmove(1));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.balloon.dontmove,";
            assert!(kero_balloon_dontmove(case).is_err());
        }
    }

    mod char_balloon_dontmove {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.balloon.dontmove,1";
            let (remain, result) = char_balloon_dontmove(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::CharBalloonDontmove(5, 1));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char.balloon.dontmove,";
            assert!(char_balloon_dontmove(case).is_err());
        }
    }

    mod balloon_position {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "none";
            let (remain, result) = balloon_position(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, BalloonPosition::None);

            let case = "left";
            let (remain, result) = balloon_position(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, BalloonPosition::Left);

            let case = "right";
            let (remain, result) = balloon_position(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, BalloonPosition::Right);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(balloon_position(case).is_err());
        }
    }

    mod balloon_dontmove {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "1";
            let (remain, result) = balloon_dontmove(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, 1);

            let case = "true";
            let (remain, result) = balloon_dontmove(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, 1);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(balloon_dontmove(case).is_err());
        }
    }
}
