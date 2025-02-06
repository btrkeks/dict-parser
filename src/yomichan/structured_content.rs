use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "content")]
pub enum StructuredDefinition {
    Text(String), // TODO
    // Image, // TODO
    #[serde(rename = "structured-content")]
    StructuredContent(StructuredContent),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StructuredContent {
    TextContent(String),
    ArrayContent(Vec<StructuredContent>),
    ObjectContent(StructuredContentObject),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "tag")]
pub enum StructuredContentObject {
    #[serde(rename = "br")]
    Br {
        #[serde(default)]
        data: StructuredContentData,
    },
    Ruby {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Rt {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Rp {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Table {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Thead {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Tbody {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Tfoot {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Tr {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        lang: Option<String>,
    },
    Td {
        content: Option<Box<StructuredContent>>,
        #[serde(default)]
        data: StructuredContentData,
        col_span: Option<i32>,
        row_span: Option<i32>,
        style: Option<StructuredContentStyle>,
        lang: Option<String>,
    },
    Th {
        content: Option<Box<StructuredContent>>,
        #[serde(default)]
        data: StructuredContentData,
        col_span: Option<i32>,
        row_span: Option<i32>,
        style: Option<StructuredContentStyle>,
        lang: Option<String>,
    },
    Span {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        style: Option<StructuredContentStyle>,
        title: Option<String>,
        lang: Option<String>,
    },
    Div {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        style: Option<StructuredContentStyle>,
        title: Option<String>,
        open: Option<bool>,
        lang: Option<String>,
    },
    Ol {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        style: Option<StructuredContentStyle>,
        title: Option<String>,
        open: Option<bool>,
        lang: Option<String>,
    },
    Ul {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        style: Option<StructuredContentStyle>,
        title: Option<String>,
        open: Option<bool>,
        lang: Option<String>,
    },
    Li {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        style: Option<StructuredContentStyle>,
        title: Option<String>,
        open: Option<bool>,
        lang: Option<String>,
    },
    Details {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        style: Option<StructuredContentStyle>,
        title: Option<String>,
        open: Option<bool>,
        lang: Option<String>,
    },
    Summary {
        content: Box<StructuredContent>,
        #[serde(default)]
        data: StructuredContentData,
        style: Option<StructuredContentStyle>,
        title: Option<String>,
        open: Option<bool>,
        lang: Option<String>,
    },
    Img {
        #[serde(default)]
        data: StructuredContentData,
        path: String,
        width: Option<f64>,
        height: Option<f64>,
        title: Option<String>,
        alt: Option<String>,
        description: Option<String>,
        pixelated: Option<bool>,
        image_rendering: Option<ImageRendering>,
        appearance: Option<Appearance>,
        background: Option<bool>,
        collapsed: Option<bool>,
        collapsible: Option<bool>,
        vertical_align: Option<VerticalAlign>,
        border: Option<String>,
        border_radius: Option<String>,
        size_units: Option<SizeUnits>,
    },
    A {
        content: Box<StructuredContent>,
        href: String,
        lang: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct StructuredContentData {
    content: Option<String>,
    code: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructuredContentStyle {
    font_style: Option<String>,
    font_weight: Option<String>,
    font_size: Option<String>,
    color: Option<String>,
    background: Option<String>,
    background_color: Option<String>,
    text_decoration_line: Option<String>,
    text_decoration_style: Option<String>,
    text_decoration_color: Option<String>,
    border_color: Option<String>,
    border_style: Option<String>,
    border_radius: Option<String>,
    border_width: Option<String>,
    clip_path: Option<String>,
    vertical_align: Option<String>,
    text_align: Option<String>,
    text_emphasis: Option<String>,
    text_shadow: Option<String>,
    margin: Option<String>,
    margin_top: Option<String>,
    margin_left: Option<String>,
    margin_right: Option<String>,
    margin_bottom: Option<String>,
    padding: Option<String>,
    padding_top: Option<String>,
    padding_left: Option<String>,
    padding_right: Option<String>,
    padding_bottom: Option<String>,
    word_break: Option<WordBreak>,
    white_space: Option<String>,
    cursor: Option<String>,
    list_style_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FontWeight {
    Normal,
    Bold,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TextDecorationLine {
    Single(TextDecorationLineSingle),
    Multiple(Vec<TextDecorationLineSingle>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TextDecorationLineSingle {
    None,
    Underline,
    Overline,
    LineThrough,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    Baseline,
    Sub,
    Super,
    TextTop,
    TextBottom,
    Middle,
    Top,
    Bottom,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    JustifyAll,
    MatchParent,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MarginValue {
    Number(f64),
    StringValue(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum WordBreak {
    Normal,
    BreakAll,
    KeepAll,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ImageRendering {
    Auto,
    Pixelated,
    CrispEdges,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    Auto,
    Monochrome,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SizeUnits {
    Px,
    Em,
}