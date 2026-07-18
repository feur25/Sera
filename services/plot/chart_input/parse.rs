use super::types::{ChartArgs, ChartOpts};
use serde::Deserialize;

pub fn sanitize_non_finite_json(input: &str) -> String {
    const TOKENS: [(&str, &str); 3] = [("NaN", "0"), ("Infinity", "0"), ("-Infinity", "0")];
    let chars: Vec<char> = input.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;
    let is_word_char = |c: char| c.is_ascii_alphanumeric() || c == '_';
    while i < n {
        let c = chars[i];
        if in_string {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
            continue;
        }
        let mut matched = false;
        for (token, replacement) in TOKENS.iter() {
            let tlen = token.chars().count();
            if i + tlen > n {
                continue;
            }
            if chars[i..i + tlen].iter().collect::<String>() != *token {
                continue;
            }
            let before_ok = i == 0 || !is_word_char(chars[i - 1]);
            let after_ok = i + tlen >= n || !is_word_char(chars[i + tlen]);
            if before_ok && after_ok {
                out.push_str(replacement);
                i += tlen;
                matched = true;
                break;
            }
        }
        if matched {
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

fn parse_or_default<T: serde::de::DeserializeOwned + Default>(sanitized: &str, what: &str) -> T {
    match serde_json::from_str(sanitized) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("seraplot: failed to parse {what} JSON ({e}), using defaults");
            T::default()
        }
    }
}

pub fn parse_opts(opts: &str) -> ChartOpts {
    let sanitized = sanitize_non_finite_json(opts);
    parse_or_default(&sanitized, "chart opts")
}

pub fn parse_args(args: &str) -> ChartArgs {
    let sanitized = sanitize_non_finite_json(args);
    parse_or_default(&sanitized, "chart args")
}

pub fn parse_all(input: &str) -> (String, ChartArgs, ChartOpts) {
    #[derive(Deserialize, Default)]
    struct All {
        #[serde(default)]
        title: String,
        #[serde(flatten)]
        args: ChartArgs,
        #[serde(flatten)]
        opts: ChartOpts,
    }
    let sanitized = sanitize_non_finite_json(input);
    let all: All = parse_or_default(&sanitized, "chart input");
    (all.title, all.args, all.opts)
}

#[cfg(test)]
mod sanitize_tests {
    use super::*;

    #[test]
    fn leaves_normal_json_untouched() {
        let s = r#"{"title":"t","values":[1.0,2.0,3.0]}"#;
        assert_eq!(sanitize_non_finite_json(s), s);
    }

    #[test]
    fn replaces_bare_nan_and_infinity_tokens_with_zero() {
        let s = r#"{"values":[1.0, NaN, Infinity, -Infinity, 3.0]}"#;
        let out = sanitize_non_finite_json(s);
        let parsed: serde_json::Value = serde_json::from_str(&out).expect("sanitized output must be valid JSON");
        let values = parsed["values"].as_array().unwrap();
        assert_eq!(values.len(), 5);
        assert_eq!(values[1], 0);
        assert_eq!(values[2], 0);
        assert_eq!(values[3], 0);
    }

    #[test]
    fn does_not_corrupt_string_values_containing_those_words() {
        let s = r#"{"labels":["NaN Corp","Infinity Stones","-Infinity Ltd"],"values":[1,2,3]}"#;
        let out = sanitize_non_finite_json(s);
        assert_eq!(out, s);
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(parsed["labels"][0], "NaN Corp");
        assert_eq!(parsed["labels"][1], "Infinity Stones");
        assert_eq!(parsed["labels"][2], "-Infinity Ltd");
    }

    #[test]
    fn parse_all_recovers_valid_points_instead_of_whole_payload_going_empty() {
        let input = r#"{"title":"t","labels":["A","B","C"],"values":[1.0, NaN, 3.0]}"#;
        let (title, args, _opts) = parse_all(input);
        assert_eq!(title, "t");
        let values = args.values.expect("values should still deserialize");
        assert_eq!(values, vec![1.0, 0.0, 3.0]);
        assert_eq!(args.labels.unwrap().len(), 3);
    }
}
