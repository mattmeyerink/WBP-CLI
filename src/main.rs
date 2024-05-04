use clap::Parser;

#[derive(Parser)]
struct CLI {
    app: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();
    let content = std::fs::read_to_string(&args.path).expect("Could not not read file");

    for line in content.lines() {
        println!("{}", line);
    }
}
