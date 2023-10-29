use opentelemetry::{sdk::trace::TracerProvider, trace::TracerProvider as _};
use opentelemetry_stdout as stdout;
use sqlx::sqlite::SqlitePool;
use tracing::warn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let provider = TracerProvider::builder()
        .with_simple_exporter(stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("sqlx-example");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    run().await
}

#[tracing::instrument]
async fn run() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite://:memory:").await?;
    create_table(&pool).await?;
    let id = insert_row(&pool).await?;
    let row = select_row(&pool, id).await?.unwrap();
    warn!(id = row.id);
    warn!(name = row.name);
    Ok(())
}

#[tracing::instrument]
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

#[tracing::instrument]
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

#[tracing::instrument]
async fn select_row(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Row>> {
    let row = sqlx::query_as("SELECT id, name FROM a WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}
