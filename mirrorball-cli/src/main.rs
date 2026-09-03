use crate::args::Args;

mod args;

fn main() {
    let args = Args::parse_from_env().unwrap();
    dbg!(args);
}
