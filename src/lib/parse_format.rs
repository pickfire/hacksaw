extern crate nom;

use self::nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{complete, map},
    multi::many0,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub enum FormatToken {
    WindowId,
    Geometry,
    Width,
    Height,
    X,
    Y,
    Literal(String),
}

// This newtype is needed to sidestep StructOpt's Vec behaviour
#[derive(Debug)]
pub struct Format(pub Vec<FormatToken>);

pub fn parse_format_string(input: &str) -> Result<Format, String> {
    match parse_all(input) {
        Ok(("", v)) => Ok(Format { 0: v }),
        Err(s) => Err(format!("Format string parse error: {:?}", s)),
        Ok((s, _)) => Err(format!("Format string parse error near \"{}\"", s)),
    }
}

fn parse_format(input: &str) -> IResult<&str, FormatToken> {
    preceded(
        tag("%"),
        alt((
            map(tag("i"), |_| FormatToken::WindowId),
            map(tag("g"), |_| FormatToken::Geometry),
            map(tag("w"), |_| FormatToken::Width),
            map(tag("h"), |_| FormatToken::Height),
            map(tag("x"), |_| FormatToken::X),
            map(tag("y"), |_| FormatToken::Y),
            map(tag("%"), |_| FormatToken::Literal("%".to_owned())),
        )),
    )(input)
}

fn parse_literal(input: &str) -> IResult<&str, FormatToken> {
    // Parse a literal by taking the entire string until a % sign and wrapping
    // it in a FormatToken::Literal.
    map(is_not("%"), |s: &str| FormatToken::Literal(s.to_owned()))(input)
}

fn parse_anything(input: &str) -> IResult<&str, FormatToken> {
    // Parse a single token - either a %-token or a literal.
    alt((parse_format, parse_literal))(input)
}

fn parse_all(input: &str) -> IResult<&str, Vec<FormatToken>> {
    // Parse as many individual tokens as we can, using the entire string.
    complete(many0(parse_anything))(input)
}