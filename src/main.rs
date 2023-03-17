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
        .output();
    let output = match output {
        Ok(o) => o,
        Err(e) => panic!("I have no idea what the fuck happened you should try again or fix your shit. You can look this up {}", e),
    };
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

fn update_flatpak() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("flatpak update -y")
        .stdout(Stdio::piped())
        .output()
        .expect("Bro fix your shit I could not run this command");
    println!("{}",String::from_utf8(output.stdout).unwrap()); 
}
