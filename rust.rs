use std::io::{stdin, stdout, Write};
use std::collections::HashMap;
use std::collections::HashSet;
use std::process;

fn main() {
	let mut signs = HashSet::new();
  signs.insert("+");
  signs.insert("-");
  signs.insert("/");
  signs.insert("*");
	let mut check = false;
  print!("Please input expression = ");
  stdout().flush().unwrap();
  let mut expr = String::new();
  let _ = stdin().read_line(&mut expr);
  let expr = expr.trim();
  let parts = expr.split(" ");
  let mut oper: Vec<String> = Vec::new();
  for part in parts{
    if part.len() > 0{
      if signs.contains(part){
        oper.push(part.to_string());
      } else if check{
        let mut sign_op = oper.pop().unwrap();
        print!("{} {} ", sign_op, part)
      } else {
        print!("{} ", part);
        check = true
      }
    }
  }
}
