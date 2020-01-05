use super::schema::users;

#[derive(Serialize, Queryable)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub display_name: Option<String>,
  pub email: Option<String>,
  pub password: Option<String>,
  pub created_at: chrono::NaiveDateTime, // only NaiveDateTime works here due to diesel limitations
  pub updated_at: chrono::NaiveDateTime,
  pub disable_at: chrono::NaiveDateTime,
  pub score: i32,
  pub weight: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub username: &'a str,
}
