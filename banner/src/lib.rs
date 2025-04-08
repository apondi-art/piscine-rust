use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let short_hand = format!("-{}", name.chars().next().unwrap());
        let long_hand = format!("--{}", name);
        Flag {
            short_hand,
            long_hand,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        match self.flags.get(input) {
            Some(&callback) => {
                if argv.len() < 2 {
                    return Err("not enough arguments".to_string());
                }
                callback(argv[0], argv[1]).map_err(|e| e.to_string())
            }
            None => Err("flag not found".to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num = a.parse::<f64>()?;
    let b_num = b.parse::<f64>()?;
    Ok((a_num / b_num).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num = a.parse::<f64>()?;
    let b_num = b.parse::<f64>()?;
    Ok((a_num % b_num).to_string())
}