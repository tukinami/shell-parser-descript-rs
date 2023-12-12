use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::ast::{CharacterIdType, ShellDescriptLine, SurfacePosition};

use super::parts::{char_id, digit, digit_neg};

pub(super) fn shell_representation<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((
        seriko_zorder,
        seriko_sticky_window,
        seriko_alignmenttodesktop,
        sakura_seriko_alignmenttodesktop,
        kero_seriko_alignmenttodesktop,
        char_seriko_alignmenttodesktop,
        sakura_defaultx,
        kero_defaultx,
        char_defaultx,
        sakura_defaulty,
        kero_defaulty,
        char_defaulty,
        sakura_defaultleft,
        kero_defaultleft,
        char_defaultleft,
        sakura_defaulttop,
        kero_defaulttop,
        char_defaulttop,
    ))(input)
}

fn seriko_zorder<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("seriko.zorder,"), scope_ids), |v| {
        ShellDescriptLine::SerikoZOrder(v)
    })(input)
}

fn seriko_sticky_window<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("seriko.sticky-window,"), scope_ids), |v| {
        ShellDescriptLine::SerikoStickyWindow(v)
    })(input)
}

fn seriko_alignmenttodesktop<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("seriko.alignmenttodesktop,"), surface_poition),
        |v| ShellDescriptLine::SerikoAlignmenttodesktop(v),
    )(input)
}
fn sakura_seriko_alignmenttodesktop<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("sakura.seriko.alignmenttodesktop,"), surface_poition),
        |v| ShellDescriptLine::SakuraSerikoAlignmenttodesktop(v),
    )(input)
}

fn kero_seriko_alignmenttodesktop<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("kero.seriko.alignmenttodesktop,"), surface_poition),
        |v| ShellDescriptLine::KeroSerikoAlignmenttodesktop(v),
    )(input)
}

fn char_seriko_alignmenttodesktop<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            char_id,
            preceded(tag(".seriko.alignmenttodesktop,"), surface_poition),
        )),
        |(id, v)| ShellDescriptLine::CharSerikoAlignmenttodesktop(id, v),
    )(input)
}

fn char_defaultx<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((char_id, preceded(tag(".defaultx,"), digit_neg))),
        |(id, v)| ShellDescriptLine::CharDefaultx(id, v),
    )(input)
}

fn char_defaulty<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((char_id, preceded(tag(".defaulty,"), digit_neg))),
        |(id, v)| ShellDescriptLine::CharDefaulty(id, v),
    )(input)
}

fn char_defaultleft<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((char_id, preceded(tag(".defaultleft,"), digit_neg))),
        |(id, v)| ShellDescriptLine::CharDefaultleft(id, v),
    )(input)
}

fn char_defaulttop<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((char_id, preceded(tag(".defaulttop,"), digit_neg))),
        |(id, v)| ShellDescriptLine::CharDefaulttop(id, v),
    )(input)
}

macro_rules! line_has_only_desktop_position {
    ($name:ident, $tag:expr, $pat:path) => {
        fn $name<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
            map(preceded(tag($tag), digit_neg), |v| $pat(v))(input)
        }
    };
}

line_has_only_desktop_position!(
    sakura_defaultx,
    "sakura.defaultx,",
    ShellDescriptLine::SakuraDefaultx
);
line_has_only_desktop_position!(
    kero_defaultx,
    "kero.defaultx,",
    ShellDescriptLine::KeroDefaultx
);

line_has_only_desktop_position!(
    sakura_defaulty,
    "sakura.defaulty,",
    ShellDescriptLine::SakuraDefaulty
);
line_has_only_desktop_position!(
    kero_defaulty,
    "kero.defaulty,",
    ShellDescriptLine::KeroDefaulty
);

line_has_only_desktop_position!(
    sakura_defaultleft,
    "sakura.defaultleft,",
    ShellDescriptLine::SakuraDefaultleft
);
line_has_only_desktop_position!(
    kero_defaultleft,
    "kero.defaultleft,",
    ShellDescriptLine::KeroDefaultleft
);

line_has_only_desktop_position!(
    sakura_defaulttop,
    "sakura.defaulttop,",
    ShellDescriptLine::SakuraDefaulttop
);
line_has_only_desktop_position!(
    kero_defaulttop,
    "kero.defaulttop,",
    ShellDescriptLine::KeroDefaulttop
);

fn scope_ids<'a>(input: &'a str) -> IResult<&'a str, Vec<CharacterIdType>, ShellParseError> {
    separated_list1(tag(","), digit)(input)
}

fn surface_poition<'a>(input: &'a str) -> IResult<&'a str, SurfacePosition, ShellParseError> {
    alt((
        map(tag("top"), |_| SurfacePosition::Top),
        map(tag("bottom"), |_| SurfacePosition::Bottom),
        map(tag("free"), |_| SurfacePosition::Free),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod shell_representation {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.zorder,10\r\n";
            let (remain, result) = shell_representation(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::SerikoZOrder(vec![10]));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.zorder,a";
            assert!(shell_representation(case).is_err());
        }
    }

    mod seriko_zorder {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.zorder,10\r\n";
            let (remain, result) = seriko_zorder(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::SerikoZOrder(vec![10]));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.zorder,a";
            assert!(seriko_zorder(case).is_err());
        }
    }

    mod seriko_sticky_window {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.sticky-window,10\r\n";
            let (remain, result) = seriko_sticky_window(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::SerikoStickyWindow(vec![10]));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.sticky-window,a";
            assert!(seriko_sticky_window(case).is_err());
        }
    }

    mod seriko_alignmenttodesktop {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.alignmenttodesktop,free";
            let (remain, result) = seriko_alignmenttodesktop(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SerikoAlignmenttodesktop(SurfacePosition::Free)
            );
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "seriko.alignmenttodesktop,";
            assert!(seriko_alignmenttodesktop(case).is_err());
        }
    }

    mod sakura_seriko_alignmenttodesktop {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.seriko.alignmenttodesktop,free";
            let (remain, result) = sakura_seriko_alignmenttodesktop(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraSerikoAlignmenttodesktop(SurfacePosition::Free)
            );
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "sakura.seriko.alignmenttodesktop,";
            assert!(sakura_seriko_alignmenttodesktop(case).is_err());
        }
    }

    mod kero_seriko_alignmenttodesktop {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.seriko.alignmenttodesktop,free";
            let (remain, result) = kero_seriko_alignmenttodesktop(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroSerikoAlignmenttodesktop(SurfacePosition::Free)
            );
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "kero.seriko.alignmenttodesktop,";
            assert!(kero_seriko_alignmenttodesktop(case).is_err());
        }
    }

    mod char_seriko_alignmenttodesktop {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.seriko.alignmenttodesktop,free";
            let (remain, result) = char_seriko_alignmenttodesktop(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::CharSerikoAlignmenttodesktop(5, SurfacePosition::Free)
            );
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "char5.seriko.alignmenttodesktop,";
            assert!(char_seriko_alignmenttodesktop(case).is_err());
        }
    }

    mod char_defaultx {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.defaultx,10";
            let (remain, result) = char_defaultx(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::CharDefaultx(5, 10));
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "char5.defaultx,";
            assert!(char_defaultx(case).is_err());
        }
    }

    mod char_defaulty {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.defaulty,10";
            let (remain, result) = char_defaulty(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::CharDefaulty(5, 10));
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "char5.defaulty,";
            assert!(char_defaulty(case).is_err());
        }
    }

    mod char_defaultleft {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.defaultleft,10";
            let (remain, result) = char_defaultleft(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::CharDefaultleft(5, 10));
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "char5.defaultleft,";
            assert!(char_defaultleft(case).is_err());
        }
    }

    mod char_defaulttop {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.defaulttop,10";
            let (remain, result) = char_defaulttop(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::CharDefaulttop(5, 10));
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "char5.defaulttop,";
            assert!(char_defaulttop(case).is_err());
        }
    }

    mod sakura_defaultx {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.defaultx,10";
            let (remain, result) = sakura_defaultx(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraDefaultx(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.defaultx,";
            assert!(sakura_defaultx(case).is_err());
        }
    }

    mod kero_defaultx {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.defaultx,10";
            let (remain, result) = kero_defaultx(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroDefaultx(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.defaultx,";
            assert!(kero_defaultx(case).is_err());
        }
    }

    mod sakura_defaulty {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.defaulty,10";
            let (remain, result) = sakura_defaulty(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraDefaulty(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.defaulty,";
            assert!(sakura_defaulty(case).is_err());
        }
    }

    mod kero_defaulty {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.defaulty,10";
            let (remain, result) = kero_defaulty(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroDefaulty(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.defaulty,";
            assert!(kero_defaulty(case).is_err());
        }
    }

    mod sakura_defaultleft {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.defaultleft,10";
            let (remain, result) = sakura_defaultleft(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraDefaultleft(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.defaultleft,";
            assert!(sakura_defaultleft(case).is_err());
        }
    }

    mod kero_defaultleft {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.defaultleft,10";
            let (remain, result) = kero_defaultleft(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroDefaultleft(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.defaultleft,";
            assert!(kero_defaultleft(case).is_err());
        }
    }

    mod sakura_defaulttop {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.defaulttop,10";
            let (remain, result) = sakura_defaulttop(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraDefaulttop(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.defaulttop,";
            assert!(sakura_defaulttop(case).is_err());
        }
    }

    mod kero_defaulttop {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.defaulttop,10";
            let (remain, result) = kero_defaulttop(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroDefaulttop(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.defaulttop,";
            assert!(kero_defaulttop(case).is_err());
        }
    }

    mod scope_ids {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "10,20,30,40,50\r\n";
            let (remain, result) = scope_ids(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, vec![10, 20, 30, 40, 50]);
        }

        #[test]
        fn failed_when_valid_str() {
            let case = "a,20,30,40,\r\n";
            assert!(scope_ids(case).is_err());
        }
    }

    mod surface_poition {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "top";
            let (remain, result) = surface_poition(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfacePosition::Top);

            let case = "bottom";
            let (remain, result) = surface_poition(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfacePosition::Bottom);

            let case = "free";
            let (remain, result) = surface_poition(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfacePosition::Free);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(surface_poition(case).is_err());
        }
    }
}
