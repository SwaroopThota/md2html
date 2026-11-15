pub fn md2html(content: &str, file_name: Option<&str>, css: Option<&str>) -> String {
    let html_string = format!(
        r#"
<!DOCTYPE html><html>
    <head>
        <meta charset=\"UTF-8\">
        <style>{}</style>
        <title>{}</title>
    </head>
    <body>{}</body>
</html>
"#,
        css.unwrap_or(default_css()),
        file_name.unwrap_or(default_file_name()),
        md_to_html_body(content)
    );

    html_string
}

fn md_to_html_body(content: &str) -> String {
    let mut html_string = String::new();
    let mut in_code_block = false;
    let mut in_list = false;

    for mut line in content.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        if in_list && !line.starts_with("- ") {
            html_string.push_str("</ul>");
            in_list = false;
        }

        if line.starts_with("```") {
            in_code_block = !in_code_block;
            if in_code_block {
                html_string.push_str("<pre><code>");
            } else {
                html_string.push_str("</code></pre>");
            }
        } else if in_code_block {
            html_string.push_str(line);
            html_string.push('\n');
        } else if line.starts_with("# ") {
            html_string.push_str(&format!("<h1>{}</h1>", &line[2..]));
        } else if line.starts_with("## ") {
            html_string.push_str(&format!("<h2>{}</h2>", &line[3..]));
        } else if line.starts_with("### ") {
            html_string.push_str(&format!("<h3>{}</h3>", &line[4..]));
        } else if line.starts_with("#### ") {
            html_string.push_str(&format!("<h4>{}</h4>", &line[4..]));
        } else if line.starts_with("##### ") {
            html_string.push_str(&format!("<h5>{}</h5>", &line[5..]));
        } else if line.starts_with("###### ") {
            html_string.push_str(&format!("<h6>{}</h6>", &line[6..]));
        } else if line.starts_with("- ") {
            if !in_list {
                html_string.push_str("<ul>");
                in_list = true;
            }
            html_string.push_str(&format!("<li>{}</li>", &line[2..]));
        } else if line.starts_with("---") {
            html_string.push_str("<hr />");
        } else if line.starts_with("> ") {
            html_string.push_str(&format!("<blockquote>{}</blockquote>", &line[2..]));
        } else {
            html_string.push_str(&format!("<p>{}</p>", line));
        }
    }

    if in_list {
        html_string.push_str("</ul>");
    }

    return html_string;
}

fn default_file_name() -> &'static str {
    "HTML Document"
}

fn default_css() -> &'static str {
    r#"
    body {
    font-family: Arial, sans-serif;
    line-height: 1.6;
    margin: 20px;
    padding: 0;
    background-color: #f4f4f4;
    }
    h1, h2, h3, h4, h5, h6 {
        color: #333;
    }
    p {
        color: #666;
    }
    a {
        color: #1a0dab;
        text-decoration: none;
    }
    a:hover {
        text-decoration: underline;
    }
    code {
        background-color: #f4f4f4;
        padding: 2px 4px;
        border-radius: 4px;
    }
    pre {
        background-color: #f4f4f4;
        padding: 10px;
        border-radius: 4px;
        overflow-x: auto;
    }
    blockquote {
        border-left: 4px solid #ccc;
        padding-left: 10px;
        color: #666;
    }
    ul {
        list-style-type: disc;
        margin-left: 20px;
    }
    ul >li {
        margin-bottom: 5px;
    }"#
}
