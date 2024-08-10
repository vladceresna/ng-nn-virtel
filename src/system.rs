
use std::collections::HashMap;
use std::io;
use regex::Regex;
use crate::logger::LogType;

use super::logger;
use std::fs;
use std::error::Error;
use super::step;

pub struct System {
    lists: HashMap<String, Vec<String>>,
    vars: HashMap<String, String>,
    bins: Vec<String>,
}
impl System {
    pub fn new(bins : Vec<String>) -> System {
        System { lists: HashMap::new(), vars: HashMap::new(), bins }
    }

    pub fn var_del(&mut self, name: String) -> Result<bool, String>{
        self.vars.remove(&name);
        Ok(true)
    }
    pub fn var_set(&mut self, name: String, value: String) -> Result<bool, String>{
        self.vars.insert(name, value);
        Ok(true)
    }
    pub fn var_get(&mut self, name: String) -> Result<String, String>{
        if !self.vars.contains_key(&name) {
            logger::log(format!("Variable {name} not found"), LogType::ERROR);
            return Err(String::from("Variable not found"));
        }
        Ok(self.vars.get(&name).unwrap().to_string())
    }
    pub fn var_is_exists(&mut self, name: String) -> Result<bool, String>{
        Ok(self.vars.contains_key(&name))
    }

    pub fn lists_new(&mut self, name: String) -> Result<bool, String>{
        self.lists.insert(name, Vec::new());
        Ok(true)
    }
    pub fn lists_del(&mut self, name: String) -> Result<bool, String>{
        self.lists.remove(&name);
        Ok(true)
    }
    pub fn lists_get(&mut self, name: String) -> Result<&Vec<String>, String>{
        Ok(self.lists.get(&name).unwrap())
    }
    pub fn lists_item_get(&mut self, name: String, index: usize) -> Result<String, String>{
        Ok(self.lists.get(&name).unwrap().get(index).unwrap().to_string())
    }
    pub fn lists_item_add(&mut self, name: String, value: String) -> Result<bool, String>{
        self.lists.get_mut(&name).unwrap().push(value);
        Ok(true)
    }
    pub fn lists_item_del(&mut self, name: String, index: usize) -> Result<bool, String>{
        self.lists.get_mut(&name).unwrap().remove(index);
        Ok(true)
    }




    
    pub fn start(&mut self){
        let code: String = self.get_bin_with_name("start.steps".to_string()).expect("REASON");
        self.run(code);
    }
    fn get_bin_with_name(&mut self, name: String) -> Result<String, Box<dyn Error>>{
        for file_path in self.bins.clone() {
            if file_path.ends_with(&name){
                return Ok(fs::read_to_string(file_path)?);
            }
        }
        Err(String::from("File start.steps not found").into())
    }

    pub fn bin_run(&mut self, file_name: String){
        let code: String = self.get_bin_with_name(file_name).expect("REASON");
        self.run(code);
    }

    pub fn sys_out(&mut self, str: String) -> Result<bool, String> {
        logger::log(self.expression_unwrap(str)?, logger::LogType::APP);
        Ok(true)
    }
    pub fn sys_in(&mut self) -> Result<String, String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        Ok(input)
    }



    pub fn get_as_numbers(&mut self, cort: (String, String)) -> (f64,f64) {
        let a = self.get_as_number(cort.0);
        let b = self.get_as_number(cort.1);
        (a,b)
    }

    pub fn math_plus(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        self.var_set(c,(a + b).to_string())
    }
    pub fn math_min(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        self.var_set(c,(a - b).to_string())
    }
    pub fn math_mult(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        self.var_set(c,(a * b).to_string())
    }
    pub fn math_div(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        self.var_set(c,(a / b).to_string())
    }
    pub fn math_exp(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        self.var_set(c,(a.powf(b)).to_string())
    }
    pub fn math_root(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        self.var_set(c,(a.powf(1.0 / b)).to_string())
    }
    pub fn math_mod(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        self.var_set(c,(a % b).to_string())
    }
    pub fn math_floor(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let (a, b) = self.get_as_numbers((a,b));
        let (a, b) = (a as i64, b as i64);
        self.var_set(c,(a / b).to_string())
    }
    pub fn math_incr(&mut self, a: String) -> Result<bool, String> {
        let a_value = self.get_as_number(a.clone());
        self.var_set(a,(a_value + 1.0).to_string())
    }
    pub fn math_decr(&mut self, a: String) -> Result<bool, String> {
        let a_value = self.get_as_number(a.clone());
        self.var_set(a,(a_value - 1.0).to_string())
    }


    /*
    fn dir_new(path: String) -> Result<(bool), String>;
    fn dir_del(path: String) -> Result<(bool), String>;
    fn dir_is_exists(path: String) -> Result<(bool), String>;

    fn file_new(path: String) -> Result<(bool), String>;
    fn file_del(path: String) -> Result<(bool), String>;
    fn file_write(path: String, value: String) -> Result<(bool), String>;
    fn file_push(path: String, value: String) -> Result<(bool), String>;
    fn file_get(path: String) -> Result<(String), String>;
    fn file_is_exists(path: String) -> Result<(bool), String>;

    fn server_new(name: String, port: String) -> Result<(bool), String>;

    fn client_new(name: String) -> Result<(bool), String>;
    */





    pub fn get_as_number(&mut self, name: String) -> f64 {
        let name = self.expression_unwrap(name);
        self.to_number_str(name)
    }
    pub fn to_number_str(&mut self, str: Result<String, String>) -> f64 {
        match str {
            Ok(str) => str.parse::<f64>().unwrap(),
            Err(_) => 0.0
        }
    }


    fn expression_unwrap(&mut self, fragment: String) -> Result<String, String>{
        let fragment = fragment.trim().to_string();
        
        if self.is_string(fragment.clone())? {
            return Ok(fragment[1..fragment.len() - 1].to_string());
        } else {
            return Ok(self.var_get(fragment)?);
        }
    }


    fn is_value(&mut self, fragment: String) -> Result<bool, String>{
        if self.is_string(fragment.clone())? || self.is_number(fragment)? {
            return Ok(true);
        }
        Err(String::from("Error"))
    }

    fn is_number(&mut self, fragment: String) -> Result<bool, String>{
        let re = Regex::new(r"^[\d.]+$").unwrap();
        if re.is_match(fragment.as_str()) {
            return Ok(true);
        }
        Err(String::from("Error"))
    }
    fn is_string(&mut self, fragment: String) -> Result<bool, String>{
        if fragment.starts_with("\"") && fragment.ends_with("\"") {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }



    ///lexer
    pub fn run(&mut self, code: String) -> Result<bool, String> {

        let mut last_string = String::new();
        let mut el_id = 0;
        let mut line = 0;
        let mut step = step::Step::new();
    
        for char in code.chars() {
            if char == ' ' {
                if el_id == 0 {
                    step.set_module(last_string.clone().trim().to_string());
                } else if el_id == 1 {
                    step.set_command(last_string.clone());
                } else {
                    step.add_arg(last_string.clone());
                }
                el_id += 1;
                last_string = String::new();
            } else if char == ';' {
                el_id = 0;
                step.add_arg(last_string);
                step.set_line(line);
                last_string = String::new();
                self.run_step(&step);
                step = step::Step::new();
                line += 1;
            } else {
                last_string.push(char);
            }
        }
    
        Ok(true)
    }


    ///parser
    pub fn run_step(&mut self, step: &step::Step) {

        match step.get_module().as_str() {
            "sys" => match step.get_command().as_str() {
                "out" => {
                    let mut message = String::new();
                    for arg in step.get_args() {
                        message.push_str(arg.as_str());
                        message.push(' ');
                    }
                    self.sys_out(message.clone());
                },
                "in" => {
                    self.sys_in();
                }
                _ => {}
            },
            "var" => match step.get_command().as_str() {
                "set" => {
                    self.var_set(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone());
                },
                "get" => {
                    self.var_get(step.get_args().get(0).unwrap().clone());
                },
                "del" => {
                    self.var_del(step.get_args().get(0).unwrap().clone());
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "math" => match step.get_command().as_str() {
                "plus" => {
                    self.math_plus(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "min" => {
                    self.math_min(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "mult" => {
                    self.math_mult(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "div" => {
                    self.math_div(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "exp" => {
                    self.math_exp(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "root" => {
                    self.math_root(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "mod" => {
                    self.math_mod(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "floor" => {
                    self.math_floor(step.get_args().get(0).unwrap().clone(), step.get_args().get(1).unwrap().clone(), step.get_args().get(2).unwrap().clone());
                },
                "incr" => {
                    self.math_incr(step.get_args().get(0).unwrap().clone());
                },
                "decr" => {
                    self.math_decr(step.get_args().get(0).unwrap().clone());
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "bin" => match step.get_command().as_str() {
                "run" => {
                    self.bin_run(step.get_args().get(0).unwrap().clone());
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            _ => {
                logger::log(format!("Unknown module: {} on line {}", step.get_module(), step.get_line()), logger::LogType::ERROR);
            }
        }
    }
    
}
