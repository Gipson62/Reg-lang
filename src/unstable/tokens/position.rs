use std::fmt;

#[derive(Clone, Debug, PartialEq)]
/// Struct used by the Language to help find the position of an error.
pub(crate) struct Position {
    pub idx : u32, 
    pub ln : u32,
    pub col : u32,
    pub file_name : String,
}
impl Position {
    /// Creates a new `Position` instance.
    pub fn new(idx : u32, ln : u32, col : u32, file_name : String) -> Position {
        Position {
            idx,
            ln,
            col,
            file_name,
        }
    }
    /// Advance the position by 1 character
    pub fn advance(&mut self, current_char:Option<char>) -> &Position {
        self.idx += 1;
        self.col += 1;
        match current_char {
            Some(p) => {
                if p == '\n'{
                    self.ln += 1;
                    self.col = 0;
                }
            },
            None => println!("None"),
        }
        return self
    }
}

impl fmt::Display for Position {
    /// Formats the position as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    /// use std::fmt::Display;
    /// use std::fmt::Formatter;
    ///
    /// let pos = Position {
    ///     idx: 0,
    ///     ln: 1,
    ///     col: 1,
    ///     file_name: "test.txt".to_string(),
    /// };
    ///
    /// assert_eq!(format!("{}", pos), "Position [idx: 0, ln: 1, col: 1, file_name: test.txt]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position [idx: {}, ln: {}, col: {}, file_name: {}]", self.idx, self.ln, self.col, self.file_name)
    }
}