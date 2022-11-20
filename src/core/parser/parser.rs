///Import errors
use crate::core::{
    errors::errors::Error,
    nodes::nodes::{
        Nodes,
    },
};


pub(crate) struct ParseResult {
    pub(crate) error: Option<Error>,
    pub(crate) node: Nodes,
    pub(crate) advance_count: usize,
}