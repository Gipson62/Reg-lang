use crate::basic::{
    symboltable::{
        SymbolTable
    },
    position::{
        Position
    },
};

pub(crate) struct Context {
    pub display_name:String,
    pub parent:Option<Box<Context>>,
    pub parent_entry_pos:Option<Box<Position>>,
    pub symbol_table:Option<Box<SymbolTable>>,
}
impl Context {
    pub fn new(display_name:String, parent:Option<Box<Context>>, parent_entry_pos:Option<Box<Position>>) -> Context {
        Context {
            display_name,
            parent,
            parent_entry_pos,
            symbol_table:None,
        }
    }
}