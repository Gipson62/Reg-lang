pub mod repl;

use repl::REPL;
use std::io::Write;
use std::io;

use reg_lang_compiler::RegCompiler;
use reg_lang_vm::RegLangVM;
fn main() {
    let mut repl = REPL::new();
    repl.run_console();
}