
use std::collections::HashMap;
use std::io::Write;
use std::{error, io};
use std::num::ParseFloatError;
use regex::Regex;
use crate::logger::LogType;

use super::logger;
use std::fs;
use std::path;
use std::error::Error;
use super::step;

pub struct System {
    /* lists: HashMap<String, Vec<String>>, */
    id: String,
    vars: HashMap<String, String>,
    bins: Vec<String>,
}
impl System {
    pub fn new(id: String, bins: Vec<String>) -> System {
        System {/*  lists: HashMap::new(),  */ id, vars: HashMap::new(), bins }
    }

    pub fn var_del(&mut self, name: String) -> Result<bool, String>{
        self.vars.remove(&name);
        Ok(true)
    }
    /// Need to set value as it inputed by user
    /// For example: 
    /// "value" for clear value and
    /// var for copy value
    pub fn var_set(&mut self, name: String, value: String) -> Result<bool, String>{
        let value = self.var_get(value).unwrap();
        self.vars.insert(name.clone(), value);
        Ok(true)
    }
    /// Gets from value: "value"=value
    /// and as var: var=value
    pub fn var_get(&mut self, name: String) -> Result<String, String>{
        let name = name.trim().to_string();
        if is_value(name.clone()).unwrap() {
            return Ok(from_value(name).unwrap());
        } else {
            if self.var_is_exists(name.clone()).unwrap() {
                return Ok(self.vars.get(&name).unwrap().to_string());
            }
            logger::log(format!("Variable {name} not found"), LogType::ERROR);
            return Err(String::from("Variable not found"));
            
        }
    }
    pub fn var_is_exists(&mut self, name: String) -> Result<bool, String>{
        Ok(self.vars.contains_key(&name))
    }
/* 
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
    } */




    
    pub fn start(&mut self){
        let result = self.get_bin_with_name("start.steps".to_string());
        match result {
            Ok(_) => {self.run(result.unwrap());},
            Err(_) => {logger::log(format!("Bin file start.steps not found in scope of {} application", self.id), logger::LogType::ERROR);}
        }
    }
    fn get_bin_with_name(&mut self, name: String) -> Result<String, Box<dyn Error>>{
        for file_path in self.bins.clone() {
            if file_path.ends_with(&name){
                return Ok(fs::read_to_string(file_path)?);
            }
        }
        Err(format!("File {name} not found").into())
    }

    pub fn bin_run(&mut self, file_name: String){
        let file_name = self.var_get(file_name).unwrap();
        let get_code_res = self.get_bin_with_name(file_name.clone());
        let code: String;
        match get_code_res {
            Ok(_) => {
                self.run(get_code_res.unwrap());
            },
            Err(_) => {
                logger::log(format!("Bin with name {file_name} not found"), logger::LogType::ERROR);
            }
        }
    }

    pub fn sys_out(&mut self, str: String) -> Result<bool, String> {
        logger::log(self.var_get(str).unwrap(), logger::LogType::APP);
        Ok(true)
    }
    pub fn sys_in(&mut self, var_name: String) -> Result<bool, String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.var_set(var_name, to_value_str(input.trim().to_string().as_mut_str()));
        Ok(true)
    }


    pub fn math_plus(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap());
        let b = to_number_str(self.var_get(b.clone()).unwrap());
        self.var_set(c,to_value_num(a + b))
    }
    pub fn math_min(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap());
        let b = to_number_str(self.var_get(b.clone()).unwrap());
        self.var_set(c,to_value_num(a - b))
    }
    pub fn math_mult(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap());
        let b = to_number_str(self.var_get(b.clone()).unwrap());
        self.var_set(c,to_value_num(a * b))
    }
    pub fn math_div(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap());
        let b = to_number_str(self.var_get(b.clone()).unwrap());
        self.var_set(c,to_value_num(a / b))
    }
    pub fn math_exp(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap());
        let b = to_number_str(self.var_get(b.clone()).unwrap());
        self.var_set(c,to_value_num(a.powf(b)))
    }
    pub fn math_root(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap());
        let b = to_number_str(self.var_get(b.clone()).unwrap());
        self.var_set(c,to_value_num(a.powf(1.0 / b)))
    }
    pub fn math_mod(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap());
        let b = to_number_str(self.var_get(b.clone()).unwrap());
        self.var_set(c,to_value_num(a % b))
    }
    pub fn math_floor(&mut self, a: String, b: String, c: String) -> Result<bool, String> {
        let a = to_number_str(self.var_get(a.clone()).unwrap()) as i64;
        let b = to_number_str(self.var_get(b.clone()).unwrap()) as i64;
        self.var_set(c,to_value_num((a / b) as f64))
    }
    pub fn math_incr(&mut self, a: String) -> Result<bool, String> {
        let a_value = to_number_str(self.var_get(a.clone()).unwrap());
        self.var_set(a,to_value_num(a_value + 1.0))
    }
    pub fn math_decr(&mut self, a: String) -> Result<bool, String> {
        let a_value = to_number_str(self.var_get(a.clone()).unwrap());
        self.var_set(a,to_value_num(a_value - 1.0))
    }



    pub fn str_plus(&mut self, mut args: Vec<String>) -> Result<bool, String> {
        let res_var = args.pop().unwrap();
        let mut res_value = String::from("");
        for arg in args {
            res_value.push_str(self.var_get(arg).unwrap().as_str());
        }
        self.var_set(res_var, to_value_str(res_value.as_mut_str()))
    }
    pub fn str_cut(&mut self, source: String, left: String, right: String, res_var: String) -> Result<bool, String> {
        let source = self.var_get(source).unwrap();
        let left = to_number_str(self.var_get(left).unwrap()) as u64;
        let right = to_number_str(self.var_get(right).unwrap()) as u64;
        let mut i = 0;
        let mut res_value = String::from("");
        for char in source.chars() {
            if i>=left && i<=right {
                res_value.push(char);
            }
            i += 1;
        }
        self.var_set(res_var, to_value_str(res_value.as_mut_str()))
    }
    pub fn str_len(&mut self, source: String, res_var: String) -> Result<bool, String> {
        let source = self.var_get(source).unwrap();
        let res_value = source.chars().count();
        self.var_set(res_var, to_value_num(res_value as f64))
    }
    fn str_eq(&mut self, source1: String, source2: String, res_var: String) -> Result<bool, String> {
        let source1 = self.var_get(source1).unwrap();
        let source2 = self.var_get(source2).unwrap();
        let mut res_value = (source1 == source2).to_string();
        self.var_set(res_var, to_value_str(res_value.as_mut_str()))
    }



    fn bool_and(&mut self, source1: String, source2: String, res_var: String) -> Result<bool, String> {
        let source1 = self.var_get(source1).unwrap();
        let source2 = self.var_get(source2).unwrap();
        let mut res_value = ((source1 == "true") && (source2 == "true")).to_string();
        self.var_set(res_var, to_value_str(res_value.as_mut_str()))
    }
    fn bool_or(&mut self, source1: String, source2: String, res_var: String) -> Result<bool, String> {
        let source1 = self.var_get(source1).unwrap();
        let source2 = self.var_get(source2).unwrap();
        let mut res_value = ((source1 == "true") || (source2 == "true")).to_string();
        self.var_set(res_var, to_value_str(res_value.as_mut_str()))
    }
    fn bool_not(&mut self, source: String, res_var: String) -> Result<bool, String> {
        let source = self.var_get(source).unwrap();
        let mut res_value = ((source != "true")).to_string();
        self.var_set(res_var, to_value_str(res_value.as_mut_str()))
    }



    
    fn dir_new(&mut self, path: String) -> Result<bool, String>{
        let path = self.var_get(path).unwrap();
        match fs::create_dir_all(path.clone()) {
            Ok(_) => {
                return Ok(true);
            },
            Err(error) => {
                logger::log(format!("Error while creating dir {}: {}", path, error), logger::LogType::ERROR);
                return Ok(false);
            }
        }
    }
    fn dir_exists(&mut self, path: String, res_var: String) -> Result<bool, String>{
        let path = self.var_get(path).unwrap();
        match path::Path::new(path.clone().as_str()).try_exists() {
            Ok(v) => {
                self.var_set(res_var, to_value_str(v.to_string().as_mut_str()));
                return Ok(true);
            },
            Err(error) => {
                logger::log(format!("Error while creating dir {}: {}", path, error), logger::LogType::ERROR);
                return Ok(false);
            }
        }
    }
    fn dir_del(&mut self, path: String) -> Result<bool, String>{
        let path = self.var_get(path).unwrap();
        match fs::remove_dir_all(path.clone()) {
            Ok(_) => {
                return Ok(true);
            },
            Err(error) => {
                logger::log(format!("Error while deleting dir {}: {}", path, error), logger::LogType::ERROR);
                return Ok(false);
            }
        }
    }
    fn file_write(&mut self, path: String, value: String) -> Result<bool, String>{
        let path = self.var_get(path).unwrap();
        let value = self.var_get(value).unwrap();
        
        match fs::write(path.clone(), value.clone()) {
            Ok(_) => {
                return Ok(true);
            },
            Err(error) => {
                logger::log(format!("Error while writing file {}: {}", path, error), logger::LogType::ERROR);
                return Ok(false);
            }
        }
    }
    fn file_get(&mut self, path: String, res_var: String) -> Result<bool, String>{
        let path = self.var_get(path).unwrap();
        match fs::read_to_string(path.clone()) {
            Ok(mut v) => {
                self.var_set(res_var, to_value_str(v.as_mut_str()));
                return Ok(true);
            },
            Err(error) => {
                logger::log(format!("Error while creating dir {}: {}", path, error), logger::LogType::ERROR);
                return Ok(false);
            }
        }
    }
    fn file_exists(&mut self, path: String, res_var: String) -> Result<bool, String>{
        self.dir_exists(path, res_var)
    }
    fn file_del(&mut self, path: String) -> Result<bool, String>{
        let path = self.var_get(path).unwrap();
        match fs::remove_file(path.clone()) {
            Ok(_) => {
                return Ok(true);
            },
            Err(error) => {
                logger::log(format!("Error while creating dir {}: {}", path, error), logger::LogType::ERROR);
                return Ok(false);
            }
        }
    }
    
    
    
    
    
    /*
    fn server_new(name: String, port: String) -> Result<(bool), String>;

    fn client_new(name: String) -> Result<(bool), String>;
    */




    



    ///lexer
    pub fn run(&mut self, code: String) -> Result<bool, String> {

        let mut last_string = String::new();
        let mut el_id = 0;
        let mut line = 0;
        let mut step = step::Step::new();

        let mut cav = false;

        for char in code.chars() {
            if cav {
                last_string.push(char.clone());
                if char == '"' {
                    cav = false;
                }
            } else {
                if char == '"' {
                    last_string.push(char);
                    cav = true;
                } else if char == ' ' {
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
                    self.sys_in(step.args_get(0));
                }
                _ => {}
            },
            "var" => match step.get_command().as_str() {
                "set" => {
                    self.var_set(step.args_get(0), step.args_get(1));
                },
                /* "get" => {
                    self.var_get(step.args_get(0));
                }, */
                "del" => {
                    self.var_del(step.args_get(0));
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "math" => match step.get_command().as_str() {
                "plus" => {
                    self.math_plus(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "min" => {
                    self.math_min(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "mult" => {
                    self.math_mult(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "div" => {
                    self.math_div(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "exp" => {
                    self.math_exp(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "root" => {
                    self.math_root(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "mod" => {
                    self.math_mod(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "floor" => {
                    self.math_floor(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "incr" => {
                    self.math_incr(step.args_get(0));
                },
                "decr" => {
                    self.math_decr(step.args_get(0));
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "str" => match step.get_command().as_str() {
                "plus" => {
                    self.str_plus(step.get_args());
                },
                "cut" => {
                    self.str_cut(step.args_get(0), step.args_get(1), step.args_get(2), step.args_get(3));
                },
                "len" => {
                    self.str_len(step.args_get(0), step.args_get(1));
                },
                "eq" => {
                    self.str_eq(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "dir" => match step.get_command().as_str() {
                "new" => {
                    self.dir_new(step.args_get(0));
                },
                "del" => {
                    self.dir_del(step.args_get(0));
                },
                "exists" => {
                    self.dir_exists(step.args_get(0), step.args_get(1));
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "file" => match step.get_command().as_str() {
                "write" => {
                    self.file_write(step.args_get(0),step.args_get(1));
                },
                "get" => {
                    self.file_get(step.args_get(0),step.args_get(1));
                },
                "del" => {
                    self.file_del(step.args_get(0));
                },
                "exists" => {
                    self.file_exists(step.args_get(0),step.args_get(1));
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "bool" => match step.get_command().as_str() {
                "and" => {
                    self.bool_and(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "or" => {
                    self.bool_or(step.args_get(0), step.args_get(1), step.args_get(2));
                },
                "not" => {
                    self.bool_not(step.args_get(0), step.args_get(1));
                },
                _ => {
                    logger::log(format!("Unknown command: {} in module: {} on line {}", step.get_command(), step.get_module(), step.get_line()), logger::LogType::ERROR);
                }
            },
            "bin" => match step.get_command().as_str() {
                "run" => {
                    self.bin_run(step.args_get(0));
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

/* types */
fn from_value(fragment: String) -> Result<String, String>{
    let fragment = fragment.trim().to_string();
    return Ok(fragment[1..fragment.len() - 1].to_string());
}
fn to_value_str(value: &mut str) -> String {
    format!("\"{}\"",value)
}
fn to_value_num(value: f64) -> String {
    format!("\"{}\"",value)
}
fn to_number_str(str: String) -> f64 {
    let res: Result<f64, ParseFloatError> = str.parse::<f64>();
    match res {
        Ok(_) => res.unwrap(),
        Err(_) => {
            logger::log(format!("Its not a number: {str}"), logger::LogType::ERROR);
            return 0.0;
        }
    }
}
fn is_value(fragment: String) -> Result<bool, String>{
    Ok(fragment.starts_with("\"") && fragment.ends_with("\""))
}
fn is_number(fragment: String) -> Result<bool, String>{
    let re = Regex::new(r"^[\d.]+$").unwrap();
    Ok(re.is_match(fragment.as_str()))
}
fn is_string(fragment: String) -> Result<bool, String>{
    Ok(fragment.starts_with("\"") && fragment.ends_with("\""))
}