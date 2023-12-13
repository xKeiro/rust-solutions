use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,
    #[arg(short, long("number"), conflicts_with = "number_nonblank_lines")]
    pub number_lines: bool,
    #[arg(short = 'b', long("number-nonblank"))]
    pub number_nonblank_lines: bool,
}

pub fn run(args: &Args) -> MyResult<()> {
    for filename in &args.files {
        process_file(&filename, &args)?;
    }
    Ok(())
}

fn process_file(filename: &str, args: &Args) -> MyResult<()> {
    match open(&filename) {
        Err(err) => {
            eprintln!("Failed to open {}: {}", filename, err);
            Ok(())
        }
        Ok(file) => process_lines(file, &args),
    }
}

fn process_lines(file: Box<dyn BufRead>, args: &Args) -> MyResult<()> {
    let mut last_num = 0;
    for (line_index, line_result) in file.lines().enumerate() {
        let line = line_result?;
        if args.number_lines {
            print_line_with_number(line_index + 1, &line);
        } else if args.number_nonblank_lines {
            last_num = print_line_if_not_empty(last_num, &line);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

fn print_line_with_number(line_number: usize, line: &str) {
    println!("{:>6}\t{}", line_number, line);
}

fn print_line_if_not_empty(last_num: usize, line: &str) -> usize {
    if line.is_empty() {
        println!();
        last_num
    } else {
        println!("{:>6}\t{}", last_num + 1, line);
        last_num + 1
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
