const ROMAN_CONVERSIONS: [(i32, &'static str); 9] = [
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub fn to_roman(x: i32) -> String {
    for (decimal, roman) in ROMAN_CONVERSIONS {
        if x >= decimal {
            return String::from(roman) + &to_roman(x - decimal);
        }
    }
    return String::new();
}

pub fn to_decimal(s: &str) -> i32 {
    for (decimal, roman) in ROMAN_CONVERSIONS {
        if s.starts_with(roman) {
            return decimal + to_decimal(&s[(roman).len()..s.len()]);
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: [(i32, &'static str); 14] = [
        (1, "I"),
        (2, "II"),
        (3, "III"),
        (4, "IV"),
        (5, "V"),
        (7, "VII"),
        (9, "IX"),
        (14, "XIV"),
        (39, "XXXIX"),
        (42, "XLII"),
        (76, "LXXVI"),
        (91, "XCI"),
        (101, "CI"),
        (109, "CIX"),
    ];

    #[test]
    fn converting_to_roman_numerals() {
        for (decimal, roman) in TEST_CASES {
            assert_eq!(to_roman(decimal), roman);
        }
    }

    #[test]
    fn converting_from_roman_numerals() {
        for (decimal, roman) in TEST_CASES {
            assert_eq!(to_decimal(roman), decimal);
        }
    }
}
