//! AST for `ShellDescript`.

use shell_parser_common_rs::charset::Charset;

pub type CharacterIdType = u32;
pub type DesktopPositionType = i64;
pub type FlagType = u8;
pub type FontSizeType = u32;
pub type ColorType = u8;
pub type AnimationIdType = u32;

/// Root of `ShellDescript`.
#[derive(Debug, Clone, PartialEq)]
pub struct ShellDescript {
    lines: Vec<LineContainer>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineContainer {
    EmptyLine,
    CommentLine(String),
    Body(ShellDescriptLine),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShellDescriptLine {
    // base
    Charset(Charset),
    Name(String),
    Id(String),
    Type,
    Craftman(String),
    Craftmanw(String),
    Craftmanurl(String),
    Homeurl(String),
    Readme(String),
    ReadmeCharset(Charset),
    MenuHidden,
    SakuraName(String),
    SakuraName2(String),
    KeroName(String),
    CharName(CharacterIdType, String),
    // shell representation
    SerikoZOrder(Vec<CharacterIdType>),
    SerikoStickyWindow(Vec<CharacterIdType>),
    SerikoAlignmenttodesktop(SurfacePosition),
    SakuraSerikoAlignmenttodesktop(SurfacePosition),
    KeroSerikoAlignmenttodesktop(SurfacePosition),
    CharSerikoAlignmenttodesktop(CharacterIdType, SurfacePosition),
    SakuraDefaultx(DesktopPositionType),
    KeroDefaultx(DesktopPositionType),
    CharDefaultx(CharacterIdType, DesktopPositionType),
    SakuraDefaulty(DesktopPositionType),
    KeroDefaulty(DesktopPositionType),
    CharDefaulty(CharacterIdType, DesktopPositionType),
    SakuraDefaultleft(DesktopPositionType),
    KeroDefaultleft(DesktopPositionType),
    CharDefaultleft(CharacterIdType, DesktopPositionType),
    SakuraDefaulttop(DesktopPositionType),
    KeroDefaulttop(DesktopPositionType),
    CharDefaulttop(CharacterIdType, DesktopPositionType),
    // balloon representation
    SakuraBalloonOffsetx(DesktopPositionType),
    SakuraBalloonOffsety(DesktopPositionType),
    KeroBalloonOffsetx(DesktopPositionType),
    KeroBalloonOffsety(DesktopPositionType),
    SakuraBalloonAlignment(BalloonPosition),
    KeroBalloonAlignment(BalloonPosition),
    SakuraBalloonDontmove(FlagType),
    KeroBalloonDontmove(FlagType),
    CharBalloonDontmove(CharacterIdType, FlagType),
    // menu
    MenuFontName(String),
    MenuFontHeight(FontSizeType),
    MenuBackgroundBitmapFilename(String),
    MenuForegroundBitmapFilename(String),
    MenuSidebarBitmapFilename(String),
    MenuBackgroundFontColorR(ColorType),
    MenuBackgroundFontColorG(ColorType),
    MenuBackgroundFontColorB(ColorType),
    MenuForegroundFontColorR(ColorType),
    MenuForegroundFontColorG(ColorType),
    MenuForegroundFontColorB(ColorType),
    MenuSeparatorColorR(ColorType),
    MenuSeparatorColorG(ColorType),
    MenuSeparatorColorB(ColorType),
    MenuFrameColorR(ColorType),
    MenuFrameColorG(ColorType),
    MenuFrameColorB(ColorType),
    MenuDisableFontColorR(ColorType),
    MenuDisableFontColorG(ColorType),
    MenuDisableFontColorB(ColorType),
    MenuBackgroundAlignment(
        MenuPositionForegroundBackgroundBase,
        Option<MenuPositionForegroundBackgroundRepeat>,
        Option<MenuPositionForegroundBackgroundRepeat>,
    ),
    MenuForegroundAlignment(
        MenuPositionForegroundBackgroundBase,
        Option<MenuPositionForegroundBackgroundRepeat>,
        Option<MenuPositionForegroundBackgroundRepeat>,
    ),
    MenuSidebarAlignment(MenuPositionSidebarBase, Option<MenuPositionSidebarRepeat>),
    // binding
    SakuraBindgroupName(BindGroupNameProps),
    SakuraBindgroupDefault(AnimationIdType, FlagType),
    SakuraBindgroupAddid(AnimationIdType, Vec<AnimationIdType>),
    SakuraBindoptionGroup(BindOption),
    SakuraMenuitem(u32, BindMenuItem),
    SakuraMenuitemex(u32, String, BindMenuItem),
    SakuraMenu(BindMenuVisibility),
    KeroBindgroupName(BindGroupNameProps),
    KeroBindgroupDefault(AnimationIdType, FlagType),
    KeroBindgroupAddid(AnimationIdType, Vec<AnimationIdType>),
    KeroBindoptionGroup(BindOption),
    KeroMenuitem(u32, BindMenuItem),
    KeroMenuitemex(u32, String, BindMenuItem),
    KeroMenu(BindMenuVisibility),
    CharBindgroupName(CharacterIdType, BindGroupNameProps),
    CharBindgroupDefault(CharacterIdType, AnimationIdType, FlagType),
    CharBindgroupAddid(CharacterIdType, AnimationIdType, Vec<AnimationIdType>),
    CharBindoptionGroup(CharacterIdType, BindOption),
    CharMenuitem(CharacterIdType, u32, BindMenuItem),
    CharMenuitemex(CharacterIdType, u32, String, BindMenuItem),
    CharMenu(CharacterIdType, BindMenuVisibility),
    // alpha
    SerikoPaintTransparentRegionBlack(FlagType),
    SerikoUseSelfAlpha(FlagType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SurfacePosition {
    Top,
    Bottom,
    Free,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BalloonPosition {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuPositionForegroundBackgroundBase {
    Lefttop,
    Centertop,
    Righttop,
    Leftbottom,
    Centerbottom,
    Rightbottom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuPositionForegroundBackgroundRepeat {
    RepeatX,
    RepeatY,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuPositionSidebarBase {
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuPositionSidebarRepeat {
    RepeatY,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindGroupNameProps {
    id: AnimationIdType,
    category: String,
    part_name: String,
    thumbnail_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindOption {
    id: u32,
    category: String,
    is_mustselect: bool,
    is_multiple: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindMenuItem {
    Line,
    Id(AnimationIdType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindMenuVisibility {
    Auto,
    Hidden,
}

impl ShellDescript {
    pub fn new(lines: Vec<LineContainer>) -> ShellDescript {
        ShellDescript { lines }
    }

    pub fn lines(&self) -> &Vec<LineContainer> {
        &self.lines
    }
}

impl BindGroupNameProps {
    pub fn new(
        id: AnimationIdType,
        category: String,
        part_name: String,
        thumbnail_name: Option<String>,
    ) -> BindGroupNameProps {
        BindGroupNameProps {
            id,
            category,
            part_name,
            thumbnail_name,
        }
    }

    pub fn id(&self) -> &AnimationIdType {
        &self.id
    }
    pub fn category(&self) -> &String {
        &self.category
    }
    pub fn part_name(&self) -> &String {
        &self.part_name
    }
    pub fn thumbnail_name(&self) -> &Option<String> {
        &self.thumbnail_name
    }
}

impl BindOption {
    pub fn new(id: u32, category: String, is_mustselect: bool, is_multiple: bool) -> BindOption {
        BindOption {
            id,
            category,
            is_mustselect,
            is_multiple,
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }
    pub fn category(&self) -> &String {
        &self.category
    }
    pub fn is_mustselect(&self) -> &bool {
        &self.is_mustselect
    }
    pub fn is_multiple(&self) -> &bool {
        &self.is_multiple
    }
}
