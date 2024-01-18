use std::env;

use lib::{helpers, Labyrinth};
#[derive(Debug)]
struct Config{
    action: String,
    path_from: String,
    path_to: String
}

fn parse_config() -> Result<Config, &'static str>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 4{
        return Err("Invalid args count!");
    }
    if args[1] != "serialize".to_string() && args[1] != "deserialize".to_string(){
        return Err("Invalid action!");
    }
    match helpers::check_file(&args[2]){
        Ok(_) => {}
        Err(_) => return Err("Incorrect input file")
    }
    let config = Config{
        action: args[1].clone(),
        path_from: args[2].clone(),
        path_to: args[3].clone(),
    };
    return Ok(config);
}
fn main() -> Result<(), &'static str> {
    let config: Config;
    let lab: Labyrinth;
    match parse_config(){
        Ok(conf) => config = conf,
        Err(err) => return Err(err),
    };
    if config.action == String::from("serialize"){
        match lib::Labyrinth::from_text_file(&config.path_from) {
            Ok(res) => lab = res,
            Err(err) => return Err(err),
        }
        match lab.to_bin_file(&config.path_to) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
    else{
        match lib::Labyrinth::from_bin_file(&config.path_from) {
            Ok(res) => lab = res,
            Err(err) => return Err(err),
        }
        match lab.to_text_file(&config.path_to) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
