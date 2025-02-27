//! Includes `WhitelistMintMode` enum and `EndSettingType` enum

use std::io::Write;

use diesel::{
    deserialize::{self, FromSql},
    not_none,
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    AsExpression, FromSqlRow, SqlType,
};

#[derive(SqlType, Debug, Clone, Copy)]
#[postgres(type_name = "settingtype")]
/// Represents database 'settingtype' type
pub struct SettingType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Clone, Copy)]
#[sql_type = "SettingType"]
/// `EndSettingType` enum in `EndSettings`
pub enum EndSettingType {
    /// EndSettingtype
    /// Date : Enable the use of a date to stop the mint
    Date,
    /// Amount: Enable stopping the mint after a specific amount is minted
    Amount,
}

impl ToSql<SettingType, Pg> for EndSettingType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            EndSettingType::Date => out.write_all(b"Date")?,
            EndSettingType::Amount => out.write_all(b"Amount")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<SettingType, Pg> for EndSettingType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"date" => Ok(EndSettingType::Date),
            b"amount" => Ok(EndSettingType::Amount),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(SqlType, Debug, Clone, Copy)]
#[postgres(type_name = "mode")]
/// Represents database 'mode' type
pub struct Mode;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Clone, Copy)]
#[sql_type = "Mode"]
/// `WhitelistMintMode` enum in `WhitelistSettings`
pub enum WhitelistMintMode {
    /// Whitelist token is burned after the mint
    BurnEveryTime,
    /// Whitelist token is returned to holder
    NeverBurn,
}

impl ToSql<Mode, Pg> for WhitelistMintMode {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            WhitelistMintMode::BurnEveryTime => out.write_all(b"BurnEveryTime")?,
            WhitelistMintMode::NeverBurn => out.write_all(b"NeverBurn")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Mode, Pg> for WhitelistMintMode {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"BurnEveryTime" => Ok(WhitelistMintMode::BurnEveryTime),
            b"NeverBurn" => Ok(WhitelistMintMode::NeverBurn),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(SqlType, Debug, Clone, Copy)]
#[postgres(type_name = "token_standard")]
/// Represents database `token_standard` type
pub struct TokenStandard;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Clone, Copy)]
#[sql_type = "TokenStandard"]
/// `TokenStandard` enum in `Metadata` struct
pub enum TokenStandardEnum {
    /// This is a master edition
    NonFungible,
    /// A token with metadata that can also have attributes, sometimes called Semi Fungible
    FungibleAsset,
    /// A token with simple metadata
    Fungible,
    /// This is a limited edition
    NonFungibleEdition,
}

impl ToSql<TokenStandard, Pg> for TokenStandardEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            TokenStandardEnum::NonFungible => out.write_all(b"NonFungible")?,
            TokenStandardEnum::FungibleAsset => out.write_all(b"FungibleAsset")?,
            TokenStandardEnum::Fungible => out.write_all(b"Fungible")?,
            TokenStandardEnum::NonFungibleEdition => out.write_all(b"NonFungibleEdition")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<TokenStandard, Pg> for TokenStandardEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"NonFungible" => Ok(TokenStandardEnum::NonFungible),
            b"FungibleAsset" => Ok(TokenStandardEnum::FungibleAsset),
            b"Fungible" => Ok(TokenStandardEnum::Fungible),
            b"NonFungibleEdition" => Ok(TokenStandardEnum::NonFungibleEdition),
            _ => Err("invalid enum entry".into()),
        }
    }
}
