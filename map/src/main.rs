enum TokenType{
    Keyword,
    Id,
    Num,
    String,
    Split,
    Op
}

struct Token<'a>{
    t:TokenType,
    s:&'a str,
}

fn printx(s:&str){
    println!("{}",s);
}

fn add(t:TokenType,s:&str){
    println!("{0}:{1}",t as i32,s);
}

fn main(){
    let input = "{range  (1,10)}[3]+'a'";
    let mut start = 0;
    let len = input.len();
    let bytes = input.as_bytes();
    while start < len{
        let s = bytes[start];
        if s == b' ' {
            start += 1;
        }
        else if s == b'{' || s == b'}' || s == b'(' || s == b')' || s == b'[' || s == b']' || s == b','{
            add(TokenType::Split, &input[start..start+1]);
            start += 1;
        }
        else if (s >= b'a' && s<=b'z') || (s >= b'A' && s <= b'Z') || s == b'_'{
            let mut n = 1;
            while start + n < len {
                let e = bytes[start + n];
                if(e >= b'0' && e <= b'9') || (e >= b'a' && e<=b'z') || (e >= b'A' && e <= b'Z') || e == b'_'{
                    n += 1;
                }else{
                    break;
                }
            }
            add(TokenType::Id, &input[start..start + n]);
            start += n;
        }
        else if s >= b'0' && s <= b'9'{
            let mut n = 1;
            while start + n < len{
                let e = bytes[start + n];
                if e >= b'0' && e <= b'9' {
                    n += 1;
                }
                else{
                    break;
                }
            }
            add(TokenType::Num, &input[start..start + n]);
            start += n;
        }
        else if s==b'"'{
            let mut n = 1;
            while start + n < len{
                let e = bytes[start + n];
                if e != b'"' {
                    n += 1;
                }
                else{
                    n += 1;
                    break;
                }
            }
            add(TokenType::String, &input[start + 1..start + n - 1]);
            start += n;
        }
        else if s==b'\''{
            let mut n = 1;
            while start + n < len{
                let e = bytes[start + n];
                if e != b'\'' {
                    n += 1;
                }
                else{
                    n += 1;
                    break;
                }
            }
            add(TokenType::String, &input[start + 1..start + n - 1]);
            start += n;
        }
        else{
            start += 1;
        }
    }
}