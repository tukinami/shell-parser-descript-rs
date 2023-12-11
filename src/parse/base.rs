use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
use shell_parser_common_rs::{charset::parse_charset, ShellParseError};

use crate::ast::ShellDescriptLine;

use super::parts::char_id;

pub(super) fn base<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((
        charset,
        name,
        id,
        descript_type,
        craftman,
        craftmanw,
        craftmanurl,
        homeurl,
        readme,
        readme_charset,
        menu_hidden,
        sakura_name,
        sakura_name_2,
        kero_name,
        char_name,
    ))(input)
}

pub(super) fn charset<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    preceded(
        tag("charset,"),
        map(parse_charset, |v| ShellDescriptLine::Charset(v)),
    )(input)
}

fn descript_type<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(tag("type,shell"), |_| ShellDescriptLine::Type)(input)
}

fn readme_charset<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    preceded(
        tag("readme.charset,"),
        map(parse_charset, |v| ShellDescriptLine::ReadmeCharset(v)),
    )(input)
}

fn menu_hidden<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(tag("menu,hidden"), |_| ShellDescriptLine::MenuHidden)(input)
}

fn char_name<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((char_id, preceded(tag(".name,"), is_not("\r\n")))),
        |(id, s)| ShellDescriptLine::CharName(id, s.to_string()),
    )(input)
}

macro_rules! line_has_only_string {
    ($name:ident, $tag:expr, $pat:path) => {
        fn $name<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
            map(preceded(tag($tag), is_not("\r\n")), |v: &str| {
                $pat(v.to_string())
            })(input)
        }
    };
}

line_has_only_string!(name, "name,", ShellDescriptLine::Name);
line_has_only_string!(id, "id,", ShellDescriptLine::Id);

line_has_only_string!(craftman, "craftman,", ShellDescriptLine::Craftman);
line_has_only_string!(craftmanw, "craftmanw,", ShellDescriptLine::Craftmanw);
line_has_only_string!(craftmanurl, "craftmanurl,", ShellDescriptLine::Craftmanurl);
line_has_only_string!(homeurl, "homeurl,", ShellDescriptLine::Homeurl);
line_has_only_string!(readme, "readme,", ShellDescriptLine::Readme);

line_has_only_string!(sakura_name, "sakura.name,", ShellDescriptLine::SakuraName);
line_has_only_string!(
    sakura_name_2,
    "sakura.name2,",
    ShellDescriptLine::SakuraName2
);
line_has_only_string!(kero_name, "kero.name,", ShellDescriptLine::KeroName);

#[cfg(test)]
mod tests {
    use super::*;

    mod base {
        use shell_parser_common_rs::charset::Charset;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "charset,UTF-8\r\n";
            let (remain, result) = base(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Charset(Charset::UTF8));
        }

        #[test]
        fn failed_when_vaind_str() {
            let case = "somethingwrong,00\r\n";
            assert!(base(case).is_err());
        }
    }

    mod charset {
        use shell_parser_common_rs::charset::Charset;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "charset,ASCII\r\n";
            let (remain, result) = charset(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Charset(Charset::ASCII));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "charsetASCII";
            assert!(charset(case).is_err());
        }
    }

    mod descript_type {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "type,shell\r\n";
            let (remain, result) = descript_type(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Type);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "type,ghost";
            assert!(descript_type(case).is_err());
        }
    }

    mod readme_charset {
        use shell_parser_common_rs::charset::Charset;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "readme.charset,Shift_JIS\r\n";
            let (remain, result) = readme_charset(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::ReadmeCharset(Charset::ShiftJIS));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "readme.Shift_JIS";
            assert!(readme_charset(case).is_err());
        }
    }

    mod menu_hidden {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "menu,hidden\r\n";
            let (remain, result) = menu_hidden(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::MenuHidden);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "menu.hidden";
            assert!(menu_hidden(case).is_err());
        }
    }

    mod char_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.name,あいう\r\n";
            let (remain, result) = char_name(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::CharName(5, "あいう".to_string()));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char.name,abc";
            assert!(char_name(case).is_err());
        }
    }

    mod name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "name,abc\r\n";
            let (remain, result) = name(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Name("abc".to_string()));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "nameabc";
            assert!(name(case).is_err());
        }
    }

    mod id {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "id,abc\r\n";
            let (remain, result) = id(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Id("abc".to_string()));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "idabc";
            assert!(id(case).is_err());
        }
    }

    mod craftman {
        use super::*;

        #[test]
        fn success_when_valcraftman_str() {
            let case = "craftman,abc\r\n";
            let (remain, result) = craftman(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Craftman("abc".to_string()));
        }

        #[test]
        fn failed_when_invalcraftman_str() {
            let case = "craftmanabc";
            assert!(craftman(case).is_err());
        }
    }

    mod craftmanw {
        use super::*;

        #[test]
        fn success_when_valcraftmanw_str() {
            let case = "craftmanw,abc\r\n";
            let (remain, result) = craftmanw(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Craftmanw("abc".to_string()));
        }

        #[test]
        fn failed_when_invalcraftmanw_str() {
            let case = "craftmanwabc";
            assert!(craftmanw(case).is_err());
        }
    }

    mod craftmanurl {
        use super::*;

        #[test]
        fn success_when_valcraftmanurl_str() {
            let case = "craftmanurl,abc\r\n";
            let (remain, result) = craftmanurl(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Craftmanurl("abc".to_string()));
        }

        #[test]
        fn failed_when_invalcraftmanurl_str() {
            let case = "craftmanurlabc";
            assert!(craftmanurl(case).is_err());
        }
    }

    mod homeurl {
        use super::*;

        #[test]
        fn success_when_valhomeurl_str() {
            let case = "homeurl,abc\r\n";
            let (remain, result) = homeurl(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Homeurl("abc".to_string()));
        }

        #[test]
        fn failed_when_invalhomeurl_str() {
            let case = "homeurlabc";
            assert!(homeurl(case).is_err());
        }
    }

    mod readme {
        use super::*;

        #[test]
        fn success_when_valreadme_str() {
            let case = "readme,abc\r\n";
            let (remain, result) = readme(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::Readme("abc".to_string()));
        }

        #[test]
        fn failed_when_invalreadme_str() {
            let case = "readmeabc";
            assert!(readme(case).is_err());
        }
    }

    mod sakura_name {
        use super::*;

        #[test]
        fn success_when_valname_str() {
            let case = "sakura.name,abc\r\n";
            let (remain, result) = sakura_name(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::SakuraName("abc".to_string()));
        }

        #[test]
        fn failed_when_invalname_str() {
            let case = "sakura.nameabc";
            assert!(sakura_name(case).is_err());
        }
    }

    mod sakura_name_2 {
        use super::*;

        #[test]
        fn success_when_valname_str() {
            let case = "sakura.name2,abc\r\n";
            let (remain, result) = sakura_name_2(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::SakuraName2("abc".to_string()));
        }

        #[test]
        fn failed_when_invalname_str() {
            let case = "sakura.name2abc";
            assert!(sakura_name_2(case).is_err());
        }
    }

    mod kero_name {
        use super::*;

        #[test]
        fn success_when_valname_str() {
            let case = "kero.name,abc\r\n";
            let (remain, result) = kero_name(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, ShellDescriptLine::KeroName("abc".to_string()));
        }

        #[test]
        fn failed_when_invalname_str() {
            let case = "kero.nameabc";
            assert!(kero_name(case).is_err());
        }
    }
}
