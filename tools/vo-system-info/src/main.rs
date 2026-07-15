use std::env;

fn main() {
    println!("VO System Info");
    println!("==============");

    println!("Operating System: {}", env::consts::OS);
    println!("Architecture: {}", env::consts::ARCH);

    println!();
    println!("VO-OS Hardware Advisor foundation");
}
