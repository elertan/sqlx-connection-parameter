use sqlx::{Connection, Postgres, PgPool};
use std::env;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::builder().build(&env::var("DATABASE_URL")?).await?;
    let mut conn = pool.acquire().await?;
    log("Hello, world!", &mut conn).await?;

    let mut tx = conn.begin().await?;
    create_user("John Doe", &mut tx).await?;
    tx.commit().await?;

    // log("Bye, world!", &mut conn).await?;

    Ok(())
}

async fn log<C>(message: &str, conn: &mut C) -> Result<(), sqlx::Error>
    where C: Connection<Database = Postgres>
{
    sqlx::query("SELECT 1")
        .execute(conn)
        .await?;

    Ok(())
}

async fn create_user<C>(name: &str, conn: &mut C) -> Result<(), sqlx::Error>
    where C: Connection<Database = Postgres> {
    let mut tx = conn.begin().await?;
    log("Creating user...", &mut tx).await?;

    sqlx::query("SELECT 1")
        .execute(&mut tx)
        .await?;

    log("User created.", &mut tx).await?;
    tx.commit().await?;
    Ok(())
}
