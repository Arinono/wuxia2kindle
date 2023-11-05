mod styles;
mod html;

use std::fs::File;

use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};
use uuid::Uuid;

use self::{
    styles::{stylesheet, custom_styles},
    html::wrap_html,
};

#[derive(Debug)]
pub struct Epub {
    pub title: String,
    pub author: Option<String>,
    pub translator: Option<String>,
    pub chapters: Vec<(String, String)>,
    pub cover: Option<String>,
}

impl Epub {
    pub fn generate(&self) -> Result<String> {
        let mut builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();

        builder
            .epub_version(EpubVersion::V30)
            .metadata("title", &self.title)
            .unwrap()
            .stylesheet(format!("{}\n{}", stylesheet(), custom_styles()).as_bytes())
            .unwrap();

        if let Some(author) = &self.author {
            builder.metadata("author", author).unwrap();
        }
        if let Some(translator) = &self.translator {
            builder.metadata("author", translator).unwrap();
        }
        if let Some(cover) = &self.cover {
            if let Some((mime, rhs)) = cover.split_once(";base64,") {
                // data:image/png
                if let Some((_, mime_type)) = mime.split_once(":") {
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
                    wrap_html(format!("<h1>{}</h1>", self.title)).as_bytes(),
                )
                .title(&self.title)
                .reftype(ReferenceType::TitlePage),
            )
            .unwrap()
            .inline_toc();

        for (idx, chapter) in self.chapters.iter().enumerate() {
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
        let filename = format!("{}.epub", Uuid::new_v4());
        let filepath = format!("{}{filename}", temp_dir.display());

        let mut fd = File::create(&filepath).unwrap();

        builder.generate(&mut fd).unwrap();

        Ok(filepath)
    }
}


