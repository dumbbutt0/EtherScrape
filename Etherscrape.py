#!/usr/bin/env python3
import argparse
import requests
from bs4 import BeautifulSoup
import os

def scrape_contract_files(address):
    url = f"https://etherscan.io/address/{address}#code"
    headers = {
        "User-Agent": (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
            "AppleWebKit/537.36 (KHTML, like Gecko) "
            "Chrome/90.0.4430.93 Safari/537.36"
        )
    }
    try:
        response = requests.get(url, headers=headers)
        response.raise_for_status()
    except requests.RequestException as e:
        print(f"[Error] Fetching URL {url}: {e}")
        return []
    html = response.text
    soup = BeautifulSoup(html, "lxml")
    file_info_divs = soup.find_all("div", class_="d-flex justify-content-between")
    files = []
    for div in file_info_divs:
        span = div.find("span", class_="text-muted")
        if span and span.get_text(strip=True).startswith("File"):
            text = span.get_text(strip=True)
            try:
                _, file_name = text.split(":", 1)
                file_name = file_name.strip()
            except ValueError:
                continue
            pre_tag = div.find_next("pre")
            if pre_tag:
                code_text = pre_tag.get_text()
                files.append((file_name, code_text))
    return files

def main():
    parser = argparse.ArgumentParser(description="Read or download smart contract files from Etherscan.")
    parser.add_argument("address", help="Ethereum contract address to scrape")
    parser.add_argument("-r", action="store_true", help="Read and output file contents, names, and count in terminal")
    parser.add_argument("-d", help="Download files to the specified directory")
    args = parser.parse_args()

    files = scrape_contract_files(args.address)

    if args.r:
        print(f"Number of files: {len(files)}")
        for file_name, content in files:
            print(f"File: {file_name}")
            print(content)
            print("-" * 80)
    elif args.d:
        os.makedirs(args.d, exist_ok=True)
        for file_name, content in files:
            output_path = os.path.join(args.d, file_name)
            with open(output_path, "w", encoding="utf-8") as f:
                f.write(content)
        print(f"Downloaded {len(files)} files to {args.d}")
    else:
        print("Please specify either -r to read or -d <directory> to download.")

if __name__ == "__main__":
    main()
