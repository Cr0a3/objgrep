pub mod read;

use std::error::Error;
use clap::Parser;
use PrintLib::colorize::Colorize;
use pad::PadStr;

/// Simple bingrep clone
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file to dump
    infile: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let info = read::read(args.infile)?;

    let mut name_longest = 0;
    for i in &info {
        if i.name.len() > name_longest { name_longest = i.name.len(); }
    }

    // for more pretty backround color
    name_longest += 1;

    println!("Sections:");
    println!("   {}  {} \t {}  {}", "Name".pad_to_width(name_longest).bold(), "Size".bold(), "Align".bold(), "Kind".bold());

    // For the table
    let mut other_color = false;
    
    for i in &info {
        println!("   {}  {} \t {} \t {}", 
            {
                let fmt = i.name.pad_to_width(name_longest);

                if other_color {
                    fmt.bg_color(150, 150, 150).bold()
                } else {
                    fmt.bg_color(50, 50, 50).bold()
                }
            },
            format!("{:#x}", i.size).to_string().blue(), 
            format!("{:#x}", i.align).to_string().cyan(),
            i.typ.yellow(),
        );

        other_color = !other_color;
    }

    println!("Syms:");

    Ok(())
}