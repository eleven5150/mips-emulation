use crate::directive::MpDirectiveLoc;
use std::rc::Rc;

use crate::{
    attribute::{Attribute},
    constant::{MpConst},
    instruction::{MpInstruction},
    label::{MpLabel},
};
use nom::AsBytes;
use nom_locate::LocatedSpan;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct TaggedFile<'tag, 'file> {
    tag: Option<&'tag str>,
    file_contents: &'file str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Position {
    line: u32,
    line_end: u32,
    col: u32,
    col_end: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MpProgram {
    pub(crate) items: Vec<MpAttributedItem>,
    pub(crate) file_attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MpAttributedItem {
    pub(crate) item: MpItem,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) file_tag: Option<Rc<str>>,
    pub(crate) line_number: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MpItem {
    Instruction(MpInstruction),
    Directive(MpDirectiveLoc),
    Label(MpLabel),
    Constant(MpConst),
}

impl<'tag, 'file> TaggedFile<'tag, 'file> {
    pub fn new(tag: Option<&'tag str>, file_contents: &'file str) -> Self {
        Self { tag, file_contents }
    }

    pub fn tag(&self) -> Option<&'tag str> {
        self.tag
    }

    pub fn file_contents(&self) -> &'file str {
        self.file_contents
    }
}

impl Position {
    pub fn new(line: u32, line_end: u32, col: u32, col_end: u32) -> Self {
        Self {
            line,
            line_end,
            col,
            col_end,
        }
    }

    pub fn from_positions<S, E>(pos_start: LocatedSpan<S>, pos_end: LocatedSpan<E>) -> Self
    where
        S: AsBytes,
        E: AsBytes,
    {
        Self {
            line: pos_start.location_line(),
            line_end: pos_end.location_line(),
            col: pos_start.get_column() as _,
            col_end: pos_end.get_column() as _,
        }
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn line_end(&self) -> u32 {
        self.line_end
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn col_end(&self) -> u32 {
        self.col_end
    }
}

impl MpAttributedItem {
    pub fn new(
        item: MpItem,
        attributes: Vec<Attribute>,
        file_tag: Option<Rc<str>>,
        line_number: u32,
    ) -> Self {
        Self {
            item,
            attributes,
            file_tag,
            line_number,
        }
    }

    pub fn item(&self) -> &MpItem {
        &self.item
    }

    pub fn item_mut(&mut self) -> &mut MpItem {
        &mut self.item
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn file_tag(&self) -> Option<Rc<str>> {
        self.file_tag.clone()
    }

    pub fn line_number(&self) -> u32 {
        self.line_number
    }
}

impl MpProgram {
    pub fn new(items: Vec<MpAttributedItem>, file_attributes: Vec<Attribute>) -> Self {
        Self {
            items,
            file_attributes,
        }
    }

    pub fn items(&self) -> &[MpAttributedItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<MpAttributedItem> {
        &mut self.items
    }
}