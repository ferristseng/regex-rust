// define a bunch of possible errors 
// that we know can be generated

// parsing codes 
pub mod ParseError {
  pub enum ParseCode {
    ParseOk,
    
    ParseEmptyAlternate,
    ParseEmptyConcatenate,
    ParseRepeatedRepetition,
    ParseEmptyRepetition,

    ParseExpectedClosingParen,
    ParseExpectedClosingBracket,
    ParseExpectedClosingBrace,
    ParseExpectedOperand,
    ParseUnexpectedOperand,

    // char class errors
    ParseEmptyCharClassRange,

    // library errors
    ParseInternalError,
    ParseEmptyStack,
    ParseUnknownError
  }
}
