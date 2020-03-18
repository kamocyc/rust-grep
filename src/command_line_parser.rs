use std::env;
use std::collections::HashMap;

//parse command line, and return as hashmap

#[derive(Debug)]
pub struct CommandLineArgument {
  pub arguments: Vec::<String>,
  pub options: HashMap::<String, Option<String>>,
}

fn get_option_name(arg: &str) -> Option<&str> {
  let mut arg_chars = arg.chars();
  if let Some(first_char) = arg_chars.next() {
    if first_char != '-' {
      //argument
      None
    } else if let Some(second_char) = arg_chars.next() {
      if second_char == '-' {
        Some(&arg[2..])
      } else {
        Some(&arg[1..])
      }
    } else {
      None
    }
  } else {
    None
  }
}

impl CommandLineArgument {
  pub fn parse(args_iter: env::Args) -> Result<CommandLineArgument, String> {
    let args: Vec<String> = args_iter.collect();
    
    let mut cl_arg = CommandLineArgument {
      arguments: Vec::new(),
      options: HashMap::new(),
    };
    
    //skip the first element (because it is this program's name)
    //なんとかしてcloneなしでできないのか？ => たぶんindexを静的に区別する方法は無いので無理
    //結局2回コピーしている
    for i in 1..args.len() {
      let curr = &args[i];
      let next = args.get(i + 1);
      
      if let Some(option_name) = get_option_name(&curr) {
        if next == None || get_option_name(next.as_ref().unwrap()) != None {
          cl_arg.options.insert(String::from(option_name), None);
        } else {
          cl_arg.options.insert(String::from(option_name), Some(next.unwrap().clone()));
        }
      } else {
        cl_arg.arguments.push(curr.clone());
      }  
    }
    
    Ok(cl_arg)
  }
}
