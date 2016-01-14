extern crate rustc_serialize;
extern crate hyper;

use std::env;
use std::io::Read;
use std::io::Error;
use std::string::String;

use hyper::Client;

use rustc_serialize::json::Json;
use rustc_serialize::json;

fn get_definition(definition: &str) -> String{
    let client = Client::new();
    let base_url = "http://api.urbandictionary.com/v0/define?term=".to_string();
    
    let get_url = base_url + definition; 

    let mut res = client.get(&get_url).send().unwrap();

        
    let mut json = String::new();
    res.read_to_string(&mut json).unwrap();
   
    return json;
}

#[derive(RustcDecodable)]
struct Definition{
    author: String,
    current_vote: String,
    defid: i32,
    definition: String,
    example: String,
    permalink: String,
    thumbs_down: i32,
    thumbs_up: i32,
    word: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let mut definition = String::new();
        for i in 1..args.len() {
            definition.push_str(&args[i]);
            definition.push_str(" ");
        }
        let data = get_definition(&definition);
        let json_data = Json::from_str(&data).unwrap();
        let res = json_data.find_path(&["list"]).unwrap().to_string();
        let definitions: Vec<Definition> = json::decode(&res).unwrap();
        
        // do we have definitions?
        if definitions.len() > 0 {
            // is it overly elaborate?
            if definitions[0].definition.len() > 500 {
                // join lines
                println!("{} [...]", definitions[0].definition.split_at(
                    490).0.replace("\r\n", " "));
                println!("Long definition: {}", definitions[0].permalink);
            }else{
                // join lines
                println!("{}", definitions[0].definition.replace("\r\n", " "));
                // show example
                println!("Example: {}",
                    definitions[0].example.replace("\n", " "));
            }
        }else{
            println!("No definitions :/");
        }
    }else{
        println!("Invalid number of arguments");
    }

}


