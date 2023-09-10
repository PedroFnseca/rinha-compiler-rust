use std::fs;
mod specs;
use serde_json::Value;

pub struct Compiler {
  file_content: String,
  file: Option<specs::File>,
  functions: Option<specs::Functions>
}

impl Compiler {
  pub fn new(file_path: String) -> Compiler {
    let file_content = Compiler::read_file(file_path.clone());

    Compiler {
      file_content: file_content,
      file: None,
      functions: None
    }
  }

  fn read_file(file_path: String) -> String {
    fs::read_to_string(file_path)
      .expect("Something went wrong reading the file")
  }

  pub fn get_content(&self) -> String {
    self.file_content.clone()
  }

  fn find_key_value_recursive<'a>(
    json: &'a Value,
    target_keys: &[&str],
    results: &mut Vec<(&'a str, &'a Value)>
) {
    match json {
        Value::Object(map) => {
            if let Some(kind_value) = map.get("kind") {
                if let Value::String(kind_str) = kind_value {
                    if target_keys.contains(&kind_str.as_str()) {
                        results.push((kind_str.as_str(), json));
                    }
                }
            }

            for (_, value) in map {
                Self::find_key_value_recursive(value, target_keys, results);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                Self::find_key_value_recursive(item, target_keys, results);
            }
        }
        _ => {}
    }
  }

  fn handler_functions(&mut self) {
    let targets = ["Call", "Function", "Print", "First", "Second"];

    let mut results = Vec::new();

    if let Some(file) = &self.file {
      let file_value = serde_json::to_value(file).unwrap();
      {
        Self::find_key_value_recursive(&file_value, &targets, &mut results);
      }
      let mut functions = specs::Functions {
        calles: Vec::new(),
        functions: Vec::new(),
        prints: Vec::new(),
        firsts: Vec::new(),
        seconds: Vec::new()
      };

      for (kind, value) in results {
        match kind {
          "Call" => {
            let call: specs::Call = serde_json::from_value(value.clone()).unwrap();
            functions.calles.push(call);
          },
          "Function" => {
            let function: specs::Function = serde_json::from_value(value.clone()).unwrap();
            functions.functions.push(function);
          },
          "Print" => {
            let print: specs::Print = serde_json::from_value(value.clone()).unwrap();
            functions.prints.push(print);
          },
          "First" => {
            let first: specs::First = serde_json::from_value(value.clone()).unwrap();
            functions.firsts.push(first);
          },
          "Second" => {
            let second: specs::Second = serde_json::from_value(value.clone()).unwrap();
            functions.seconds.push(second);
          },
          _ => {}
        }
      }

      self.functions = Some(functions);
    }
  }

  pub fn parse(&mut self) {
    let file_content = self.get_content();

    let _file: specs::File = serde_json::from_str(&file_content).unwrap();

    self.file = Some(_file);

    self.handler_functions();
  }
}

fn main() {
  let mut compiler = Compiler::new(String::from("files/sum.json"));

  compiler.parse();
}