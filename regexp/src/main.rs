enum Op{
    Char(u8),
    FromTo(u8,u8),
    Repeat(i32),
    Append,
    Or,
}

struct Node{
    v:Op,
    c0:Option<Box<Node>>,
    c1:Option<Box<Node>>
}

fn parse(input:&str){
    let mut start = 0;
    let len = input.len();
    let bytes = input.as_bytes();
    while start < len{
        let c = bytes[start];
        if (c >= b'a' && c<=b'z') || (c>=b'A' && c<=b'Z') || (c>=b'0' && c<=b'9'){
            let node = Node{v:Op::Char(c),c0:Option::None,c1:Option::None};
        }
    }
}


fn main() {
    parse("a-zA-Z");
    println!("Hello, world!");
}
