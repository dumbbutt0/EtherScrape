//EtherScrape: a smart contract scraper made in rust
use std::io;
use std::error::Error;
use scraper::{ElementRef, Html, Selector};


//step 1: learn to make request (https://sepolia.etherscan.io/address/0x69e9041bde787979d6f7e972716f30d38dc799b0#code)
async fn get_page(address:&str) -> Result<String,reqwest::Error>{
    let address =address.trim();
   let url = format!("https://etherscan.io/address/{address}#code");
   let body = reqwest::get(url)
   .await?.text().await?;
    //println!("Test: {body}");
    Ok(body)
}

//step 2: parse html into vector tuples (filename,code)
fn parse_code(html: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    //Parses string int html and targets css selectors to extract
    let docs: Html = Html::parse_document(html);
    //`?` is used in case the parsing fails and returns an Err
    let div_selector = Selector::parse(r#"div.d-flex.justify-content-between"#)?;
    let span_selector = Selector::parse("span.text-muted")?;
    //let pre_selector = Selector::parse("pre")?;

    let mut files = Vec::new();

    //for div in HTML response
    for div in docs.select(&div_selector) {
        if let Some(span) = div.select(&span_selector).next() {
            //collects all text elements and joins them as one big string
            let text = span.text().collect::<String>().trim().to_string();
            //checks if the text starts with "File"
            if text.starts_with("File") {
                /*returns the text before and after modified and strips the file name
                  by extracting the text befor the colon (the file name)*/
                if let Some((_, file_name)) = text.split_once(':') {
                    //`trim()` is used to remove leading and trailing whitespaces
                    let file_name = file_name.trim().to_string();

                    //loop throu div sibling to find all pre tags
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let mut address: String = String::new();
    println!("Enter target address: ");
    std::io::stdin().read_line(&mut address)?;
    
    let page = get_page(&address).await?;
    let files = parse_code(&page)?;
    for (name,code) in files{
        println!("File {name}: {code}")
    }
    Ok(())
}
