use std::process::Command;

pub fn init(path:String) {
    Command::new("mkdir").args(["-p", &path]).spawn();
}

pub fn init_all(path:String) {
    let txt_path = [&path, "text"].concat();
    let md_path = [&path, "md"].concat();
    Command::new("mkdir").args(["-p", &txt_path]).spawn();
    Command::new("mkdir").args(["-p", &md_path]).spawn();
}
