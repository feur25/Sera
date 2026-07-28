use super::types::{ChartArgs, ChartOpts};
use serde::Deserialize;

pub fn sanitize_non_finite_json(input: &str) -> String {
    let bytes = input.as_bytes();
    let n = bytes.len();
    if !bytes.iter().any(|&b| b == b'N' || b == b'I' || b == b'-') {
        return input.to_string();
    }
    #[inline]
    fn is_word_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
    let mut out = Vec::<u8>::with_capacity(n);
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;
    while i < n {
        let b = bytes[i];
        if in_string {
            out.push(b);
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if b == b'"' {
            in_string = true;
            out.push(b);
            i += 1;
            continue;
        }
        let matched_len = if b == b'N' && bytes[i..].starts_with(b"NaN") {
            3
        } else if b == b'-' && bytes[i..].starts_with(b"-Infinity") {
            9
        } else if b == b'I' && bytes[i..].starts_with(b"Infinity") {
            8
        } else {
            0
        };
        if matched_len > 0 {
            let before_ok = i == 0 || !is_word_byte(bytes[i - 1]);
            let after_ok = i + matched_len >= n || !is_word_byte(bytes[i + matched_len]);
            if before_ok && after_ok {
                out.push(b'0');
                i += matched_len;
                continue;
            }
        }
        out.push(b);
        i += 1;
    }
    unsafe { String::from_utf8_unchecked(out) }
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
