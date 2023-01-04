#![allow(unused)]
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use home::home_dir;
use std::env;
use std::fs;
use std::io::{stdout, Write};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::string;
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::{Attribute::*, Color::*},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, help = "ドキュメントの参照")]
    read: bool,
    #[arg(short, help = "ドキュメントの作成、編集")]
    write: bool,
    #[arg(short, long, help = "Markdown")]
    markdown: bool,
    file: String,
}

#[derive(Debug)]
struct CustomError(String);

fn vim_open(dir: String) {
    let _ = Command::new("vim")
        .arg(dir)
        .exec();
}

// terminalに表示するスタイル
fn view_area() -> Area {
    let mut area = Area::full_screen();
    // Max横幅
    area.pad_for_max_width(120);
    area
}

fn md_reader(skin: MadSkin, md: &str) -> Result<(), Error> {
    let mut w = stdout(); // we could also have used stderr
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let mut view = MadView::from(md.to_owned(), view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

// md-readerのスタイル
fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

fn init(path: String) {
    Command::new("mkdir").args(["-p", &path]).spawn();
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let home_path = home_dir().unwrap();
    let base_path: String = home_path.to_str().unwrap().to_string() + "/Documents/helpman/";

    // println!("{}", base_path);
    if args.markdown {
        fs::create_dir_all([&base_path, "md"].concat())?;
        let md_path = [&base_path, "md/", &args.file, ".md"].concat();
        // -mdの時
        if args.write {
            // 書き込み
            init([&base_path, "md"].concat());
            vim_open(md_path);
        } else {
            // 読み込み
            let skin = make_skin();
            let md = fs::read_to_string(PathBuf::from(md_path))
                .with_context(|| format!("エラー内容 : {}.md が見つかりません", args.file))?;
            md_reader(skin, &md);
        }
    } else {
        fs::create_dir_all([&base_path, "text"].concat())?;
        let txt_path = [&base_path, "text/", &args.file, ".txt"].concat();
        if args.write {
            init([&base_path, "text"].concat());
            // 書き込み
            vim_open(txt_path);
        } else {
            // 読み込み
            let content = fs::read_to_string(txt_path)
                .with_context(|| format!("エラー内容 : {}.text が見つかりません", args.file))?;

            // println!("{}", content);
        }
    }
    Ok(())
}
