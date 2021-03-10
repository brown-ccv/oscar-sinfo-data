use std::{thread, time};
use sqlx::Connection;

mod sinfo; 




#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    
    let seconds = time::Duration::from_secs(5);
    let conn = SqliteConnection::connect("sqlite::oscar.db").await?; 

    loop { 
        let cpu_status = sinfo::cpu_status();

        let _row = sqlx::query("INSERT INTO cpu (time, allocated, idle, other, total)
                               VALUES (NOW(), ?, ?, ?, ?)"
            )
            .bind(&cpu_status[0])
            .bind(&cpu_status[1])
            .bind(&cpu_status[2])
            .bind(&cpu_status[3])
            .execute(&conn).await?;

        thread::sleep(seconds);
    };

    Ok(())
}