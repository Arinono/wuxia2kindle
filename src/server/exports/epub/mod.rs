mod html;
mod styles;

use std::fs::File;

use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};
pub use models::epub::Epub;
use uuid::Uuid;

use self::{
    html::wrap_html,
    styles::{custom_styles, stylesheet},
};

pub struct MyEpub(pub Epub);

impl MyEpub {
    pub fn generate(&self) -> Result<String> {
        let mut builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
        let epub = &self.0;

        builder
            .epub_version(EpubVersion::V30)
            .metadata("title", &epub.title)
            .unwrap()
            .stylesheet(format!("{}\n{}", stylesheet(), custom_styles()).as_bytes())
            .unwrap();

        if let Some(author) = &epub.author {
            builder.metadata("author", author).unwrap();
        }
        if let Some(translator) = &epub.translator {
            builder.metadata("author", translator).unwrap();
        }
        if let Some(cover) = &epub.cover {
            if let Some((mime, rhs)) = cover.split_once(";base64,") {
                // data:image/png
                if let Some((_, mime_type)) = mime.split_once(':') {
                    let bin = general_purpose::STANDARD.decode(rhs).unwrap();
                    builder
                        // book cover for file system
                        .add_cover_image("cover.png", bin.as_slice(), mime_type)
                        .unwrap()
                        // actual cover when opening the epub
                        .add_content(
                            EpubContent::new(
                                "cover.xhtml",
                                wrap_html(r#"<img src="cover.png" />"#.to_string()).as_bytes(),
                            )
                            .title("Cover")
                            .reftype(ReferenceType::Cover),
                        )
                        .unwrap();
                }
            }
        }

        builder
            .add_content(
                EpubContent::new(
                    "title.xhtml",
                    wrap_html(format!("<h1>{}</h1>", epub.title)).as_bytes(),
                )
                .title(&epub.title)
                .reftype(ReferenceType::TitlePage),
            )
            .unwrap()
            .inline_toc();

        for (idx, chapter) in epub.chapters.iter().enumerate() {
            let title = format!("<h2>{}</h2>", chapter.0);
            let chapter_idx = idx + 1;

            builder
                .add_content(
                    EpubContent::new(
                        format!("chapter_{chapter_idx}.xhtml"),
                        wrap_html(format!("{}{}", title, chapter.1)).as_bytes(),
                    )
                    .title(&chapter.0)
                    .reftype(ReferenceType::Text),
                )
                .unwrap();
        }

        let temp_dir = std::env::temp_dir();
        let cleaned_title = epub
            .title
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join("_");
        let filename = format!("{}-{}.epub", cleaned_title, Uuid::new_v4());
        let temp_dir_str = temp_dir
            .display()
            .to_string()
            .trim_end_matches('/')
            .to_string();
        let filepath = format!("{}/{filename}", temp_dir_str);

        let mut fd = File::create(&filepath).unwrap();

        builder.generate(&mut fd).unwrap();

        Ok(filepath)
    }
}
