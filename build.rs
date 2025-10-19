use std::io;
mod generate_ast;
use generate_ast::*;

fn main() -> io::Result<()> {
    let output_dir = "src";
    gerenate_ast(&output_dir)
}