use std::fmt;

//struct Node{
//    token:Token,
//    c0:Option<Box<Node>>,
//    c1:Option<Box<Node>>
//}

enum Node{
    Char(u8),
    FromTo(u8,u8),
    Repeat(Box<Node>),
    Or(Box<Node>,Box<Node>),
    Link(Box<Node>,Box<Node>),
    None,
}

#[derive(Debug)]
enum Token{
    Char(u8),
    FromTo(u8,u8),//-
    Repeat,//*
    Or,//|
    Split(u8),//[]()
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Token::Char(c) => write!(f, "Char:{}",c as char),
           Token::FromTo(c0,c1) => write!(f, "FromTo:{}-{}",c0 as char,c1 as char),
           Token::Repeat => write!(f, "Repeat"),
           Token::Or => write!(f, "Or"),
           Token::Split(c) => write!(f, "Split:{}",c as char),
       }
    }
}

fn parse_tokens(input:&str)->Vec<Token>{
    let mut start = 0;
    let len = input.len();
    let bytes = input.as_bytes();
    let mut result:Vec<Token> = Vec::new();
    
    while start < len{
        let c = bytes[start];
        if (c >= b'a' && c<=b'z') || (c>=b'A' && c<=b'Z') || (c>=b'0' && c<=b'9'){
            let c1 = if start + 1 < len { bytes[start + 1]} else{0};
            let c2 = if start + 2 < len { bytes[start + 2]} else{0};
            if c1 == b'-' && (c2 >= b'a' && c2<=b'z') || (c2>=b'A' && c2<=b'Z') || (c2>=b'0' && c2<=b'9'){
                result.push(Token::FromTo(c,c2));
                start += 3;
            }
            else{
                result.push(Token::Char(c));
                start += 1;
            }
        }
        else if c == b'|'{
            result.push(Token::Or);
            start += 1;
        }
        else if c == b'*'{
            result.push(Token::Repeat);
            start += 1;
        }
        else if c == b'[' || c == b']' || c == b'(' || c == b')'{
            result.push(Token::Split(c));
            start += 1;
        }
        else{
            start += 1;
        }
    }
    return result;
}

/*
fn to_ast(tokens:&[Token], node:Node)->Node{
    match tokens[0] {
        Token::Char(c)=>Node::Char(c),
        Token::FromTo(c0,c1)=>Node::FromTo(c0,c1),
        Token::Repeat=>Node::Repeat(Box::new(node)),
        Token::Or=>Node::Or(Box::new(node),Box::new(to_ast(&tokens[1..],Node::None))),
        Token::Split(c)=>{
            if c == b'['{
                
                to_ast(&tokens[1..],Node::None)
            }
        }
    }
}
*/

fn to_ast(stack:&Vec<Token>,tokens:&Vec<Token>)->Node{
    let mut index = 0;
    match tokens[index] {
        Token::Char(c)=>Node::Char(c),
        Token::FromTo(c0,c1)=>Node::FromTo(c0,c1),
        Token::Repeat=>Node::Repeat(Box::new(node)),
        Token::Or=>Node::Or(Box::new(node),Box::new(to_ast(&tokens[1..],Node::None))),
        Token::Split(c)=>{
            if c == b'['{
                
                to_ast(&tokens[1..],Node::None)
            }
        }
    }
    Node::None
}

fn main() {
    let tokens:Vec<Token> = parse_tokens("[a-z]*");
    let mut tempStack:Vec<Token> = Vec::new(); 
    
    let node:Node = to_ast(tokens.as_slice(),Node::None);
    for t in tokens{
        println!("{}",t);
    }
    println!("Hello, world!");
}
