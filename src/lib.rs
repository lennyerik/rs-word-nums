use proc_macro::{Ident, Literal, Span, TokenStream, TokenTree};

type NumType = i128;

#[proc_macro]
pub fn num(token_stream: TokenStream) -> TokenStream {
    match parse_tokens(token_stream) {
        Ok(mut num_tokens) => {
            let sign = get_sign(&mut num_tokens);

            // Add the implicit 1 at the start for number strings that start with
            // a multiplier, like "hundred fifity two"
            if let Some(NumToken::Multiplier(_)) = num_tokens.first() {
                num_tokens.insert(0, NumToken::Literal(1));
            }

            // Generate the number literal
            let mut sum = 0;
            let mut acc = 0;
            for (i, num_token) in num_tokens.iter().enumerate() {
                match num_token {
                    NumToken::Literal(value) => acc += value,
                    NumToken::Multiplier(value) => {
                        acc *= value;
                        if !num_tokens
                            .iter()
                            .skip(i + 1)
                            .any(|x| is_larger_multiplier(*x, *value))
                        {
                            sum += acc;
                            acc = 0;
                        }
                    }

                    // Any subsequent signs are invalid and should be ignored.
                    // We should never get here anyways, because parse_tokens is going to return an error in this case.
                    NumToken::Sign(_) => {}
                }
            }

            sum += acc;
            if let Sign::Negative = sign {
                sum = -sum;
            }

            let literal = make_sized_num_literal(sign, sum);

            let mut out = TokenStream::new();
            out.extend([TokenTree::Literal(literal)]);
            out
        }

        Err(err) => {
            let (err_str, span) = match err {
                NumTokenParseError::NonIdentToken(tt) => ("Non-identifier encountered", tt.span()),
                NumTokenParseError::InvalidToken(ident) => {
                    ("Invalid token encountered", ident.span())
                }
                NumTokenParseError::UnexpectedSign(ident) => {
                    ("Unexpected sign descriptor encountered", ident.span())
                }
            };

            let compile_err = format!(r#"compile_error!("{}")"#, err_str)
                .parse()
                .expect("Failed to output compile error");
            attach_span(compile_err, span)
        }
    }
}

fn parse_tokens(token_stream: TokenStream) -> Result<Vec<NumToken>, NumTokenParseError> {
    let stream_iter = token_stream.into_iter();

    let mut num_tokens = match stream_iter.size_hint().1 {
        Some(sz) => Vec::with_capacity(sz),
        None => Vec::new(),
    };

    let mut first = true;
    for token in stream_iter {
        match token {
            TokenTree::Ident(ident) => {
                if let Some(parsed_token) = parse_single_token(&ident)? {
                    // Error if we encounter a sign that is not in the first position
                    if matches!(parsed_token, NumToken::Sign(_)) && !first {
                        return Err(NumTokenParseError::UnexpectedSign(ident));
                    }

                    num_tokens.push(parsed_token);
                    first = false;
                }
            }

            // We just ignore dashes, since they can occur in numbers like twenty-five
            TokenTree::Punct(punct) if punct.as_char() == '-' => {}

            _ => return Err(NumTokenParseError::NonIdentToken(token)),
        }
    }

    Ok(num_tokens)
}

fn parse_single_token(ident: &Ident) -> Result<Option<NumToken>, NumTokenParseError> {
    match ident.to_string().to_lowercase().as_str() {
        "zero" => Ok(Some(NumToken::Literal(0))),
        "one" | "a" => Ok(Some(NumToken::Literal(1))),
        "two" => Ok(Some(NumToken::Literal(2))),
        "three" => Ok(Some(NumToken::Literal(3))),
        "four" => Ok(Some(NumToken::Literal(4))),
        "five" => Ok(Some(NumToken::Literal(5))),
        "six" => Ok(Some(NumToken::Literal(6))),
        "seven" => Ok(Some(NumToken::Literal(7))),
        "eight" => Ok(Some(NumToken::Literal(8))),
        "nine" => Ok(Some(NumToken::Literal(9))),
        "ten" => Ok(Some(NumToken::Literal(10))),
        "eleven" => Ok(Some(NumToken::Literal(11))),
        "twelve" => Ok(Some(NumToken::Literal(12))),
        "thirteen" => Ok(Some(NumToken::Literal(13))),
        "fourteen" => Ok(Some(NumToken::Literal(14))),
        "fifteen" => Ok(Some(NumToken::Literal(15))),
        "sixteen" => Ok(Some(NumToken::Literal(16))),
        "seventeen" => Ok(Some(NumToken::Literal(17))),
        "eighteen" => Ok(Some(NumToken::Literal(18))),
        "nineteen" => Ok(Some(NumToken::Literal(19))),

        "twenty" => Ok(Some(NumToken::Literal(20))),
        "thirty" => Ok(Some(NumToken::Literal(30))),
        "forty" | "fourty" => Ok(Some(NumToken::Literal(40))),
        "fifty" => Ok(Some(NumToken::Literal(50))),
        "sixty" => Ok(Some(NumToken::Literal(60))),
        "seventy" => Ok(Some(NumToken::Literal(70))),
        "eighty" => Ok(Some(NumToken::Literal(80))),
        "ninety" => Ok(Some(NumToken::Literal(90))),

        "hundred" => Ok(Some(NumToken::Multiplier(100))),
        "thousand" => Ok(Some(NumToken::Multiplier(1000))),
        "million" => Ok(Some(NumToken::Multiplier(1000000))),
        "billion" => Ok(Some(NumToken::Multiplier(1000000000))),
        "trillion" => Ok(Some(NumToken::Multiplier(1000000000000))),
        "quadrillion" => Ok(Some(NumToken::Multiplier(1000000000000000))),
        "quintillion" => Ok(Some(NumToken::Multiplier(1000000000000000000))),
        "septillion" => Ok(Some(NumToken::Multiplier(1000000000000000000000))),
        "octillion" => Ok(Some(NumToken::Multiplier(1000000000000000000000000))),

        "plus" | "positive" => Ok(Some(NumToken::Sign(Sign::Positive))),
        "minus" | "negative" => Ok(Some(NumToken::Sign(Sign::Negative))),

        "and" => Ok(None),

        _ => Err(NumTokenParseError::InvalidToken(ident.clone())),
    }
}

macro_rules! return_if_ok {
    ($e:expr) => {
        if let Ok(x) = $e {
            return x;
        }
    };
}

fn make_sized_num_literal(sign: Sign, value: NumType) -> Literal {
    match sign {
        Sign::Unspecified | Sign::Negative => {
            return_if_ok!(value.try_into().map(Literal::i8_suffixed));
            return_if_ok!(value.try_into().map(Literal::i16_suffixed));
            return_if_ok!(value.try_into().map(Literal::i32_suffixed));
            return_if_ok!(value.try_into().map(Literal::i64_suffixed));
            Literal::i128_suffixed(value)
        }
        Sign::Positive => {
            return_if_ok!(value.try_into().map(Literal::u8_suffixed));
            return_if_ok!(value.try_into().map(Literal::u16_suffixed));
            return_if_ok!(value.try_into().map(Literal::u32_suffixed));
            return_if_ok!(value.try_into().map(Literal::u64_suffixed));
            Literal::u128_suffixed(value as u128)
        }
    }
}

fn get_sign(num_tokens: &mut Vec<NumToken>) -> Sign {
    if let Some(NumToken::Sign(sign)) = num_tokens.first().copied() {
        num_tokens.remove(0);
        sign
    } else {
        Sign::Unspecified
    }
}

fn is_larger_multiplier(x: NumToken, than: NumType) -> bool {
    if let NumToken::Multiplier(value) = x {
        value > than
    } else {
        false
    }
}

fn attach_span(token_stream: TokenStream, span: Span) -> TokenStream {
    let mut ret = TokenStream::new();
    ret.extend(token_stream.into_iter().map(|token| {
        let mut new = token;
        new.set_span(span);
        new
    }));
    ret
}

#[derive(Debug, Copy, Clone)]
enum NumToken {
    Literal(NumType),
    Multiplier(NumType),
    Sign(Sign),
}

#[derive(Debug, Copy, Clone)]
enum Sign {
    Unspecified,
    Positive,
    Negative,
}

#[derive(Debug)]
enum NumTokenParseError {
    NonIdentToken(TokenTree),
    InvalidToken(Ident),
    UnexpectedSign(Ident),
}
