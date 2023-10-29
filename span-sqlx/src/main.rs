use sqlx::sqlite::SqlitePool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite://:memory:").await?;
    create_table(&pool).await?;
    let id = insert_row(&pool).await?;
    let row = select_row(&pool, id).await?.unwrap();
    assert_eq!(row.id, 1);
    assert_eq!(row.name.as_str(), "foo");
    Ok(())
}

async fn create_table(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r"
    CREATE TABLE a
    (
        id int primary key,
        name string
    )
    ",
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn insert_row(pool: &SqlitePool) -> anyhow::Result<i64> {
    let id = sqlx::query("INSERT INTO a (id, name) VALUES (?, ?)")
        .bind(1)
        .bind("foo")
        .execute(pool)
        .await?
        .last_insert_rowid();
    Ok(id)
}

#[derive(Debug, sqlx::FromRow)]
struct Row {
    id: i64,
    name: String,
}

async fn select_row(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Row>> {
    let row = sqlx::query_as("SELECT id, name FROM a WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}
