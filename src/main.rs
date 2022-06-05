use std::fs::File;
use std::io::{Result, Write};
use std::process::Command;

mod ast;
mod generator;
mod parser;

fn main() -> Result<()> {
    let filename = "example.pest";
    let file = std::fs::read_to_string(filename)?;
    let response = parser::parse(&file, false);
    if let Err(error) = response {
        println!("{}", error);
        std::process::exit(1);
    }
    let rules = response.unwrap();
    println!("{rules:#?}");
    let dot_string = generator::generate(&rules);
    let dot_file = "input.dot";
    let mut output = File::create(dot_file)?;
    output.write(dot_string.as_bytes())?;
    let output_file = "output.svg";
    Command::new("dot")
        .arg("-Tsvg")
        .arg(&format!("{dot_file}"))
        .args(["-o", output_file])
        .output()
        .expect("failed to execute dot");
    Ok(())
}
