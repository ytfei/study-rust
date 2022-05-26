mod hello;

use hello::say;

macro_rules! foo {
    ($l:tt) => { bar!($l); }
}

macro_rules! bar {
    (3) => {
        println!("Here we are, bar!");
    }
}



fn main() {
    println!("Hello, world!");

    say("Mason");

    foo!(3);
}
