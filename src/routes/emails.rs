use actix_web::web;
use crate::handlers::emails::{list_emails, get_email_by_id, get_email_by_sender, get_email_by_recipient, delete_email};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/emails")
            .route("", web::get().to(list_emails))
            .route("/{id}", web::get().to(get_email_by_id))
            .route("/sender/{sender}", web::get().to(get_email_by_sender))
            .route("/recipient/{recipient}", web::get().to(get_email_by_recipient))
            .route("/{id}", web::delete().to(delete_email))
    );
}