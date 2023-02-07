extern crate rot13;

use std::io;
use std::io::Cursor;
use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
 
#[tokio::main]
async fn main() {
    fetch_url("https://raw.githubusercontent.com/WesleyWong420/FSEC-SS-Sharing-Session/main/flag.txt".to_string(), "flag.txt".to_string()).await.unwrap();
    
    let mut cipher = String::new();
    cipher = read();

    let mut input = String::new();
    println!("Enter flag:");
    io::stdin().read_line(&mut input).expect("Action Failed!");

    let mut encrypted = String::new();
    encrypted = rot13::rot13(&input);

    check_flag(encrypted, cipher);
}

fn check_flag(input: String, remote: String){
    if input.trim() == remote.trim() {
        println!("Correct!")
    } else {
        println!("Wrong!")
    }
}

fn read() -> String {
    let mut file = File::open("flag.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}