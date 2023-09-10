// reference: https://github.com/aripiprazole/rinha-de-compiler/blob/main/SPECS.md

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Loc {
  start: i32,
  end: i32,
  filename: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
  text: String,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
  name: String,
  expression: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct If {
  kind: String,
  condition: serde_json::Value,
  then: serde_json::Value,
  otherwise: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Let {
  kind: String,
  name: Parameter,
  value: serde_json::Value,
  next: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Str {
  kind: String,
  value: String,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bool {
  kind: String,
  value: bool,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Int {
  kind: String,
  value: i32,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Eq,
  Neq,
  Lt,
  Gt,
  Lte,
  Gte,
  And,
  Or,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binary {
  kind: String,
  op: BinaryOp,
  lhs: serde_json::Value,
  rhs: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Call {
  kind: String,
  callee: serde_json::Value,
  arguments: Vec<serde_json::Value>,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
  kind: String,
  parameters: Vec<Parameter>,
  value: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Print {
  kind: String,
  value: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct First {
  kind: String,
  value: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Second {
  kind: String,
  value: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tuple {
  kind: String,
  first: serde_json::Value,
  second: serde_json::Value,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Var {
  kind: String,
  text: String,
  location: Loc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Functions {
  pub calles: Vec<Call>,
  pub functions: Vec<Function>,
  pub prints: Vec<Print>,
  pub firsts: Vec<First>,
  pub seconds: Vec<Second>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Term {
  If(If),
  Let(Let),
  Str(Str),
  Bool(Bool),
  Int(Int),
  Binary(Binary),
  Call(Call),
  Function(Function),
  Print(Print),
  First(First),
  Second(Second),
  Tuple(Tuple),
  Var(Var),
  None
}