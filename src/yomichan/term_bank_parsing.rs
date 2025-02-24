use std::fmt::{Display};
use crate::yomichan::structured_content::StructuredDefinition;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
        // Estimate capacities to avoid reallocations
        let def_tags_count = arr.2.split_ascii_whitespace().count();
        let rules_count = arr.3.split_ascii_whitespace().count();
        let term_tags_count = arr.7.split_ascii_whitespace().count();

        // Preallocate vectors with the right capacity
        let mut def_tags = Vec::with_capacity(def_tags_count);
        let mut rules = Vec::with_capacity(rules_count);
        let mut term_tags = Vec::with_capacity(term_tags_count);

        // Fill vectors efficiently
        for tag in arr.2.split_ascii_whitespace() {
            def_tags.push(tag.to_string());
        }

        for rule_str in arr.3.split_ascii_whitespace() {
            if let Ok(rule) = rule_str.parse() {
                rules.push(rule);
            }
        }

        for tag in arr.7.split_ascii_whitespace() {
            term_tags.push(tag.to_string());
        }

        YomichanTermBankEntry {
            term: arr.0,
            reading: arr.1,
            definition_tags: def_tags,
            rules,
            score: arr.4,
            definitions: arr.5,
            sequence_number: arr.6,
            term_tags,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "YomichanTermBankEntryArray")]
pub struct YomichanTermBankEntry {
    /// The text for the term (e.g. "犬")
    pub term: String,
    /// Reading of the term, or an empty string if the reading is the same as the term (e.g. "いぬ")
    pub reading: String,
    /// Tags for the definition (e.g. "⭐"). References a tag in the tag bank.
    pub definition_tags: Vec<String>,
    /// Rule identifiers for the definition which can be used to validate deinflection.
    /// Empty for words which aren't inflected.
    pub rules: Vec<DeinfletionRule>,
    /// Score used to determine popularity.
    /// Negative values are more rare and positive values are more frequent.
    pub score: i64,
    /// Array of definitions for the term.
    pub definitions: Vec<Definition>,
    /// Sequence number for the term.
    pub sequence_number: i64,
    /// Tags for the term
    pub term_tags: Vec<String>,
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
    Deinflection((String, Vec<String>)),
}

impl Default for Definition {
    fn default() -> Self {
        Definition::Plain(String::default())
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        write!(f, "<definition>")
    }
}

pub(crate) fn parse_term_bank_from_bytes(
    term_bank: &[u8],
) -> anyhow::Result<Vec<YomichanTermBankEntry>> {
    Ok(sonic_rs::from_slice(term_bank)?)
}

#[cfg(test)]
mod tests {
    use sonic_rs::Index;
    use super::*;
    use crate::yomichan::structured_content::{StructuredContent, StructuredContentObject};

    #[test]
    fn test_correctly_parses_unstructured_definition() {
        let string: &[u8] = r#"[["六大州", "ろくだいしゅう", "", "", 1, ["ろくだい‐しゅう【六大州】――シウ\n地球上の六つの州。アジア州・アフリカ州・北アメリカ州・南アメリカ州・ヨーロッパ州・大洋州（オセアニア州）。転じて、全世界。"], 160620, ""], ["禄高", "ろくだか", "", "", 1, ["ろく‐だか【△禄高】\n武士が主人から与えられた給与の額。"], 160622, ""], ["陸で無し", "ろくでなし", "", "", 1, ["ろく‐で‐なし【◇陸で無し・△碌で無し】\n役に立たない者。つまらない者。のらくら者。「この―め」「陸（ろく）」は水平な状態・平らの意。下に打ち消しの語を伴って、平らではないという意から、物事のようす、性質などが正しくないこと、まともでないさまを表し、「ろくでなし」で役に立たない人をいうようになった。"], 160624, ""], ["碌で無し", "ろくでなし", "", "", 1, ["ろく‐で‐なし【◇陸で無し・△碌で無し】\n役に立たない者。つまらない者。のらくら者。「この―め」「陸（ろく）」は水平な状態・平らの意。下に打ち消しの語を伴って、平らではないという意から、物事のようす、性質などが正しくないこと、まともでないさまを表し、「ろくでなし」で役に立たない人をいうようになった。"], 160624, ""], ["陸でも無い", "ろくでもない", "", "", 1, ["ろく‐でも‐ない【◇陸でも無い・△碌でも無い】\nなんの役にも立たない。つまらない。「―ことをする」"], 160626, ""], ["碌でも無い", "ろくでもない", "", "", 1, ["ろく‐でも‐ない【◇陸でも無い・△碌でも無い】\nなんの役にも立たない。つまらない。「―ことをする」"], 160626, ""]]"#.as_bytes();
        let parsed = parse_term_bank_from_bytes(string).unwrap();
        // TODO: assert
    }

    #[test]
    fn test_correctly_parses_structured_content_only_text() {
        let string = r#"[["強い","つよい","1 adj-i","adj-i",1999800,[{"content":"strong","type":"structured-content"}],1236070,"⭐ ichi news9k"]]"#.as_bytes();

        let parsed = parse_term_bank_from_bytes(string).unwrap();
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
        let string = r#"[["強い","つよい","1 adj-i","adj-i",1999800,[{"content":[{"content":"strong","data":{"content":"glossary"},"lang":"en","style":{"listStyleType":"circle"},"tag":"ul"}],"type":"structured-content"}],1236070,"⭐ ichi news9k"]]"#.as_bytes();

        let parsed = parse_term_bank_from_bytes(string).unwrap();
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
        "#.as_bytes();
        let parsed = parse_term_bank_from_bytes(string).unwrap();
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
        let parsed: StructuredContentObject = sonic_rs::from_str(string).unwrap();
    }

    #[test]
    fn test_parse_structured_content_object_4() {
        let string = r##"{"tag": "tr","content":[{"tag": "td"},{"tag":"th"}]}"##;
        let parsed: StructuredContentObject = sonic_rs::from_str(string).unwrap();
    }

    #[test]
    fn test_parse_structured_content_object_5() {
        let string = r#"[
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
        let parsed: Vec<Definition> = sonic_rs::from_str(string).unwrap();

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
        let string = r#"[["メタ数学", "メタすうがく", "", "", 0, [{"type": "structured-content", "content": [{"tag": "div", "content": [{"tag": "span", "title": "noun (common) (futsuumeishi)", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "565656", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "data": {"code": "n"}, "content": "noun"}, {"tag": "span", "title": "mathematics", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "purple", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "data": {"code": "math"}, "content": "math"}, {"tag": "div", "content": {"tag": "ul", "style": {"listStyleType": "none", "paddingLeft": "0"}, "data": {"content": "glossary"}, "content": {"tag": "li", "content": "metamathematics"}}}]}, {"tag": "div", "style": {"fontSize": "0.7em", "textAlign": "right"}, "data": {"content": "attribution"}, "content": {"tag": "a", "href": "https://www.edrdg.org/jmwsgi/entr.py?svc=jmdict&q=1969080", "content": "JMdict"}}]}], 1969080, ""]]"#.as_bytes();
        let parsed = parse_term_bank_from_bytes(string).unwrap();
        for entry in parsed {
            println!("{:?}", entry);
        }
    }

    #[test]
    fn correctly_parses_structured_content_3() {
        let string = r##"[["ライトウェルター級", "ライトウェルターきゅう", "", "", 0, [{"type": "structured-content", "content": [{"tag": "div", "content": [{"tag": "span", "title": "noun (common) (futsuumeishi)", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "#565656", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "data": {"code": "n"}, "content": "noun"}, {"tag": "span", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "purple", "color": "white", "marginRight": "0.25em"}, "data": {"code": "sports"}, "content": "sports"}, {"tag": "div", "content": {"tag": "ul", "style": {"listStyleType": "none", "paddingLeft": "0"}, "data": {"content": "glossary"}, "content": {"tag": "li", "content": "light welterweight (boxing)"}}}]}, {"tag": "div", "style": {"marginTop": "0.5rem"}, "data": {"content": "forms"}, "content": [{"tag": "span", "title": "spelling and reading variants", "style": {"fontSize": "0.8em", "fontWeight": "bold", "padding": "0.2em 0.3em", "wordBreak": "keep-all", "borderRadius": "0.3em", "verticalAlign": "text-bottom", "backgroundColor": "#565656", "color": "white", "cursor": "help", "marginRight": "0.25em"}, "content": "forms"}, {"tag": "div", "style": {"marginTop": "0.2em"}, "content": {"tag": "table", "content": [{"tag": "tr", "content": [{"tag": "th"}, {"tag": "th", "style": {"fontSize": "1.2em", "textAlign": "center", "fontWeight": "normal"}, "content": "ライトウェルター級"}, {"tag": "th", "style": {"fontSize": "1.2em", "textAlign": "center", "fontWeight": "normal"}, "content": "ライトウエルター級"}]}, {"tag": "tr", "content": [{"tag": "th", "style": {"fontWeight": "normal"}, "content": "ライトウェルターきゅう"}, {"tag": "td", "style": {"textAlign": "center"}, "content": {"tag": "div", "title": "valid form/reading combination", "style": {"cursor": "help", "padding": "0 0.5em", "color": "var(--background-color, var(--canvas, #f8f9fa))", "background": "radial-gradient(var(--text-color, var(--fg, #333)) 50%, white 100%)", "clipPath": "circle()", "fontWeight": "bold"}, "content": "◇"}}, {"tag": "td"}]}, {"tag": "tr", "content": [{"tag": "th", "style": {"fontWeight": "normal"}, "content": "ライトウエルターきゅう"}, {"tag": "td"}, {"tag": "td", "style": {"textAlign": "center"}, "content": {"tag": "div", "title": "valid form/reading combination", "style": {"cursor": "help", "padding": "0 0.5em", "color": "var(--background-color, var(--canvas, #f8f9fa))", "background": "radial-gradient(var(--text-color, var(--fg, #333)) 50%, white 100%)", "clipPath": "circle()", "fontWeight": "bold"}, "content": "◇"}}]}]}}]}, {"tag": "div", "style": {"fontSize": "0.7em", "textAlign": "right"}, "data": {"content": "attribution"}, "content": {"tag": "a", "href": "https://www.edrdg.org/jmwsgi/entr.py?svc=jmdict&q=1969520", "content": "JMdict"}}]}], 1969520, ""]]"##.as_bytes();
        let parsed = parse_term_bank_from_bytes(string).unwrap();
        for entry in parsed {
            println!("{:?}", entry);
        }
    }

    #[test]
    fn correctly_parses_structured_content_4() {
        let string = r#"[["飴とムチ", "", "", "", -102, [{"type": "structured-content", "content": {"tag": "div", "lang": "ja", "style": {"fontSize": "180%", "marginTop": "0.2em"}, "content": ["⟶", {"tag": "a", "href": "?query=%E3%82%A2%E3%83%A1%E3%81%A8%E3%83%A0%E3%83%81&wildcards=off", "content": "アメとムチ"}]}}, ["アメとムチ", ["redirected from 飴とムチ"]]], -1970680, ""]]"#.as_bytes();
        let parsed = parse_term_bank_from_bytes(string).unwrap();
        for entry in parsed {
            println!("{:?}", entry);
        }
    }
}
