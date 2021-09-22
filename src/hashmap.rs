pub fn identify_subvector(input: &str) -> Option<&str> {
    let mut map = std::collections::HashMap::<isize, usize>::new();
    let mut sum = 0_isize;
    let mut start = 0_usize;
    let mut max_length = 0_usize;
    map.insert(0, 0);

    input.chars().enumerate().for_each(|(i, ch)| {
        if ch.is_digit(10) {
            sum += 1
        } else {
            sum -= 1
        }
        if let Some(earlier_index) = map.get(&sum) {
            if max_length < i - earlier_index {
                max_length = i - earlier_index;
                start = *earlier_index;
            }
        } else {
            map.insert(sum, i + 1);
        }
    });
    if max_length == 0 {
        None
    } else {
        input.get(start..=start + max_length)
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
