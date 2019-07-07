use nom::AsChar;
use nom::InputTakeAtPosition;
use nom::{error::ErrorKind, Err, IResult, Needed};

/// <SPACE>    ::= ' ' { ' ' }
/// One space and then as much space as you want
/// return Ok( (nextBytes, eatenSpaces) )
/// Or if there is no space at the beginning Err( (input, TakeWhile1) )
pub fn space(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::take_while1(|item| item == b' ')(input)
}

/// <crlf>     ::= CR LF
/// Recognizes the string "\r\n".
/// Return an error if there is not enough data: Err( (input, Crlf) )
pub fn crlf(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::character::complete::crlf(input)
}

pub fn one_char(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if input.is_empty() {
        return Err(Err::Error((input, ErrorKind::Char)));
    }
    Ok((&input[1..], &input[0..1]))
}

/// <letter>     ::= 'a' ... 'z' | 'A' ... 'Z'
/// Extract the first char of the input
/// Return an error if there is not enough data or if it’s not a letter:
/// Err( (input, Char) )
pub fn letter(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if input.is_empty() || !input[0].is_alpha() {
        return Err(Err::Error((input, ErrorKind::Char)));
    }
    Ok((&input[1..], &input[0..1]))
}

/// <number>     ::= '0' ... '9'
/// Extract the first char of the input
/// Return an error if there is not enough data or if it’s not a number:
/// Err( (input, Char) )
pub fn number(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if input.is_empty() || !input[0].is_dec_digit() {
        return Err(Err::Error((input, ErrorKind::Char)));
    }
    Ok((&input[1..], &input[0..1]))
}

/// <special>    ::= '-' | '[' | ']' | '\' | '`' | '^' | '{' | '}'
/// Extract the first char of the input
/// Return an error if there is not enough data or if it’s not a special char:
/// Err( (input, Char) )
pub fn special(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if input.is_empty()
        || !(input[0] == b'-'
            || input[0] == b'['
            || input[0] == b']'
            || input[0] == b'\\'
            || input[0] == b'`'
            || input[0] == b'^'
            || input[0] == b'{'
            || input[0] == b'}')
    {
        return Err(Err::Error((input, ErrorKind::Char)));
    }
    Ok((&input[1..], &input[0..1]))
}

///  <nonwhite>   ::= <any 8bit code except SPACE (0x20), NUL (0x0), CR
///                    (0xd), and LF (0xa)>
/// Extract the first char of the input
/// Return an error if there is not enough data or if it’s not a nonwhite char:
/// Err( (input, Char) )
pub fn nonwhite(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if input.is_empty()
        || input[0] == b' '
        || input[0] == 0x00 // NUL
        || input[0] == b'\r'
        || input[0] == b'\n'
    {
        return Err(Err::Error((input, ErrorKind::Char)));
    }
    Ok((&input[1..], &input[0..1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_empty() {
        let empty: &[u8] = b"";
        assert_eq!(
            space(empty),
            Err(Err::Error((empty, ErrorKind::TakeWhile1)))
        );
    }

    #[test]
    fn space_characters() {
        let a: &[u8] = b"abcd";
        assert_eq!(space(a), Err(Err::Error((a, ErrorKind::TakeWhile1))));
    }

    #[test]
    fn space_spaces() {
        let s: &[u8] = b"    ";
        assert_eq!(space(s), Ok((&b""[..], s)));
    }

    #[test]
    fn space_spaces_and_chars() {
        let s: &[u8] = b"    abcd";
        assert_eq!(space(s), Ok((&b"abcd"[..], &b"    "[..])));
    }

    #[test]
    fn crlf_empty() {
        let empty: &[u8] = b"";
        assert_eq!(crlf(empty), Err(Err::Error((empty, ErrorKind::CrLf))));
    }

    #[test]
    fn crlf_alone() {
        let c: &[u8] = b"\r\n";
        assert_eq!(crlf(c), Ok((&b""[..], c)));
    }

    #[test]
    fn crlf_with_chars() {
        let c: &[u8] = b"\r\nabcd";
        assert_eq!(crlf(c), Ok((&b"abcd"[..], &b"\r\n"[..])));
    }

    #[test]
    fn one_char_empty() {
        let empty: &[u8] = b"";
        assert_eq!(one_char(empty), Err(Err::Error((empty, ErrorKind::Char))));
    }

    #[test]
    fn one_char_alone() {
        let a: &[u8] = b"a";
        assert_eq!(one_char(a), Ok((&b""[..], a)));
    }

    #[test]
    fn one_char_with_chars() {
        let a: &[u8] = b"ab1-";
        assert_eq!(one_char(a), Ok((&b"b1-"[..], &b"a"[..])));
    }

    #[test]
    fn letter_empty() {
        let empty: &[u8] = b"";
        assert_eq!(letter(empty), Err(Err::Error((empty, ErrorKind::Char))));
    }

    #[test]
    fn letter_alone() {
        let a: &[u8] = b"a";
        assert_eq!(letter(a), Ok((&b""[..], a)));
    }

    #[test]
    fn letter_with_num() {
        let a: &[u8] = b"ab1-";
        assert_eq!(letter(a), Ok((&b"b1-"[..], &b"a"[..])));
        let a: &[u8] = b"1";
        assert_eq!(letter(a), Err(Err::Error((a, ErrorKind::Char))));
    }

    #[test]
    fn number_empty() {
        let empty: &[u8] = b"";
        assert_eq!(number(empty), Err(Err::Error((empty, ErrorKind::Char))));
    }

    #[test]
    fn number_alone() {
        let a: &[u8] = b"1";
        assert_eq!(number(a), Ok((&b""[..], a)));
    }

    #[test]
    fn number_with_char() {
        let a: &[u8] = b"12a-";
        assert_eq!(number(a), Ok((&b"2a-"[..], &b"1"[..])));
        let a: &[u8] = b"a";
        assert_eq!(number(a), Err(Err::Error((a, ErrorKind::Char))));
    }

    #[test]
    fn special_empty() {
        let empty: &[u8] = b"";
        assert_eq!(special(empty), Err(Err::Error((empty, ErrorKind::Char))));
    }

    #[test]
    fn special_alone() {
        let a: &[u8] = b"-";
        assert_eq!(special(a), Ok((&b""[..], a)));
    }

    #[test]
    fn special_with_char() {
        let a: &[u8] = b"[2a-";
        assert_eq!(special(a), Ok((&b"2a-"[..], &b"["[..])));
        let a: &[u8] = b"a";
        assert_eq!(special(a), Err(Err::Error((a, ErrorKind::Char))));
    }

    #[test]
    fn nonwhite_empty() {
        let empty: &[u8] = b"";
        assert_eq!(nonwhite(empty), Err(Err::Error((empty, ErrorKind::Char))));
    }

    #[test]
    fn nonwhite_alone() {
        let a: &[u8] = b"a";
        assert_eq!(nonwhite(a), Ok((&b""[..], a)));
    }

    #[test]
    fn nonwhite_with_char() {
        let a: &[u8] = b"\t2a-";
        assert_eq!(nonwhite(a), Ok((&b"2a-"[..], &b"\t"[..])));
        let a: &[u8] = b" ";
        assert_eq!(nonwhite(a), Err(Err::Error((a, ErrorKind::Char))));
    }
}
