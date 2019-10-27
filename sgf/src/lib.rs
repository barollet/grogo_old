use std::collections::HashMap;
use std::iter::FromIterator;

#[macro_use]
extern crate nom;

use nom::bytes::complete::{escaped_transform, is_not, tag, take_while1};
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, tuple};
use nom::IResult;

/// Parses a SGF value between [], supports escaping
//TODO escaping
fn parse_value(input: &str) -> IResult<&str, String> {
    delimited(
        char('['),
        escaped_transform(is_not("\\]"), '\\', char(']')),
        char(']'),
    )(input)
}

/// Parses a SGF property identifier
fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(take_while1(|c: char| c.is_ascii_uppercase()), |s: &str| {
        s.to_owned()
    })(input)
}

/*
/// Matches all whitespaces
fn separators(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_ascii_whitespace())(input)
}
*/

/// Parses a SGF property
fn parse_property(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, property_name) = parse_identifier(input)?;
    let (input, values) = many1(parse_value)(input)?;

    Ok((input, (property_name, values)))
}

/// Parses a SGF node
fn parse_node(input: &str) -> IResult<&str, SGFNode> {
    let (input, _) = tag(";")(input)?;
    let (input, props) = many0(parse_property)(input)?;

    Ok((
        input,
        SGFNode {
            properties: HashMap::from_iter(props),
        },
    ))
}

/// Parses a SGF node sequence
fn parse_sequence(input: &str) -> IResult<&str, Vec<SGFNode>> {
    many1(parse_node)(input)
}

/// Parses a SGF game tree recursively
fn parse_game_tree(input: &str) -> IResult<&str, SGFTree> {
    let (input, (nodes, children)) = delimited(
        char('('),
        tuple((parse_sequence, many0(parse_game_tree))),
        char(')'),
    )(input)?;

    Ok((input, SGFTree { nodes, children }))
}

/// This is a map of properties in a node of the game tree
pub struct SGFNode {
    properties: HashMap<String, Vec<String>>,
}

/// This is a node of the game tree, it is composed of a list of nodes (properties) and a list of
/// children.
/// This structure is used to navigate through the tree.
pub struct SGFTree {
    nodes: Vec<SGFNode>,
    children: Vec<SGFTree>,
}

/// This is the main function of the library, it takes a SGF string as input and returns a list of
/// game trees (there is usually only one).
pub fn parse_sgf_string(input: &str) -> Result<Vec<SGFTree>, ()> {
    match many1(parse_game_tree)(input) {
        Ok((input, game_trees)) => {
            if !input.is_empty() {
                Ok(game_trees)
            } else {
                Err(())
            }
        }
        Err(_) => Err(()),
    }
}

// TODO generate SGF

// TODO proper testing
#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn parse_value_test() {
        assert_eq!(parse_value("[abc]123").unwrap(), ("123", "abc".to_owned()));
        assert_eq!(
            parse_value("[ab\\]c]123").unwrap(),
            ("123", "ab]c".to_owned())
        );
        assert!(parse_value("abc\\]").is_err());
    }

    #[test]
    fn parse_identifier_test() {
        assert_eq!(
            parse_identifier("ABC[123]").unwrap(),
            ("[123]", "ABC".to_owned())
        );
        assert!(parse_identifier("ab").is_err());
    }

    #[test]
    fn parse_property_test() {
        assert_eq!(
            parse_property("AB[12][34]").unwrap(),
            (
                "",
                ("AB".to_owned(), vec!["12".to_owned(), "34".to_owned()])
            )
        );
        // TODO whitespace separator
        /*
        assert_eq!(
            parse_property("AB[12] [34]").unwrap(),
            (
                "",
                ("AB".to_owned(), vec!["12".to_owned(), "34".to_owned()])
            )
        );
        */
    }
}
