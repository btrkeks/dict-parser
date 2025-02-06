use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::yomichan::structured_content::StructuredDefinition;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct YomichanTermBankEntryArray(
    String,
    String,
    String,
    String,
    i64,
    Vec<Definition>,
    i64,
    String,
);

impl From<YomichanTermBankEntryArray> for YomichanTermBankEntry {
    fn from(arr: YomichanTermBankEntryArray) -> Self {
        YomichanTermBankEntry {
            term: arr.0,
            reading: arr.1,
            definition_tags: arr
                .2
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
            rules: arr
                .3
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            score: arr.4,
            definitions: arr.5,
            sequence_number: arr.6,
            term_tags: arr
                .7
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct YomichanTermBankEntry {
    /// The text for the term (e.g. "犬")
    term: String,
    /// Reading of the term, or an empty string if the reading is the same as the term (e.g. "いぬ")
    reading: String,
    /// Tags for the definition (e.g. "⭐"). References a tag in the tag bank.
    definition_tags: Vec<String>,
    /// Rule identifiers for the definition which can be used to validate deinflection.
    /// Empty for words which aren't inflected.
    rules: Vec<DeinfletionRule>,
    /// Score used to determine popularity.
    /// Negative values are more rare and positive values are more frequent.
    score: i64,
    /// Array of definitions for the term.
    definitions: Vec<Definition>,
    /// Sequence number for the term.
    sequence_number: i64,
    /// Tags for the term
    term_tags: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeinfletionRule {
    Vs,
    Vz,
    Vk,
    V5,
    V1,
    AdjI,
}

impl FromStr for DeinfletionRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vs" => Ok(DeinfletionRule::Vs),
            "v5" => Ok(DeinfletionRule::V5),
            "v1" => Ok(DeinfletionRule::V1),
            "vz" => Ok(DeinfletionRule::Vz),
            "vk" => Ok(DeinfletionRule::Vk),
            "adj-i" => Ok(DeinfletionRule::AdjI),
            _ => Err(format!("Invalid deinflection rule: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Definition {
    Plain(String),
    Structured(StructuredDefinition),
    Deinflection((String, Vec<String>))
}

pub(crate) fn parse_term_bank_from_string(term_bank: &str) -> anyhow::Result<Vec<YomichanTermBankEntry>> {
    let entries: Vec<YomichanTermBankEntryArray> = serde_json::from_str(term_bank)?;
    Ok(entries.into_iter().map(|e| e.into()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yomichan::structured_content::{StructuredContent, StructuredContentObject};

    #[test]
    fn test_correctly_parses_unstructured_definition() {
        let string = r#"[["六大州", "ろくだいしゅう", "", "", 1, ["ろくだい‐しゅう【六大州】――シウ\n地球上の六つの州。アジア州・アフリカ州・北アメリカ州・南アメリカ州・ヨーロッパ州・大洋州（オセアニア州）。転じて、全世界。"], 160620, ""], ["禄高", "ろくだか", "", "", 1, ["ろく‐だか【△禄高】\n武士が主人から与えられた給与の額。"], 160622, ""], ["陸で無し", "ろくでなし", "", "", 1, ["ろく‐で‐なし【◇陸で無し・△碌で無し】\n役に立たない者。つまらない者。のらくら者。「この―め」「陸（ろく）」は水平な状態・平らの意。下に打ち消しの語を伴って、平らではないという意から、物事のようす、性質などが正しくないこと、まともでないさまを表し、「ろくでなし」で役に立たない人をいうようになった。"], 160624, ""], ["碌で無し", "ろくでなし", "", "", 1, ["ろく‐で‐なし【◇陸で無し・△碌で無し】\n役に立たない者。つまらない者。のらくら者。「この―め」「陸（ろく）」は水平な状態・平らの意。下に打ち消しの語を伴って、平らではないという意から、物事のようす、性質などが正しくないこと、まともでないさまを表し、「ろくでなし」で役に立たない人をいうようになった。"], 160624, ""], ["陸でも無い", "ろくでもない", "", "", 1, ["ろく‐でも‐ない【◇陸でも無い・△碌でも無い】\nなんの役にも立たない。つまらない。「―ことをする」"], 160626, ""], ["碌でも無い", "ろくでもない", "", "", 1, ["ろく‐でも‐ない【◇陸でも無い・△碌でも無い】\nなんの役にも立たない。つまらない。「―ことをする」"], 160626, ""]]"#;
        let parsed = parse_term_bank_from_string(string).unwrap();
        // TODO: assert
    }

    #[test]
    fn test_correctly_parses_structured_content_only_text() {
        let string = r#"[["強い","つよい","1 adj-i","adj-i",1999800,[{"content":"strong","type":"structured-content"}],1236070,"⭐ ichi news9k"]]"#;

        let parsed = parse_term_bank_from_string(string).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].term, "強い");
        assert_eq!(parsed[0].reading, "つよい");
        assert_eq!(parsed[0].definition_tags, vec!["1", "adj-i"]);
        assert_eq!(parsed[0].rules, vec![DeinfletionRule::AdjI]);
        assert_eq!(parsed[0].score, 1999800);
        assert_eq!(parsed[0].sequence_number, 1236070);
        assert_eq!(parsed[0].term_tags, vec!["⭐", "ichi", "news9k"]);
        match &parsed[0].definitions[0] {
            Definition::Structured(StructuredDefinition::StructuredContent(content)) => {
                match content {
                    StructuredContent::TextContent(text) => assert_eq!(text, "strong"),
                    _ => panic!("Expected TextContent"),
                }
            }
            _ => panic!("Expected Structured Definition"),
        }
    }

    #[test]
    fn test_correctly_parses_structured_content_1() {
        let string = r#"[["強い","つよい","1 adj-i","adj-i",1999800,[{"content":[{"content":"strong","data":{"content":"glossary"},"lang":"en","style":{"listStyleType":"circle"},"tag":"ul"}],"type":"structured-content"}],1236070,"⭐ ichi news9k"]]"#;

        let parsed = parse_term_bank_from_string(string).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].term, "強い");
        assert_eq!(parsed[0].reading, "つよい");
        assert_eq!(parsed[0].definition_tags, vec!["1", "adj-i"]);
        assert_eq!(parsed[0].rules, vec![DeinfletionRule::AdjI]);
        assert_eq!(parsed[0].score, 1999800);
        assert_eq!(parsed[0].sequence_number, 1236070);
        assert_eq!(parsed[0].term_tags, vec!["⭐", "ichi", "news9k"]);
    }

    #[test]
    fn test_correctly_parses_structured_content_with_lists() {
        let string = r#"
        [
          [
            "強い",
            "つよい",
            "1 adj-i",
            "adj-i",
            1999800,
            [
              {
                "content": [
                  {
                    "content": [
                      {
                        "content": "strong",
                        "tag": "li"
                      },
                      {
                        "content": "potent",
                        "tag": "li"
                      }
                    ],
                    "data": {
                      "content": "glossary"
                    },
                    "lang": "en",
                    "style": {
                      "listStyleType": "circle"
                    },
                    "tag": "ul"
                  }
                ],
                "type": "structured-content"
              }
            ],
            1236070,
            "⭐ ichi news9k"
          ]
        ]
        "#;
        let parsed = parse_term_bank_from_string(string).unwrap();
        for entry in parsed {
            println!("{:?}", entry);
        }
    }

    #[test]
    fn test_parse_structured_content_object_3() {
        let string = r##"{
                "tag": "span",
                "title": "noun (common) (futsuumeishi)",
                "style": {
                  "fontSize": "0.8em",
                  "fontWeight": "bold",
                  "padding": "0.2em 0.3em",
                  "wordBreak": "keep-all",
                  "borderRadius": "0.3em",
                  "verticalAlign": "text-bottom",
                  "backgroundColor": "#565656",
                  "color": "white",
                  "cursor": "help",
                  "marginRight": "0.25em"
                },
                "data": {
                  "code": "n"
                },
                "content": "noun"
              }"##;
        let parsed: StructuredContentObject = serde_json::from_str(string).unwrap();
    }

    #[test]
    fn test_parse_structured_content_object_4() {
        let string = r##"{"tag": "tr","content":[{"tag": "td"},{"tag":"th"}]}"##;
        let parsed: StructuredContentObject = serde_json::from_str(string).unwrap();
    }

    #[test]
    fn test_parse_structured_content_object_5() {
        let string =
            r#"[
      {
        "type": "structured-content",
        "content": {
          "tag": "div",
          "lang": "ja",
          "style": {
            "fontSize": "180%",
            "marginTop": "0.2em"
          },
          "content": [
            "⟶",
            {
              "tag": "a",
              "href": "?query=%E3%82%A2%E3%83%A1%E3%81%A8%E3%83%A0%E3%83%81&wildcards=off",
              "content": "アメとムチ"
            }
          ]
        }
      },
      [
        "アメとムチ",
        [
          "redirected from 飴とムチ"
        ]
      ]
    ]"#;
        let parsed: Vec<Definition> = serde_json::from_str(string).unwrap();

        match &parsed[1] {
            Definition::Deinflection((s, d)) => {
                assert_eq!(s, "アメとムチ");
                assert_eq!(*d, vec!["redirected from 飴とムチ"]);
            }
            _ => panic!("Expected Deinflection"),
        }
    }

    #[test]
    fn correctly_parses_structured_content2() {
        let string = r#"[["メタ数学", "メタすうがく", "", "", 0, [{"type": "structured-content", "content": [{"tag": "div", "content": [{"tag": "span", "title": "noun (common) (futsuumeishi)", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "565656", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "data": {"code": "n"}, "content": "noun"}, {"tag": "span", "title": "mathematics", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "purple", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "data": {"code": "math"}, "content": "math"}, {"tag": "div", "content": {"tag": "ul", "style": {"listStyleType": "none", "paddingLeft": "0"}, "data": {"content": "glossary"}, "content": {"tag": "li", "content": "metamathematics"}}}]}, {"tag": "div", "style": {"fontSize": "0.7em", "textAlign": "right"}, "data": {"content": "attribution"}, "content": {"tag": "a", "href": "https://www.edrdg.org/jmwsgi/entr.py?svc=jmdict&q=1969080", "content": "JMdict"}}]}], 1969080, ""]]"#;
        let parsed = parse_term_bank_from_string(string).unwrap();
        for entry in parsed {
            println!("{:?}", entry);
        }
    }

    #[test]
    fn correctly_parses_structured_content_3() {
        let string = r##"[["ライトウェルター級", "ライトウェルターきゅう", "", "", 0, [{"type": "structured-content", "content": [{"tag": "div", "content": [{"tag": "span", "title": "noun (common) (futsuumeishi)", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "#565656", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "data": {"code": "n"}, "content": "noun"}, {"tag": "span", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "purple", "color": "white", "marginRight": "0.25em"}, "data": {"code": "sports"}, "content": "sports"}, {"tag": "div", "content": {"tag": "ul", "style": {"listStyleType": "none", "paddingLeft": "0"}, "data": {"content": "glossary"}, "content": {"tag": "li", "content": "light welterweight (boxing)"}}}]}, {"tag": "div", "style": {"marginTop": "0.5rem"}, "data": {"content": "forms"}, "content": [{"tag": "span", "title": "spelling and reading variants", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "#565656", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "content": "forms"}, {"tag": "div", "style": {"marginTop": "0.2em"}, "content": {"tag": "table", "content": [{"tag": "tr", "content": [{"tag": "th"}, {"tag": "th", "style": {"fontSize": "1.2em", "textAlign": "center", "fontWeight": "normal"}, "content": "ライトウェルター級"}, {"tag": "th", "style": {"fontSize": "1.2em", "textAlign": "center", "fontWeight": "normal"}, "content": "ライトウエルター級"}]}, {"tag": "tr", "content": [{"tag": "th", "style": {"fontWeight": "normal"}, "content": "ライトウェルターきゅう"}, {"tag": "td", "style": {"textAlign": "center"}, "content": {"tag": "div", "title": "valid form/reading combination", "style": {"cursor": "help", "padding": "0 0.5em", "color": "var(--background-color, var(--canvas, #f8f9fa))", "background": "radial-gradient(var(--text-color, var(--fg, #333)) 50%, white 100%)", "clipPath": "circle()", "fontWeight": "bold"}, "content": "◇"}}, {"tag": "td"}]}, {"tag": "tr", "content": [{"tag": "th", "style": {"fontWeight": "normal"}, "content": "ライトウエルターきゅう"}, {"tag": "td"}, {"tag": "td", "style": {"textAlign": "center"}, "content": {"tag": "div", "title": "valid form/reading combination", "style": {"cursor": "help", "padding": "0 0.5em", "color": "var(--background-color, var(--canvas, #f8f9fa))", "background": "radial-gradient(var(--text-color, var(--fg, #333)) 50%, white 100%)", "clipPath": "circle()", "fontWeight": "bold"}, "content": "◇"}}]}]}}]}, {"tag": "div", "style": {"fontSize": "0.7em", "textAlign": "right"}, "data": {"content": "attribution"}, "content": {"tag": "a", "href": "https://www.edrdg.org/jmwsgi/entr.py?svc=jmdict&q=1969520", "content": "JMdict"}}]}], 1969520, ""]]"##;
        let parsed = parse_term_bank_from_string(string).unwrap();
        for entry in parsed {
            println!("{:?}", entry);
        }
    }

    #[test]
    fn correctly_parses_structured_content_4() {
        let string = r#"[["飴とムチ", "", "", "", -102, [{"type": "structured-content", "content": {"tag": "div", "lang": "ja", "style": {"fontSize": "180%", "marginTop": "0.2em"}, "content": ["⟶", {"tag": "a", "href": "?query=%E3%82%A2%E3%83%A1%E3%81%A8%E3%83%A0%E3%83%81&wildcards=off", "content": "アメとムチ"}]}}, ["アメとムチ", ["redirected from 飴とムチ"]]], -1970680, ""]]"#;
        let parsed = parse_term_bank_from_string(string).unwrap();
        for entry in parsed {
            println!("{:?}", entry);
        }
    }
}