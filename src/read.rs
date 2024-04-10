use std::{error::Error, fs};
use object::{Object, ObjectSection, SectionKind};

#[derive(Clone)]
pub struct SectionData {
    pub name: String,
    pub size: u64,
    pub align: u64,
    pub typ: String,
}


pub fn read(path: String) -> Result<Vec<(SectionData)>, Box<dyn Error>> {
    let binary_data = fs::read(path)?;
    let file = object::File::parse(&*binary_data)?;

    let mut ret = vec![];

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
        }.into();

        ret.push(
            SectionData {
                name: section.name()?.to_string(),
                size: section.size(),
                align: section.align(),
                typ: kind,
            }
        )
    }

    Ok(ret)
}