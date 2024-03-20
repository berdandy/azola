use std::collections::HashMap;
use libs::tera::{from_value, to_value, Function as TeraFn, Result, Value};

use chatr::BuildTemplate;
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
        let build = BuildTemplate::from_string(&code[..]);
        let markup = armory(build).expect("failed to process provided chatlink");

        Ok(to_value(markup).unwrap())
    }
}



