use question::{Answer, Question};
use std::process::Command;

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
