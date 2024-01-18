use std::env;

use lib::{helpers, Labyrinth};
#[derive(Debug)]
struct Config{
    action: String,
    path_from: String,
    path_to: String
}
fn check_config(conf: &Config) -> Result<(), &'static str>{
    let actions: Vec<String> = vec![String::from("serialize"), String::from("deserialize")];
    if !actions.contains(&conf.action){
        return Err("Incorrect action!");
    }
    match helpers::check_file(&conf.path_from){
        Ok(_) => {}
        Err(_) => return Err("Incorrect input file")
    }
    Ok(())
}
fn get_config() -> Result<Config, &'static str>{
    let args: Vec<String> = env::args().collect();
    let config = Config{
        action: args[1].clone(),
        path_from: args[2].clone(),
        path_to: args[3].clone(),
    };

    match check_config(&config) {
        Ok(_) => return Ok(config),
        Err(err) => return Err(err),
    }
}
fn main() -> Result<(), &'static str> {
    let config: Config;
    let lab: Labyrinth;
    match get_config(){
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
