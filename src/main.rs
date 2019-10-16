use ferris_says::say;
use std::io::{stdout, Bufwriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello World Rust!";
    let width = 24;
    let mut writer = Bufwriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();    
}
