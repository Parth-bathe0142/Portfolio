use std::fmt::Display;

use anyhow::{Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use spin_sdk::sqlite::Connection;

use crate::{db::FromRow, utils::text};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Viewer,
    Admin,
}
impl Role {
    fn from_str(str: &str) -> Result<Self> {
        match str {
            "viewer" => Ok(Self::Viewer),
            "admin" => Ok(Self::Admin),
            _ => bail!("invalid role"),
        }
    }
}
impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Viewer => write!(f, "viewer"),
            Self::Admin => write!(f, "admin"),
        }
    }
}

pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub role: Role,
    pub password: Option<String>,
}

impl User {
    pub fn no_pass(mut self) -> Self {
        self.password = None;
        self
    }
}

impl FromRow for User {
    fn from_row(row: spin_sdk::sqlite::Row) -> Result<Self> {
        Ok(Self {
            id: row.get::<i64>("id"),

            name: row
                .get::<&str>("name")
                .ok_or_else(|| anyhow!("no field 'name' in row"))?
                .to_owned(),

            role: Role::from_str(
                row.get::<&str>("role")
                    .ok_or_else(|| anyhow!("no field 'role' in row"))?,
            )?,

            password: Some(
                row.get::<&str>("password")
                    .ok_or_else(|| anyhow!("no field 'password' in row"))?
                    .to_owned(),
            ),
        })
    }
}

pub fn get_user_by_name(conn: &Connection, name: String) -> Result<Option<User>> {
    let res = conn.execute("SELECT * FROM p_users WHERE name = ?", &[text(name)])?;

    if let Some(row) = res.rows().next() {
        Ok(Some(User::from_row(row)?))
    } else {
        Ok(None)
    }
}

pub fn add_user(user: User, conn: &Connection) -> Result<i64> {
    conn.execute(
        "INSERT INTO p_users (name, role, password) VALUES (?, ?, ?)",
        &[
            text(user.name),
            text(user.role.to_string()),
            text(user.password.ok_or_else(|| anyhow!("missing password"))?),
        ],
    )?;

    let id = conn
        .execute("select last_insert_rowid()", &[])?
        .rows
        .first()
        .ok_or_else(|| anyhow!("error saving user"))?
        .get::<i64>(0)
        .ok_or_else(|| anyhow!("error saving user"))?;

    Ok(id)
}

pub fn get_admin(conn: &Connection) -> Result<Option<User>> {
    let res = conn.execute("SELECT * FROM p_users where role = 'admin'", &[])?;

    if let Some(row) = res.rows().next() {
        Ok(Some(User::from_row(row)?))
    } else {
        Ok(None)
    }
}
