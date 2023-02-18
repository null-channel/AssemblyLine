use std::collections::HashMap;
use crate::runtime::v1::KeyValue;

#[derive(Debug)]
pub struct RunContainerRequest {
    pub image: String,
    pub cmd: Vec<String>,
    pub args: Vec<String>,
    pub clean_up: CleanUp,
    pub envs: HashMap<String, String>,
}

#[derive(Debug)]
pub enum CleanUp {
    Never,
    After(u64),
    Immediate
}

impl Default for RunContainerRequest {
    fn default() -> Self {
        RunContainerRequest { 
            image: String::default(), 
            cmd: vec![], 
            args: vec![],
            clean_up: CleanUp::Immediate,
            envs: HashMap::new(),
        }
    }
}

impl RunContainerRequest {
   // E0117 stops me from implementing From as KeyValue is generated and HashMap<T,U> is std lib
   pub(crate) fn envs_as_vec(&self) -> Vec<KeyValue> {
        let mut result = vec![];
        self.envs
            .iter()
            .for_each(|(key, value)| result.push(KeyValue { 
                key: String::from(key), 
                value: String::from(value) 
            }));
        result
   }
}
