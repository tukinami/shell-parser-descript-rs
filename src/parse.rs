//! Parses a [`ShellDescript`] from `&str`.
//!
//! [`ShellDescript`]: crate::ast::ShellDescript

use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{eof, map, not, opt},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use shell_parser_common_rs::{
    charset::{parse_charset, Charset},
    ShellParseError,
};

use crate::ast::{LineContainer, ShellDescript, ShellDescriptLine};

use self::parts::{empty_line, newline_body, parse_line_func};

mod alpha;
mod balloon_representation;
mod base;
mod binding;
mod menu;
mod parts;
mod shell_representation;

/// Decodes bytes to `Cow<'a, str>` from specified charset.
///
/// # Example
///
/// ```
/// use encoding_rs::SHIFT_JIS;
///
/// use shell_parser_descript_rs::decode_bytes;
///
/// let case_raw = r#"charset,Shift_JIS
/// craftman,ukadog
/// craftmanw,うか犬
/// craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html
/// "#;
///
/// let (case, _, _) = SHIFT_JIS.encode(case_raw);
/// let result = match decode_bytes(&case) {
///     Ok(v) => v,
///     Err(e) => {
///         eprintln!("{:?}", e);
///         return;
///     }
/// };
/// assert_eq!(result, case_raw);
/// ```
pub fn decode_bytes<'a>(input: &'a [u8]) -> Result<Cow<'a, str>, String> {
    let temp_str = String::from_utf8_lossy(input);
    let charset = parse_for_decode(&temp_str)
        .map(|(_, v)| v)
        .unwrap_or(Charset::Default);

    match charset.decode(input) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Encoding failed: to {:?}", charset)),
    }
}

fn parse_for_decode<'a>(input: &'a str) -> IResult<&'a str, Charset, ShellParseError> {
    preceded(
        many0(tuple((
            not(parse_for_decode_charset),
            tuple((opt(is_not("\r\n")), newline_body)),
        ))),
        parse_for_decode_charset,
    )(input)
}

fn parse_for_decode_charset<'a>(input: &'a str) -> IResult<&'a str, Charset, ShellParseError> {
    terminated(
        preceded(tag("charset,"), parse_charset),
        alt((newline_body, eof)),
    )(input)
}

/// Parses a [`ShellDescript`] from `&str`.
///
/// [`ShellDescript`]: crate::ast::ShellDescript
///
/// # Example
///
/// ```
/// use shell_parser_descript_rs::parse;
///
/// let case = r#"charset,Shift_JIS
/// type,shell
/// name,master
///
/// craftman,ukadog
/// craftmanw,うか犬
/// craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html
///
/// menu.background.bitmap.filename,menu_background.png
/// menu.foreground.bitmap.filename,menu_foreground.png
/// menu.sidebar.bitmap.filename,menu_sidebar.png
///
/// sakura.balloon.offsetx,0
/// sakura.balloon.offsety,80
/// kero.balloon.offsetx,-20
/// kero.balloon.offsety,10
/// sakura.balloon.alignment,none
/// kero.balloon.alignment,none
/// "#;
///
/// let shell_descript = match parse(case) {
///     Ok(v) => v,
///     Err(e) => {
///         eprintln!("{:?}", e);
///         return;
///     }
/// };
///
/// assert_eq!(shell_descript.lines().len(), 18);
/// ```
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
    alt((
        base::base,
        shell_representation::shell_representation,
        balloon_representation::balloon_representation,
        menu::menu,
        binding::binding,
        alpha::alpha,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod decode_bytes {
        use std::{ffi::OsString, str::FromStr};

        use super::*;

        use encoding_rs::SHIFT_JIS;

        #[test]
        fn success_when_valid_str() {
            let case_raw = r#"
type,shell
name,master

craftman,ukadog
craftmanw,うか犬
craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html

menu.background.bitmap.filename,menu_background.png
menu.foreground.bitmap.filename,menu_foreground.png
menu.sidebar.bitmap.filename,menu_sidebar.png

sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
charset,Shift_JIS
"#;
            let (case, _, _) = SHIFT_JIS.encode(case_raw);
            let result = decode_bytes(&case).unwrap();
            assert_eq!(result, case_raw);

            let case_raw = r#"type,shell
name,master
"#;
            let case_temp = OsString::from_str(case_raw).unwrap();
            let case = &case_temp.into_encoded_bytes();
            let result = decode_bytes(case).unwrap();
            assert_eq!(result, case_raw);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case_raw = r#"charset,UTF-8
craftman,ukadog
craftmanw,うか犬
craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html
"#;
            let (case, _, _) = SHIFT_JIS.encode(case_raw);
            assert!(decode_bytes(&case).is_err());
        }
    }

    mod parse_for_decode {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"
type,shell
name,master

craftman,ukadog
craftmanw,うか犬
craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html

menu.background.bitmap.filename,menu_background.png
menu.foreground.bitmap.filename,menu_foreground.png
menu.sidebar.bitmap.filename,menu_sidebar.png

sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
charset,Shift_JIS
"#;
            let (remain, result) = parse_for_decode(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, Charset::ShiftJIS);

            let case = r#"
type,shell
name,master

craftman,ukadog
craftmanw,うか犬
craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html

menu.background.bitmap.filename,menu_background.png
menu.foreground.bitmap.filename,menu_foreground.png
menu.sidebar.bitmap.filename,menu_sidebar.png
charset,Shift_JIS

sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
"#;
            let (remain, result) = parse_for_decode(case).unwrap();
            assert_eq!(
                remain,
                r#"
sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
"#
            );
            assert_eq!(result, Charset::ShiftJIS);

            let case = r#"charset,Shift_JIS
type,shell
name,master
"#;
            let (remain, result) = parse_for_decode(case).unwrap();
            assert_eq!(
                remain,
                r#"type,shell
name,master
"#
            );
            assert_eq!(result, Charset::ShiftJIS);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
"#;
            assert!(parse_for_decode(case).is_err());
        }
    }

    mod parse_for_decode_charset {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "charset,Shift_JIS";
            let (remain, result) = parse_for_decode_charset(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, Charset::ShiftJIS);

            let case = "charset,Shift_JIS\r\n";
            let (remain, result) = parse_for_decode_charset(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, Charset::ShiftJIS);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.use_self_alpha,";
            assert!(parse_for_decode_charset(case).is_err());
        }
    }

    mod parse {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"
charset,Shift_JIS
type,shell
name,master

craftman,ukadog
craftmanw,うか犬
craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html

menu.background.bitmap.filename,menu_background.png
menu.foreground.bitmap.filename,menu_foreground.png
menu.sidebar.bitmap.filename,menu_sidebar.png

sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
"#;
            let result = parse(case).unwrap();
            assert_eq!(result.lines().len(), 19);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.use_self_alpha,";
            assert!(parse(case).is_err());
        }
    }

    mod shell_descript {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"
charset,Shift_JIS
type,shell
name,master

craftman,ukadog
craftmanw,うか犬
craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html

menu.background.bitmap.filename,menu_background.png
menu.foreground.bitmap.filename,menu_foreground.png
menu.sidebar.bitmap.filename,menu_sidebar.png

sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
"#;
            let (remain, result) = shell_descript(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.lines().len(), 19);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.use_self_alpha,";
            assert!(shell_descript(case).is_err());
        }
    }

    mod parse_lines {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"
charset,Shift_JIS
type,shell
name,master

craftman,ukadog
craftmanw,うか犬
craftmanurl,http://ssp.shillest.net/ukadoc/manual/index.html

menu.background.bitmap.filename,menu_background.png
menu.foreground.bitmap.filename,menu_foreground.png
menu.sidebar.bitmap.filename,menu_sidebar.png

sakura.balloon.offsetx,0
sakura.balloon.offsety,80
kero.balloon.offsetx,-20
kero.balloon.offsety,10
sakura.balloon.alignment,none
kero.balloon.alignment,none
"#;
            let (remain, result) = parse_lines(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.len(), 19);
        }
    }

    mod parse_line {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.use_self_alpha,1\r\n";
            let (remain, result) = parse_line(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                LineContainer::Body(ShellDescriptLine::SerikoUseSelfAlpha(1))
            );

            let case = "\r\n";
            let (remain, result) = parse_line(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, LineContainer::EmptyLine);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.use_self_alpha,";
            assert!(parse_line(case).is_err());
        }
    }

    mod shell_descript_line {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "seriko.use_self_alpha,1\r\n";
            let (remain, result) = shell_descript_line(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::SerikoUseSelfAlpha(1))
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "seriko.use_self_alpha,";
            assert!(shell_descript_line(case).is_err());
        }
    }
}
