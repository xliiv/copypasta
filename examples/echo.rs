extern crate copypasta;

use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut previous = "".to_string();
    loop {
        let current = ctx.get_contents().unwrap();
        dbg!(&current);

        if current != previous {
            dbg!(&previous);
            ctx.set_contents(current.clone()).unwrap();
            // DOESN'T WORK WITH THIS LINE
            //previous = current.clone();
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
