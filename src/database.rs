pub trait ToSql {
    fn as_sql_string(&self) -> String;
}
