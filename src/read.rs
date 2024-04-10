use object::{
    File, Object, ObjectSection, ObjectSymbol, SectionKind, SymbolKind, SymbolScope, SymbolSection,
};
use std::error::Error;
use PrintLib::colorize::Colorize;

pub struct FileInfo {
    pub sections: Vec<SectionData>,
    pub symbols: Vec<SymbolData>,
    pub fmt_info: String,
    pub entry: u64,
    pub debug: bool,
}

pub struct SectionData {
    pub name: String,
    pub size: u64,
    pub align: u64,
    pub typ: String,
}

pub struct SymbolData {
    pub typ: String,
    pub scope: String,
    pub name: String,
    pub section: String,
}

pub fn read(file: &File) -> Result<FileInfo, Box<dyn Error>> {
    let mut ret = FileInfo {
        sections: vec![],
        symbols: vec![],
        fmt_info: format!(
            "{:#?} {}-{:#?}-endian",
            file.format(),
            format!("{:#?}", file.architecture()).gray().bold(),
            file.endianness(),
        ),
        entry: file.entry(),
        debug: file.has_debug_symbols(),
    };

    for section in file.sections() {
        let kind = match section.kind() {
            SectionKind::Unknown => "Unkown",
            SectionKind::Text => "Text",
            SectionKind::Data => "Data",
            SectionKind::ReadOnlyData => "Read Only Data",
            SectionKind::ReadOnlyDataWithRel => "Read Only Data With Rel",
            SectionKind::ReadOnlyString => "Read Only String",
            SectionKind::UninitializedData => "Uninitialized Data",
            SectionKind::Common => "Common",
            SectionKind::Tls => "Tls",
            SectionKind::UninitializedTls => "Uninitialized Tls",
            SectionKind::TlsVariables => "Tls Variables",
            SectionKind::OtherString => "Other String",
            SectionKind::Other => "Other",
            SectionKind::Debug => "Debug",
            SectionKind::Linker => "Linker",
            SectionKind::Note => "Note",
            SectionKind::Metadata => "Metadata",
            _ => "",
        }
        .into();

        ret.sections.push(SectionData {
            name: section.name()?.to_string(),
            size: section.size(),
            align: section.align(),
            typ: kind,
        })
    }

    for sym in file.symbols() {
        let kind: String = match sym.kind() {
            SymbolKind::Unknown => "Unknown",
            SymbolKind::Null => "Null",
            SymbolKind::Text => "Func",
            SymbolKind::Data => "Data",
            SymbolKind::Section => {
                continue;
            }
            SymbolKind::File => "File",
            SymbolKind::Label => "Label",
            SymbolKind::Tls => "Tls",
            _ => "",
        }
        .into();

        let mut scope: String = match sym.scope() {
            SymbolScope::Unknown => "Unknown ".bold().bg_red(),
            SymbolScope::Compilation => "Private ".bold().bg_blue(),
            SymbolScope::Linkage => "Public  ".bold().bg_magenta(),
            SymbolScope::Dynamic => "Import  ".bold().bg_yellow(),
        };

        if sym.is_global() {
            scope = "Global  ".bold().bg_magenta();
        };

        let section = match sym.section() {
            SymbolSection::Unknown => "Unknown",
            SymbolSection::None => "",
            SymbolSection::Undefined => "Undefined",
            SymbolSection::Absolute => "",
            SymbolSection::Common => "",
            SymbolSection::Section(index) => {
                let sec = file.section_by_index(index)?;
                sec.name()?
            }
            _ => "",
        };

        ret.symbols.push(SymbolData {
            typ: kind,
            scope: scope,
            name: sym.name()?.to_string(),
            section: section.into(),
        });
    }

    Ok(ret)
}
