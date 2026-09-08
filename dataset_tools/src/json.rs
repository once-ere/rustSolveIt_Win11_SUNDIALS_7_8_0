//! A small, dependency-free JSON value with a parser and an emitter.
//!
//! The sidecar deliberately hand-rolls JSON the same way `posim`'s
//! machine mode does, so the only external crates in this package are
//! the three the project owner specified: rusqlite, r2d2, r2d2_sqlite.
//! The wire format is line-delimited JSON objects; the parser accepts
//! the complete JSON grammar (objects, arrays, strings with escapes,
//! numbers, booleans, null) and the emitter produces valid JSON with
//! keys in deterministic (alphabetical) order.

use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    /// SQLite INTEGER values keep their integer identity end to end:
    /// `COUNT(*)` must render as `42`, never `42.0`.
    Int(i64),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    /// A BTreeMap keeps key order deterministic (alphabetical), which
    /// keeps every reply byte-stable across runs — a repo invariant.
    Obj(BTreeMap<String, Json>),
}

impl Json {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            Json::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }
    pub fn as_obj(&self) -> Option<&BTreeMap<String, Json>> {
        match self {
            Json::Obj(o) => Some(o),
            _ => None,
        }
    }
    pub fn get<'a>(&'a self, key: &str) -> Option<&'a Json> {
        self.as_obj().and_then(|o| o.get(key))
    }
}

/// Emits `v` as compact JSON. Non-finite numbers become `null`: JSON has
/// no NaN/Infinity, and a dataset row must never render invalid JSON.
pub fn emit(v: &Json) -> String {
    let mut out = String::new();
    emit_into(v, &mut out);
    out
}

fn emit_into(v: &Json, out: &mut String) {
    match v {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Int(i) => {
            let _ = write!(out, "{i}");
        }
        Json::Num(n) => {
            if n.is_finite() {
                /* {:?} is Rust's shortest round-trip form; its output
                 * (e.g. `1.5`, `1e-9`, `-0.0`) is valid JSON. */
                let _ = write!(out, "{n:?}");
            } else {
                out.push_str("null");
            }
        }
        Json::Str(s) => emit_str(s, out),
        Json::Arr(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                emit_into(item, out);
            }
            out.push(']');
        }
        Json::Obj(map) => {
            out.push('{');
            for (i, (k, item)) in map.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                emit_str(k, out);
                out.push(':');
                emit_into(item, out);
            }
            out.push('}');
        }
    }
}

fn emit_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Parses one JSON value from `text`, requiring nothing but whitespace
/// to follow it.
pub fn parse(text: &str) -> Result<Json, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut pos = 0usize;
    let v = parse_value(&chars, &mut pos)?;
    skip_ws(&chars, &mut pos);
    if pos != chars.len() {
        return Err(format!("trailing content at offset {pos}"));
    }
    Ok(v)
}

fn skip_ws(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}

fn parse_value(chars: &[char], pos: &mut usize) -> Result<Json, String> {
    skip_ws(chars, pos);
    match chars.get(*pos) {
        None => Err("unexpected end of input".to_string()),
        Some('{') => {
            *pos += 1;
            let mut map = BTreeMap::new();
            skip_ws(chars, pos);
            if chars.get(*pos) == Some(&'}') {
                *pos += 1;
                return Ok(Json::Obj(map));
            }
            loop {
                skip_ws(chars, pos);
                let key = match parse_value(chars, pos)? {
                    Json::Str(s) => s,
                    other => return Err(format!("object key must be a string, got {other:?}")),
                };
                skip_ws(chars, pos);
                if chars.get(*pos) != Some(&':') {
                    return Err(format!("expected `:` at offset {pos}"));
                }
                *pos += 1;
                let val = parse_value(chars, pos)?;
                map.insert(key, val);
                skip_ws(chars, pos);
                match chars.get(*pos) {
                    Some(',') => *pos += 1,
                    Some('}') => {
                        *pos += 1;
                        return Ok(Json::Obj(map));
                    }
                    other => return Err(format!("expected `,` or `}}`, got {other:?}")),
                }
            }
        }
        Some('[') => {
            *pos += 1;
            let mut items = Vec::new();
            skip_ws(chars, pos);
            if chars.get(*pos) == Some(&']') {
                *pos += 1;
                return Ok(Json::Arr(items));
            }
            loop {
                items.push(parse_value(chars, pos)?);
                skip_ws(chars, pos);
                match chars.get(*pos) {
                    Some(',') => *pos += 1,
                    Some(']') => {
                        *pos += 1;
                        return Ok(Json::Arr(items));
                    }
                    other => return Err(format!("expected `,` or `]`, got {other:?}")),
                }
            }
        }
        Some('"') => parse_string(chars, pos).map(Json::Str),
        Some('t') => parse_word(chars, pos, "true", Json::Bool(true)),
        Some('f') => parse_word(chars, pos, "false", Json::Bool(false)),
        Some('n') => parse_word(chars, pos, "null", Json::Null),
        Some(c) if *c == '-' || c.is_ascii_digit() => parse_number(chars, pos),
        Some(c) => Err(format!("unexpected character `{c}` at offset {pos}")),
    }
}

fn parse_word(chars: &[char], pos: &mut usize, word: &str, val: Json) -> Result<Json, String> {
    for w in word.chars() {
        if chars.get(*pos) != Some(&w) {
            return Err(format!("bad literal at offset {pos}, expected `{word}`"));
        }
        *pos += 1;
    }
    Ok(val)
}

fn parse_number(chars: &[char], pos: &mut usize) -> Result<Json, String> {
    let start = *pos;
    if chars.get(*pos) == Some(&'-') {
        *pos += 1;
    }
    while matches!(chars.get(*pos), Some(c) if c.is_ascii_digit()) {
        *pos += 1;
    }
    if chars.get(*pos) == Some(&'.') {
        *pos += 1;
        while matches!(chars.get(*pos), Some(c) if c.is_ascii_digit()) {
            *pos += 1;
        }
    }
    if matches!(chars.get(*pos), Some('e') | Some('E')) {
        *pos += 1;
        if matches!(chars.get(*pos), Some('+') | Some('-')) {
            *pos += 1;
        }
        while matches!(chars.get(*pos), Some(c) if c.is_ascii_digit()) {
            *pos += 1;
        }
    }
    let text: String = chars[start..*pos].iter().collect();
    /* A token with no `.` and no exponent is an integer if it fits i64;
     * everything else is a double. This preserves SQLite's INTEGER vs
     * REAL distinction across the wire. */
    if !text.contains('.') && !text.contains('e') && !text.contains('E') {
        if let Ok(i) = text.parse::<i64>() {
            return Ok(Json::Int(i));
        }
    }
    text.parse::<f64>()
        .map(Json::Num)
        .map_err(|e| format!("bad number `{text}`: {e}"))
}

fn parse_string(chars: &[char], pos: &mut usize) -> Result<String, String> {
    /* chars[*pos] is the opening quote */
    *pos += 1;
    let mut out = String::new();
    loop {
        match chars.get(*pos) {
            None => return Err("unterminated string".to_string()),
            Some('"') => {
                *pos += 1;
                return Ok(out);
            }
            Some('\\') => {
                *pos += 1;
                match chars.get(*pos) {
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some('/') => out.push('/'),
                    Some('b') => out.push('\u{0008}'),
                    Some('f') => out.push('\u{000c}'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some('u') => {
                        let mut code = 0u32;
                        for _ in 0..4 {
                            *pos += 1;
                            let d = chars
                                .get(*pos)
                                .and_then(|c| c.to_digit(16))
                                .ok_or_else(|| "bad \\u escape".to_string())?;
                            code = code * 16 + d;
                        }
                        /* Surrogate pairs: a leading surrogate must be
                         * followed by `\uDCxx`; anything else is an error
                         * rather than silent replacement. */
                        if (0xD800..0xDC00).contains(&code) {
                            if chars.get(*pos + 1) == Some(&'\\')
                                && chars.get(*pos + 2) == Some(&'u')
                            {
                                *pos += 2;
                                let mut low = 0u32;
                                for _ in 0..4 {
                                    *pos += 1;
                                    let d = chars
                                        .get(*pos)
                                        .and_then(|c| c.to_digit(16))
                                        .ok_or_else(|| "bad \\u escape".to_string())?;
                                    low = low * 16 + d;
                                }
                                if !(0xDC00..0xE000).contains(&low) {
                                    return Err("bad low surrogate".to_string());
                                }
                                code = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                            } else {
                                return Err("lone high surrogate".to_string());
                            }
                        } else if (0xDC00..0xE000).contains(&code) {
                            return Err("lone low surrogate".to_string());
                        }
                        out.push(
                            char::from_u32(code)
                                .ok_or_else(|| format!("bad code point {code:#x}"))?,
                        );
                    }
                    other => return Err(format!("bad escape {other:?}")),
                }
                *pos += 1;
            }
            Some(c) => {
                out.push(*c);
                *pos += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_nested_document() {
        let text = r#"{"id":3,"op":"append","rows":[[0,1.5,-2e-3,null,true],"x\n\"y\""]}"#;
        let v = parse(text).unwrap();
        let emitted = emit(&v);
        /* Parse the emitted form again: value-level round trip. */
        assert_eq!(parse(&emitted).unwrap(), v);
        assert_eq!(v.get("id").and_then(Json::as_i64), Some(3));
        assert_eq!(v.get("op").and_then(Json::as_str), Some("append"));
    }

    #[test]
    fn integers_stay_integers_and_reals_stay_real() {
        assert_eq!(parse("42").unwrap(), Json::Int(42));
        assert_eq!(emit(&Json::Int(42)), "42");
        assert_eq!(parse("42.0").unwrap(), Json::Num(42.0));
        assert_eq!(emit(&Json::Num(42.0)), "42.0");
        /* Larger than i64: falls back to f64. */
        assert!(matches!(parse("99999999999999999999").unwrap(), Json::Num(_)));
    }

    #[test]
    fn rejects_trailing_garbage_and_bad_escapes() {
        assert!(parse("{} x").is_err());
        assert!(parse(r#""\q""#).is_err());
        assert!(parse(r#""\ud800""#).is_err());
    }

    #[test]
    fn non_finite_numbers_emit_null() {
        assert_eq!(emit(&Json::Num(f64::NAN)), "null");
        assert_eq!(emit(&Json::Num(f64::INFINITY)), "null");
    }
}
