use r2d2;
use r2d2_postgres;
use postgres;
use std::thread;
use std::collections::HashMap;

pub trait ToSql {
    fn as_sql_string(&self) -> String;
}
