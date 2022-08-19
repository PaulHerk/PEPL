use std::fs;
fn main() {
    let file = fs::read_to_string("pkg/pepl.js").unwrap();
    match fs::write("pkg/pepl.js", format!("/* eslint-disable @next/next/no-assign-module-variable */\n{}", file)) {
        Ok(_) => (),
        Err(error) => println!("{:?}", error)
    } // I am a perfectionist so pls let me do this :D (it would still work if I wouldn't)
}
