use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::ast::{
    BindGroupNameProps, BindMenuItem, BindMenuVisibility, BindOption, ShellDescriptLine,
};

use super::parts::{char_id, digit};

pub(super) fn binding<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    alt((
        sakura_bindgroup_name,
        sakura_bindgroup_default,
        sakura_bindgroup_addid,
        sakura_bindoption_group,
        sakura_menuitem,
        sakura_menuitemex,
        sakura_menu,
        kero_bindgroup_name,
        kero_bindgroup_default,
        kero_bindgroup_addid,
        kero_bindoption_group,
        kero_menuitem,
        kero_menuitemex,
        kero_menu,
        char_bindgroup_name,
        char_bindgroup_default,
        char_bindgroup_addid,
        char_bindoption_group,
        char_menuitem,
        char_menuitemex,
        char_menu,
    ))(input)
}

fn sakura_bindgroup_name<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("sakura"), bind_grounp_name_props), |v| {
        ShellDescriptLine::SakuraBindgroupName(v)
    })(input)
}

fn sakura_bindgroup_default<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("sakura.bindgroup"), digit),
            preceded(tag(".default,"), digit),
        )),
        |(id, flag)| ShellDescriptLine::SakuraBindgroupDefault(id, flag),
    )(input)
}

fn sakura_bindgroup_addid<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("sakura.bindgroup"), digit),
            preceded(tag(".addid,"), separated_list1(tag(","), digit)),
        )),
        |(id, ids)| ShellDescriptLine::SakuraBindgroupAddid(id, ids),
    )(input)
}

fn sakura_bindoption_group<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("sakura"), bind_option), |v| {
        ShellDescriptLine::SakuraBindoptionGroup(v)
    })(input)
}

fn sakura_menuitem<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("sakura.menuitem"), digit),
            preceded(tag(","), bind_menu_item),
        )),
        |(id, item)| ShellDescriptLine::SakuraMenuitem(id, item),
    )(input)
}

fn sakura_menuitemex<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("sakura.menuitemex"), digit),
            preceded(tag(","), is_not(",\r\n")),
            preceded(tag(","), bind_menu_item),
        )),
        |(id, menu_name, item)| {
            ShellDescriptLine::SakuraMenuitemex(id, menu_name.to_string(), item)
        },
    )(input)
}

fn sakura_menu<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("sakura.menu,"), bind_menu_visibility), |v| {
        ShellDescriptLine::SakuraMenu(v)
    })(input)
}

fn kero_bindgroup_name<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("kero"), bind_grounp_name_props), |v| {
        ShellDescriptLine::KeroBindgroupName(v)
    })(input)
}

fn kero_bindgroup_default<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("kero.bindgroup"), digit),
            preceded(tag(".default,"), digit),
        )),
        |(id, flag)| ShellDescriptLine::KeroBindgroupDefault(id, flag),
    )(input)
}

fn kero_bindgroup_addid<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("kero.bindgroup"), digit),
            preceded(tag(".addid,"), separated_list1(tag(","), digit)),
        )),
        |(id, ids)| ShellDescriptLine::KeroBindgroupAddid(id, ids),
    )(input)
}

fn kero_bindoption_group<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("kero"), bind_option), |v| {
        ShellDescriptLine::KeroBindoptionGroup(v)
    })(input)
}

fn kero_menuitem<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("kero.menuitem"), digit),
            preceded(tag(","), bind_menu_item),
        )),
        |(id, item)| ShellDescriptLine::KeroMenuitem(id, item),
    )(input)
}

fn kero_menuitemex<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            preceded(tag("kero.menuitemex"), digit),
            preceded(tag(","), is_not(",\r\n")),
            preceded(tag(","), bind_menu_item),
        )),
        |(id, menu_name, item)| ShellDescriptLine::KeroMenuitemex(id, menu_name.to_string(), item),
    )(input)
}

fn kero_menu<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(preceded(tag("kero.menu,"), bind_menu_visibility), |v| {
        ShellDescriptLine::KeroMenu(v)
    })(input)
}

fn char_bindgroup_name<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(tuple((char_id, bind_grounp_name_props)), |(id, v)| {
        ShellDescriptLine::CharBindgroupName(id, v)
    })(input)
}

fn char_bindgroup_default<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            char_id,
            preceded(tag(".bindgroup"), digit),
            preceded(tag(".default,"), digit),
        )),
        |(char_id, id, flag)| ShellDescriptLine::CharBindgroupDefault(char_id, id, flag),
    )(input)
}

fn char_bindgroup_addid<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            char_id,
            preceded(tag(".bindgroup"), digit),
            preceded(tag(".addid,"), separated_list1(tag(","), digit)),
        )),
        |(char_id, id, ids)| ShellDescriptLine::CharBindgroupAddid(char_id, id, ids),
    )(input)
}

fn char_bindoption_group<'a>(
    input: &'a str,
) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(tuple((char_id, bind_option)), |(id, v)| {
        ShellDescriptLine::CharBindoptionGroup(id, v)
    })(input)
}

fn char_menuitem<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            char_id,
            preceded(tag(".menuitem"), digit),
            preceded(tag(","), bind_menu_item),
        )),
        |(char_id, id, item)| ShellDescriptLine::CharMenuitem(char_id, id, item),
    )(input)
}

fn char_menuitemex<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((
            char_id,
            preceded(tag(".menuitemex"), digit),
            preceded(tag(","), is_not(",\r\n")),
            preceded(tag(","), bind_menu_item),
        )),
        |(char_id, id, menu_name, item)| {
            ShellDescriptLine::CharMenuitemex(char_id, id, menu_name.to_string(), item)
        },
    )(input)
}

fn char_menu<'a>(input: &'a str) -> IResult<&'a str, ShellDescriptLine, ShellParseError> {
    map(
        tuple((terminated(char_id, tag(".menu,")), bind_menu_visibility)),
        |(id, v)| ShellDescriptLine::CharMenu(id, v),
    )(input)
}

fn bind_grounp_name_props<'a>(
    input: &'a str,
) -> IResult<&'a str, BindGroupNameProps, ShellParseError> {
    map(
        tuple((
            preceded(tag(".bindgroup"), digit),
            preceded(tag(".name,"), is_not(",\r\n")),
            preceded(tag(","), is_not(",\r\n")),
            opt(preceded(tag(","), is_not(",\r\n"))),
        )),
        |(id, category, part_name, thumbnail_name)| {
            BindGroupNameProps::new(
                id,
                category.to_string(),
                part_name.to_string(),
                thumbnail_name.map(|v| v.to_string()),
            )
        },
    )(input)
}

fn bind_option<'a>(input: &'a str) -> IResult<&'a str, BindOption, ShellParseError> {
    map(
        tuple((
            preceded(tag(".bindoption"), digit),
            preceded(tag(".group,"), is_not(",\r\n")),
            preceded(tag(","), bind_option_body),
        )),
        |(id, category, (is_mustselect, is_multiple))| {
            BindOption::new(id, category.to_string(), is_mustselect, is_multiple)
        },
    )(input)
}

fn bind_option_body<'a>(input: &'a str) -> IResult<&'a str, (bool, bool), ShellParseError> {
    const TAG_MUSTSELECT: &str = "mustselect";
    const TAG_MULTIPLE: &str = "multiple";

    map(
        separated_list1(tag("+"), alt((tag(TAG_MUSTSELECT), tag(TAG_MULTIPLE)))),
        |l| {
            (
                l.iter().find(|v| *v == &TAG_MUSTSELECT).is_some(),
                l.iter().find(|v| *v == &TAG_MULTIPLE).is_some(),
            )
        },
    )(input)
}

fn bind_menu_item<'a>(input: &'a str) -> IResult<&'a str, BindMenuItem, ShellParseError> {
    alt((
        map(digit, |v| BindMenuItem::Id(v)),
        map(tag("-"), |_| BindMenuItem::Line),
    ))(input)
}

fn bind_menu_visibility<'a>(
    input: &'a str,
) -> IResult<&'a str, BindMenuVisibility, ShellParseError> {
    alt((
        map(tag("auto"), |_| BindMenuVisibility::Auto),
        map(tag("hidden"), |_| BindMenuVisibility::Hidden),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sakura_bindgroup_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.bindgroup5.name,カテゴリ名,パーツ名,サムネイル名";
            let (remain, result) = sakura_bindgroup_name(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraBindgroupName(BindGroupNameProps::new(
                    5,
                    "カテゴリ名".to_string(),
                    "パーツ名".to_string(),
                    Some("サムネイル名".to_string())
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.bindgroup5.name,カテゴリ名,";
            assert!(sakura_bindgroup_name(case).is_err());
        }
    }

    mod sakura_bindgroup_default {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.bindgroup5.default,1";
            let (remain, result) = sakura_bindgroup_default(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraBindgroupDefault(5, 1));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.bindgroup5.default,";
            assert!(sakura_bindgroup_default(case).is_err());
        }
    }

    mod sakura_bindgroup_addid {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.bindgroup5.addid,1";
            let (remain, result) = sakura_bindgroup_addid(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::SakuraBindgroupAddid(5, vec![1]));

            let case = "sakura.bindgroup5.addid,1,2,3";
            let (remain, result) = sakura_bindgroup_addid(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraBindgroupAddid(5, vec![1, 2, 3])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.bindgroup5.addid,";
            assert!(sakura_bindgroup_addid(case).is_err());
        }
    }

    mod sakura_bindoption_group {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.bindoption5.group,カテゴリ名,mustselect";
            let (remain, result) = sakura_bindoption_group(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraBindoptionGroup(BindOption::new(
                    5,
                    "カテゴリ名".to_string(),
                    true,
                    false
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.bindoption*.group,カテゴリ名,";
            assert!(sakura_bindoption_group(case).is_err());
        }
    }

    mod sakura_menuitem {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.menuitem5,10";
            let (remain, result) = sakura_menuitem(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraMenuitem(5, BindMenuItem::Id(10))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.menuitem5,";
            assert!(sakura_menuitem(case).is_err());
        }
    }

    mod sakura_menuitemex {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.menuitemex5,name,10";
            let (remain, result) = sakura_menuitemex(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraMenuitemex(5, "name".to_string(), BindMenuItem::Id(10))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.menuitemex5,name,";
            assert!(sakura_menuitemex(case).is_err());
        }
    }

    mod sakura_menu {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.menu,auto";
            let (remain, result) = sakura_menu(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::SakuraMenu(BindMenuVisibility::Auto)
            )
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.menu,";
            assert!(sakura_menu(case).is_err());
        }
    }

    mod kero_bindgroup_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.bindgroup5.name,カテゴリ名,パーツ名,サムネイル名";
            let (remain, result) = kero_bindgroup_name(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroBindgroupName(BindGroupNameProps::new(
                    5,
                    "カテゴリ名".to_string(),
                    "パーツ名".to_string(),
                    Some("サムネイル名".to_string())
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.bindgroup5.name,カテゴリ名,";
            assert!(kero_bindgroup_name(case).is_err());
        }
    }

    mod kero_bindgroup_default {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.bindgroup5.default,1";
            let (remain, result) = kero_bindgroup_default(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroBindgroupDefault(5, 1));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.bindgroup5.default,";
            assert!(kero_bindgroup_default(case).is_err());
        }
    }

    mod kero_bindgroup_addid {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.bindgroup5.addid,1";
            let (remain, result) = kero_bindgroup_addid(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::KeroBindgroupAddid(5, vec![1]));

            let case = "kero.bindgroup5.addid,1,2,3";
            let (remain, result) = kero_bindgroup_addid(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroBindgroupAddid(5, vec![1, 2, 3])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.bindgroup5.addid,";
            assert!(kero_bindgroup_addid(case).is_err());
        }
    }

    mod kero_bindoption_group {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.bindoption5.group,カテゴリ名,mustselect";
            let (remain, result) = kero_bindoption_group(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroBindoptionGroup(BindOption::new(
                    5,
                    "カテゴリ名".to_string(),
                    true,
                    false
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.bindoption*.group,カテゴリ名,";
            assert!(kero_bindoption_group(case).is_err());
        }
    }

    mod kero_menuitem {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.menuitem5,10";
            let (remain, result) = kero_menuitem(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroMenuitem(5, BindMenuItem::Id(10))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.menuitem5,";
            assert!(kero_menuitem(case).is_err());
        }
    }

    mod kero_menuitemex {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.menuitemex5,name,10";
            let (remain, result) = kero_menuitemex(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroMenuitemex(5, "name".to_string(), BindMenuItem::Id(10))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.menuitemex5,name,";
            assert!(kero_menuitemex(case).is_err());
        }
    }

    mod kero_menu {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.menu,auto";
            let (remain, result) = kero_menu(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::KeroMenu(BindMenuVisibility::Auto)
            )
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.menu,";
            assert!(kero_menu(case).is_err());
        }
    }

    mod char_bindgroup_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char1.bindgroup5.name,カテゴリ名,パーツ名,サムネイル名";
            let (remain, result) = char_bindgroup_name(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::CharBindgroupName(
                    1,
                    BindGroupNameProps::new(
                        5,
                        "カテゴリ名".to_string(),
                        "パーツ名".to_string(),
                        Some("サムネイル名".to_string())
                    )
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char1.bindgroup5.name,カテゴリ名,";
            assert!(char_bindgroup_name(case).is_err());
        }
    }

    mod char_bindgroup_default {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char1.bindgroup5.default,1";
            let (remain, result) = char_bindgroup_default(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::CharBindgroupDefault(1, 5, 1));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char.bindgroup5.default,";
            assert!(char_bindgroup_default(case).is_err());
        }
    }

    mod char_bindgroup_addid {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char1.bindgroup5.addid,1";
            let (remain, result) = char_bindgroup_addid(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, ShellDescriptLine::CharBindgroupAddid(1, 5, vec![1]));

            let case = "char1.bindgroup5.addid,1,2,3";
            let (remain, result) = char_bindgroup_addid(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::CharBindgroupAddid(1, 5, vec![1, 2, 3])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char1.bindgroup5.addid,";
            assert!(char_bindgroup_addid(case).is_err());
        }
    }

    mod char_bindoption_group {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char1.bindoption5.group,カテゴリ名,mustselect";
            let (remain, result) = char_bindoption_group(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::CharBindoptionGroup(
                    1,
                    BindOption::new(5, "カテゴリ名".to_string(), true, false)
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char5.bindoption*.group,カテゴリ名,";
            assert!(char_bindoption_group(case).is_err());
        }
    }

    mod char_menuitem {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char1.menuitem5,10";
            let (remain, result) = char_menuitem(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::CharMenuitem(1, 5, BindMenuItem::Id(10))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char5.menuitem5,";
            assert!(char_menuitem(case).is_err());
        }
    }

    mod char_menuitemex {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char1.menuitemex5,name,10";
            let (remain, result) = char_menuitemex(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::CharMenuitemex(1, 5, "name".to_string(), BindMenuItem::Id(10))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char5.menuitemex5,name,";
            assert!(char_menuitemex(case).is_err());
        }
    }

    mod char_menu {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "char5.menu,auto";
            let (remain, result) = char_menu(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                ShellDescriptLine::CharMenu(5, BindMenuVisibility::Auto)
            )
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "char5.menu,";
            assert!(char_menu(case).is_err());
        }
    }

    mod bind_grounp_name_props {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = ".bindgroup5.name,カテゴリ名,パーツ名,サムネイル名";
            let (remain, result) = bind_grounp_name_props(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                BindGroupNameProps::new(
                    5,
                    "カテゴリ名".to_string(),
                    "パーツ名".to_string(),
                    Some("サムネイル名".to_string())
                )
            );

            let case = ".bindgroup5.name,カテゴリ名,パーツ名";
            let (remain, result) = bind_grounp_name_props(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                BindGroupNameProps::new(5, "カテゴリ名".to_string(), "パーツ名".to_string(), None)
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = ".bindgroup*.name,カテゴリ名,パーツ名,";
            assert!(bind_grounp_name_props(case).is_err());
        }
    }

    mod bind_option {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = ".bindoption0.group,カテゴリ名,mustselect";
            let (remain, result) = bind_option(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                BindOption::new(0, "カテゴリ名".to_string(), true, false)
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = ".bindoption0.group,カテゴリ名,";
            assert!(bind_option(case).is_err());
        }
    }

    mod bind_option_body {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "mustselect";
            let (remain, result) = bind_option_body(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, (true, false));

            let case = "multiple";
            let (remain, result) = bind_option_body(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, (false, true));

            let case = "mustselect+multiple";
            let (remain, result) = bind_option_body(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, (true, true));

            let case = "multiple+mustselect";
            let (remain, result) = bind_option_body(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, (true, true));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "";
            assert!(bind_option_body(case).is_err());
        }
    }

    mod bind_menu_item {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "10";
            let (remain, result) = bind_menu_item(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, BindMenuItem::Id(10));

            let case = "-";
            let (remain, result) = bind_menu_item(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, BindMenuItem::Line);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "a";
            assert!(bind_menu_item(case).is_err());
        }
    }

    mod bind_menu_visibility {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "auto";
            let (remain, result) = bind_menu_visibility(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, BindMenuVisibility::Auto);

            let case = "hidden";
            let (remain, result) = bind_menu_visibility(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, BindMenuVisibility::Hidden);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(bind_menu_visibility(case).is_err());
        }
    }
}
