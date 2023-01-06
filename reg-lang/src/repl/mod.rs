use super::*;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: RegLangVM,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            command_buffer: Vec::new(),
            vm: RegLangVM::new(Vec::new()),
        }
    }
    pub fn run_console(&mut self) {
        println!("Reg-Lang REPL (v0.0.1) RedGear-Studio");
        println!("Type .quit or .exit to exit the REPL");
        println!("Type .history to see the command history");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!("Reg-Lang > ");
            io::stdout().flush().expect("ERROR: Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("ERROR: Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".quit" | ".exit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                },
                _ => {
                    let bytecode = RegCompiler::compile(buffer).program;
                    self.vm.program = bytecode;
                    self.vm.run();
                }
            }
        }
    }
}