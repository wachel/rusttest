use std::fmt;

//struct Node{
//    token:Token,
//    c0:Option<Box<Node>>,
//    c1:Option<Box<Node>>
//}

#[derive(PartialEq)]
enum Node{
    Char(u8),
    FromTo(u8,u8),
    Repeat(Box<Node>),
    Or(Box<Node>,Box<Node>),
    Link(Box<Node>,Box<Node>),
    None,
    Error,
}

#[derive(Debug,PartialEq)]
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
fn parse_factor(tokens:&[Token])->(Node,usize){
    match tokens[0] {
        Token::Char(c) => {
            let node:Node = Node::Char(c);
            return (node,1);
        },
        Token::Split(c) => {
            if c == b'('{
                let (node,length) = parse_expression(&tokens[1..]);
                if node != Node::None && length > 0{
                    return (node,length);
                }
            }
        },
        _=>{}
    }
    (Node::Error,0)
}

fn parse_term(tokens:&[Token])->(Node,usize){
    let mut index:usize = 0;
    let (node0,length0) = parse_factor(tokens);
    if node0 != Node::None && length0 > 0{
        index += length0;
        if index <= tokens.len(){
            if tokens[index] == Token::Repeat {
                let node:Node = Node::Repeat(Box::new(node0));
                return (node,index);
            }
            else{
                let (node1,length1) = parse_term(&tokens[index..]);
                if node1 != Node::None && length1 > 0 {
                    index += length1;
                    let node:Node = Node::Link(Box::new(node0),Box::new(node1));
                    return (node,index);
                }
            }
        }
        else{
            return (node0,index);
        }
    }

    (Node::Error,0)
}

fn parse_expression(tokens:&[Token])->(Node,usize){
    let mut index:usize = 0;
    let (node0,length0) = parse_term(tokens);
    if node0 != Node::None && length0 > 0{
        index += length0;
        if index <= tokens.len(){
            if tokens[index] == Token::Or {
                let (node1,length1) = parse_expression(&tokens[index..]);
                if node1 != Node::None && length1 > 0 {
                    index += length1;
                    let node:Node = Node::Or(Box::new(node0),Box::new(node1));
                    return (node,index);
                }
            }
        }
        else{
            return (node0,index);
        }
    }

    (Node::Error,0)
}

// fn to_ast(tokens:&Vec<Token>)->Node{
//     let mut index = 0;
//     match tokens[index] {
//         Token::Char(c)=>Node::Char(c),
//         Token::FromTo(c0,c1)=>Node::FromTo(c0,c1),
//         Token::Repeat=>Node::Repeat(Box::new(node)),
//         Token::Or=>Node::Or(Box::new(node),Box::new(to_ast(&tokens[1..],Node::None))),
//         Token::Split(c)=>{
//             if c == b'['{
//                 stack.push(tokens[index]);
//                 to_ast(&tokens[1..],Node::None)
//             }
//         }
//     }
//     Node::None
// }

fn main() {
    let tokens:Vec<Token> = parse_tokens("a");
    let (node,length) = parse_expression(&tokens[0..]);
    for t in tokens{
        println!("{}",t);
    }
    println!("Hello, world!");
}
