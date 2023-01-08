use anyhow::{Context, Result};
use std::fs::read_dir;
use std::io::{stdout, BufWriter, Write};

struct CustomError(String);
// list(フォルダ名, 拡張子)で該当の拡張子のファイルを取り出す。
pub fn list_view(dirname: String, extension: &str) -> Result<()> {
    let check_files = read_dir(dirname);
    let files = match check_files {
        Ok(content) => content,
        Err(error) => {
            panic!("フォルダが見つかりません。")
        }
    };
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for entry in files {
        let path = entry.unwrap().path();
        //  拡張子がmdのとき
        if path.extension().is_some() && path.extension().unwrap().to_str().unwrap() == extension {
            writeln!(out, "{}", path.file_stem().unwrap().to_str().unwrap()).unwrap();
        }
    }
    Ok(())
}
