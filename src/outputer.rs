

pub struct Outputer {
  
}

impl Outputer {
  pub fn msg(mes: &str) {
    println!("{}", mes);
  }
  pub fn err(mes: &str) {
    eprintln!("{}", mes);
  }
}