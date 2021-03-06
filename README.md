# rustorm


Rustorm is an SQL-centered ORM with focus on ease of use on conversion of database types to
their appropriate rust type.

Selecting records

```rust
#[macro_use]
extern crate rustorm_codegen;
extern crate rustorm_dao as dao;
extern crate rustorm_dao;
extern crate rustorm;
use rustorm::TableName;
use rustorm_dao::ToColumnNames;
use rustorm_dao::ToTableName;
use rustorm_dao::{FromDao, ToDao};
use rustorm::Pool;
use rustorm::DbError;

#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
struct Actor {
    actor_id: i32,
    first_name: String,
}

fn main(){
    let db_url = "postgres://postgres:p0stgr3s@localhost/sakila";
    let mut pool = Pool::new();
    let em = pool.em(db_url).unwrap();
    let sql = "SELECT * FROM actor LIMIT 10";
    let actors: Result<Vec<Actor>, DbError> = em.execute_sql_with_return(sql, &[]);
    println!("Actor: {:#?}", actors);
    let actors = actors.unwrap();
    assert_eq!(actors.len(), 10);
    for actor in actors {
        println!("actor: {:?}", actor);
    }
}
```
Inserting and displaying the inserted records

```rust
#[macro_use]
extern crate rustorm_codegen;
extern crate rustorm_dao as dao;
extern crate rustorm_dao;
extern crate rustorm;
extern crate chrono;

use rustorm::TableName;
use rustorm_dao::ToColumnNames;
use rustorm_dao::ToTableName;
use rustorm_dao::{FromDao, ToDao};
use rustorm::Pool;
use rustorm::DbError;
use chrono::offset::Utc;
use chrono::{DateTime, NaiveDate};

  fn main() {
      mod for_insert {
          use super::*;
          #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
          pub struct Actor {
              pub first_name: String,
              pub last_name: String,
          }
      }

      mod for_retrieve {
          use super::*;
          #[derive(Debug, FromDao, ToColumnNames, ToTableName)]
          pub struct Actor {
              pub actor_id: i32,
              pub first_name: String,
              pub last_name: String,
              pub last_update: DateTime<Utc>,
          }
      }

      let db_url = "postgres://postgres:p0stgr3s@localhost/sakila";
      let mut pool = Pool::new();
      let em = pool.em(db_url).unwrap();
      let tom_cruise = for_insert::Actor {
          first_name: "TOM".into(),
          last_name: "CRUISE".to_string(),
      };
      let tom_hanks = for_insert::Actor {
          first_name: "TOM".into(),
          last_name: "HANKS".to_string(),
      };

      let actors: Result<Vec<for_retrieve::Actor>, DbError> =
          em.insert(&[&tom_cruise, &tom_hanks]);
      println!("Actor: {:#?}", actors);
      assert!(actors.is_ok());
      let actors = actors.unwrap();
      let today = Utc::now().date();
      assert_eq!(tom_cruise.first_name, actors[0].first_name);
      assert_eq!(tom_cruise.last_name, actors[0].last_name);
      assert_eq!(today, actors[0].last_update.date());
      assert_eq!(tom_hanks.first_name, actors[1].first_name);
      assert_eq!(tom_hanks.last_name, actors[1].last_name);
      assert_eq!(today, actors[1].last_update.date());
  }
```
Rustorm is wholly used by [diwata](https://github.com/ivanceras/diwata)


License: MIT
