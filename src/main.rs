// vim:set ft=rust fenc=utf-8 si sw=4:

mod prettify;

fn main() {
    std::process::exit(prettify::uumain(std::env::args().collect()));
}
