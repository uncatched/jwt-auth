use super::super::models::user::{NewUser, User};
use super::super::schema::users::dsl::*;
use super::super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, get, post, delete};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_users(db) )
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_user_by_id(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

#[post("/users")]
async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_create_user(db, item))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

#[delete("/users/{id}")]
async fn delete_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_delete_user(db, user_id.into_inner()))
        .await
        .map(|count| HttpResponse::Ok().json(count))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_all_users(db: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

fn db_get_user_by_id(db: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

fn db_create_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn db_delete_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}