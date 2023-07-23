use std::io::Error;

use database::Database;

pub async fn server(_db: Database) -> Result<(), Error> {
    println!("server starting");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), Error> {
        let test_db = Database::new();
        let result = server(test_db).await?;
        assert_eq!(result, ());
        Ok(())
    }
}
