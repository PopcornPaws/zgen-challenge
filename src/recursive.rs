pub fn identify_subvector(input: &str) -> Option<&str> {
    match input {
        "" => None,
        input => {
            match input
                .chars()
                .fold(0_isize, |acc, ch| if ch.is_alphabetic() { acc + 1 } else { acc - 1 })
                //.map(|char| if char.is_alphabetic() { 1 } else { -1 })
                //.sum()
            {
                0 => Some(input),
                _ => {
                    let left_subvector = identify_subvector(&input[..input.len() - 1]);
                    let right_subvector = identify_subvector(&input[1..]);
                    match [left_subvector, right_subvector] {
                        [Some(left), Some(right)] => Some(if right.len() > left.len() {
                            right
                        } else {
                            left
                        }),
                        [Some(left), None] => Some(left),
                        [None, Some(right)] => Some(right),
                        [None, None] => return None,
                    }
                }
            }
        }
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
