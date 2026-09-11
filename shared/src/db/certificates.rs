use anyhow::Result;
use spin_sdk::sqlite::Connection;

use crate::utils::{blob, int, text};

pub struct CertificateMeta {
    pub id: Option<i64>,
    pub name: String,
    pub issuer: String,
    pub date: String,
    pub category: String,
}

impl CertificateMeta {
    pub fn new(name: String, issuer: String, date: String, category: String) -> Self {
        Self {
            id: None,
            name,
            issuer,
            date,
            category,
        }
    }

    pub fn with_id(id: i64, name: String, issuer: String, date: String, category: String) -> Self {
        Self {
            id: Some(id),
            name,
            issuer,
            date,
            category,
        }
    }
}

pub struct Image {
    pub id: Option<i64>,
    pub data: Vec<u8>,
}

pub fn get_certificates_paginated(
    conn: &Connection,
    page: i64,
    per_page: i64,
) -> Result<Vec<CertificateMeta>> {
    let offset = (page - 1).max(0) * per_page;

    let data = conn.execute(
        "SELECT * FROM p_certificates_meta ORDER BY date DESC LIMIT ? OFFSET ?",
        &[int(per_page), int(offset)],
    )?;

    let mut certificates = Vec::new();

    for row in data.rows() {
        certificates.push(CertificateMeta::with_id(
            row.get::<i64>("id").unwrap(),
            row.get::<&str>("name").unwrap().to_string(),
            row.get::<&str>("issuer").unwrap().to_string(),
            row.get::<&str>("date").unwrap().to_string(),
            row.get::<&str>("category").unwrap().to_string(),
        ));
    }

    Ok(certificates)
}

pub fn count_certificates(conn: &Connection) -> Result<i64> {
    let data = conn.execute("SELECT COUNT(*) as total FROM p_certificates_meta", &[])?;

    Ok(data
        .rows()
        .next()
        .and_then(|r| r.get::<i64>("total"))
        .unwrap_or(0))
}

pub fn get_image(conn: &Connection, id: i64) -> Result<Option<Image>> {
    let data = conn.execute("SELECT * FROM images WHERE id = ?", &[int(id)])?;

    let Some(row) = data.rows().next() else {
        return Ok(None);
    };

    let data = row.get::<&[u8]>("image").unwrap();

    Ok(Some(Image {
        id: Some(id),
        data: data.to_vec(),
    }))
}

pub fn get_categories(conn: &Connection) -> Result<Vec<String>> {
    let data = conn.execute("SELECT DISTINCT category FROM p_certificates_meta", &[])?;
    let categories = data
        .rows()
        .map(|r| r.get::<&str>("category").unwrap().to_owned())
        .collect::<Vec<String>>();
    Ok(categories)
}

pub fn add_certificate(conn: &Connection, cert: CertificateMeta, image: Image) -> Result<i64> {
    conn.execute(
        "INSERT INTO p_certificates_meta (name, issuer, date, category) VALUES (?, ?, ?, ?)",
        &[
            text(cert.name),
            text(cert.issuer),
            text(cert.date),
            text(cert.category),
        ],
    )?;

    let res = conn.execute("SELECT last_insert_rowid()", &[])?;
    let id = res.rows.first().unwrap().get::<i64>(0).unwrap();

    conn.execute(
        "INSERT INTO p_certificates_images (id, image) VALUES (?, ?)",
        &[int(id), blob(image.data)],
    )?;

    Ok(id)
}

pub fn edit_certificate(
    conn: &Connection,
    id: i64,
    cert: Option<CertificateMeta>,
    image: Option<Image>,
) -> Result<()> {
    if let Some(cert) = cert {
        conn.execute(
            "UPDATE p_certificates_meta SET name = ?, issuer = ?, date = ?, category = ? WHERE id = ?",
            &[
                text(cert.name),
                text(cert.issuer),
                text(cert.date),
                text(cert.category),
                int(id),
            ],
        )?;
    }

    if let Some(image) = image {
        conn.execute(
            "UPDATE p_certificates_images SET image = ? WHERE id = ?",
            &[blob(image.data), int(id)],
        )?;
    }

    Ok(())
}

pub fn delete_certificate(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM p_certificates_meta WHERE id = ?", &[int(id)])?;

    Ok(())
}
