//EtherScrape: a smart contract scraper made in rust
use std::{path::Path};
use std::error::Error;
use reqwest;
use scraper::{ElementRef, Html, Selector};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::process;
use std::env;


//step 1: learn to make request (https://sepolia.etherscan.io/address/0x69e9041bde787979d6f7e972716f30d38dc799b0#code)
async fn get_page(address:&str) -> Result<String,reqwest::Error>{
    let address = address.trim();
    let url = format!("https://etherscan.io/address/{address}#code");
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

//step 2: parse html into vector tuples (filename,code)
fn parse_code(html: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let docs: Html = Html::parse_document(html);
    let div_selector = Selector::parse(r#"div.d-flex.justify-content-between"#)?;
    let span_selector = Selector::parse("span.text-muted")?;

    let mut files = Vec::new();

    for div in docs.select(&div_selector) {
        if let Some(span) = div.select(&span_selector).next() {
            let text = span.text().collect::<String>().trim().to_string();
            if text.starts_with("File") {
                if let Some((_, file_name)) = text.split_once(':') {
                    let file_name = file_name.trim().to_string();

                    let mut siblings = div.next_siblings();
                    while let Some(sib) = siblings.next() {
                        if let Some(pre_elem) = ElementRef::wrap(sib) {
                            if pre_elem.value().name() == "pre" {
                                let codes = pre_elem.text().collect::<String>();
                                files.push((file_name, codes));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(files)
}

async fn save_file(dir: &str, files: &[(String, String)]) -> Result<(), Box<dyn Error>> {
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir).await?;
    }
    for (filename, code) in files {
        let path = Path::new(dir).join(filename);
        let mut file = fs::File::create(&path).await?;
        file.write_all(code.as_bytes()).await?;
        println!("Saved: {}", path.display());
    }
    Ok(())
}

static  HELP_DOCS:&str =
    "\n\
     ==========================\n\
     EtherScrape Help (Follow @dumbbutt0 on X)\n\
     ==========================\n\
     Usage:\n\
         etherscrape.rs <input> [output_base_dir]\n\
     \n\
     Arguments:\n\
         <input>                If this is exactly 42 characters and starts with '0x', it is treated as a single Ethereum address to scrape.\n\
                                Otherwise, it is treated as the path to a text file containing one Ethereum address per line.\n\
         [output_base_dir]      (Optional) Base directory to save scraped contracts.\n\
                               Defaults to 'Contracts'.\n\
     \n\
     Description:\n\
         This tool scrapes smart contract source code from Etherscan for each address\n\
         (either a single one or those listed in the input file), then saves the source files to separate directories.\n\
         Each directory is named as the output base directory appended with the last 5\n\
         characters of the address to ensure uniqueness.\n\
     \n\
     Example (multiple addresses from file):\n\
         etherscrape.rs addresses.txt ./MyContracts\n\
     \n\
     Example (single address):\n\
         etherscrape.rs 0x69e9041bde787979d6f7e972716f30d38dc799b0 ./MyContracts\n\
     \n\
     Notes:\n\
         - Ensure the input file is properly formatted with one address per line.\n\
         - The output directories will be created automatically if they do not exist.\n\
     ==========================\n";


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{HELP_DOCS}");
        process::exit(1);
    }

    if args[1] == "-h" || args[1] == "h" {
        println!("{HELP_DOCS}");
        process::exit(0);
    }

    let input = &args[1];
    let single = input.len() == 42 && input.starts_with("0x");


    let base_dir = if args.len() > 2 {
        args[2].clone()
    } else {
        String::from("Contracts")
    };

    // Read addresses from file, one per line
    let addresses: Vec<String> = if single{
        vec![input.clone()]
    }else{
        let file_path = input;
        let content =tokio::fs::read_to_string(file_path).await?;
        content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
    };

    for addy in addresses {
        println!("Scraping address: {}", &addy);

        let page = match get_page(&addy).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to fetch {}: {}", &addy, e);
                continue;
            }
        };

        let files = match parse_code(&page) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to parse {}: {}", &addy, e);
                continue;
            }
        };

        // Use last 5 chars of address or full if shorter
        let suffix = if addy.len() > 5 {
            &addy[addy.len() - 5..]
        } else {
            &addy
        };

        let out_dir = format!("{}_{}", base_dir, suffix);

        if let Err(e) = save_file(&out_dir, &files).await {
            eprintln!("Failed to save files for {}: {}", &addy, e);
        }
    }

    Ok(())
}
