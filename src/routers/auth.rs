use cookie::Cookie;
use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::Users, users};
use crate::hoops::jwt;
use crate::{db, json_ok, utils, AppResult, JsonResult};

#[handler]
pub async fn login_page(res: &mut Response) -> AppResult<()> {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct LoginTemplate {}
    if let Some(cookie) = res.cookies().get("jwt_token") {
        let token = cookie.value().to_string();
        if jwt::decode_token(&token) {
            res.render(Redirect::other("/users"));
            return Ok(());
        }
    }
    let hello_tmpl = LoginTemplate {};
    res.render(Text::Html(hello_tmpl.render().unwrap()));
    Ok(())
}

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct LoginInData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct LoginOutData {
    pub id: String,
    pub email: String,
    pub is_vip: bool,
    #[serde(serialize_with = "crate::models::serialize_optional_primitive_datetime")]
    pub vip_start_time: Option<time::PrimitiveDateTime>,
    #[serde(serialize_with = "crate::models::serialize_optional_primitive_datetime")]
    pub vip_end_time: Option<time::PrimitiveDateTime>,
    pub vip_level: i32,
    #[serde(serialize_with = "crate::models::serialize_primitive_datetime")]
    pub created_at: time::PrimitiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_primitive_datetime")]
    pub updated_at: time::PrimitiveDateTime,
    pub token: String,
    pub exp: i64,
}

#[endpoint(tags("auth"))]
pub async fn post_login(
    idata: JsonBody<LoginInData>,
    res: &mut Response,
) -> JsonResult<LoginOutData> {
    let idata = idata.into_inner();
    let conn = db::pool();
    let Some(user) = Users::find()
        .filter(users::Column::Email.eq(idata.email))
        .one(conn)
        .await?
    else {
        return Err(StatusError::unauthorized()
            .brief("User does not exist.")
            .into());
    };

    if utils::verify_password(&idata.password, &user.password).is_err()
    {
        return Err(StatusError::unauthorized()
            .brief("Account not exist or password is incorrect.")
            .into());
    }

    let (token, exp) = jwt::get_token(&user.id)?;
    let odata = LoginOutData {
        id: user.id,
        email: user.email,
        is_vip: user.is_vip,
        vip_start_time: user.vip_start_time,
        vip_end_time: user.vip_end_time,
        vip_level: user.vip_level,
        created_at: user.created_at,
        updated_at: user.updated_at,
        token,
        exp,
    };
    let cookie = Cookie::build(("jwt_token", odata.token.clone()))
        .path("/")
        .http_only(true)
        .build();
    res.add_cookie(cookie);
    json_ok(odata)
}
