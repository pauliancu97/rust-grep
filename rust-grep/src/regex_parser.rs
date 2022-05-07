#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum RegexChar {
    StartAnchor,
    EndAnchor,
    MoreOrZero(u8),
    OneOrZero(u8),
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
            _ => {
                if index + 1 < string.len() {
                    match string[index + 1] {
                        b'*' => {
                            let chr = RegexChar::MoreOrZero(string[index]);
                            index += 2;
                            chr
                        },
                        b'?' => {
                            let chr = RegexChar::OneOrZero(string[index]);
                            index += 2;
                            chr
                        },
                        _ => {
                            index += 1;
                            RegexChar::Literal(string[index - 1])
                        }
                    }
                } else {
                    index += 1;
                    RegexChar::Literal(string[index - 1])
                }
            }
        };
        result.push(regex_char);
    }
    Ok(result)
}

pub struct Regex {
    pattern: Vec<RegexChar>
}

impl Regex {
    pub fn new(string: &str) -> Result<Self, &'static str> {
        get_parsed_pattern(string.as_bytes())
            .map(|pattern| Self { pattern })
    }
}

