use std::env;
use std::io::{self, Read};
use std::process;
use regex::Regex;

mod command_line_parser;
use command_line_parser::CommandLineArgument;
mod outputer;
use outputer::Outputer;

fn read_stdin() -> io::Result<String> {
  let mut buffer = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();
  
  handle.read_to_string(&mut buffer)?;
  
  Ok(buffer)
}

fn do_main() -> Result<(), String> {
  let res = CommandLineArgument::parse(env::args()).map_err(|_e| "Parsing command line arguments failed")?;
  if res.arguments.len() != 1 {
    return Err(String::from("A number of arguments should be one."));
  }
  //TODO: raise error if undefined option is specified
  
  //grep [options] pattern
  let stdin = read_stdin().map_err(|_e| "Reading stdin failed")?;
  
  let re = Regex::new(res.arguments.first().unwrap()).map_err(|_e| "Parsing regex failed")?;
  
  let count_of_line_mode = res.options.contains_key("c");
  let mut matched_line_num = 0;
  
  //各行を検索
  let mut line_num = 1;
  for line in stdin.lines() {
    if re.is_match(line) {
      if count_of_line_mode {
        matched_line_num += 1;
      } else {
        Outputer::msg(&(line_num.to_string() + ": " + line));
      }
    }
    
    line_num += 1;
  }
  
  if count_of_line_mode {
    Outputer::msg(&(matched_line_num.to_string() + " line(s) has matched"));
  }
  
  Ok(())
}

fn main() {
  match do_main() {
    Err(e) => {
      Outputer::err(&e);
      process::exit(1);
    }
    _ => process::exit(0),
  }  
}
