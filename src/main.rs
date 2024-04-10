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

    let info = read::read(&args.infile)?;

    println!("{} {}", args.infile, info.fmt_info);
    println!();

    let mut name_longest = 0;
    for i in &info.sections {
        if i.name.len() > name_longest { name_longest = i.name.len(); }
    }

    // for more pretty backround color
    name_longest += 1;

    println!("Sections:");
    println!("   {}  {} \t {}  {}", "Name".pad_to_width(name_longest).bold(), "Size".bold(), "Align".bold(), "Kind".bold());

    // For the table
    let mut other_color = false;
    
    for i in &info.sections {
        println!("   {}  {} \t {} \t {}", 
            {
                let fmt = i.name.pad_to_width(name_longest);

                if other_color {
                    fmt.bg_color(150, 150, 150).bold()
                } else {
                    fmt.bg_color(50, 50, 50).bold()
                }
            },
            format!("{:#x}", i.size).blue(), 
            format!("{:#x}", i.align).cyan(),
            i.typ.yellow(),
        );

        other_color = !other_color;
    }

    println!("Syms:");

    let mut sym_longest = 0;
    for i in &info.symbols {
        if i.name.len() > sym_longest { sym_longest = i.name.len(); }
    }

    // for more pretty backround color
    sym_longest += 1;

    println!("   {}  {} \t {}  {}", "Bind   ".bold(), "Typ".bold(), "Symbol".bold().pad_to_width(sym_longest), "Section".bold());

    for sym in &info.symbols {
        println!("   {}  {} \t {} {}",
                sym.scope,
                sym.typ,
                sym.name.pad_to_width(sym_longest).bold().cyan(),
                sym.section.yellow(),
            );
    }

    println!();
    println!("entry: {}", &info.entry);

    Ok(())
}