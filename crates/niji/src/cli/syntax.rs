use clap::Parser;

pub const NAME: &str = "niji";
const AUTHOR: &str = "Lina Roether <lina.roether@proton.me>";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
struct Niji {}
