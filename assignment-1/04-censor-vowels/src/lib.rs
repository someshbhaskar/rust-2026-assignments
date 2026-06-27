pub fn censor_vowels(s: &mut String) {
    // let _ = s;
    // todo!("implement censor_vowels")

    let bytes = unsafe { s.as_bytes_mut() };
    for byte in bytes.iter_mut() {
        match byte {
            b'a' | b'A' | b'e' | b'E' | b'i' |
            b'I' | b'o' | b'O' | b'u' | b'U' => {
                *byte = b'*';
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> String {
        let mut s = String::from(input);
        censor_vowels(&mut s);
        s
    }

    #[test]
    fn example_hello_world() {
        assert_eq!(run("Hello, World!"), "H*ll*, W*rld!");
    }

    #[test]
    fn all_uppercase_vowels() {
        assert_eq!(run("AEIOU"), "*****");
    }

    #[test]
    fn empty_input() {
        assert_eq!(run(""), "");
    }

    #[test]
    fn no_vowels() {
        assert_eq!(run("bcdfg"), "bcdfg");
    }

    #[test]
    fn all_lowercase_vowels() {
        assert_eq!(run("aeiou"), "*****");
    }

    #[test]
    fn mixed_case() {
        assert_eq!(run("AaEeIi"), "******");
    }

    #[test]
    fn digits_and_letters() {
        assert_eq!(run("h3ll0 wOrld"), "h3ll0 w*rld");
    }

    #[test]
    fn punctuation_only() {
        assert_eq!(run("!@#$%"), "!@#$%");
    }
}
