mod hello;
mod util;

use hello::say;



fn main() {
    println!("Hello, world!");

    say("Mason");

    foo!(3);
}
