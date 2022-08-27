use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use std::io::Write;
use std::str::FromStr;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    AsExpression,
    FromSqlRow,
    EnumString,
    Display,
    EnumCount,
    EnumDiscriminants,
)]
#[diesel(sql_type = Text)]
pub enum UserRole {
    Admin,
    User,
}

impl ToSql<UserRole, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let _ = out.write_all(self.to_string().as_ref());
        Ok(IsNull::No)
    }
}

impl ToSql<Text, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let _ = out.write_all(self.to_string().as_ref());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserRole
{
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match std::str::from_utf8(bytes.as_bytes()) {
            Ok(str) => deserialize::Result::Ok(UserRole::from_str(str).unwrap()),
            Err(_e) => Err("Unrecognized enumerates variant".into()),
        }
    }

    fn from_nullable_sql(bytes: Option<PgValue<'_>>) -> deserialize::Result<Self> {
        match bytes {
            None => Err("Unrecognized enumerates variant".into()),
            Some(_bytes) => match std::str::from_utf8(_bytes.as_bytes()) {
                Ok(str) => deserialize::Result::Ok(UserRole::from_str(str).unwrap()),
                Err(_e) => Err("Unrecognized enumerates variant".into()),
            },
        }
    }
}
