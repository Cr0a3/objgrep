use pad::PadStr;
use std::error::Error;

use object::{File, Object, ObjectSection, ObjectSymbol, Relocation, RelocationTarget, SymbolKind};
use PrintLib::colorize::Colorize;

use capstone::prelude::*;

fn disasm(
    cs: &Capstone,
    relocs: Vec<(u64, Relocation, &str)>,
    machine_code: &[u8],
    pos_tuple: (u64, u64),
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let disasm = cs.disasm_all(machine_code, 0)?;
    let mut ret: Vec<(String, String)> = vec![];
    let mut instr = vec![];
    let mut reloc_vec = vec![];

    let mut last_index: i128 = 0;
    let mut index: i128 = 0;

    for instruction in disasm.iter() {
        last_index = index;

        index += (instruction.bytes().len() - 1) as i128;

        let fmt_arry = {
            let bytes = instruction.bytes();
            let mut fmt = String::new();

            for b in bytes {
                fmt.push_str(&format!("{:#02X} ", b));
            }

            fmt
        };

        let instruction = format!("{}", instruction);
        let fmt_instruction = instruction.split(':').collect::<Vec<&str>>()[1].to_string();

        let mut pushed_reloc_vec = false;

        for _reloc in relocs.iter() {
            let offset = _reloc.0 as i128 - pos_tuple.0 as i128;

            if (last_index..index).contains(&offset) {
                reloc_vec.push( format!("# {}", _reloc.2).magenta().bold());
                pushed_reloc_vec = true;
            }
        }
        
        if !pushed_reloc_vec { reloc_vec.push("".into()); }

        instr.push((fmt_arry, fmt_instruction))
    }

    let mut longest = 0;
    for i in &instr {
        if i.1.len() > longest {
            longest = i.1.len()
        }
    }

    for index in 0..(instr.len()) {
        let inst = instr.get(index).unwrap();

        ret.push(( (*inst.0).to_string(), format!("{}{}", inst.1.pad_to_width(longest + 1), reloc_vec.get(index).unwrap() )));
    }

    Ok(ret)
}

pub fn print_func(file: &File, name: &str) -> Result<(), Box<dyn Error>> {
    let mut symbol = None;
    for sym in file.symbols() {
        if sym.kind() == SymbolKind::Text {
            if sym.name()? == name {
                symbol = Some(sym);
            }
        }
    }

    if symbol.is_none() {
        return Ok(());
    }

    let symbol = symbol.unwrap();

    let section = match symbol.section() {
        object::SymbolSection::Section(index) => file.section_by_index(index)?,
        _ => {
            return Ok(());
        }
    };

    let relocs = { section.relocations() };

    // In function
    let adr = {
        if symbol.size() > 0 {
            symbol.address()
        } else {
            section.address()
        }
    };

    let size = {
        if symbol.size() > 0 {
            symbol.size()
        } else {
            section.size()
        }
    };

    let reloc_map: Vec<(u64, Relocation, &str)> = {
        let mut ret = vec![];
        for reloc in relocs {
            if !(adr..(size + adr)).contains(&reloc.0) {
                continue;
            }

            let target = reloc.1.target();

            match target {
                RelocationTarget::Symbol(index) => {
                    let symbol = file.symbol_by_index(index)?;
                    ret.push((reloc.0, reloc.1, symbol.name()?));
                }
                _ => {
                    continue;
                }
            }
        }

        ret
    };

    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");

    let name = symbol.name()?;

    println!();
    println!("{}: ", name.bold());

    let data = section.data()?;

    let instrs = disasm(&cs, reloc_map, data, (adr, size))?;

    let mut longest = 0;
    for i in &instrs {
        if i.0.len() > longest {
            longest = i.0.len();
        }
    }

    for instr in instrs {
        println!(
            "   {} {} {}",
            instr.0.pad_to_width(longest),
            "|".gray().bold(),
            instr.1.green()
        );
    }

    println!();

    Ok(())
}
