pub mod models;
pub mod db;
pub mod filesys;

pub mod schema;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_connection_test() {
        // Just checking it does not panic
        db::init_connection();
    }
}
