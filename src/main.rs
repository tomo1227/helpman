#![allow(unused)]
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use home::home_dir;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{stdout, BufWriter, Write};
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

mod list;
use list::list_view;

mod init;
use init::{init, init_all};

mod path;
use path::{reset_base_path, set_base_path};

mod delete;
use delete::delete_folder;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, help = "ドキュメントの参照")]
    read: bool,
    #[arg(short, help = "ドキュメントの作成、編集")]
    write: bool,
    #[arg(short, long, help = "Markdown")]
    markdown: bool,
    file: Option<String>,
    #[arg(short, long, help = "ドキュメントリストの表示")]
    list: bool,
    #[arg(short = 'S', help = "パスのセット", value_name = "PATH")]
    set_path: Option<String>,
    #[arg(short = 'R', help = "パスのリセット")]
    reset_path: bool,
    #[arg(short = 'v', help = "現在のパスを表示")]
    view_path: bool,
    #[arg(short, long, help = "helpman初期化")]
    init: bool,
    #[arg(short, long, help = "ドキュメントデータの削除")]
    delete: bool,
}

#[derive(Debug)]
struct CustomError(String);

fn vim_open(dir: String) {
    let _ = Command::new("vi").arg(dir).exec();
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
    let home_path = home_dir().unwrap();
    let default_base_path: String = home_path.to_str().unwrap().to_string() + "/Documents/";
    dotenv().ok();
    let base_path = [
        &env::var("BASE_PATH").unwrap_or(default_base_path.clone()),
        "helpman/",
    ]
    .concat();

    // -mdの時
    if args.init {
        init_all(base_path)
    } else if args.set_path.is_some() {
        set_base_path(args.set_path.unwrap());
    } else if args.reset_path {
        reset_base_path();
    } else if args.view_path {
        println!("{}", &env::var("BASE_PATH").unwrap_or(default_base_path));
    } else if args.delete {
        delete_folder(base_path)
    } else if args.markdown {
        if args.list {
            let dirname = [&base_path, "md"].concat();
            list_view(dirname, "md");
        } else {
            fs::create_dir_all([&base_path, "md"].concat())?;
            let md_path = [&base_path, "md/", &args.file.as_ref().unwrap(), ".md"].concat();
            if args.write {
                // 書き込み
                // init([&base_path, "md"].concat());
                vim_open(md_path);
            } else {
                // 読み込み
                let skin = make_skin();
                let md = fs::read_to_string(PathBuf::from(md_path)).with_context(|| {
                    format!("エラー内容 : {}.md が見つかりません", &args.file.unwrap())
                })?;
                md_reader(skin, &md);
            }
        }
    // -mdないとき
    } else {
        // -l argのFileなし
        if args.list {
            // let dirname = [&base_path, "text"].concat();
            // let files = fs::read_dir(dirname).unwrap();
            // let out = stdout();
            // let mut out = BufWriter::new(out.lock());
            // for entry in files {
            //     let path = entry.unwrap().path();
            //     if (path.extension().unwrap().to_str().unwrap() == "txt") {
            //         writeln!(out, "{}", path.file_stem().unwrap().to_str().unwrap()).unwrap();
            //     }
            // }
            let dirname = [&base_path, "txt"].concat();
            list_view(dirname, "txt");
        } else {
            fs::create_dir_all([&base_path, "text"].concat())?;
            let txt_path = [&base_path, "text/", &args.file.as_ref().unwrap(), ".txt"].concat();
            // -w　書き込み
            if args.write {
                init([&base_path, "text"].concat());
                vim_open(txt_path);
            } else {
                // -r 読み込み
                let content = fs::read_to_string(txt_path).with_context(|| {
                    format!("エラー内容 : {}.text が見つかりません", args.file.unwrap())
                })?;
                // println!("{}", content);
            }
        }
    }
    Ok(())
}
