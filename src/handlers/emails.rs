use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{postgres::Postgres, redis::Redis, schema};

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::emails)]
struct PartialEmail {
    id: i32,
    subject: Option<String>,
    is_read: Option<bool>,
    received_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::emails)]
struct CompleteEmail {
    id: i32,
    sender: String,
    recipient: String,
    subject: Option<String>,
    body: Option<String>,
    is_read: Option<bool>,
    received_at: Option<NaiveDateTime>,
}

pub async fn list_emails(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::emails::table
        .select(PartialEmail::as_select())
        .load::<PartialEmail>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(emails)) => {
            return HttpResponse::Ok().json(emails);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialEmail>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_email_by_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let result = schema::emails::table
        .select(CompleteEmail::as_select())
        .filter(schema::emails::id.eq(search_id.into_inner()))
        .first::<CompleteEmail>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(email)) => {
            return HttpResponse::Ok().json(email);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_email_by_sender(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_recipient: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::emails::table
        .select(CompleteEmail::as_select())
        .filter(schema::emails::recipient.eq(&search_recipient.into_inner()))
        .first::<CompleteEmail>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(email)) => {
            return HttpResponse::Ok().json(email);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_email_by_recipient(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_recipient: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::emails::table
        .select(CompleteEmail::as_select())
        .filter(schema::emails::recipient.eq(&search_recipient.into_inner()))
        .first::<CompleteEmail>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(email)) => {
            return HttpResponse::Ok().json(email);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn delete_email(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    delete_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let count = diesel::delete(schema::emails::table)
        .filter(schema::emails::id.eq(delete_id.into_inner()))
        .returning(CompleteEmail::as_select())
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}
