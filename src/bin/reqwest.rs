use std::error::Error;

use reqwest::blocking::*;

fn main() -> Result<(), Box<dyn Error>> {
    let response = get("http://api.icndb.com/jokes/random/")?.text()?;
    println!("body = {:?}", response);
    Ok(())
}