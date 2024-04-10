use std::error::Error;
use pad::PadStr;

use object::{File, Object, ObjectSection, ObjectSymbol, Symbol, SymbolKind};
use PrintLib::colorize::Colorize;

use capstone::prelude::*;

fn disasm(cs: &Capstone, machine_code: &[u8]) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let disasm = cs.disasm_all(machine_code, 0)?;
    let mut ret = vec![];

    for instruction in disasm.iter() {

        let fmt_arry = {
            let bytes = instruction.bytes();
            let mut fmt = String::new();

            for b in bytes {
                fmt.push_str(&format!("{:#02x} ", b));
            }

            fmt
        };
        let instruction = format!("{}", instruction);
        let fmt_instruction = instruction.split(':').collect::<Vec<&str>>()[1];

        ret.push(
            (fmt_arry, fmt_instruction.to_string())
        )
    }

    Ok(ret)
}

    

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

    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Att)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");

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

        let instrs = disasm(&cs, data)?;

        let mut longest = 0;
        for i in &instrs {
            if i.0.len() > longest {
                longest = i.0.len();
            }
        }

        for instr in instrs {
            println!("   {} {} {}", instr.0.pad_to_width(longest), "|".gray().bold(), instr.1.green());
        }


        println!();
    }

    Ok(())
}
