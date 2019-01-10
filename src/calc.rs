// calc.rs use my crate onigiri 0.1.11.
// This is a library for handling chars.
// `is_integer` validate whether it is integer.
// `is_float` validate whether it is float.

use onigiri::validator::{is_integer, is_float};

const NOT_NUMBER: &str = "Contain not-number.";
const NOT_EXIST_TYPE: &str = "Not exist type.";


macro_rules! parse_num {
    ( $v:expr, $t:ty) => ( $v.parse::<$t>() );
}

macro_rules! sum {
    ( $v:expr, $t:ty ) => ({
        let result = $v.iter()
            .map(|ref v| parse_num!(v, $t).unwrap())
            .sum::<$t>();
        format!("{}", result)
    });
}

macro_rules! product {
    ( $v:expr, $t:ty ) => ({
        let result = $v.iter()
            .map(|ref v| parse_num!(v, $t).unwrap())
            .product::<$t>();
        format!("{}", result)
    });
}

/// Return the sum of numbers.
/// ```no_run
/// let test_v = vec!["u8", "1", "2"];
/// assert_eq!(
///     sum(test_v),
///     "3".to_string()
/// );
/// ```
/// If you don't input "u8", Error. 
pub fn sum(v: &Vec<String>) -> String {
    let (typ, values) = (&v[..1], &v[1..]);

    let vvc: Vec<Vec<char>> = values.iter()
        .map(|ref p| p.chars().collect())
        .collect();

    // If vvc contains not-integer or not-float, 
    // return error string-message. And main loop continue.
    // Therefore, I don't use Result.
    if vvc.iter().all(|r| is_integer(&r) == true) {
        match typ[0].as_str() {
            "u8" => sum!(values, u8),
            "i8" => sum!(values, i8),
            "u16" => sum!(values, u16),
            "i16" => sum!(values, i16),
            "u32" => sum!(values, u32),
            "i32" => sum!(values, i32),
            "u64" => sum!(values, u64),
            "i64" => sum!(values, i64),
            "u128" => sum!(values, u128),
            "i128" => sum!(values, i128),
            "usize" => sum!(values, usize),
            "isize" => sum!(values, isize),
            _ => format!(
                "{} or {}",
                NOT_EXIST_TYPE,
                NOT_NUMBER,
                )

        }
    } else if vvc.iter().all(|r| is_float(&r) == true) {
        match typ[0].as_str() {
            "f32" => sum!(values, f32),
            "f64" => sum!(values, f64),
            _ => format!(
                "{} or {}",
                NOT_EXIST_TYPE,
                NOT_NUMBER,
                )
        }
    } else {
        format!("{}", NOT_NUMBER)
    }
}

/// Return the product of numbers.
/// ```no_run
/// let test_v = vec!["f64", "100.0", "0.75"];
/// assert_eq!(
///     prod(test_v),
///     "75".to_string()
/// );
/// ```
pub fn prod(v: &Vec<String>) -> String {
    let (typ, values) = (&v[..1], &v[1..]);

    let vvc: Vec<Vec<char>> = values.iter()
        .map(|ref p| p.chars().collect())
        .collect();

    if vvc.iter().all(|r| is_integer(&r) == true) {
        match typ[0].as_str() {
            "u8" => product!(values, u8),
            "i8" => product!(values, i8),
            "u16" => product!(values, u16),
            "i16" => product!(values, i16),
            "u32" => product!(values, u32),
            "i32" => product!(values, i32),
            "u64" => product!(values, u64),
            "i64" => product!(values, i64),
            "u128" => product!(values, u128),
            "i128" => product!(values, i128),
            "usize" => product!(values, usize),
            "isize" => product!(values, isize),
            _ => format!(
                "{} or {}",
                NOT_EXIST_TYPE,
                NOT_NUMBER,
                )

        }
    } else if vvc.iter().all(|r| is_float(&r) == true) {
        match typ[0].as_str() {
            "f32" => product!(values, f32),
            "f64" => product!(values, f64),
            _ => format!(
                "{} or {}",
                NOT_EXIST_TYPE,
                NOT_NUMBER,
                )
        }
    } else {
        format!("{}", NOT_NUMBER)
    }
}
