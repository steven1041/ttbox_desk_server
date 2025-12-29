use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryFilter, QuerySelect, ColumnTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;
use crate::hoops::jwt;

use crate::entities::{prelude::Users, users};
use crate::models::SafeUser;
use crate::{db, empty_ok, json_ok, utils, AppResult, EmptyResult, JsonResult};

#[derive(Template)]
#[template(path = "user_list_page.html")]
pub struct UserListPageTemplate {}

#[derive(Template)]
#[template(path = "user_list_frag.html")]
pub struct UserListFragTemplate {}

#[handler]
pub async fn list_page(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let is_fragment = req.headers().get("X-Fragment-Header");
    if let Some(cookie) = res.cookies().get("jwt_token") {
        let token = cookie.value().to_string();
        if !jwt::decode_token(&token) {
            res.render(Redirect::other("/login"));
        }
    }
    match is_fragment {
        Some(_) => {
            let hello_tmpl = UserListFragTemplate {};
            res.render(Text::Html(hello_tmpl.render().unwrap()));
        }
        None => {
            let hello_tmpl = UserListPageTemplate {};
            res.render(Text::Html(hello_tmpl.render().unwrap()));
        }
    }
    Ok(())
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct CreateInData {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    pub password: String,
}
#[endpoint(tags("users"))]
pub async fn create_user(idata: JsonBody<CreateInData>) -> JsonResult<SafeUser> {
    let CreateInData { email, password } = idata.into_inner();
    let id = Ulid::new().to_string();
    let password = utils::hash_password(&password)?;
    let conn = db::pool();
    let now = time::OffsetDateTime::now_utc();
    let now_primitive = time::PrimitiveDateTime::new(now.date(), now.time());
    let model = users::ActiveModel {
        id: Set(id.clone()),
        email: Set(email.clone()),
        password: Set(password.clone()),
        is_vip: Set(false),
        vip_start_time: Set(None),
        vip_end_time: Set(None),
        vip_level: Set(0),
        created_at: Set(now_primitive),
        updated_at: Set(now_primitive),
    };
    Users::insert(model).exec(conn).await?;

    json_ok(SafeUser {
        id,
        email,
        is_vip: false,
        vip_start_time: None,
        vip_end_time: None,
        vip_level: 0,
        created_at: now_primitive,
        updated_at: now_primitive,
    })
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
struct UpdateInData {
    #[validate(email(message = "Please enter a valid email address"))]
    email: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    password: String,
}
#[endpoint(tags("users"), parameters(("user_id", description = "user id")))]
pub async fn update_user(
    user_id: PathParam<String>,
    idata: JsonBody<UpdateInData>,
) -> JsonResult<SafeUser> {
    let user_id = user_id.into_inner();
    let UpdateInData { email, password } = idata.into_inner();
    let conn = db::pool();

    let Some(user) = Users::find_by_id(user_id).one(conn).await? else {
        return Err(anyhow::anyhow!("User does not exist.").into());
    };
    let mut user: users::ActiveModel = user.into();
    user.email = Set(email.to_owned());
    user.password = Set(utils::hash_password(&password)?);

    let now = time::OffsetDateTime::now_utc();
    let now_primitive = time::PrimitiveDateTime::new(now.date(), now.time());
    user.updated_at = Set(now_primitive);
    let user: users::Model = user.update(conn).await?;
    json_ok(SafeUser {
        id: user.id,
        email: user.email,
        is_vip: user.is_vip,
        vip_start_time: user.vip_start_time,
        vip_end_time: user.vip_end_time,
        vip_level: user.vip_level,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })
}

#[endpoint(tags("users"))]
pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult {
    let user_id = user_id.into_inner();
    let conn = db::pool();
    Users::delete_by_id(user_id).exec(conn).await?;
    empty_ok()
}

#[derive(Debug, Deserialize, Validate, Extractible, ToSchema)]
#[salvo(extract(default_source(from = "query")))]
pub struct UserListQuery {
    pub email: Option<String>,
    #[serde(default = "default_page")]
    pub current_page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 { 1 }
fn default_page_size() -> u64 { 10 }

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub data: Vec<SafeUser>,
    pub total: u64,
    pub current_page: u64,
    pub page_size: u64,
}

#[endpoint(tags("users"))]
pub async fn list_users(query: &mut Request) -> JsonResult<UserListResponse> {
    let query: UserListQuery = query.extract().await?;
    let conn = db::pool();
    
    let mut select = Users::find();
    
    // Apply email filter if provided
    if let Some(email) = query.email.as_ref() {
        select = select.filter(users::Column::Email.contains(email));
    }
    
    // Get total count
    let total = select.clone().count(conn).await?;
    
    // Apply pagination
    let users = select
        .offset(((query.current_page - 1) * query.page_size) as u64)
        .limit(query.page_size)
        .all(conn)
        .await?
        .into_iter()
        .map(|user| SafeUser {
            id: user.id,
            email: user.email,
            is_vip: user.is_vip,
            vip_start_time: user.vip_start_time,
            vip_end_time: user.vip_end_time,
            vip_level: user.vip_level,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
        .collect::<Vec<_>>();
    
    json_ok(UserListResponse {
        data: users,
        total,
        current_page: query.current_page,
        page_size: query.page_size,
    })
}
