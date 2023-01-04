#![allow(unused)]
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::{stdout, Write};
use std::os::unix::process::CommandExt;
use std::path::Path;
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

fn vim_open(path:String) {
    let _ = Command::new("view")
            .arg(path)
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

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.markdown {
        let md_path = ["doc/md/", &args.file, ".md"].concat();
        // -mdの時
        if args.write {
            // 書き込み
            vim_open(md_path);
        } else {
            // 読み込み
            let skin = make_skin();
            let md = std::fs::read_to_string(md_path)
                .with_context(|| format!("エラー内容 : {} が見つかりません", args.file))?;
            md_reader(skin, &md);
        }
    } else {
        let txt_path = ["doc/text/", &args.file, ".txt"].concat();
        if args.write {
            // 書き込み
            vim_open(txt_path);
            
        } else {
            // 読み込み
            let content = std::fs::read_to_string(txt_path)
                .with_context(|| format!("エラー内容 : {}.md が見つかりません", args.file))?;

            println!("{}", content);
        }
    }
    Ok(())
}
