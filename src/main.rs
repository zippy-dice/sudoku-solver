use clap::Parser;

#[derive(Parser)]
struct Cli {
    input: String
}

fn main() {
    let cli = Cli::parse();

    let input_file = cli.input;
    println!("{}", input_file);
}
