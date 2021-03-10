use std::{thread, time};
use sqlx::SqlitePool;

mod sinfo; 




#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    
    let seconds = time::Duration::from_secs(900);
    let pool = SqlitePool::connect("sqlite:/users/pstey/projects/oscar-sinfo-data/src/sql/oscar.db").await?; 

    loop { 
        let cpu_status = sinfo::cpu_status();

        let _row = sqlx::query("INSERT INTO cpu (time, allocated, idle, other, total)
                               VALUES (DATETIME(), ?, ?, ?, ?)"
            )
            .bind(&cpu_status[0])
            .bind(&cpu_status[1])
            .bind(&cpu_status[2])
            .bind(&cpu_status[3])
            .execute(&pool).await?;

        thread::sleep(seconds);
    };

    Ok(())
}
