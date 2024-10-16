use std::collections::HashMap;
use libs::tera::{from_value, to_value, Function as TeraFn, Result, Value};

use chatr::markup::armory;

#[derive(Debug)]
pub struct Gw2Chatlink {
}
impl Gw2Chatlink {
    pub fn new() -> Self {
        Self { }
    }
}
impl TeraFn for Gw2Chatlink {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let code = optional_arg!(
            String,
            args.get("code"),
            "`gw2_chatlink` requires a `code` argument with a string value"
        ).unwrap();

        if let Ok(code) = chatr::ChatCode::build(&code[..]) {
            if let Ok(skill) = chatr::Skill::try_from_chatcode(&code) {
                return Ok(to_value(format!("Skill: {}", skill.id))?);
            }

            if let Ok(build) = chatr::BuildTemplate::try_from_chatcode(&code) {
                return Ok(to_value(armory(build).expect("error with build chatlink"))?);
            }
        }

        Err("unknown chat link type".to_string().into())
    }
}



