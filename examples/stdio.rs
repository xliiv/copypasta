extern crate copypasta;

use std::env;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ctx = ClipboardContext::new().unwrap();
    let the_string = &args[1];
    ctx.set_contents(the_string.to_owned()).unwrap();

    loop {
        dbg!("loop");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
