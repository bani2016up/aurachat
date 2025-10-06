use crate::application::dtos::{AccountDto, CreateAccountDto, UpdateAccountDto};
use crate::application::services::AccountService;
use rocket::serde::json::Json;
use rocket::{State, http::Status};
use rocket::{get, post, put, delete};
use rocket_okapi::openapi;
use std::sync::Arc;

#[openapi(tag = "Accounts")]
#[get("/accounts/<id>")]
pub async fn get_account(
    id: i32,
    service: &State<Arc<AccountService>>,
) -> Result<Json<AccountDto>, Status> {
    service
        .get_account(id)
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(Json)
        .ok_or(Status::NotFound)
}

#[openapi(tag = "Accounts")]
#[post("/accounts", data = "<dto>")]
pub async fn create_account(
    dto: Json<CreateAccountDto>,
    service: &State<Arc<AccountService>>,
) -> Result<Json<AccountDto>, Status> {
    service
        .create_account(dto.into_inner())
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[openapi(tag = "Accounts")]
#[put("/accounts/<id>", data = "<dto>")]
pub async fn update_account(
    id: i32,
    dto: Json<UpdateAccountDto>,
    service: &State<Arc<AccountService>>,
) -> Result<Json<AccountDto>, Status> {
    service
        .update_account(id, dto.into_inner())
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[openapi(tag = "Accounts")]
#[delete("/accounts/<id>")]
pub async fn delete_account(
    id: i32,
    service: &State<Arc<AccountService>>,
) -> Result<Status, Status> {
    service
        .delete_account(id)
        .await
        .map(|_| Status::NoContent)
        .map_err(|_| Status::InternalServerError)
}
