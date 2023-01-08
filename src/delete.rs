use question::{Answer, Question};
use std::process::Command;
use std::fs;

pub fn delete_folder(path: String) {
    let ans =Question::new("ドキュメントデータを全て消していいですか？")
        .yes_no()
        .until_acceptable()
        .show_defaults()
        .clarification("Please enter either 'yes' or 'no'\n")
        .ask();
    if ans.unwrap() == Answer::YES {
        Command::new("rm").args(["-rf", &path]).spawn();
    }
}

pub fn delete_file(path: String) {
    let ans =Question::new("削除してもよろしいですか？")
    .yes_no()
    .until_acceptable()
    .show_defaults()
    .clarification("Please enter either 'yes' or 'no'\n")
    .ask();
    
    // ファイルを削除
    fs::remove_file(path).unwrap_or_else(|why| {
        println!("Error : {:?}", why.kind());
    });
}