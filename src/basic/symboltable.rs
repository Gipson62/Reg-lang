use std::collections::HashMap;



pub(crate) struct SymbolTable {
    pub symbols:Vec<String>,
    pub parent:Option<Box<SymbolTable>>,
}