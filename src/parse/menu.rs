use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::ast::{
    MenuPositionForegroundBackgroundBase, MenuPositionForegroundBackgroundRepeat,
    MenuPositionSidebarBase, MenuPositionSidebarRepeat, ShellDescriptLine,
};

use super::parts::digit;

pub(super) fn menu<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((menu_1, menu_2))(input)
}

fn menu_1<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((
        menu_font_name,
        menu_font_height,
        menu_background_bitmap_filename,
        menu_foreground_bitmap_filename,
        menu_sidebar_bitmap_filename,
        menu_background_font_color_r,
        menu_background_font_color_g,
        menu_background_font_color_b,
        menu_foreground_font_color_r,
        menu_foreground_font_color_g,
        menu_foreground_font_color_b,
        menu_separator_color_r,
        menu_separator_color_g,
        menu_separator_color_b,
        menu_frame_color_r,
        menu_frame_color_g,
        menu_frame_color_b,
        menu_disable_font_color_r,
        menu_disable_font_color_g,
        menu_disable_font_color_b,
    ))(input)
}

fn menu_2<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((
        menu_background_alignment,
        menu_foreground_alignment,
        menu_sidebar_alignment,
    ))(input)
}

fn menu_font_name<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("menu.font.name,"), is_not("\r\n")),
        |v: &str| ShellDescriptLine::MenuFontName(v.to_string()),
    )(input)
}

fn menu_font_height<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("menu.font.height,"), digit), |v| {
        ShellDescriptLine::MenuFontHeight(v)
    })(input)
}

fn menu_background_bitmap_filename<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("menu.background.bitmap.filename,"), is_not("\r\n")),
        |v: &str| ShellDescriptLine::MenuBackgroundBitmapFilename(v.to_string()),
    )(input)
}

fn menu_foreground_bitmap_filename<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("menu.foreground.bitmap.filename,"), is_not("\r\n")),
        |v: &str| ShellDescriptLine::MenuForegroundBitmapFilename(v.to_string()),
    )(input)
}

fn menu_sidebar_bitmap_filename<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(tag("menu.sidebar.bitmap.filename,"), is_not("\r\n")),
        |v: &str| ShellDescriptLine::MenuSidebarBitmapFilename(v.to_string()),
    )(input)
}

fn menu_background_alignment<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(
            tag("menu.background.alignment,"),
            menu_position_foreground_and_background,
        ),
        |(b, r1, r2)| ShellDescriptLine::MenuBackgroundAlignment(b, r1, r2),
    )(input)
}

fn menu_foreground_alignment<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(
            tag("menu.foreground.alignment,"),
            menu_position_foreground_and_background,
        ),
        |(b, r1, r2)| ShellDescriptLine::MenuForegroundAlignment(b, r1, r2),
    )(input)
}

fn menu_sidebar_alignment<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        preceded(
            tag("menu.sidebar.alignment,"),
            tuple((
                menu_position_sidebar_base,
                opt(menu_position_sidebar_repeat),
            )),
        ),
        |(b, r)| ShellDescriptLine::MenuSidebarAlignment(b, r),
    )(input)
}

fn menu_position_foreground_and_background<'a>(
    input: &'a str,
) -> IResult<
    &'a str,
    (
        MenuPositionForegroundBackgroundBase,
        Option<MenuPositionForegroundBackgroundRepeat>,
        Option<MenuPositionForegroundBackgroundRepeat>,
    ),
    ShellParseError,
> {
    tuple((
        menu_position_foreground_and_background_base,
        opt(menu_position_foreground_and_background_repeat),
        opt(menu_position_foreground_and_background_repeat),
    ))(input)
}

fn menu_position_foreground_and_background_base<'a>(
    input: &'a str,
) -> IResult<&'a str, MenuPositionForegroundBackgroundBase, ShellParseError> {
    alt((
        map(tag("lefttop"), |_| {
            MenuPositionForegroundBackgroundBase::Lefttop
        }),
        map(tag("centertop"), |_| {
            MenuPositionForegroundBackgroundBase::Centertop
        }),
        map(tag("righttop"), |_| {
            MenuPositionForegroundBackgroundBase::Righttop
        }),
        map(tag("leftbottom"), |_| {
            MenuPositionForegroundBackgroundBase::Leftbottom
        }),
        map(tag("centerbottom"), |_| {
            MenuPositionForegroundBackgroundBase::Centerbottom
        }),
        map(tag("rightbottom"), |_| {
            MenuPositionForegroundBackgroundBase::Rightbottom
        }),
    ))(input)
}

fn menu_position_foreground_and_background_repeat<'a>(
    input: &'a str,
) -> IResult<&'a str, MenuPositionForegroundBackgroundRepeat, ShellParseError> {
    alt((
        map(tag("+repeat-x"), |_| {
            MenuPositionForegroundBackgroundRepeat::RepeatX
        }),
        map(tag("+repeat-y"), |_| {
            MenuPositionForegroundBackgroundRepeat::RepeatY
        }),
    ))(input)
}

fn menu_position_sidebar_base<'a>(
    input: &'a str,
) -> IResult<&'a str, MenuPositionSidebarBase, ShellParseError> {
    alt((
        map(tag("top"), |_| MenuPositionSidebarBase::Top),
        map(tag("bottom"), |_| MenuPositionSidebarBase::Bottom),
    ))(input)
}

fn menu_position_sidebar_repeat<'a>(
    input: &'a str,
) -> IResult<&'a str, MenuPositionSidebarRepeat, ShellParseError> {
    map(tag("+repeat-y"), |_| MenuPositionSidebarRepeat::RepeatY)(input)
}

macro_rules! line_has_only_color_type {
    ($name:ident, $tag:expr, $pat:path) => {
        fn $name<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
            map(preceded(tag($tag), digit), |v| $pat(v))(input)
        }
    };
}

line_has_only_color_type!(
    menu_background_font_color_r,
    "menu.background.font.color.r,",
    ShellDescriptLine::MenuBackgroundFontColorR
);
line_has_only_color_type!(
    menu_background_font_color_g,
    "menu.background.font.color.g,",
    ShellDescriptLine::MenuBackgroundFontColorG
);
line_has_only_color_type!(
    menu_background_font_color_b,
    "menu.background.font.color.b,",
    ShellDescriptLine::MenuBackgroundFontColorB
);
line_has_only_color_type!(
    menu_foreground_font_color_r,
    "menu.foreground.font.color.r,",
    ShellDescriptLine::MenuForegroundFontColorR
);
line_has_only_color_type!(
    menu_foreground_font_color_g,
    "menu.foreground.font.color.g,",
    ShellDescriptLine::MenuForegroundFontColorG
);
line_has_only_color_type!(
    menu_foreground_font_color_b,
    "menu.foreground.font.color.b,",
    ShellDescriptLine::MenuForegroundFontColorB
);
line_has_only_color_type!(
    menu_separator_color_r,
    "menu.separator.color.r,",
    ShellDescriptLine::MenuSeparatorColorR
);
line_has_only_color_type!(
    menu_separator_color_g,
    "menu.separator.color.g,",
    ShellDescriptLine::MenuSeparatorColorG
);
line_has_only_color_type!(
    menu_separator_color_b,
    "menu.separator.color.b,",
    ShellDescriptLine::MenuSeparatorColorB
);
line_has_only_color_type!(
    menu_frame_color_r,
    "menu.frame.color.r,",
    ShellDescriptLine::MenuFrameColorR
);
line_has_only_color_type!(
    menu_frame_color_g,
    "menu.frame.color.g,",
    ShellDescriptLine::MenuFrameColorG
);
line_has_only_color_type!(
    menu_frame_color_b,
    "menu.frame.color.b,",
    ShellDescriptLine::MenuFrameColorB
);
line_has_only_color_type!(
    menu_disable_font_color_r,
    "menu.disable.font.color.r,",
    ShellDescriptLine::MenuDisableFontColorR
);
line_has_only_color_type!(
    menu_disable_font_color_g,
    "menu.disable.font.color.g,",
    ShellDescriptLine::MenuDisableFontColorG
);
line_has_only_color_type!(
    menu_disable_font_color_b,
    "menu.disable.font.color.b,",
    ShellDescriptLine::MenuDisableFontColorB
);

#[cfg(test)]
mod tests {
    use super::*;

    mod menu {
        use super::*;

        #[test]
        fn succes_when_valid_str() {
            let case = "menu.font.name,ＭＳ ゴシック";
            let (remain, result) = menu(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuFontName("ＭＳ ゴシック".to_string())
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.font.name,";
            assert!(menu(case).is_err());
        }
    }

    mod menu_font_name {
        use super::*;

        #[test]
        fn succes_when_valid_str() {
            let case = "menu.font.name,ＭＳ ゴシック";
            let (remain, result) = menu_font_name(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuFontName("ＭＳ ゴシック".to_string())
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.font.name,";
            assert!(menu_font_name(case).is_err());
        }
    }

    mod menu_font_height {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.font.height,10";
            let (remain, result) = menu_font_height(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuFontHeight(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.font.height,";
            assert!(menu_font_height(case).is_err());
        }
    }

    mod menu_background_bitmap_filename {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.background.bitmap.filename,something.png";
            let (remain, result) = menu_background_bitmap_filename(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuBackgroundBitmapFilename("something.png".to_string())
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.background.bitmap.filename,";
            assert!(menu_background_bitmap_filename(case).is_err());
        }
    }

    mod menu_foreground_bitmap_filename {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.foreground.bitmap.filename,something.png";
            let (remain, result) = menu_foreground_bitmap_filename(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuForegroundBitmapFilename("something.png".to_string())
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.foreground.bitmap.filename,";
            assert!(menu_foreground_bitmap_filename(case).is_err());
        }
    }

    mod menu_sidebar_bitmap_filename {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.sidebar.bitmap.filename,something.png";
            let (remain, result) = menu_sidebar_bitmap_filename(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuSidebarBitmapFilename("something.png".to_string())
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.sidebar.bitmap.filename,";
            assert!(menu_sidebar_bitmap_filename(case).is_err());
        }
    }

    mod menu_background_alignment {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.background.alignment,lefttop";
            let (remain, result) = menu_background_alignment(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuBackgroundAlignment(
                    MenuPositionForegroundBackgroundBase::Lefttop,
                    None,
                    None
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.background.alignment,";
            assert!(menu_background_alignment(case).is_err());
        }
    }

    mod menu_foreground_alignment {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.foreground.alignment,lefttop";
            let (remain, result) = menu_foreground_alignment(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuForegroundAlignment(
                    MenuPositionForegroundBackgroundBase::Lefttop,
                    None,
                    None
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.foreground.alignment,";
            assert!(menu_foreground_alignment(case).is_err());
        }
    }

    mod menu_sidebar_alignment {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.sidebar.alignment,top";
            let (remain, result) = menu_sidebar_alignment(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuSidebarAlignment(MenuPositionSidebarBase::Top, None)
            );

            let case = "menu.sidebar.alignment,bottom+repeat-y";
            let (remain, result) = menu_sidebar_alignment(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::MenuSidebarAlignment(
                    MenuPositionSidebarBase::Bottom,
                    Some(MenuPositionSidebarRepeat::RepeatY)
                )
            );
        }
    }

    mod menu_position_foreground_and_background {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "lefttop";
            let (remain, result) = menu_position_foreground_and_background(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                (MenuPositionForegroundBackgroundBase::Lefttop, None, None)
            );

            let case = "lefttop+repeat-x";
            let (remain, result) = menu_position_foreground_and_background(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                (
                    MenuPositionForegroundBackgroundBase::Lefttop,
                    Some(MenuPositionForegroundBackgroundRepeat::RepeatX),
                    None
                )
            );

            let case = "lefttop+repeat-y+repeat-x";
            let (remain, result) = menu_position_foreground_and_background(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                (
                    MenuPositionForegroundBackgroundBase::Lefttop,
                    Some(MenuPositionForegroundBackgroundRepeat::RepeatY),
                    Some(MenuPositionForegroundBackgroundRepeat::RepeatX)
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong+repeat-x";
            assert!(menu_position_foreground_and_background(case).is_err());
        }
    }

    mod menu_position_foreground_and_background_base {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "lefttop";
            let (remain, result) = menu_position_foreground_and_background_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundBase::Lefttop);

            let case = "centertop";
            let (remain, result) = menu_position_foreground_and_background_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundBase::Centertop);

            let case = "righttop";
            let (remain, result) = menu_position_foreground_and_background_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundBase::Righttop);

            let case = "leftbottom";
            let (remain, result) = menu_position_foreground_and_background_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundBase::Leftbottom);

            let case = "centerbottom";
            let (remain, result) = menu_position_foreground_and_background_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundBase::Centerbottom);

            let case = "rightbottom";
            let (remain, result) = menu_position_foreground_and_background_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundBase::Rightbottom);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(menu_position_foreground_and_background_base(case).is_err());
        }
    }

    mod menu_position_foreground_and_background_repeat {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "+repeat-x";
            let (remain, result) = menu_position_foreground_and_background_repeat(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundRepeat::RepeatX);

            let case = "+repeat-y";
            let (remain, result) = menu_position_foreground_and_background_repeat(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionForegroundBackgroundRepeat::RepeatY);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "+repeat-W";
            assert!(menu_position_foreground_and_background_repeat(case).is_err());
        }
    }

    mod menu_position_sidebar_base {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "top";
            let (remain, result) = menu_position_sidebar_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionSidebarBase::Top);

            let case = "bottom";
            let (remain, result) = menu_position_sidebar_base(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionSidebarBase::Bottom);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(menu_position_sidebar_base(case).is_err());
        }
    }

    mod menu_position_sidebar_repeat {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "+repeat-y";
            let (remain, result) = menu_position_sidebar_repeat(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, MenuPositionSidebarRepeat::RepeatY);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(menu_position_sidebar_repeat(case).is_err());
        }
    }

    mod menu_background_font_color_r {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.background.font.color.r,10";
            let (remain, result) = menu_background_font_color_r(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuBackgroundFontColorR(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.background.font.color.r,";
            assert!(menu_background_font_color_r(case).is_err());
        }
    }

    mod menu_background_font_color_g {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.background.font.color.g,10";
            let (remain, result) = menu_background_font_color_g(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuBackgroundFontColorG(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.background.font.color.g,";
            assert!(menu_background_font_color_g(case).is_err());
        }
    }

    mod menu_background_font_color_b {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.background.font.color.b,10";
            let (remain, result) = menu_background_font_color_b(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuBackgroundFontColorB(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.background.font.color.b,";
            assert!(menu_background_font_color_b(case).is_err());
        }
    }

    mod menu_foreground_font_color_r {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.foreground.font.color.r,10";
            let (remain, result) = menu_foreground_font_color_r(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuForegroundFontColorR(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.foreground.font.color.r,";
            assert!(menu_foreground_font_color_r(case).is_err());
        }
    }

    mod menu_foreground_font_color_g {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.foreground.font.color.g,10";
            let (remain, result) = menu_foreground_font_color_g(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuForegroundFontColorG(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.foreground.font.color.g,";
            assert!(menu_foreground_font_color_g(case).is_err());
        }
    }

    mod menu_foreground_font_color_b {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.foreground.font.color.b,10";
            let (remain, result) = menu_foreground_font_color_b(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuForegroundFontColorB(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.foreground.font.color.b,";
            assert!(menu_foreground_font_color_b(case).is_err());
        }
    }

    mod menu_separator_color_r {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.separator.color.r,10";
            let (remain, result) = menu_separator_color_r(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuSeparatorColorR(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.separator.color.r,";
            assert!(menu_separator_color_r(case).is_err());
        }
    }

    mod menu_separator_color_g {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.separator.color.g,10";
            let (remain, result) = menu_separator_color_g(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuSeparatorColorG(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.separator.color.g,";
            assert!(menu_separator_color_g(case).is_err());
        }
    }

    mod menu_separator_color_b {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.separator.color.b,10";
            let (remain, result) = menu_separator_color_b(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuSeparatorColorB(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.separator.color.b,";
            assert!(menu_separator_color_b(case).is_err());
        }
    }

    mod menu_frame_color_r {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.frame.color.r,10";
            let (remain, result) = menu_frame_color_r(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuFrameColorR(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.frame.color.r,";
            assert!(menu_frame_color_r(case).is_err());
        }
    }

    mod menu_frame_color_g {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.frame.color.g,10";
            let (remain, result) = menu_frame_color_g(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuFrameColorG(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.frame.color.g,";
            assert!(menu_frame_color_g(case).is_err());
        }
    }

    mod menu_frame_color_b {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.frame.color.b,10";
            let (remain, result) = menu_frame_color_b(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuFrameColorB(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.frame.color.b,";
            assert!(menu_frame_color_b(case).is_err());
        }
    }

    mod menu_disable_font_color_r {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.disable.font.color.r,10";
            let (remain, result) = menu_disable_font_color_r(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuDisableFontColorR(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.disable.font.color.r,";
            assert!(menu_disable_font_color_r(case).is_err());
        }
    }

    mod menu_disable_font_color_g {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.disable.font.color.g,10";
            let (remain, result) = menu_disable_font_color_g(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuDisableFontColorG(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.disable.font.color.g,";
            assert!(menu_disable_font_color_g(case).is_err());
        }
    }

    mod menu_disable_font_color_b {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu.disable.font.color.b,10";
            let (remain, result) = menu_disable_font_color_b(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::MenuDisableFontColorB(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.disable.font.color.b,";
            assert!(menu_disable_font_color_b(case).is_err());
        }
    }
}
