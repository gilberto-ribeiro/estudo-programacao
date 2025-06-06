use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".bold().red(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_data = replace(&args.target, &args.replacement, &data).unwrap_or_else(|e| {
        eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
        std::process::exit(1);
    });

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write file to file '{}': {:?}",
                "Error:".red().bold(),
                args.output,
                e
            );
            std::process::exit(1);
        }
    };
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurances of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expect 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}
