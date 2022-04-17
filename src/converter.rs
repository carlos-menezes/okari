use comrak::{markdown_to_html, ComrakOptions};
use yaml_rust::{Yaml, YamlLoader};

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
        {}
    </head>
    <body class='markdown-body'>
        {}
    </body>
</html>",
            Self::generate_head(frontmatter),
            content
        )
    }

    fn generate_head(frontmatter: &Yaml) -> String {
        let title = frontmatter["title"]
            .as_str()
            .expect("`title` tag missing in frontmatter");
        let description = frontmatter["description"]
            .as_str()
            .expect("`description` tag missing in frontmatter");

        format!("
            <title>{title}</title>
            <meta property='og:title' content='{title}' />
            <meta property='og:description' content='{description}' />
            <meta property='og:image' content='https://og.railway.app/api/image?fileType=jpeg&layoutName=Pattern&Pattern=topography&Text={title}&Text+Color=%23ffffff&Foreground=%23808080&Background=%23000000&Opacity=0.5' />", title = title, description = description)
    }
}
