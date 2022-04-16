use comrak::{markdown_to_html, ComrakOptions};
use yaml_rust::YamlLoader;

pub struct Converter {}

impl Converter {
    pub fn run(data: &str) -> String {
        let split_plus = data.split_once("+++").unwrap();
        let frontmatter = &YamlLoader::load_from_str(split_plus.0).unwrap()[0];
        let content = markdown_to_html(split_plus.1, &ComrakOptions::default());
        format!(
            "
<!DOCTYPE html>
<html lang='en'>
    <head>
        <title>{}</title>
    </head>
    <body class='markdown-body'>
        {}
    </body>
</html>",
            frontmatter["title"].as_str().unwrap(),
            content
        )
    }
}
