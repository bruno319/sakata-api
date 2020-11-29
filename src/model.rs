use std::io::Write;

use diesel::{deserialize, serialize};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::TinyInt;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, FromSqlRow, AsExpression, Clone, Copy)]
#[repr(i8)]
#[sql_type = "TinyInt"]
pub enum Class {
    Unknown = -1,
    Fighter = 1,
    Magician = 2,
    Technician = 3,
    Swordsman = 4,
    Ranger = 5,
    Support = 6,
    Beast = 7,
    Pilot = 8,
    Supernatural = 9,
    Scholar = 10,
    Worker = 11,
    Musician = 12,
    Waifu = 13,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, AsExpression, FromSqlRow, Clone, Copy)]
#[repr(i8)]
#[sql_type = "TinyInt"]
pub enum Genre {
    Unknown = -1,
    Action = 1,
    Adventure = 2,
    SciFi = 3,
    Sports = 4,
    Mystery = 5,
    SliceOfLife = 6,
    Comedy = 7,
    Romance = 8,
    Ecchi = 9,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, AsExpression, FromSqlRow, Clone, Copy)]
#[repr(i8)]
#[sql_type = "TinyInt"]
pub enum Rarity {
    Unknown = -1,
    Silver = 1,
    Gold = 2,
    Epic = 3,
    Legend = 4,
}

impl_tinyint_sql_op!(Class);
impl_tinyint_sql_op!(Genre);
impl_tinyint_sql_op!(Rarity);

impl Default for Class {
    fn default() -> Self {
        Class::Unknown
    }
}

impl Default for Genre {
    fn default() -> Self {
        Genre::Unknown
    }
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::Unknown
    }
}