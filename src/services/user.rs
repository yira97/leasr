use super::super::models;
use super::super::schema;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

const USERNAME: &str = "username";

pub fn user_scope(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/join")
      .route(web::get().to(get_user_create_token))
      .route(web::post().to(create_user)),
  );
  cfg.service(web::resource("/{username}").route(web::get().to(get_userinfo)));
  cfg.service(
    web::resource("")
      .name("list-user")
      .route(web::get().to(list_userinfo)),
  );
}

fn query(
  name: String,
  pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<models::User, diesel::result::Error> {
  use self::schema::users::dsl::*;
  let new_user = models::NewUser {
    username: name.as_str(),
  };
  let conn: &PgConnection = &pool.get().unwrap();
  diesel::insert_into(users).values(&new_user).execute(conn)?;
  let mut items = users
    .filter(username.eq(name.as_str()))
    .load::<models::User>(conn)?;
  Ok(items.pop().unwrap())
}

async fn get_user_create_token(
  pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("sdfdsfsdf"))
}

async fn create_user(
  pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || query(String::from("testname"), pool))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

async fn get_userinfo(
  pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let username: String = req.match_info().get(USERNAME).unwrap().parse().unwrap();
  Ok(
    web::block(move || query(String::from(username), pool))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

async fn list_userinfo(
  pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let username: String = req.match_info().get(USERNAME).unwrap().parse().unwrap();
  Ok(
    web::block(move || query(String::from(username), pool))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}
