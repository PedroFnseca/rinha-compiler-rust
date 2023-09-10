use std::fs;
mod specs;
use serde_json::Value;

pub struct Compiler {
  file_content: String,
  file: Option<specs::File>
}

impl Compiler {
  pub fn new(file_path: String) -> Compiler {
    let file_content = Compiler::read_file(file_path.clone());

    Compiler {
      file_content: file_content,
      file: None
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

  fn sort_results_by_location_start(&self, results: &mut Vec<(&str, &Value)>) {
    results.sort_by(|a, b| {
        let a_start = a.1.get("location").expect("REASON").get("start");
        let b_start = b.1.get("location").expect("REASON").get("start");

        if let (Some(a_start), Some(b_start)) = (a_start, b_start) {
            if let (Some(a_start), Some(b_start)) = (a_start.as_i64(), b_start.as_i64()) {
                return a_start.cmp(&b_start);
            }
        }

        std::cmp::Ordering::Equal
    });
}

  fn handler_terms(&mut self) {
    let targets = [
      "If",
      "Let",
      "Str",
      "Bool",
      "Int",
      "Binary",
      "Call",
      "Function",
      "Print",
      "First",
      "Second",
      "Tuple",
      "Var"
    ];

    let mut results = Vec::new();

    if let Some(file) = &self.file {
      let file_value = serde_json::to_value(file).unwrap();
      {
        Self::find_key_value_recursive(&file_value, &targets, &mut results);
      }

      self.sort_results_by_location_start(&mut results);
      
      for (kind, value) in results {
        println!("{}: {:?}", kind, value.get("location").expect("REASON").get("start"));
      }
    }
  }

  pub fn parse(&mut self) {
    let file_content = self.get_content();

    let _file: specs::File = serde_json::from_str(&file_content).unwrap();

    self.file = Some(_file);

    self.handler_terms();
  }
}

fn main() {
  let mut compiler = Compiler::new(String::from("files/sum.json"));

  compiler.parse();
}