extern crate mud;

fn main() {
    if let Err(e) = mud::run() {
        println!("{}", e);
    };
}
