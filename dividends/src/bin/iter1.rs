<<<<<<< HEAD
use std::io;
use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use chrono::NaiveDate;

#[derive (Debug)]
pub struct Dividend {

    pub id : i32,
    pub date : NaiveDate,
    pub account : Option<String>,
    pub broker : Option<String>,
    pub name : Option<String>,
    pub country : Option<String>,
    pub amount : Option<i32>
}

#[actix_rt::main] 
async fn main() -> io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    
    let db_pool = PgPool::new(&database_url).await.unwrap();
    
    let dividend_rows = sqlx::query!(r#"select * from dividends
     where Country='Angleterre'"#)
=======


use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Dividend {

    pub id :i32,
    pub date : NaiveDateTime,
    pub account : String,
    pub broker : String,
    pub name : String,
    pub country : String,
    amount : f32
}




#[actix_rt::main]


async fn main() -> io::Result<()> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL non renseignée dans le .env file");
    let db_pool = PgPool::new(&database_url).await.unwrap();

    let dividend_rows = sqlx::query!(
        r#"select * from dividends where id  = $1"#,
        1
    )
>>>>>>> 2d0cff52409aaa4a8460e7b4a712f3a8197e2348
    .fetch_all(&db_pool)
    .await
    .unwrap();

<<<<<<< HEAD
     let mut dividends_list = vec![];
     for dividend_row in dividend_rows {
         dividends_list.push(Dividend {
             id : dividend_row.id,
             date : chrono::NaiveDate::from(dividend_row.date.unwrap()), 
             account : dividend_row.account,
             broker : dividend_row.broker,
             name  : dividend_row.name,
             country : dividend_row.country,
             amount : dividend_row.amount
         })
     }
     println!("Dividendes = {:?}", dividends_list);
     Ok(())
   }



=======
    let mut dividend_list = vec![];

    for dividend_row in dividend_rows {

        dividend_list.push(Dividend{
            id : dividend_row.id,
            date : dividend_row.date,
            account : dividend_row.account,
            broker : dividend_row.broker,
            name : dividend_row.name,
            country : dividend_row.country,
            amount : dividend_row.amount

        })
    }

    println!("Le dividende est {:?}", dividend_list);
    Ok(())
}
>>>>>>> 2d0cff52409aaa4a8460e7b4a712f3a8197e2348
