use anyhow::{Context, Result};
use lopdf::Document;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
struct Page {
    page_number: u32,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PdfContent {
    file_name: String,
    total_pages: u32,
    pages: Vec<Page>,
}


fn extract_pdf_content<P: AsRef<Path>>(pdf_path: P) -> Result<PdfContent> {
    let pdf_path = pdf_path.as_ref();
    let doc = Document::load(pdf_path)
        .with_context(|| format!("Failed to load PDF: {}", pdf_path.display()))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    let mut page_contents = Vec::new();
    for (&page_num, _) in pages.iter() {
        let content = doc.extract_text(&[page_num])
            .with_context(|| format!("Failed to extract text from page {}", page_num))?;
        
        page_contents.push(Page {
            page_number: page_num,
            content,
        });
    }

    // Sort pages by page number
    page_contents.sort_by_key(|p| p.page_number);

    Ok(PdfContent {
        file_name: pdf_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        total_pages,
        pages: page_contents,
    })
}

fn save_to_json<P: AsRef<Path>>(content: &PdfContent, output_path: P) -> Result<()> {
    let json = serde_json::to_string_pretty(content)?;
    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_pdf_path> <output_json_path>", args[0]);
        std::process::exit(1);
    }

    let pdf_path = &args[1];
    let json_path = &args[2];

    println!("Reading PDF file: {}", pdf_path);
    let content = extract_pdf_content(pdf_path)?;

    println!("Saving content to JSON file: {}", json_path);
    save_to_json(&content, json_path)?;

    println!("Successfully converted PDF to JSON!");
    Ok(())
}
