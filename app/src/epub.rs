use std::fs::File;

use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};
use uuid::Uuid;

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
            let bin = general_purpose::STANDARD.decode(cover).unwrap();
            builder
                // book cover for file system
                .add_cover_image("cover.png", bin.as_slice(), "image/png")
                .unwrap()
                // actual cover when opening the epub
                .add_content(
                    EpubContent::new(
                        "cover.xhtml",
                        wrap_html(format!(r#"<img src="cover.png" />"#)).as_bytes(),
                    )
                    .title("Cover")
                    .reftype(ReferenceType::Cover),
                )
                .unwrap();
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

fn wrap_html(content: String) -> String {
    format!(
        r#"<?xml version='1.0' encoding='utf-8'?>
<html xmlns="http://www.w3.org/1999/xhtml">
    <head>
        <title>Unknown</title>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    </head>
    <body class="calibre">{}</body></html>"#,
        content
    )
}

fn custom_styles() -> &'static str {
    r###"
        #toc + nav {
            display: none;
        }
    "###
}

fn stylesheet() -> &'static str {
    r###"
    /* BB eBooks BoilerPlate EPUB */
    /* Modify as Needed */
    /* NOT SUITABLE for Kindle */
    /* visit us @ http://bbebooksthailand.com/developers.html */

    /* This adds margins around every page to stop ADE's line numbers from being superimposed over content  */
    @page {
      margin: 10px;
    }

    /*===Reset code to prevent cross-reader strangeness===*/
    html,
    body,
    div,
    span,
    applet,
    object,
    iframe,
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    p,
    blockquote,
    pre,
    a,
    abbr,
    acronym,
    address,
    big,
    cite,
    code,
    del,
    dfn,
    em,
    img,
    ins,
    kbd,
    q,
    s,
    samp,
    small,
    strike,
    strong,
    sub,
    sup,
    tt,
    var,
    b,
    u,
    i,
    center,
    fieldset,
    form,
    label,
    legend,
    table,
    caption,
    tbody,
    tfoot,
    thead,
    tr,
    th,
    td,
    article,
    aside,
    canvas,
    details,
    embed,
    figure,
    figcaption,
    footer,
    header,
    hgroup,
    menu,
    nav,
    output,
    ruby,
    section,
    summary,
    time,
    mark,
    audio,
    video {
      margin: 0;
      padding: 0;
      border: 0;
      font-size: 100%;
      vertical-align: baseline;
    }

    table {
      border-collapse: collapse;
      border-spacing: 0;
    }

    ol,
    ul,
    li,
    dl,
    dt,
    dd {
      margin: 0;
      padding: 0;
      border: 0;
      font-size: 100%;
      vertical-align: baseline;
    }

    /*===GENERAL PRESENTATION===*/

    /*===Body Presentation and Margins===*/
    /* Text alignment is still a matter of debate. Feel free to change to text-align: left; */
    body {
      text-align: justify;
      line-height: 120%;
    }

    /*===Headings===*/
    /* After page breaks, eReaders sometimes do not render margins above the content. Adjusting padding-top can help */

    h1 {
      text-indent: 0;
      text-align: center;
      margin: 100px 0 0 0;
      font-size: 2em;
      font-weight: bold;
      page-break-before: always;
      line-height: 150%; /*gets squished otherwise on ADE */
    }

    h2 {
      text-indent: 0;
      text-align: center;
      margin: 50px 0 0 0;
      font-size: 1.5em;
      font-weight: bold;
      page-break-before: always;
      line-height: 135%; /*get squished otherwise on ADE */
    }

    h3 {
      text-indent: 0;
      text-align: left;
      font-size: 1.4em;
      font-weight: bold;
    }

    h4 {
      text-indent: 0;
      text-align: left;
      font-size: 1.2em;
      font-weight: bold;
    }

    h5 {
      text-indent: 0;
      text-align: left;
      font-size: 1.1em;
      font-weight: bold;
    }

    h6 {
      text-indent: 0;
      text-align: left;
      font-size: 1em;
      font-weight: bold;
    }

    /* Hyphen and pagination Fixer */
    /* Note: Do not try on the Kindle, it does not recognize the hyphens property */
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
      -webkit-hyphens: none !important;
      hyphens: none;
      page-break-after: avoid;
      page-break-inside: avoid;
    }

    /*===Paragraph Elements===*/
    /* Margins are usually added on the top, left, and right, but not on the bottom to prevent certain eReaders not collapsing white space properly */

    /*first-line indent paragraph for fiction*/
    p {
      text-indent: 1.25em;
      margin: 0;
      widows: 2;
      orphans: 2;
    }

    /* block type paragraph for non-fiction* /
    /*
    p
    {
    text-indent: 0;
    margin: 1.0em 0 0 0;
    widows: 2;
    orphans: 2;
    }
    */

    /* for centered text and wrappers on images */
    p.centered {
      text-indent: 0;
      margin: 1em 0 0 0;
      text-align: center;
    }

    /* section Breaks (can use centered-style for non-fiction) */
    p.centeredbreak {
      text-indent: 0;
      margin: 1em 0 1em 0;
      text-align: center;
    }

    /* First sentence in chapters following heading */
    p.texttop {
      margin: 1.5em 0 0 0;
      text-indent: 0;
    }

    /* Use for second sentence to clear drop cap's float */
    p.clearit {
      clear: both;
    }

    /* 1st level TOC */
    p.toctext {
      margin: 0 0 0 1.5em;
      text-indent: 0;
    }

    /* 2nd level TOC */
    p.toctext2 {
      margin: 0 0 0 2.5em;
      text-indent: 0;
    }

    /*==LISTS==*/
    ul {
      margin: 1em 0 0 2em;
      text-align: left;
    }

    ol {
      margin: 1em 0 0 2em;
      text-align: left;
    }

    /*===IN-LINE STYLES===*/
    /* Recommend avoiding use of <b>, <i>, and <u>. Use span tags instead */
    span.i {
      font-style: italic;
    }

    span.b {
      font-weight: bold;
    }

    span.u {
      text-decoration: underline;
    }

    span.st {
      text-decoration: line-through;
    }

    /*==in-line combinations==*/
    /* Using something like <span class="i b">... may seem okay, but it causes problems on some eReaders */
    span.ib {
      font-style: italic;
      font-weight: bold;
    }

    span.iu {
      font-style: italic;
      text-decoration: underline;
    }

    span.bu {
      font-weight: bold;
      text-decoration: underline;
    }

    span.ibu {
      font-style: italic;
      font-weight: bold;
      text-decoration: underline;
    }

    /* This fixes the bug where the text-align property of block-level elements is not recognized on iBooks 
     example: html markup would look like <p class="centered"><span class="ipadcenterfix">Centered Content</span></p> */

    span.ipadcenterfix {
      text-align: center;
    }

    /*==IMAGES==*/
    img {
      max-width: 100%;
    }

    /*==TABLES==*/
    table {
      margin: 1em auto;
    }

    tr,
    th,
    td {
      margin: 0;
      padding: 2px;
      border: 1px solid black;
      font-size: 100%;
      vertical-align: baseline;
    }

    /* Superscripted Footnote Text */
    .footnote {
      vertical-align: super;
      font-size: 0.75em;
      text-decoration: none;
    }

    /*==DROP CAPS==*/
    span.dropcap {
      font-size: 300%;
      font-weight: bold;
      height: 1em;
      float: left;
      margin: 0.3em 0.125em -0.4em 0.1em;
    }

    /*==PULL QUOTE==*/
    div.pullquote {
      margin: 2em 2em 0 2em;
      text-align: left;
    }

    div.pullquote p {
      font-weight: bold;
      font-style: italic;
    }

    div.pullquote hr {
      width: 100%;
      margin: 0;
      height: 3px;
      color: #2e8de0;
      background-color: #2e8de0;
      border: 0;
    }

    /*==BLOCK QUOTE==*/
    div.blockquote {
      margin: 1em 1.5em 0 1.5em;
      text-align: left;
      font-size: 0.9em;
    }

    /*==eBook Specific Formatting Below Here==*/
    "###
}
