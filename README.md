# EtherScrape
**Etherscan scraping tool for Solidity files**  

---

## How It Works
1. Makes a request to an EtherScan-powered website targeting a specific contract address.  
2. Parses HTML to extract smart contract files from the target address.  
3. Saves files under a directory named after the contract name and the last 5 characters of the address.  

---

## Features
- Accepts a single address or a list of addresses to grab all Solidity files in a contract.  
- Works across multiple websites (see list below).  
- Separates and filters addresses for better organization.  
- (**COMING SOON**) Additional advanced features.  

---

## Concepts Practiced & Learned

### Networking Basics
- Making HTTP requests with a custom user agent (`reqwest`).  
- Using CSS selectors to extract specific text from a webpage (`scraper`).  

### System Basics
- Creating new paths and checking if paths already exist (`std::Path`).  
- Creating new files asynchronously (`tokio::fs`).  
- Taking user input from CLI arguments (`std::env`).  

---

## Coming Soon
- Multi-site concurrent requests.  
- Handling HTTP 403 errors.  
- Cloudflare bypass.  
- *Something special…*  

---

## Working & Non-Working Sites

### ✅ Sites That Returned Files (Worked)
- https://bscscan.com/  
- https://etherscan.io/  
- https://optimistic.etherscan.io/  
- https://sepolia-optimism.etherscan.io/  
- https://polygonscan.com/  
- https://sepolia.arbiscan.io/  
- https://arbiscan.io/  
- https://testnet.bscscan.com/ *(sometimes 1 file, sometimes 7 files)*  
- https://nova.arbiscan.io/  
- https://basescan.org/  
- https://zkevm.polygonscan.com/  
- https://lineascan.build/  
- https://sepolia.basescan.org/  
- https://sepolia.etherscan.io/  

### ❌ Sites That Never Returned Files (Always Failed / 403 or Forbidden)
- https://opbnb.bscscan.com/  
- https://celoscan.io/  
- https://testnet.bttcscan.com/  
- https://bttcscan.com/  
- https://moonriver.moonscan.io/  
- https://moonscan.io/  
- https://moonbase.moonscan.io/  
- https://alfajores.celoscan.io/  
- https://gnosisscan.io/  
- https://uniscan.xyz/  
- https://sepolia.blastscan.io/  
- https://scrollscan.com/  
- https://fraxscan.com/  
- https://blastscan.io/  
- https://holesky.fraxscan.com/  
- https://sepolia.scrollscan.com/  


   
   
   
