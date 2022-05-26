mod ast;
mod parser;

fn main() {
    let filename = "example.pest";
    let file = std::fs::read_to_string(filename).expect(filename);
    let response = parser::parse(&file, false);
    if let Err(error) = response {
        println!("{}", error);
        std::process::exit(1);
    }
    let ast = response.unwrap();
    println!("{ast:#?}");
}
