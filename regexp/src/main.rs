enum Op{
    Or,
    FromTo(char,char),
    Link,
}

struct Node{
    v:Op,
    c0:Box<Node>,
    c1:Box<Node>
}

fn parse(r:&str){
    
}


fn main() {
    let s0 = "int a = 3;";
    let s1 = "if( a >= 0){}";
    println!("Hello, world!");
}
