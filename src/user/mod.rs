use actix_web::{http::StatusCode, web, web::Data, HttpResponse, Responder};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, span, Instrument, Level};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: String,
    login: String,
    firstname: String,
    lastname: String,
    description: Option<String>,
}

pub struct UserError(u16, String);

#[async_trait]
pub trait UserService {
    async fn find_all(self: &Self) -> Result<Vec<User>, UserError>;
    async fn find_one(self: &Self, login: String) -> Result<Option<User>, UserError>;
}

struct InMemoryUserService(HashMap<String, User>);

#[async_trait]
impl UserService for InMemoryUserService {
    async fn find_all(self: &Self) -> Result<Vec<User>, UserError> {
        debug!("InMemoryUserService.find_all");
        let users: Vec<User> = self.0.values().cloned().collect();
        debug!( users= ?users.len(), "found users!");
        Ok(users)
    }

    async fn find_one(self: &Self, login: String) -> Result<Option<User>, UserError> {
        debug!( ?login ,"finding...");
        let user = self.0.get(&login).map(|u| u.to_owned());
        Ok(user)
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct UserProperties {
    mock: bool,
}

pub fn configure_user_service(user_properties: &UserProperties, cfg: &mut web::ServiceConfig) {
    let user_service = user_service_factory(user_properties);
    do_routing(user_service, cfg)
}

fn do_routing<T>(service: Box<T>, cfg: &mut web::ServiceConfig)
where
    T: UserService + ?Sized + 'static,
{
    let data = Data::new(service);
    cfg.service(
        web::scope("/user")
            .app_data(data.clone())
            .service(web::resource("").to(find_all_users::<T>))
            .service(web::resource("/{login}").to(find_by_login::<T>)),
    );
}

async fn find_all_users<T>(user_service: Data<Box<T>>) -> impl Responder
where
    T: UserService + ?Sized,
{
    // span!(Level::TRACE,"all_users");
    debug!("start reading users");
    info!("all users");
    let users = user_service
        .find_all()
        .instrument(span!(Level::DEBUG, "all_users"))
        .await;
    match users {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => HttpResponse::build(
            StatusCode::from_u16(e.0).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        )
        .json(e.1),
    }
}

#[tracing::instrument(
name = "find user by login",
skip(user_service),
fields(
user_login = ?login,
)
)]
async fn find_by_login<T>(login: web::Path<String>, user_service: Data<Box<T>>) -> impl Responder
where
    T: UserService + ?Sized,
{
    let user = user_service.find_one(login.into_inner()).await;
    match user {
        Ok(o) => match o {
            None => HttpResponse::NotFound().body(""),
            Some(u) => HttpResponse::Ok().json(u),
        },
        Err(e) => HttpResponse::build(
            StatusCode::from_u16(e.0).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        )
        .json(e.1),
    }
}

fn user_service_factory(user_properties: &UserProperties) -> Box<dyn UserService> {
    let _mock = user_properties.mock;

    let mut map = HashMap::new();
    let id1 = "id1".to_string();
    let user1 = User {
        id: id1.to_owned(),
        login: "first".to_string(),
        firstname: "First".to_string(),
        lastname: "Firstly".to_string(),
        description: None,
    };
    map.insert(id1, user1);
    let id2 = "id2".to_string();
    let user2 = User {
        id: id2.to_owned(),
        login: "second".to_string(),
        firstname: "Second".to_string(),
        lastname: "Secondly".to_string(),
        description: Some("this is second user!".to_string()),
    };
    map.insert(id2, user2);
    let service = InMemoryUserService(map);
    Box::new(service)
}
