use std::io::prelude::*;
use std::process::{Command, Stdio};


pub fn cpu_status() -> Vec<String> {
    let process = match Command::new("/usr/local/bin/sinfo")
                                .arg("--all")
                                .arg("--format=%C")
                                .arg("--noheader")
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("Couldn't spawn sinfo: {}", why),
        Ok(process) => process,
    };

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read sinfo stdout: {}", why),
        Ok(_) => (),
    };
    // The String `s` will be something of the form: "8446/5042/72/13560"  

    let cpu_nums: Vec<String> = s.split('/').map(|x| String::from(x)).collect();

    cpu_nums
}
