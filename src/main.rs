use error::Error;
use lexopt::prelude::*;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Cursor, Read, Seek, StdoutLock};

mod error;
mod sixel;

fn main() {
    match try_main() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn try_main() -> Result<(), Error> {
    let mut width = None;
    let mut parser = lexopt::Parser::from_env();
    let mut files = vec![];
    while let Some(arg) = parser.next()? {
        match arg {
            Short('w') | Long("width") => {
                width = Some(parser.value()?.parse()?);
            }
            Value(f) => files.push(f),
            Short('h') | Long("help") => {
                help(true);
                std::process::exit(0);
            }
            _ => {
                println!("Unexpected argument: {}", arg.unexpected());
                help(false);
                std::process::exit(1);
            }
        }
    }

    let stdout = stdout();
    let mut stdout = stdout.lock();

    if files.is_empty() {
        let mut buf = vec![];
        stdin().read_to_end(&mut buf).map_err(|error| Error::Io {
            context: "reading image from stdin".into(),
            error,
        })?;
        print_sixel(&mut stdout, width, BufReader::new(Cursor::new(buf)))?;
    }

    for path in files {
        let file = File::open(&path).map_err(|error| Error::Io {
            context: format!("opening {:?}", &path),
            error,
        })?;
        print_sixel(&mut stdout, width, BufReader::new(file))?;
    }
    Ok(())
}

fn print_sixel<R: Read + Seek>(
    stdout: &mut StdoutLock,
    width: Option<u32>,
    input: BufReader<R>,
) -> Result<(), Error> {
    let img = image::io::Reader::new(input)
        .with_guessed_format()
        .unwrap()
        .decode()?;

    sixel::image_to_sixel(width, img, stdout)?;
    Ok(())
}

fn help(include_header: bool) {
    let version = env!("CARGO_PKG_VERSION");
    if include_header {
        println!(
            "pica {}
Simon Persson <simon@flaskpost.me>

Read image files and write them to stdout in sixel format.",
            version
        );
    }
    println!(
        "
USAGE:
    pica [OPTIONS] [<FILE>...]

    If FILE is omitted, pica reads a single image from stdin.

OPTIONS:
    -w, --width <pixels>    Output image width in pixels
    -h, --help              Display this help page"
    );
}
