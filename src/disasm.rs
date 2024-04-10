use std::error::Error;

use object::{File, Object, ObjectSection, ObjectSymbol, Symbol, SymbolKind};
use PrintLib::colorize::Colorize;

pub fn print(file: &File) -> Result<(), Box<dyn Error>> {
    let syms: Vec<Symbol> = {
        let mut ret = vec![];

        for sym in file.symbols() {
            if sym.kind() == SymbolKind::Text {
                ret.push(sym);
            }
        }

        ret
    };

    for sym in syms {
        let name = sym.name()?;

        let section = match sym.section() {
            object::SymbolSection::Section(index) => file.section_by_index(index)?,
            _ => {
                continue;
            }
        };

        println!();
        println!("{}: ", name.bold());

        let data = section.data()?;

        let mut row_bytes = 0;

        for byte in data {
            if row_bytes == 5 {
                row_bytes = 0;
                println!()
            }
            if row_bytes == 0 {
                print!("   ");
            }
            row_bytes += 1;

            print!("{:#04X} ", byte);
        }

        println!();
    }

    Ok(())
}
