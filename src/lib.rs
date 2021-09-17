pub mod test_inputs;

pub fn identify_subvector_slow(input: &str) -> Option<&str> {
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

pub fn identify_subvector_fast(input: &str) -> Option<&str> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identify_subvector_test() {
        assert_eq!(None, identify_subvector_slow(""));
        assert_eq!(None, identify_subvector_fast(""));
        assert_eq!(None, identify_subvector_slow("a"));
        assert_eq!(None, identify_subvector_fast("a"));
        assert_eq!(None, identify_subvector_slow("1"));
        assert_eq!(None, identify_subvector_fast("1"));
        assert_eq!(Some("1a"), identify_subvector_slow("1a"));
        assert_eq!(Some("1a"), identify_subvector_fast("1a"));
        assert_eq!(Some("1a"), identify_subvector_slow("1ab"));
        assert_eq!(Some("1a"), identify_subvector_fast("1ab"));
        assert_eq!(Some("ac412b"), identify_subvector_slow("0ac412b023"));
        assert_eq!(Some("ac412b"), identify_subvector_fast("0ac412b023"));
        assert_eq!(Some("dac412b0"), identify_subvector_slow("dac412b02"));
        assert_eq!(Some("dac412b0"), identify_subvector_fast("dac412b02"));
        assert_eq!(Some("b02c"), identify_subvector_slow("12c412b02c"));
        assert_eq!(Some("b02c"), identify_subvector_fast("12c412b02c"));
        assert_eq!(Some("z12c412bkc"), identify_subvector_slow("z12c412bkc"));
        assert_eq!(Some("z12c412bkc"), identify_subvector_fast("z12c412bkc"));
        assert_eq!(Some("g2"), identify_subvector_slow("abfdg2c"));
        assert_eq!(Some("z12c412bkc"), identify_subvector_fast("z12c412bkc"));
        assert_eq!(Some("1a"), identify_subvector_slow("1a345c0023e"));
        assert_eq!(Some("1a"), identify_subvector_fast("1a345c0023e"));
        assert_eq!(Some("ab19"), identify_subvector_fast("ffab19fffa9f"));
    }
}
