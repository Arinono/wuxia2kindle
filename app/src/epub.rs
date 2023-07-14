use epub_builder::{EpubBuilder, ZipLibrary};

#[derive(Debug)]
pub struct Epub {
    pub title: String,
    pub chapters: Vec<(String, String)>,
}

impl Epub {
    pub fn create() -> EpubBuilder<ZipLibrary> {
        // creates the epub w/ style, title, toc and chapters
        todo!();
    }

    pub fn write() -> String {
        // finds a place in /tmp (or a known dir) to write file
        // open FD, pass it to builder.generate(&mut fd)?
        todo!();
    }
}
