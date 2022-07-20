mod blobs;
mod codes;
pub mod file;
mod gen;
mod helpers;
mod strings;
mod tables;
mod r#type;
mod type_name;
mod value;

use super::*;
use blobs::*;
use codes::*;
pub use gen::*;
pub use helpers::*;
pub use r#type::*;
use strings::*;
pub use tables::*;
pub use type_name::*;
pub use value::*;

// TODO: provide a high-level writer that collects type information and writes it out using
// the lower level tables and PE file logic internally. 

pub struct Writer<'a> {
    reader: &'a super::reader::Reader<'a>,
    module: String,
}

impl<'a> Writer<'a> {
    pub fn new(module: &str, reader: &'a super::reader::Reader) -> Self {
        Self {reader, module: module.to_string() }
    }
}