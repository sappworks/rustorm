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
use chrono::{DateTime, NaiveDate,NaiveDateTime};
use rustorm::Value;

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
              pub actor_id: i64,
              pub first_name: String,
              pub last_name: String,
              pub last_update: NaiveDateTime,
          }
      }
      let create_sql = "CREATE TABLE actor(
                actor_id integer PRIMARY KEY AUTOINCREMENT,
                first_name text,
                last_name text,
                last_update timestamp DEFAULT current_timestamp
        )";

      let db_url = "sqlite:///tmp/sqlite.db";
      let mut pool = Pool::new();
      let em = pool.em(db_url).unwrap();
      let ret = em.db().execute_sql_with_return(create_sql, &[]);
      println!("ret: {:?}", ret);
      assert!(ret.is_ok());
      let tom_cruise = for_insert::Actor {
          first_name: "TOM".into(),
          last_name: "CRUISE".to_string(),
      };
      let tom_hanks = for_insert::Actor {
          first_name: "TOM".into(),
          last_name: "HANKS".to_string(),
      };
      println!("tom_cruise: {:#?}", tom_cruise);
      println!("tom_hanks: {:#?}", tom_hanks);

     let actors = vec![tom_cruise, tom_hanks];

     for actor in actors{
         let first_name: Value = actor.first_name.into();
         let last_name: Value = actor.last_name.into();
         let ret = em.db().execute_sql_with_return(
            "INSERT INTO actor(first_name, last_name)
            VALUES ($1, $2)", &[&first_name, &last_name]);
          assert!(ret.is_ok());
    }


      let actors: Result<Vec<for_retrieve::Actor>, DbError> =
          em.execute_sql_with_return("SELECT * from actor", &[]);
      println!("Actor: {:#?}", actors);
      assert!(actors.is_ok());
      let actors = actors.unwrap();
      let today = Utc::now().date();
  }
