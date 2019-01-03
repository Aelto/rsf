extern crate clap;

use std::fs;
use std::path::Path;

use clap::{App, Arg, SubCommand};

fn touch(filename: &str) {
  let filepath = Path::new(filename);

  if !filepath.exists() {
    match fs::write(filename, "") {
      Ok(_) => println!("file {} created", filename),
      Err(e) => println!("{}", e),
    };
  } else {
    println!("file already exists");
  }
}

fn cat(filename: &str) {
  let filepath = Path::new(filename);

  if filepath.is_file() {
    match fs::read_to_string(filename) {
      Ok(c) => println!("{}", c),
      Err(e) => println!("{}", e),
    };
  } else {
    println!("no such file {}", filename);
  }
}

fn copy(origin: &str, destination: &str) {
  println!("{} {}", origin, destination);

  let origin_path = Path::new(origin);

  if origin_path.is_file() {
    let destination_path = Path::new(destination);

    if destination_path.is_dir() {
      // do something if directory
    }

    if destination_path.is_file() {
      // do something if file
      
    }
  }
  else {
    println!("no such file {}", origin);
  }
}

fn main() {
  let matches = App::new("rsf")
    .version("0.1")
    .author("Aelto <hottou.thibault@gmail.com>")
    .about("rust filesystem utility")
    .subcommand(
      SubCommand::with_name("touch")
        .version("0.1")
        .about("creates an empty file")
        .arg(Arg::with_name("filename").index(1).help("the file name").required(true)),
    )
    .subcommand(
      SubCommand::with_name("cat")
        .version("0.1")
        .about("reads a file's content")
        .arg(Arg::with_name("filename").index(1).help("the file name").required(true))
    )
    .subcommand(
      SubCommand::with_name("cp")
        .version("0.1")
        .about("copy a file's content to another file")
        .arg(Arg::with_name("origin").index(1).help("the origin path").required(true))
        .arg(Arg::with_name("destination").index(2).help("the destination file").required(true))
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("touch") {
    match matches.value_of("filename") {
      Some(v) => touch(v),
      None => println!("{}", matches.usage()),
    }
  };

  if let Some(matches) = matches.subcommand_matches("cat") {
    match matches.value_of("filename") {
      Some(v) => cat(v),
      None => println!("{}", matches.usage()),
    }
  }

  if let Some(matches) = matches.subcommand_matches("cp") {
    if !matches.is_present("origin") || !matches.is_present("destination") {
      println!("{}", matches.usage());

      return;
    }

    let origin = matches.value_of("origin").unwrap();
    let destination = matches.value_of("destination").unwrap();

    copy(origin, destination);
  }
}
