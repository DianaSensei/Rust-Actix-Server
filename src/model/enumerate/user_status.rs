use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize};
use std::fmt::{Debug};
use std::io::Write;
use std::str::FromStr;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    AsExpression,
    FromSqlRow,
    EnumString,
    Display,
    EnumCount,
    EnumDiscriminants,
)]
#[sql_type = "Text"]
pub enum UserStatus {
    Inactive,
    Activated,
}

impl ToSql<UserStatus, Pg> for UserStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let _ = out.write_all(self.to_string().as_ref());
        Ok(IsNull::No)
    }
}

impl FromSql<UserStatus, Pg> for UserStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes {
            None => Err("Unrecognized enumerate variant".into()),
            Some(_bytes) => match std::str::from_utf8(_bytes) {
                Ok(str) => deserialize::Result::Ok(UserStatus::from_str(str).unwrap()),
                Err(_e) => Err("Unrecognized enumerate variant".into())
            }
        }
    }
}

