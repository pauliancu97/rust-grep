#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
                    return Err("invalid regex");
                }
                let chr = RegexChar::Literal(string[index + 1]);
                index += 2;
                chr
            },
            b'^' => {
                index += 1;
                RegexChar::StartAnchor
            },
            b'$' => {
                index += 1;
                RegexChar::EndAnchor
            },
            b'*' => {
                let last_regex_char = result.pop().ok_or("invalid regex")?;
                let cardinal_char = CardinalChar::get(last_regex_char).ok_or("invalid regex")?;
                index += 1;
                RegexChar::MoreOrZero(cardinal_char)
            }
            b'?' => {
                let last_regex_char = result.pop().ok_or("invalid regex")?;
                let cardinal_char = CardinalChar::get(last_regex_char).ok_or("invalid regex")?;
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
            if chr == text[0] {
                is_match_here(&regex[1..], &text[1..])
            } else {
                false
            }
        },
        RegexChar::Wildcard => is_match_here(&regex[1..], &text[1..]),
        RegexChar::OneOrZero(chr) => {
            if chr.is_match(text[0]) {
                is_match_here(&regex[1..], &text[1..])
            } else {
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

