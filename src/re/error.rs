// define a bunch of possible errors
// that we know can be generated

// parsing codes
pub mod ParseError {
  use std::fmt;

  pub enum ParseCode {
    ParseOk,

    ParseEmptyAlternate,
    ParseEmptyConcatenate,
    ParseRepeatedRepetition,
    ParseEmptyRepetition,
    ParseEmptyRepetitionRange,
    ParseEmptyGroupName,
    ParseEmptyPropertyName,

    // used internally
    ParseNotRepetition,

    ParseExpectedClosingParen,
    ParseExpectedClosingBracket,
    ParseExpectedClosingBrace,
    ParseExpectedOpeningAngleBracket,
    ParseExpectedClosingAngleBracket,
    ParseExpectedComma,
    ParseExpectedAlpha,
    ParseExpectedNumeric,
    ParseExpectedAlphaNumeric,
    ParseExpectedOperand,
    ParseExpectedAsciiCharClassClose,
    ParseUnexpectedClosingParen,
    ParseUnexpectedOperand,
    ParseUnexpectedCharacter,

    ParseInvalidUnicodeProperty,
    ParseInvalidAsciiCharClass,

    ParseIncompleteEscapeSeq,

    // char class errors
    ParseEmptyCharClassRange,

    // library errors
    ParseInternalError,
    ParseEmptyStack,
    ParseUnknownError
  }

  impl fmt::Show for ParseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
        ParseOk                          => write!(f.buf, "Ok"),
        ParseEmptyAlternate              => write!(f.buf, "Parse Error: Nothing to alternate"),
        ParseEmptyConcatenate            => write!(f.buf, "Parse Error: Nothing to concatenate"),
        ParseRepeatedRepetition          => write!(f.buf, "Parse Error: Multiple repeat operations"),
        ParseEmptyRepetition             => write!(f.buf, "Parse Error: Nothing to repeat"),
        ParseEmptyRepetitionRange        => write!(f.buf, "Parse Error: Repeat range is empty"),
        ParseEmptyGroupName              => write!(f.buf, "Parse Error: Group name is empty"),
        ParseEmptyPropertyName           => write!(f.buf, "Parse Error: Property character class name is empty"),
        ParseExpectedClosingParen        => write!(f.buf, "Parse Error: Expected ')'"),
        ParseExpectedClosingBracket      => write!(f.buf, "Parse Error: Expected ']'"),
        ParseExpectedClosingBrace        => write!(f.buf, "Parse Error: Expected '{:s}'", "}"),
        ParseExpectedOpeningAngleBracket => write!(f.buf, "Parse Error: Expected '<'"),
        ParseExpectedClosingAngleBracket => write!(f.buf, "Parse Error: Expected '>'"),
        ParseExpectedComma               => write!(f.buf, "Parse Error: Expected ','"),
        ParseExpectedAlpha               => write!(f.buf, "Parse Error: Expected alpha character"),
        ParseExpectedNumeric             => write!(f.buf, "Parse Error: Expected number"),
        ParseExpectedAlphaNumeric        => write!(f.buf, "Parse Error: Expected alphanumeric character (or underscore)"),
        ParseExpectedOperand             => write!(f.buf, "Parse Error: Expected an operand on the stack"),
        ParseExpectedAsciiCharClassClose => write!(f.buf, "Parse Error: Expected \":]\""),
        ParseUnexpectedClosingParen      => write!(f.buf, "Parse Error: Unexpected closing parenthases in input"),
        ParseUnexpectedOperand           => write!(f.buf, "Parse Error: Unexpected operand was on the stack"),
        ParseUnexpectedCharacter         => write!(f.buf, "Parse Error: Unexpected character in input"),
        ParseInvalidUnicodeProperty      => write!(f.buf, "Parse Error: Invalid Unicode property provided"),
        ParseInvalidAsciiCharClass       => write!(f.buf, "Parse Error: Invalid ASCII character class name provided"),
        ParseIncompleteEscapeSeq         => write!(f.buf, "Parse Error: Expected a character to escape"),
        ParseEmptyCharClassRange         => write!(f.buf, "Parse Error: Empty character class"),
        ParseInternalError |
        ParseNotRepetition |
        ParseUnknownError           => write!(f.buf, "Parse Error: Unknown error (probably a bug)"),
        ParseEmptyStack             => write!(f.buf, "Parse Error: Nothing on the stack")
      }
    }
  }

}
