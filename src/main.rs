use clap::Parser;

#[derive(Parser)]
struct Cli {
    port: i32,
}

fn main() {
    let args = Cli::parse();

    println!("port: {}", args.port);
}
