use reqwest::blocking::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let response = get("http://api.icndb.com/jokes/random/")?.text()?;
    println!("body = {:?}", response);
    Ok(())
}