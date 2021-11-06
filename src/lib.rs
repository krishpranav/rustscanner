mod params;
mod service;
mod async_utils;
mod output;

pub use params::Params;

pub use async_utils::*;

pub use output::*;

pub use service::Core;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const LOGO: &str = r"
RustScanner v1.0.0
A Simple Port Scanner Written In Rust
https://github.com/krishpranav/rustscanner
";


#[cfg(test)]
mod tests {
    #[test]
    fn str_p() {
        let p1 = "90-100".to_string();
        let idx1 = p1.find("-").unwrap();
        println!("{}  {} ", &p1[..idx1], &p1[idx1 + 1..]);
        let p_i = *&p1[..idx1].parse::<i32>().unwrap();
        let p_e = *&p1[idx1 + 1..].parse::<i32>().unwrap();
        for i in p_i..=p_e {
            println!("{}", i);
        }
    }

    #[test]
    fn str_p1() {
        let p1 = "80,443,50,65,56 ,565".to_string();
        let b: Vec<&str> = p1.split(",").collect();
        for i in &b {
            println!("{}", i)
        }
        println!("{:?}", b);
    }


}