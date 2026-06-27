pub fn run_length_encode(input: &str) -> Vec<(char, u32)> {
    // let _ = input;
    // todo!("implement run_length_encode")

    let mut res = Vec::new();
    let mut chars = input.chars();

    let mut curr_char = match chars.next() {
        Some(c) => c,
        None => return res,
    };
    let mut count = 1;
    for c in chars {
        if c == curr_char {
            count = count + 1;
        }
        else {
            res.push((curr_char,count));
            curr_char = c;
            count = 1;
        }
    }
    res.push((curr_char,count));
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_aaabbc() {
        assert_eq!(
            run_length_encode("aaabbc"),
            vec![('a', 3), ('b', 2), ('c', 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(run_length_encode(""), vec![]);
    }

    #[test]
    fn single_char() {
        assert_eq!(run_length_encode("x"), vec![('x', 1)]);
    }

    #[test]
    fn all_same() {
        assert_eq!(run_length_encode("aaaaa"), vec![('a', 5)]);
    }

    #[test]
    fn all_different() {
        assert_eq!(
            run_length_encode("abcd"),
            vec![('a', 1), ('b', 1), ('c', 1), ('d', 1)]
        );
    }

    #[test]
    fn alternating_runs() {
        assert_eq!(
            run_length_encode("aabbaa"),
            vec![('a', 2), ('b', 2), ('a', 2)]
        );
    }

    #[test]
    fn whitespace_counts_too() {
        assert_eq!(
            run_length_encode("aa  bb"),
            vec![('a', 2), (' ', 2), ('b', 2)]
        );
    }
}
