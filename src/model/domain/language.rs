use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::AsExpression;
use diesel::FromSqlRow;
use std::io::Write;

#[derive(Debug, Clone, Copy, Default, SqlType)]
#[postgres(type_name = "Language")]
pub struct LanguageType;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow, Clone)]
#[sql_type = "LanguageType"]
pub enum Language {
    En,
    Ru,
    De,
}

impl ToSql<LanguageType, Pg> for Language {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Language::En => out.write_all(b"en")?,
            Language::Ru => out.write_all(b"ru")?,
            Language::De => out.write_all(b"de")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<LanguageType, Pg> for Language {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes {
            None => Err("Unrecognized enumerate variant".into()),
            Some(byte) => match byte {
                b"en" => Ok(Language::En),
                b"ru" => Ok(Language::Ru),
                b"de" => Ok(Language::De),
                _ => Err("Unrecognized enumerate variant".into()),
            },
        }
    }
}
