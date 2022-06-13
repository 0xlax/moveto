// Move lexer using regular expressions
use std::str::CharIndices;
use std::fmt;
use itertools::{peek_nth, PeekNth};
use phf::phf_map;



// pub enum self::Token::{

    // Comment

    // Module,
    // Script,
    // Struct,
    // Public,
    // Has,
    // Key,
    // Store,
    // Vector<&'input str>
    // Delimiter,


    // Byte,
    // Bool,
    // String,

    // // operators
    // As,

  

    // Fun,
    // Address,
    // Abort,

    // Let,

    // // integer types
    // U8,
    // U64,
    // U128,

    // True,
    // False,
    // Else,
    // For,
    // While,
    // If,

    // Less,
    // More,
    // Div,
    // Sum,
    // Sub,
    // Mul,
    // Mod,
    // Lshift,
    // Rshift,
    // And,
    // Or,
    // Xor,
    // Equals,


    // Unused,


    // Colon,
    // OpenParan,
    // CloseParan,


// }

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token {

    Comment

    Module,
    Script,
    Struct,
    Public,
    Has,
    Key,
    Store,
    Vector<&'input str>
    Delimiter,


    Byte,
    Bool,
    String,

    // operators
    As,

  

    Fun,
    Address,
    Abort,

    Let,

    // integer types
    U8,
    U64,
    U128,

    True,
    False,
    Else,
    For,
    While,
    If,

    Less,
    More,
    Div,
    Sum,
    Sub,
    Mul,
    Mod,
    Lshift,
    Rshift,
    And,
    Or,
    Xor,
    Equals,


    Unused,


    Colon,
    OpenParan,
    CloseParan,

}




// impl<'input>  fmt::Display for Token<'input> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_'> -> fmt::Result {
//         match self {
            

//             // Types Tokenized

//         }
//     })
// }



pub fn tokenizer(input: str) -> Vec<Token> {

    // remove comments- starting with // in each line
    let comments = regex!(r"(?m)//.*\n")
    // replace comments with null string
    let processed = comments.replace_all(input, "/n")

    let mut tokenised = Vec::new();

    let token_re = regex!(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)"));

        for cap in token_re.captures_iter(preprocessed.as_str()) {
            let token = if cap.name("ident").is_some() {
                match cap.name("ident").unwrap() {
                    
                    Comment

                    "module" => Module,
                    "script" => Script,
                    "struct" => Struct,
                    "public" => Public,
                    "has"    => Has
                    "key"    => Key,
                    "store"  => Store
                
                
                    "bool"   => Bool                
                    "as"     => As,

                    "fun"    => Fun
                    "address"=> Address
                    "abort"  => Abort,
                
                    "let" => Let,

                    ident => Ident(ident.to_string())
                
         
                }
            }
        }

}






pub struct StrReader<'input> {
    input: &'input str,
    chars: PeekNth<CharIndices<'input>>,
    comments: &'input mut Vec<Comment>,
}

