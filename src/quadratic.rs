pub fn identify_subvector(input: &str) -> Option<&str> {
    let mut start = 0;
    let mut end = 0;
    let mut max_length = 0;
    let mut digit_counter = 0;
    let mut char_counter = 0;

    for i in 0..input.len() {
        // NOTE unwrap is fine here because i..input.len() is always a valid
        // range for our str
        let range = i..input.len();
        for (j, ch) in range.clone().zip(input.get(range).unwrap().chars()) {
            if ch.is_digit(10) {
                //for j in i..input.len() {
                //    if input[j..j + 1].chars().next().unwrap().is_digit(10) {
                digit_counter += 1
            } else {
                char_counter += 1
            }
            if digit_counter == char_counter && j - i > max_length {
                max_length = j - i;
                start = i;
                end = j;
            }
        }
        digit_counter = 0;
        char_counter = 0;
    }

    if start == end {
        None
    } else {
        input.get(start..=end)
    }
}

#[test]
fn identify_subvector_test() {
    assert_eq!(None, identify_subvector(""));
    assert_eq!(None, identify_subvector("a"));
    assert_eq!(None, identify_subvector("1"));
    assert_eq!(None, identify_subvector("123455"));
    assert_eq!(None, identify_subvector("abcdefg"));
    assert_eq!(Some("1a"), identify_subvector("1a"));
    assert_eq!(Some("1a"), identify_subvector("1ab"));
    assert_eq!(Some("ac412b"), identify_subvector("0ac412b023"));
    assert_eq!(Some("dac412b0"), identify_subvector("dac412b02"));
    assert_eq!(Some("b02c"), identify_subvector("12c412b02c"));
    assert_eq!(Some("z12c412bkc"), identify_subvector("z12c412bkc"));
    assert_eq!(Some("g2"), identify_subvector("abfdg2c"));
    assert_eq!(Some("1a"), identify_subvector("1a345c0023e"));
    assert_eq!(Some("ab19"), identify_subvector("ffab19fffa9f"));
}
