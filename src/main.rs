use std::process::{Command, Stdio};

fn main() {
    update_using_yay();
    update_flatpak();
}

fn update_using_yay() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("yay -Syu --noconfirm")
        .stdout(Stdio::piped())
        .output()
        .expect("yay -Syu did not work, do you have yay installed?");
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

fn update_flatpak() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("flatpak update -y")
        .stdout(Stdio::piped())
        .output()
        .expect("flatpak update did not work");
    println!("{}",String::from_utf8(output.stdout).unwrap()); 
}
