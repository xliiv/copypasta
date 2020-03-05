extern crate copypasta;

use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

// Action to reproduce
// 1. run the code `cargo run --example echo`
// 2. select text in some window e.g. Firefox
// 3. press ctrl+c
// 4. focus some edit widget, e.g. Firefox's address bar
// 5. press ctrl+v
//
// I expect the copied text will show up in the widget (see step 4)
// but the text doesn't show up.

fn fails() {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut previous = String::new();
    loop {
        let current = ctx.get_contents().unwrap();
        dbg!(&current);

        if current != previous {
            ctx.set_contents(current.clone()).unwrap();
            previous = current;
            dbg!(&previous);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn works() {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut previous = String::new();
    loop {
        let current = ctx.get_contents().unwrap();
        dbg!(&current);

        if true {
            ctx.set_contents(current.clone()).unwrap();
            previous = current;
            dbg!(&previous);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn main() {
    fails();
    //works();
}
