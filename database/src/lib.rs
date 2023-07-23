#[derive(Debug, Clone)]
pub struct Database {
    pub connection_string: String,
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection_string: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let db = Database::new();
        assert_eq!(db.connection_string, "");
    }
}
