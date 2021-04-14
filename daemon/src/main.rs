use crate::options::MangodArgs;

mod options;

#[paw::main]
fn main(args: MangodArgs) {
    println!("welcome to mango daemon");
}
