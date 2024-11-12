use std::fs::File;
use std::io::{self, Read};
use zip::read::ZipArchive;
use scraper::{Html, Selector};
use chrono::NaiveDateTime;
use std::collections::BTreeMap;

fn main() -> io::Result<()> {
    // Open the zip file
    let file = File::open(std::env::args().nth(1).expect("File argument must be provided"))?;
    let mut archive = ZipArchive::new(file)?;

    // Selectors for the date, content, and labels
    let heading_selector = Selector::parse("div.heading").unwrap();
    let content_selector = Selector::parse("div.content").unwrap();
    let label_selector = Selector::parse("span.label-name").unwrap();

    let mut notes = BTreeMap::new();

    // Iterate through the files in the zip archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        // Check if the file is an HTML file
        if file_name.ends_with(".html") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // Parse the HTML document
            let document = Html::parse_document(&contents);

            // Extract the date and content
            if let Some(heading_element) = document.select(&heading_selector).next() {
                let date_str = heading_element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                
                // Parse the date string
                if let Ok(date) = NaiveDateTime::parse_from_str(&date_str, "%b %d, %Y, %I:%M:%S %p") {
                    if let Some(content_element) = document.select(&content_selector).next() {
                        let content_html = content_element.html();
                        
                        // Convert HTML content to Markdown
                        let mut content_markdown = html2md::parse_html(&content_html);

                        // Replace checkbox symbols
                        content_markdown = content_markdown.replace("☐", "[ ] ").replace("☑", "[x] ");

                        // Extract and format labels
                        let labels: Vec<String> = document.select(&label_selector)
                            .map(|label| format!(" #{}", label.text().collect::<Vec<_>>().join(" ").trim().to_lowercase().replace(" ", "-")))
                            .collect();

                        // Join labels with a separator
                        let labels_str = if !labels.is_empty() {
                            labels.join("")
                        } else {
                            String::new()
                        };

                        // Extract the file name without path and .html extension
                        let file_name_without_extension = file_name.split('/').last().unwrap().strip_suffix(".html").unwrap();

                        // Use a tuple (date, file_name, labels) as the key to ensure uniqueness
                        notes.insert((date, format!("{} - {}", labels_str, file_name_without_extension)), content_markdown);
                    }
                }
            }
        }
    }

    // Print the notes from newest to oldest
    for ((date, file_name), content) in notes.iter().rev() {
        println!("### {}{}", date, file_name);
        println!("{}", content);
    }

    eprintln!("Found {} notes", notes.len());

    Ok(())
}