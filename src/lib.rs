pub mod api;
pub mod http;
pub mod models;

pub use models::*;

pub fn execute() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
