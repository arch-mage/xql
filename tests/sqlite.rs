use sqlx::Pool;
use sqlx::Sqlite;

use sqlx::Row;
use xql::blanket::ExprExt;
use xql::blanket::StmtExt;
use xql::select;

#[tokio::test]
async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::<Sqlite>::connect("sqlite::memory:").await?;

    let query = select([1.alias("one"), 2.alias("two"), 3.alias("three")]);

    let result = query.fetch_one(&pool).await?;
    let one: i32 = result.try_get("one")?;
    let two: i32 = result.try_get("two")?;
    let three: i32 = result.try_get("three")?;
    assert_eq!(one, 1);
    assert_eq!(two, 2);
    assert_eq!(three, 3);
    Ok(())
}
