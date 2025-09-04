use std::io::{stdin, stdout, Write};

pub fn input(prompt: &str, default: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);

    let _ = stdout().flush();
    let r = stdin().read_line(&mut s).unwrap_or(0);

    if r == 0 {
        default.to_string()
    } else {
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        s
    }
}
