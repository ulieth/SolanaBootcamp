// modules3.rs
// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Make me compile! Execute `rustlings hint modules3` for hints :)


// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
}
