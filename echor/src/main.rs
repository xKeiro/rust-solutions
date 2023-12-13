use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Arguments {
    #[arg(value_name = "TEXT", help = "Input text", required = true)]
    text: Vec<String>,
    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Arguments::parse();
    let ending = if args.omit_newline { "" } else { "\n" };
    println!("{}{}", args.text.join(" "), ending);
}
