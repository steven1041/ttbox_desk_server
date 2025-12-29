use serde::{Deserialize, Deserializer, Serialize};
use serde::ser::Serializer;
use salvo::oapi::ToSchema;
use time::PrimitiveDateTime;
use time::OffsetDateTime;

pub fn serialize_primitive_datetime<S>(
    dt: &PrimitiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let offset_dt = OffsetDateTime::new_utc(dt.date(), dt.time());
    serializer.serialize_str(&offset_dt.format(&time::format_description::well_known::Iso8601::DEFAULT).unwrap())
}

pub fn serialize_optional_primitive_datetime<S>(
    dt: &Option<PrimitiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(inner) => {
            let offset_dt = OffsetDateTime::new_utc(inner.date(), inner.time());
            serializer.serialize_str(&offset_dt.format(&time::format_description::well_known::Iso8601::DEFAULT).unwrap())
        }
        None => serializer.serialize_none(),
    }
}

#[allow(dead_code)]
pub fn deserialize_primitive_datetime<'de, D>(
    deserializer: D,
) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let offset_dt = OffsetDateTime::parse(&s, &time::format_description::well_known::Iso8601::DEFAULT)
        .map_err(serde::de::Error::custom)?;
    Ok(PrimitiveDateTime::new(offset_dt.date(), offset_dt.time()))
}

#[allow(dead_code)]
pub fn deserialize_optional_primitive_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<PrimitiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_s = Option::<String>::deserialize(deserializer)?;
    match opt_s {
        Some(s) => {
            let offset_dt = OffsetDateTime::parse(&s, &time::format_description::well_known::Iso8601::DEFAULT)
                .map_err(serde::de::Error::custom)?;
            Ok(Some(PrimitiveDateTime::new(offset_dt.date(), offset_dt.time())))
        }
        None => Ok(None),
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub struct SafeUser {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub is_vip: bool,
    #[serde(serialize_with = "crate::models::serialize_optional_primitive_datetime")]
    pub vip_start_time: Option<time::PrimitiveDateTime>,
    #[serde(serialize_with = "crate::models::serialize_optional_primitive_datetime")]
    pub vip_end_time: Option<time::PrimitiveDateTime>,
    #[serde(default)]
    pub vip_level: i32,
    #[serde(serialize_with = "crate::models::serialize_primitive_datetime")]
    pub created_at: time::PrimitiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_primitive_datetime")]
    pub updated_at: time::PrimitiveDateTime,
}

#[derive(Deserialize, ToSchema, Debug)]
#[allow(dead_code)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema, Debug)]
#[allow(dead_code)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_vip: Option<bool>,
    #[serde(deserialize_with = "crate::models::deserialize_optional_primitive_datetime")]
    pub vip_start_time: Option<time::PrimitiveDateTime>,
    #[serde(deserialize_with = "crate::models::deserialize_optional_primitive_datetime")]
    pub vip_end_time: Option<time::PrimitiveDateTime>,
    pub vip_level: Option<i32>,
}
