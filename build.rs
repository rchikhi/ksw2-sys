use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let num_jobs = env::var("NUM_JOBS").unwrap();
    let c_src_path = Path::new("ksw2");

    Command::new("sed")
        .current_dir(&c_src_path)
        .arg("s/-g -Wall -Wextra -Wc++-compat -O2/ -g -Wall -Wextra -Wc++-compat -O2 -fPIC/g")
        .output()
        .expect("Failed to modify ksw2 makefile.");

   // build the library
    Command::new("make")
        .arg(format!("-j{}", num_jobs))
        .current_dir(&c_src_path)
        .output()
        .expect("Failed to build ksw2.");

    //println!("ls {:?}",String::from_utf8(Command::new("ls").current_dir(&c_src_path).output().unwrap().stdout));

    let files = "cli.o kalloc.o ksw2_gg.o ksw2_gg2.o ksw2_gg2_sse.o ksw2_extz.o ksw2_extz2_sse.o ksw2_extd.o ksw2_extd2_sse.o ksw2_extf2_sse.o ksw2_exts2_sse.o";
    for file in files.split(" ")
    {
        Command::new("cp")
            .arg(file)
            .arg(&out_dir)
            .current_dir(&c_src_path)
            .output()
            .expect("Failed to copy ksw2 object files.");
        }
    
    Command::new("sh")
        .arg("-c")
        .arg("ar rcs libksw2.a *.o")
        .current_dir(&out_dir)
        .output()
        .unwrap();

    // clean up the temporary build files
    Command::new("make")
        .current_dir(&c_src_path)
        .arg("clean")
        .output()
        .expect("Failed to clean up ksw2 build files.");

    // let cargo know that it can find the file in the out directory
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ksw2");
}
