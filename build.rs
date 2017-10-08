
extern crate bindgen;

use std::process::{Command,Output};
use std::io;
use std::env;

/// Prints stuff about error
fn handle_err<A: AsRef<str>>(o: io::Result<Output>, cmd: A) -> Output {
    let o = match o {
        Err(e) => {
            println!("{}", cmd.as_ref());
            println!("\tIO Error on exec:\n{:?}",e);
            ::std::process::exit(1);
        }
        Ok(o) => o 
    };
    if !o.status.success() {
        let stderr = String::from_utf8_lossy(o.stderr.as_slice());
        let stdout = String::from_utf8_lossy(o.stdout.as_slice());
        println!("{}", cmd.as_ref());
        match o.status.code() {
            Option::Some(x) => println!("\tExit Code: {:?}", x),
            _ => {}
        };
        println!("\tStdErr:\n {}", stderr);
        println!("\tStdOut:\n {}", stdout);
        ::std::process::exit(1);
    }
    o
}

const BINDGEN_JOBS: &'static [(&'static str, &'static str)] = &[
    ("xed/include/public/xed/xed-interface.h", "src/xed_interface.rs"),
    ("xed/include/public/xed/xed-version.h", "src/xed_version.rs"),
];



/// Autogenerates bindings
fn build_bindings() {
    for job in BINDGEN_JOBS {
        let dot_h = job.0;
        let dot_rs = job.1;
        let bindings = match bindgen::Builder::default()
            .clang_arg("--include-directory=xed/build/obj")
            .clang_arg("--include-directory=xed/include/public/xed")
            .header(dot_h)
            .generate() {
            Ok(x) => x,
            Err(e) => panic!("Could not generate bindings for {}. Error {:?}", dot_h, e)
        };
        match bindings.write_to_file(dot_rs) {
            Ok(_) => {}
            Err(e) => panic!("Could not write generated bindings to {}. Error {:?}", dot_rs, e)
        };
    }
}

/// Build script entry point
fn main() {

   
    // linker directory
    let current_dir = env::current_dir().expect("Could not fetch current directory");
    let lib_dir = {
        let mut x = current_dir.clone();
        x.push("xed");
        x.push("build");
        x.push("obj");
        x
    };


    // Create a directory
    let setup = Command::new("mkdir")
        .arg("-p")
        .arg("xed/build")
        .output();
    handle_err(setup, "Failed to mkdir xed/build");

    // Build the project
    let output = Command::new("../mfile.py")
        .arg("--jobs=8")
        .arg("--silent")
        .arg("--static-stripped")
        .arg("--extra-ccflags=-fPIC")
        .arg("--opt=3")
        .arg("--no-werror")
        .arg("--elf-dwarf")
        .arg("--cc=/usr/bin/clang")
        .arg("--cxx=/usr/bin/clang++")
        .arg("--ar=/usr/bin/ar")
        .arg("--yasm")
        .arg("--linker=/usr/bin/ld")
        .arg("--compiler=clang")
        .current_dir("xed/build")
        .output();
    handle_err(output, "Failed to run `mfile.py`");

    // Configure linker
    println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
    println!("cargo:rustc-link-lib=static=xed");

    // auto generate bindings
    build_bindings();
}
