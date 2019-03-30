use std::io::{self, Read};

#[derive(Debug)]
enum State {
    Start,
    WaitingNewline,
    Code
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut code = String::new();

    let mut detection = 0;
    let mut state = State::Start;

    for chr in buffer.chars() {
        match state {
            State::Start => if detection == 3 {
                state = State::WaitingNewline;
                detection = 0;
            } else if chr == '`' {
                detection += 1;
            } else {
                detection = 0;
            },
            State::WaitingNewline => if chr == '\n' {
                state = State::Code;
            },
            State::Code => if detection == 3 {
                state = State::Start;
                detection = 0;
                code.pop();
                code.pop();
                code.pop();
                print!("{}", code);
                code = String::new();
            } else {
                code.push(chr);

                if chr == '`' {
                    detection += 1;
                } else {
                    detection = 0;
                }
            }
        }
    }
    
    if detection == 3 {
        code.pop();
        code.pop();
        code.pop();
        print!("{}", code);
    }

    Ok(())
}
