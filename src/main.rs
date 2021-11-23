use futures::stream::StreamExt;
use tokio_postgres::{types::ToSql, Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=10000 dbname=hang user=postgres password=postgres",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut rows = client
        // NOTE: Issue occurs when LIMIT is 500, but not when LIMIT is 20.
        .query_raw("SELECT * FROM users LIMIT 20", slice_iter(&[]))
        .await?
        .boxed();

    println!("Updating rows...");

    while let Some(row) = rows.next().await {
        let row = row?;
        let id: String = row.get(0);
        client
            .execute("UPDATE users SET done = TRUE WHERE id = $1", &[&id])
            .await?;
        println!("Updated: {}", id);
    }

    println!("Done");

    Ok(())
}

fn slice_iter<'a>(
    s: &'a [&'a (dyn ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
    s.iter().map(|s| *s as _)
}
