pub mod test_inputs;

pub fn identify_subvector_vecs(input: &str) -> Option<&str> {
    // First calculate position_vec:
    // Initialize all elements as 0
    // Iterate over characters of the slice and save all numbers:
    //      If it's a number => add one to the previous number
    //      If it's not      => subtract one from the previous number
    // position_vec basically describes a path in this alphanumeric space
    // Vector's first element always stays 0 -> starting point
    // Also, save the maximum absolute value of the "positions" for later
    let mut position_vec = vec![0_i16; input.len() + 1];

    let mut max_abs = 0_u16;
    /* More elegant solution but slower by ~30 ns/iter ( over 10% increase! ) on my computer
    input.chars().enumerate().for_each(|(i, ch)| {
        let i = i+1;
        position_vec[i] = position_vec[i-1];
        if ch.is_digit(10){
            position_vec[i] += 1;
        } else{
            position_vec[i] -= 1;
        }
        let abs = position_vec[i].unsigned_abs();
        if abs > max_abs {
            max_abs = abs;
        }
    });
    */

    let mut i = 1;
    for c in input.chars() {
        position_vec[i] = position_vec[i - 1];
        if c.is_digit(10) {
            position_vec[i] += 1;
        } else {
            position_vec[i] -= 1;
        }
        let abs = position_vec[i].unsigned_abs();
        if abs > max_abs {
            max_abs = abs;
        }
        i += 1;
    }

    let max_abs = usize::from(max_abs);
    if usize::from(max_abs) == input.len() {
        return None;
    }

    // Save the index of the first and last occurences of all possible "positions" in position_vec
    //      from (-max_abs) to max_abs (inclusive on both sides)
    // Use a separate vector for both, with length max_abs * 2 + 1 to always have just enough space
    // Initialize both vectors with appropriate values (MAX or 0)
    // Indexing these vectors is tricky: index of "position" X is (X+max_abs)
    let mut min_indices = vec![usize::MAX; max_abs.checked_mul(2).unwrap().checked_add(1).unwrap()];
    let mut max_indices = vec![0_usize; max_abs.checked_mul(2).unwrap().checked_add(1).unwrap()];

    for i in 0..position_vec.len() {
        let pos = position_vec[i];

        // The shifting is also tricky because of type constraints
        let shifted_pos;
        if pos.is_negative() {
            shifted_pos = max_abs
                .checked_sub(usize::from(pos.unsigned_abs()))
                .unwrap();
        } else {
            shifted_pos = max_abs
                .checked_add(usize::from(pos.unsigned_abs()))
                .unwrap();
        }

        // If the index is lower/higher than the currently known min/max then save it
        if i < min_indices[shifted_pos] {
            min_indices[shifted_pos] = i;
        }
        if i > max_indices[shifted_pos] {
            max_indices[shifted_pos] = i;
        }
    }

    // Then, Iterate over the index vectors to find the maximal difference between max and min of the "positions"
    //      <=> find the longest subarray that contains the same amount of letters and numbers
    //      <=> find the longest subpath in the alphanumeric space that arrives at its start location
    let mut max_len = 0;
    let mut ind = 0;
    for i in 0..min_indices.len() {
        if let Some(current_len) = max_indices[i].checked_sub(min_indices[i]) {
            // With the current indexing, if there are multiple subarrays with max length, it might not find the first one
            // The first of such subarrays' "position" must have lower absolute value
            //      -> it is closer to 0, thus closer to the starting point
            //      -> the indexes of the longest such subarray is located closer to the middle (max_abs) of min_indices and max_indices
            // AND if there were multiple such subarrays of the same size with positive "position" and negative "position",
            //      then the subarray with "position" 0 must be longer, since the whole path starts on "position" 0 and
            //      the path between the two "positions" must cross "position" 0
            if i < max_abs {
                if current_len >= max_len {
                    max_len = current_len;
                    ind = i;
                }
            } else if current_len > max_len {
                max_len = current_len;
                ind = i;
            }
        }
    }

    
    if ind < max_abs {
        ind = max_abs - ind;
    }
    Some(&input[min_indices[ind]..max_indices[ind]])
}

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
        assert_eq!(None, identify_subvector_slow("123455"));
        assert_eq!(None, identify_subvector_fast("123455"));
        assert_eq!(None, identify_subvector_slow("abcdefg"));
        assert_eq!(None, identify_subvector_fast("abcdefg"));
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
