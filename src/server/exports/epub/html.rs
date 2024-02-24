pub fn wrap_html(content: String) -> String {
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
