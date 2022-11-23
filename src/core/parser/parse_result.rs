use crate::core::{
    errors::errors::Error,
    nodes::nodes::Nodes,
};
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParseResult {
    pub error: Option<Error>,
    pub nodes: Option<Nodes>,
    pub last_registered_advance_count: u32,
    pub advance_count: u32,
    pub to_reverse_count: u32,
}
impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
            error: None,
            nodes: None,
            last_registered_advance_count: 0,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.last_registered_advance_count = 1;
        self.advance_count += 1;
    }

    pub fn register(&mut self, res: ParseResult) -> Option<Nodes> {
        self.last_registered_advance_count = res.advance_count;
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.clone().error;
        }
        return res.clone().nodes
    }

    pub fn try_register(&mut self, res: ParseResult) -> Option<Nodes> {
        if res.error.is_some() {
            self.to_reverse_count = res.advance_count;
            return None
        }
        return self.register(res)
    }

    pub fn success(&mut self, nodes: Option<Nodes>) -> ParseResult {
        self.nodes = nodes;
        return self.clone()
    }
}