## Importing
```rust
use std::io::{self, Read};
```

## `State` definition
```rust
#[derive(Debug)]
enum State {
    Start,
    WaitingNewline,
    Code
}
```
Has 3 states:
* `Start`
* `WaitingNewline`
* `Code`

## Main function
```rust
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
```

## Code accumulator
```rust
    let mut code = String::new();
```
Stores the current code block until code block end.

## Detection for state transition
```rust
    let mut detection = 0;
```
When `detection = 3` advance to the next state.

## Program state
```rust
    let mut state = State::Start;
```
Initial state set to `Start`.

## State machine
```rust
    for chr in buffer.chars() {
        match state {
```

### `Start` state
```rust
            State::Start => if detection == 3 {
                state = State::WaitingNewline;
                detection = 0;
            } else if chr == '`' {
                detection += 1;
            } else {
                detection = 0;
            }
```

### `WaitingNewline` state
```rust
            State::WaitingNewline => if chr == '\n' {
                state = State::Code;
            }
```

### `Code` state
```rust
            State::Code => if detection == 3 {
                state = State::Start;
                detection = 0;
```

```rust
                code.pop();
                code.pop();
                code.pop();
```
Triple `code.pop();` is for removing the last three instances of the ` character.
```rust
                print!("{}", code);
                code = String::new();
            } else {
```

```rust
                code.push(chr);
```
Push every `chr` to `code` while in the `Code` state.

```rust
                if chr == '`' {
                    detection += 1;
                } else {
                    detection = 0;
                }
            }
```

```rust
        }
    }
```

## Return
```rust
    Ok(())
}
```