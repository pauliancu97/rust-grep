#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum CardinalChar {
    Literal(u8),
    Wildcard
}


impl CardinalChar {
    fn get(regex_char: RegexChar) -> Option<Self> {
        match regex_char {
            RegexChar::Wildcard => Some(CardinalChar::Wildcard),
            RegexChar::Literal(chr) => Some(CardinalChar::Literal(chr)),
            _ => None 
        }
    }

    fn is_match(&self, chr: u8) -> bool {
        match self {
            &CardinalChar::Wildcard => true,
            &CardinalChar::Literal(literal) => literal == chr
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum RegexChar {
    StartAnchor,
    EndAnchor,
    MoreOrZero(CardinalChar),
    OneOrZero(CardinalChar),
    Wildcard,
    Literal(u8)
}

fn get_parsed_pattern(string: &[u8]) -> Result<Vec<RegexChar>, &'static str> {
    let mut result: Vec<RegexChar> = Vec::new();
    let mut index: usize = 0;
    while index < string.len() {
        let regex_char = match string[index] {
            b'\\' => {
                if index + 1 >= string.len() {
                    return Err("no escaped character");
                }
                let chr = RegexChar::Literal(string[index + 1]);
                index += 2;
                chr
            },
            b'^' => {
                if index != 0 {
                    return Err("invalid start anchor")
                }
                index += 1;
                RegexChar::StartAnchor
            },
            b'$' => {
                if index != string.len() - 1 {
                    return Err("invalid end anchor");
                }
                index += 1;
                RegexChar::EndAnchor
            },
            b'*' => {
                let last_regex_char = result.pop().ok_or("more or zero expected character")?;
                let cardinal_char = CardinalChar::get(last_regex_char).ok_or("invalid for more or zero")?;
                index += 1;
                RegexChar::MoreOrZero(cardinal_char)
            }
            b'?' => {
                let last_regex_char = result.pop().ok_or("one or zero expected character")?;
                let cardinal_char = CardinalChar::get(last_regex_char).ok_or("invalid for one or zero")?;
                index += 1;
                RegexChar::OneOrZero(cardinal_char)
            }
            b'.' => {
                index += 1;
                RegexChar::Wildcard
            }
            chr => {
                index += 1;
                RegexChar::Literal(chr)
            }
        };
        result.push(regex_char);
    }
    Ok(result)
}

fn is_match(regex: &[RegexChar], text: &[u8]) -> bool {
    if !regex.is_empty() && regex[0] == RegexChar::StartAnchor {
        is_match_here(&regex[1..], text)
    } else {
        for index in 0..text.len() {
            if is_match_here(regex, &text[index..]) {
                return true;
            }
        }
        false
    }
}

fn is_match_here(regex: &[RegexChar], text: &[u8]) -> bool {
    if regex.is_empty() {
        return true;
    }
    if regex[0] == RegexChar::EndAnchor && regex.len() == 1 {
        return text.is_empty();
    }
    if let RegexChar::MoreOrZero(chr) = regex[0] {
        return is_match_star(chr, &regex[1..], text);
    }
    match regex[0] {
        RegexChar::Literal(chr) => {
            if text.len() == 0 {
                false
            } else if chr == text[0] {
                is_match_here(&regex[1..], &text[1..])
            } else {
                false
            }
        },
        RegexChar::Wildcard => is_match_here(&regex[1..], &text[1..]),
        RegexChar::OneOrZero(chr) => {
            if text.len() == 0 {
                is_match_here(&regex[1..], text)
            } else {
                (chr.is_match(text[0]) && is_match_here(&regex[1..], &text[1..])) ||
                    is_match_here(&regex[1..], text)
            }
        }
        _ => false
    }
}

fn is_match_star(cardinal_char: CardinalChar, regex: &[RegexChar], text: &[u8]) -> bool {
    let mut index: usize = 0;
    let mut is_matched = is_match_here(regex, text);
    while index < text.len() && !is_matched && cardinal_char.is_match(text[index]) {
        index += 1;
        is_matched = is_match_here(regex, &text[index..]);
    }
    is_matched
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Regex {
    pattern: Vec<RegexChar>
}

impl Regex {
    pub fn new(string: &str) -> Result<Self, &'static str> {
        get_parsed_pattern(string.as_bytes())
            .map(|pattern| Self { pattern })
    }

    pub fn is_match(&self, string: &str) -> bool {
        is_match(&self.pattern, string.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn simple_regex() {
        let regex = Regex::new("abc").unwrap();
        assert!(regex.is_match("abc"));
        assert!(!regex.is_match("cba"));
    }

    #[test]
    fn simple_wildcard() {
        let regex = Regex::new("a.c").unwrap();
        assert!(regex.is_match("abc"));
        assert!(regex.is_match("adc"));
        assert!(!regex.is_match("bac"));
    }

    #[test]
    fn simple_zero_or_more() {
        let regex = Regex::new("abcd*").unwrap();
        assert!(regex.is_match("abc"));
        assert!(regex.is_match("abcd"));
        assert!(regex.is_match("abcddd"));
        assert!(!regex.is_match("abd"));
    }

    #[test]
    fn more_or_zero_with_regex_after() {
        let regex = Regex::new("abcd*efg").unwrap();
        assert!(regex.is_match("abcefg"));
        assert!(regex.is_match("abcdefg"));
        assert!(regex.is_match("abcddddefg"));
        assert!(!regex.is_match("xyzdddefg"));
    }

    #[test]
    fn more_or_more_with_wildcard() {
        let regex = Regex::new("ab.*").unwrap();
        assert!(regex.is_match("ab"));
        assert!(regex.is_match("abc"));
        assert!(regex.is_match("abccc"));
        assert!(regex.is_match("abcde"));
        assert!(!regex.is_match("acerfsdsd"));
    }

    #[test]
    fn more_or_zero_wildcard_and_regex_after() {
        let regex = Regex::new("ab.*xyz").unwrap();
        assert!(regex.is_match("abdefgxyz"));
        assert!(regex.is_match("abxyz"));
        assert!(!regex.is_match("cvxyz"));
        assert!(!regex.is_match("abqwerty"));
    }

    #[test]
    fn simple_one_or_zero() {
        let regex = Regex::new("abc?").unwrap();
        assert!(regex.is_match("ab"));
        assert!(regex.is_match("abc"));
        assert!(!regex.is_match("vbc"));
    }

    #[test]
    fn one_or_zero_regex_after() {
        let regex = Regex::new("abc?de").unwrap();
        assert!(regex.is_match("abde"));
        assert!(regex.is_match("abcde"));
        assert!(!regex.is_match("abcccde"));
    }

    #[test]
    fn one_or_zero_with_wildcard_regex() {
        let regex = Regex::new("ab.?").unwrap();
        assert!(regex.is_match("ab"));
        assert!(regex.is_match("abc"));
        assert!(regex.is_match("abf"));
        assert!(!regex.is_match("xbc"));
    }

    #[test]
    fn one_or_zero_with_wildcard_regex_after() {
        let regex = Regex::new("ab.?de").unwrap();
        assert!(regex.is_match("abde"));
        assert!(regex.is_match("abcde"));
        assert!(regex.is_match("abfde"));
        assert!(!regex.is_match("abcccde"));
    }

    #[test]
    fn match_in_middle() {
        let regex = Regex::new("ab.?de").unwrap();
        assert!(regex.is_match("xxxabcdexxx"));
    }

    #[test]
    fn simple_start_anchor() {
        let regex = Regex::new("^ab.?de").unwrap();
        assert!(regex.is_match("abcde"));
        assert!(regex.is_match("abcdeghjghjgh"));
        assert!(!regex.is_match("xxxabcde"));
    }

    #[test]
    fn simple_end_anchor() {
        let regex = Regex::new("ab.?de$").unwrap();
        assert!(regex.is_match("abcde"));
        assert!(regex.is_match("xxxabcde"));
        assert!(!regex.is_match("abcdexxx"));
    }

    #[test]
    fn escape_character() {
        let regex = Regex::new(r"ab\.cd").unwrap();
        assert!(regex.is_match("ab.cd"));
        assert!(!regex.is_match("abecd"));
        let regex = Regex::new(r"ab\*cd").unwrap();
        assert!(regex.is_match("ab*cd"));
        assert!(!regex.is_match("abecd"));
        let regex = Regex::new(r"ab\?cd").unwrap();
        assert!(regex.is_match("ab?cd"));
        assert!(!regex.is_match("abecd"));
        let regex = Regex::new(r"ab\^cd").unwrap();
        assert!(regex.is_match("ab^cd"));
        assert!(!regex.is_match("abecd"));
        let regex = Regex::new(r"ab\$cd").unwrap();
        assert!(regex.is_match("ab$cd"));
        assert!(!regex.is_match("abecd"));
        let regex = Regex::new(r"ab\\cd").unwrap();
        assert!(regex.is_match(r"ab\cd"));
        assert!(!regex.is_match("abecd"));
    }

    #[test]
    fn no_escaped_character() {
        assert_eq!(Err("no escaped character"), Regex::new(r"abc\"));
    }

    #[test]
    fn more_or_zero_expected_character() {
        assert_eq!(Err("more or zero expected character"), Regex::new(r"*sadsd"));
    }

    #[test]
    fn invalid_for_more_or_zero() {
        assert_eq!(Err("invalid for more or zero"), Regex::new(r"^*asdasd"));
    }

    #[test]
    fn one_or_zero_expected_character() {
        assert_eq!(Err("one or zero expected character"), Regex::new("?asdasd"));
    }

    #[test]
    fn invalid_for_one_or_zero() {
        assert_eq!(Err("invalid for one or zero"), Regex::new("^?asdasd"));
    }

    #[test]
    fn invalid_start_anchor() {
        assert_eq!(Err("invalid start anchor"), Regex::new("asdasd^asdasd"));
    }

    #[test]
    fn invalid_end_anchor() {
        assert_eq!(Err("invalid end anchor"), Regex::new("asdasd$asdasd"));
    }
}