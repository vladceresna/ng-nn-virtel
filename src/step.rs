
pub struct Step {
    module: String,
    command: String,
    args: Vec<String>,
    line: i32
} impl Step {
    pub fn new() -> Step {
        Step { module: String::new(), command: String::new(), args: Vec::new(), line: 0 }
    }
    pub fn set_module(&mut self, module: String) {
        self.module = module;
    }
    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }
    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }
    pub fn get_module(&self) -> String {
        self.module.clone()
    }
    pub fn get_command(&self) -> String {
        self.command.clone()
    }
    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }
    
    pub fn get_line(&self) -> i32 {
        self.line
    }
    
    pub fn set_line(&mut self, line: i32) {
        self.line = line;
    }


    pub fn args_get(&self, index: usize) -> String {
        self.get_args().get(index).unwrap().clone()
    }

}