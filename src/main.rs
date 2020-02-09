#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::NamedFile;
use std::io::Result;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Result<NamedFile> {
    NamedFile::open("site/index.html")
}

#[get("/www/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

fn main() {

    rocket::ignite()
      .mount("/hello", routes![index, files(file: PathBuf)])
      .launch();
}
