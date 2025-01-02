#![allow(non_snake_case, dead_code, non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {

    IDENTIFINE,                         // identifine an object or function example: int var = 2 or def main[arr<str> args]
    NUMBER,                             // its just numbers
    STRING,                             // its too just string

    //MATH OPERATION
    PLUS,                               // math operation +
    MINUS,                              // math operation -
    MULTIPLY,                           // math operation *
    DIVIDE,                             // math operation /
    EQUALS,                             // math operation =

    //FIGURE BRACKETS
    START_OF_CODE_SPACE,                // start function, object and just code space {
    END_OF_CODE_SPACE,                  // end function, object and just code space }

    CMD_AND_ARGS_DEVIDER,               // 
    ARGS_DEVIDER
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

impl Token {            //Create Token
    pub fn new(token_type: TokenType, value: String) -> Token { Self { token_type, value: value } }
}