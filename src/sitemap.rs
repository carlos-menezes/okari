use std::{
    fs::File,
    io::{Result, Write},
};

use chrono::{DateTime, Datelike, Utc};

use crate::BASE_URL;

pub struct Sitemap {}

impl Sitemap {
    pub fn new(entries: &Vec<String>) -> Result<()> {
        let mut sitemap_file = File::create("./build/sitemap.xml")?;
        for i in entries {
            sitemap_file.write_all(i.as_bytes())?;
        }
        Ok(())
    }

    pub fn generate_entry(path: &str, generated_at: &DateTime<Utc>) -> String {
        let month = generated_at.month();
        let day = generated_at.day();
        format!(
            "\t<url>\n\t\t<loc>{}{}</loc>\n\t\t<lastmod>{}-{}-{}</lastmod>\n\t</url>",
            BASE_URL,
            path,
            generated_at.year(),
            if month < 10 {
                format!("0{}", month)
            } else {
                format!("{}", month)
            },
            if day < 10 {
                format!("0{}", day)
            } else {
                format!("{}", day)
            },
        )
    }
}
