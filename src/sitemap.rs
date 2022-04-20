use chrono::{DateTime, Datelike, Utc};

pub struct Sitemap {}

impl Sitemap {
    pub fn generate_entry(path: &str, generated_at: &DateTime<Utc>) -> String {
        format!(
            "<url>
        <loc>{}</loc>
        <lastmod>{}-{}-{}</lastmod>
        </url>",
            path,
            generated_at.year(),
            generated_at.month(),
            generated_at.day()
        )
    }
}
