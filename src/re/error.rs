// define a bunch of possible errors
// that we know can be generated

// parsing codes
pub mod ParseError {
  static PARSE_ERR: &'static str = "Parse Error: ";

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
    ParseUnexpectedClosingParen,
    ParseUnexpectedOperand,
    ParseUnexpectedCharacter,

    ParseInvalidUnicodeProperty,

    ParseIncompleteEscapeSeq,
    ParseInvalidUTF8Encoding,

    // char class errors
    ParseEmptyCharClassRange,

    // library errors
    ParseInternalError,
    ParseEmptyStack,
    ParseUnknownError
  }

  impl ToStr for ParseCode {
    fn to_str(&self) -> ~str {
      match *self {
        ParseOk                          => ~"Ok",
        ParseEmptyAlternate              => PARSE_ERR + "Nothing to alternate",
        ParseEmptyConcatenate            => PARSE_ERR + "Nothing to concatenate",
        ParseRepeatedRepetition          => PARSE_ERR + "Multiple repeat operations",
        ParseEmptyRepetition             => PARSE_ERR + "Nothing to repeat",
        ParseEmptyRepetitionRange        => PARSE_ERR + "Repeat range is empty",
        ParseEmptyGroupName              => PARSE_ERR + "Group name is empty",
        ParseEmptyPropertyName           => PARSE_ERR + "Property character class name is empty",
        ParseExpectedClosingParen        => PARSE_ERR + "Expected ')'",
        ParseExpectedClosingBracket      => PARSE_ERR + "Expected ']'",
        ParseExpectedClosingBrace        => PARSE_ERR + "Expected '}'",
        ParseExpectedOpeningAngleBracket => PARSE_ERR + "Expected '<'",
        ParseExpectedClosingAngleBracket => PARSE_ERR + "Expected '>'",
        ParseExpectedComma               => PARSE_ERR + "Expected ','",
        ParseExpectedAlpha               => PARSE_ERR + "Expected alpha character",
        ParseExpectedNumeric             => PARSE_ERR + "Expected number",
        ParseExpectedAlphaNumeric        => PARSE_ERR + "Expected alphanumeric character (or underscore)",
        ParseExpectedOperand             => PARSE_ERR + "Expected an operand on the stack",
        ParseUnexpectedClosingParen      => PARSE_ERR + "Unexpected closing parenthases in input",
        ParseUnexpectedOperand           => PARSE_ERR + "Unexpected operand was on the stack",
        ParseUnexpectedCharacter         => PARSE_ERR + "Unexpected character in input",
        ParseInvalidUnicodeProperty      => PARSE_ERR + "Invalid Unicode property provided",
        ParseIncompleteEscapeSeq         => PARSE_ERR + "Expected a character to escape",
        ParseEmptyCharClassRange         => PARSE_ERR + "Empty character class",
        ParseInvalidUTF8Encoding         => PARSE_ERR + "Invalid UTF-8 encoding",
        ParseInternalError |
        ParseNotRepetition |
        ParseUnknownError           => PARSE_ERR + "Unknown error (probably a bug)",
        ParseEmptyStack             => PARSE_ERR + "Nothing on the stack"
      }
    }
  }
}
