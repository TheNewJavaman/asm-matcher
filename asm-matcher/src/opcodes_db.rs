//! Parser for [opcodesDB](https://github.com/MahdiSafsafi/opcodesDB), which generates JSON files
//! detailing every opcode

#![allow(dead_code)]

use serde::{de, Deserialize, Deserializer};
use serde::de::Unexpected;

#[derive(Deserialize)]
struct OpcodesDb {
    enums: Vec<Enum>,
    arch: String,
    widths: Vec<Width>,
    registers: Vec<Register>,
    version: String,
    records: Vec<Record>,
    tables: Vec<Table>,
}

#[derive(Deserialize)]
struct Enum {
    id: String,
    items: Vec<EnumItem>,
    #[serde(deserialize_with = "bool_from_int")]
    stringify: bool,
    #[serde(deserialize_with = "opt_bool_from_opt_int")]
    optional: Option<bool>,
}

#[derive(Deserialize)]
struct EnumItem {
    name: String,
}

#[derive(Deserialize)]
struct Width {
    name: String,
    width16: u32,
    width32: u32,
    width64: u32,
}

#[derive(Deserialize)]
struct Register {
    size: u32,
    #[serde(deserialize_with = "bool_from_int")]
    is_first: bool,
    #[serde(deserialize_with = "bool_from_int")]
    is_last: bool,
    #[serde(deserialize_with = "bool_from_int")]
    is_even: bool,
    encoding: u32,
    datatype: String,
    reg: String,
    kind: String,
}

#[derive(Deserialize)]
struct Record {
    id: String,
    rectype: String,
    title: Option<String>,
    extensions: Option<Vec<String>>,
    metadata: Option<RecordMetadata>,
    diagram: Option<RecordDiagram>,
    categories: Option<Vec<String>>,
    tags: Option<RecordTags>,
    templates: Option<Vec<RecordTemplate>>,
    iflags: Option<RecordIflags>,
}

#[derive(Deserialize)]
struct RecordMetadata {
    #[serde(deserialize_with = "bool_from_int_str")]
    deprecated: bool,
    isa: String,
}

#[derive(Deserialize)]
struct RecordDiagram {
    fields: Vec<RecordDiagramField>,
}

#[derive(Deserialize)]
struct RecordDiagramField {
    value: String,
    name: String,
}

#[derive(Deserialize)]
struct RecordTags {
    page: String,
}

#[derive(Deserialize)]
struct RecordTemplate {
    metadata: RecordTemplateMetadata,
    bitdiffs: Option<RecordTemplateBitdiffs>,
    syntax: RecordTemplateSyntax,
}

#[derive(Deserialize)]
struct RecordTemplateMetadata {
    #[serde(deserialize_with = "opt_bool_from_opt_int_str")]
    xacquire: Option<bool>,
    #[serde(deserialize_with = "opt_bool_from_opt_int_str")]
    lock: Option<bool>,
    #[serde(deserialize_with = "opt_bool_from_opt_int_str")]
    xrelease: Option<bool>,
}

#[derive(Deserialize)]
struct RecordTemplateBitdiffs {
    fields: Vec<RecordTemplateBitdiffsField>
}

#[derive(Deserialize)]
struct RecordTemplateBitdiffsField {
    value: String,
    name: String
}

#[derive(Deserialize)]
struct RecordTemplateSyntax {
    mnem: String,
    text: String,
    ast: Vec<RecordAst>,
}

#[derive(Deserialize)]
struct RecordAst {
    #[serde(rename = "type")]
    ast_type: String,
    value: Option<String>,
    #[serde(deserialize_with = "opt_bool_from_opt_int")]
    suppressed: Option<bool>,
    #[serde(deserialize_with = "opt_bool_from_opt_int")]
    read: Option<bool>,
    #[serde(deserialize_with = "opt_bool_from_opt_int")]
    write: Option<bool>,
    size: Option<u32>,
    datatype: Option<String>,
    encodedin: Option<String>,
    symbol: Option<String>,
}

#[derive(Deserialize)]
struct RecordIflags {
    af: String,
    of: String,
    cf: String,
    sf: String,
    zf: String,
    pf: String,
}

#[derive(Deserialize)]
struct Table {}

/// Credit: https://github.com/serde-rs/serde/issues/1344#issuecomment-410309140
fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"0 or 1",
        ))
    }
}

fn opt_bool_from_opt_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where D: Deserializer<'de>,
{
    match Option::deserialize(deserializer)? {
        None => Ok(None),
        Some(0u8) => Ok(Some(false)),
        Some(1u8) => Ok(Some(true)),
        Some(other) => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"None, 0, or 1",
        ))
    }
}

fn bool_from_int_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_str() {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &r#""0" or "1""#,
        ))
    }
}

fn opt_bool_from_opt_int_str<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where D: Deserializer<'de>,
{
    match Option::deserialize(deserializer)? {
        None => Ok(None),
        Some("0") => Ok(Some(false)),
        Some("1") => Ok(Some(true)),
        Some(other) => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &r#"None, "0", or "1""#,
        ))
    }
}
