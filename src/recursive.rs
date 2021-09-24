/*
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
*/
fn helper(
    balances: &Vec<i32>,
    from: usize,
    acc: (usize, usize),
    offset: i32,
) -> Option<(usize, usize)> {
    let max_index = balances.len() - 1;
    let current_length = acc.1 - acc.0 + 1;

    if from == max_index || max_index < current_length + from {
        if acc.0 == 0 && acc.1 == 0 {
            return None;
        }
        return Some((acc.0, acc.1));
    }

    let sign = if from == 0 {
        balances[0]
    } else {
        if balances[from] > balances[from - 1] {
            1
        } else {
            -1
        }
    };

    let rightmost_zero_position = balances[from..].iter().rposition(|n| *n - offset == 0);

    match rightmost_zero_position {
        Some(index) => helper(
            balances,
            from + 1,
            if index + 1 > current_length {
                (from, from + index + 1)
            } else {
                acc
            },
            offset + sign,
        ),
        None => helper(balances, from + 1, acc, offset + sign),
    }
}

pub fn identify_subvector(input: &str) -> Option<&str> {
    if input.len() == 0 {
        return None;
    }

    let indexes = helper(
        &input
            .chars()
            .scan(0, |state, char| {
                *state += if char.is_alphabetic() { -1 } else { 1 };
                return Some(*state);
            })
            .collect(),
        0,
        (0, 0),
        0,
    );
    match indexes {
        None => None,
        Some((from, to)) => Some(&input[from..to]),
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
