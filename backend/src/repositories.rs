pub mod users {
    use std::error::Error;

    use crate::models::User;

    pub async fn read(connection: &sqlx::PgPool) -> Result<Vec<User>, Box<dyn Error>> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = 2");
        let users = query.fetch_all(connection).await?;
        Ok(users)
    }
}
