use std::fs;
mod specs;

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
    let file = fs::read_to_string(file_path)
      .expect("Something went wrong reading the file");

    file
  }

  pub fn parse(&mut self) {
    let file: specs::File = serde_json::from_str(&self.file_content).unwrap();

    self.file = Some(file);
  }

  pub fn content(&self) -> String {
    self.file_content.clone()
  }
}

fn main() {
  let mut compiler = Compiler::new(String::from("files/sum.json"));

  compiler.parse();

  println!("{:?}", compiler.file);
}