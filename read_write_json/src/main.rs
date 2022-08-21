use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Memo {
    client_id: String,
    client_secret: String,
    mail_address: String,
    password_with_security_token: String,
    //id: isize,
    body: String,
    star: bool,
}

type Memos = HashMap<String, Memo>;

fn main() {
    write_file("hello".to_string());
    read_file();
}

fn write_file(body: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("memo.json")?;
    println!("file: {:#?}", file);

    let memo = Memo {
        client_id: "client_id".to_string(),
        client_secret: "client_secret".to_string(),
        mail_address: "mail_address".to_string(),
        password_with_security_token: "password_with_security_token".to_string(),
        //id: 100,
        body: body,
        star: true,
    };

    //let mut memos = HashMap::new();
    //memos.insert(memo.id.to_string(), memo);

    let memos_text = serde_json::to_string(&memo).unwrap();
    //let memos_text = serde_json::to_string(&memos).unwrap();
    println!("memos: {}", &memos_text);
    write!(&file, "{}", memos_text)?;

    Ok(())
}

fn read_file() -> std::io::Result<()> {
    let mut memos_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("memo.json")?;
    let reader = BufReader::new(memos_file);
    let memos: Memos = serde_json::from_reader(reader)?;
    println!("memo_text: {:?}", memos);
    Ok(())
}