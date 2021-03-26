use std::{thread, time};
use sqlx::SqlitePool;

mod sinfo; 




#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
<<<<<<< HEAD
    println!("Username:");
    let mut user = String::new();

    io::stdin()
        .read_line(&mut user)
        .expect("Failed to username");
    
    let user = user.trim();
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let conn_str = format!("mysql://{}:{}@pventmydbcit1.services.brown.edu", user, pass);
    
    println!("{}", conn_str);
    
    let seconds = time::Duration::from_secs(5);
    let pool = MySqlPool::connect(&conn_str).await;

    let pool = pool.unwrap();
=======
>>>>>>> feat-use-sqlite
    
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
<<<<<<< HEAD

=======
>>>>>>> feat-use-sqlite
