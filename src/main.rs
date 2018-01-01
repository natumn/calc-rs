use 

enum Kind {
    Print,
    Lparen,
    Rparen,
    Plus,
    Minus,
    Multi,
    Divi,
    Assgin,
    VarName,
    IntNum,
    EofTkn,
    Others,
}

struct Token {
    kind: Kind,
    val: i32,
}

fn main() {
    loop {
        input();
        token = token();
        if (token.kind == EofTkn) {
            ::std::process::exit(1);
        }
        statement();
    }
}
