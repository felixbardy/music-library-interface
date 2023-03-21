mod tests {
    use crate::db;

    #[test]
    fn init_connection_test() {
        // Just checking it does not panic
        db::init_connection(None);
    }
}
