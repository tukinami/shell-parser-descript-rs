use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::preceded, IResult};
use shell_parser_common_rs::ShellParseError;

use crate::ast::ShellDescriptLine;

use super::parts::digit;

pub(super) fn alpha<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((seriko_paint_transparent_region_black, seriko_use_self_alpha))(input)
}

fn seriko_paint_transparent_region_black<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("seriko.paint_transparent_region_black,"), digit),
        |v| ShellDescriptLine::SerikoPaintTransparentRegionBlack(v),
    )(input)
}

fn seriko_use_self_alpha<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("seriko.use_self_alpha,"), digit), |v| {
        ShellDescriptLine::SerikoUseSelfAlpha(v)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod alpha {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.paint_transparent_region_black,1";
            let (remain, result) = alpha(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SerikoPaintTransparentRegionBlack(1)
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.paint_transparent_region_black,";
            assert!(alpha(case).is_err());
        }
    }

    mod seriko_paint_transparent_region_black {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.paint_transparent_region_black,1";
            let (remain, result) = seriko_paint_transparent_region_black(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SerikoPaintTransparentRegionBlack(1)
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.paint_transparent_region_black,";
            assert!(seriko_paint_transparent_region_black(case).is_err());
        }
    }

    mod seriko_use_self_alpha {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.use_self_alpha,1";
            let (remain, result) = seriko_use_self_alpha(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SerikoUseSelfAlpha(1));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.use_self_alpha,";
            assert!(seriko_use_self_alpha(case).is_err());
        }
    }
}
