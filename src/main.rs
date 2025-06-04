fn main() {
    println!("Hello, Yocto World from Rust!");
    println!("Architecture: {}", std::env::consts::ARCH);
    println!("OS: {}", std::env::consts::OS);
}
