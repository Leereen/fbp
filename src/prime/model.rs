use diesel;

#[derive(diesel::Queryable)]
pub struct DBPrime {
    pub id: i64,
    pub value: i64,
}
