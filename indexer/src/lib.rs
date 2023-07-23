use std::io::Error;

use database::Database;

pub mod manage_entity;

pub async fn indexer(_db: Database) -> Result<(), Error> {
    println!("indexer starting");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), Error> {
        let test_db = Database::new();
        let result = indexer(test_db).await?;
        assert_eq!(result, ());
        Ok(())
    }
}
