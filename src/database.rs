use sqlx::mysql::MySqlPool;




#[tokio::main]
pub async fn insert(cpu_allocated: Vec<String>, pool: &MySqlPool) -> Result<(), sqlx::Error> {

    // Make an insert with CPU utilization
    let _row = sqlx::query("INSERT INTO cpu (time, allocated, idle, other, total)
                            VALUES (NOW(), ?, ?, ?, ?)"
        )
        .bind(&cpu_allocated[0])
        .bind(&cpu_allocated[1])
        .bind(&cpu_allocated[2])
        .bind(&cpu_allocated[3])
        .execute(pool).await?;

    Ok(())
}