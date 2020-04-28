extern crate colored;
extern crate structopt;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    ///What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    ///Make the cat's eyes appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    ///Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let message = options.message;
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog");
    }
    println!("{}", message.bright_yellow().underline().on_purple());
    let eye = if options.dead { "x" } else { "o" };
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)?;
            let cat_picture = cat_template.replace("{eye}", eye);

            println!("{}", cat_picture);
        }
        None => print_text_cat(&String::from(eye)),
    }

    Ok(())
}

fn print_text_cat(eye: &String) {
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {cat_eye} {cat_eye} )", cat_eye = eye.red().bold());
    println!("    =( I )=");
}
