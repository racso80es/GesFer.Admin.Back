use atty;
fn main() {
    println!("{}", atty::is(atty::Stream::Stdin));
}
