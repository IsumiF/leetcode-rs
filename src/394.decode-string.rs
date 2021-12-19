/*
 * @lc app=leetcode id=394 lang=rust
 *
 * [394] Decode String
 */

struct Solution {}

// @lc code=start
use std::iter::Peekable;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut token_stack = Vec::<Token>::new();
        for token in tokenize(&s) {
            if let Token::Closing = token {
                let mut partial_strs = Vec::<String>::new();
                loop {
                    let token = token_stack.pop().unwrap();
                    match token {
                        Token::String(s) => {
                            partial_strs.push(s);
                        }
                        Token::Opening(num) => {
                            let new_str = partial_strs.into_iter().rev().fold(
                                String::new(),
                                |mut acc, next| {
                                    acc.push_str(&next);
                                    acc
                                },
                            );
                            let new_str =
                                (0..num)
                                    .map(|_| &new_str)
                                    .fold(String::new(), |mut acc, next| {
                                        acc.push_str(next);
                                        acc
                                    });
                            token_stack.push(Token::String(new_str));
                            break;
                        }
                        _ => {
                            panic!("closing inside token stack");
                        }
                    }
                }
            } else {
                token_stack.push(token);
            }
        }
        token_stack
            .into_iter()
            .map(|token| {
                if let Token::String(s) = token {
                    s
                } else {
                    String::new()
                }
            })
            .fold(String::new(), |mut acc, next| {
                acc.push_str(&next);
                acc
            })
    }
}

#[derive(Debug)]
enum Token {
    Opening(usize),
    Closing,
    String(String),
}

fn tokenize(s: &str) -> Tokenize {
    Tokenize::new(s)
}

struct Tokenize<'a> {
    chars: Peekable<std::str::Chars<'a>>,
}

impl<'a> Tokenize<'a> {
    fn new(s: &'a str) -> Self {
        let chars = s.chars().peekable();
        Self { chars }
    }
}

impl<'a> Iterator for Tokenize<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ch = *self.chars.peek()?;
        if ch == ']' {
            self.chars.next();
            Some(Token::Closing)
        } else if ch.is_digit(10) {
            let mut num: usize = 0;
            while let Some(d) = char::to_digit(ch, 10) {
                num *= 10;
                num += d as usize;
                self.chars.next();
                match self.chars.peek() {
                    None => break,
                    Some(c) => ch = *c,
                }
            }
            if ch != '[' {
                panic!("expecting [ after integer")
            }
            self.chars.next();
            Some(Token::Opening(num))
        } else {
            let mut string = String::new();
            while !char::is_digit(ch, 10) && ch != ']' {
                string.push(ch);
                self.chars.next();
                match self.chars.peek() {
                    None => break,
                    Some(c) => ch = *c,
                }
            }
            Some(Token::String(string))
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("3[a]2[bc]" => "aaabcbc")]
    #[test_case("2[abc]3[cd]ef" => "abcabccdcdcdef"; "two")]
    #[test_case("hi2[abc]3[cd]ef" => "hiabcabccdcdcdef"; "has_begining")]
    #[test_case("3[a2[c]]" => "accaccacc")]
    #[test_case("3[z]2[2[y]pq4[2[jk]e1[f]]]ef" => "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef"; "long")]
    fn test(s: &str) -> String {
        Solution::decode_string(s.to_string())
    }
}
