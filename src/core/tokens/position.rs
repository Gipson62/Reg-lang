#[derive(Clone, Debug, PartialEq)]
/// Base struct for the position of a token in the source code
pub(crate) struct Position {
    pub idx: u32,
    pub ln: u32,
    pub col: u32,
    pub file_name: String,
    pub text: String,
}
impl Position {
    pub fn new(
        idx:u32,
        ln:u32,
        col:u32,
        file_name:String,
        text:String
    ) -> Position {
        Position {
            idx,
            ln,
            col,
            file_name,
            text,
        }
    }
    /// Advance the position by 1 character
    pub fn advance(
        &mut self,
        current_char:Option<char>
    ) -> &Position {
        self.idx += 1;
        self.col += 1;

        match current_char {
            Some(p) => {
                if p == '\n'{
                    self.ln += 1;
                    self.col += 0;
                }
            },
            None => println!("None"),
        }
        return self
    }
}