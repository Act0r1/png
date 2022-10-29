mod chunk_type;
mod chunk;
mod err;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;



fn main() {
    println!("Hello, world!");
}
