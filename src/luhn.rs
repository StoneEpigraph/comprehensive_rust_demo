// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let cc_str = cc_number.replace(" ", "");
    if cc_str.len() < 2 {
        return false;
    }
    if cc_str.chars().any(|c| !c.is_digit(10)) {
        return false
    }
    let rev_str: String = cc_str.chars().rev().collect();
    println!("rev str: {}", String::from(&rev_str));
    let odd_str: String = String::from(&rev_str).chars().enumerate()
        .filter(|(index, c)| index % 2 == 0)
        .map(|(_, c)| c)
        .collect();
    let even_str: String = String::from(&rev_str).chars().enumerate()
        .filter(|(index, c)| index % 2 == 1)
        .map(|(_, c)| c)
        .collect();
    println!("odd str: {}, even str: {}", odd_str, even_str);

    let odd_sum: u32 = String::from(&rev_str).chars()
        .enumerate()
        .filter(|(index, c)| index % 2 == 0)
        .map(|(index, c)| c.to_digit(10).unwrap_or(0))
        .sum();
    let even_sum: u32 = String::from(&rev_str).chars().enumerate()
        .filter(|(index, c)| index % 2 == 1)
        .map(|(_, c)| c.to_digit(10).unwrap_or(0) * 2)
        .map(|c| -> u32 {
            if c > 9 {
                c % 10 + c / 10
            } else {
                c
            }
        }).sum();
    println!("odd sum: {}", odd_sum);
    println!("even sum: {}", even_sum);
    (odd_sum + even_sum) % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}