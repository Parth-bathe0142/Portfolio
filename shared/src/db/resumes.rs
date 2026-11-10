use spin_sdk::sqlite::Connection;

use crate::{
    db::FromRow,
    errors::{AppError, Result},
    utils::{blob, int, last_rowid},
};

pub struct Resume {
    pub id: Option<i64>,
    pub created_at: i64,
    pub doc: Vec<u8>,
}
impl Resume {
    pub fn new(doc: Vec<u8>) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: None,
            created_at: now,
            doc,
        }
    }
}
impl FromRow for Resume {
    fn from_row(row: spin_sdk::sqlite::Row) -> Result<Self> {
        Ok(Self {
            id: Some(row.get("id").ok_or(AppError::ISE(
                "failed to read field 'id' from row".into(),
            ))?),
            created_at: row.get("created_at").ok_or(AppError::ISE(
                "failed to read field 'created_at' from row".into(),
            ))?,
            doc: row
                .get::<&[u8]>("doc")
                .ok_or(AppError::ISE(
                    "failed to read field 'doc' from row".into(),
                ))?
                .to_owned(),
        })
    }
}

pub fn get_resume(conn: &Connection) -> Result<Resume> {
    let data = conn
        .execute(
            "SELECT id, doc, created_at FROM p_resumes ORDER BY id DESC LIMIT 1",
            &[],
        )
        .map_err(|_| AppError::ISE("db read error".into()))?;

    if let Some(row) = data.rows().next() {
        Resume::from_row(row)
    } else {
        Err(AppError::NotFound)
    }
}

pub fn set_resume(resume: Resume, conn: &Connection) -> Result<i64> {
    let Resume {
        created_at, doc, ..
    } = resume;

    conn.execute(
        "INSERT INTO p_resumes (created_at, doc) VALUES (?,?)",
        &[int(created_at), blob(doc)],
    )
    .map_err(|_| AppError::ISE("db insert error".into()))?;

    last_rowid(&conn)
}

pub fn delete_old(conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM p_resumes
        WHERE id < (SELECT max(id) FROM p_resumes)",
        &[],
    )
    .map_err(|_| AppError::ISE("db delete error".into()))?;
    Ok(())
}
