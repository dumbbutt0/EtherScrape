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
- (**COMING SOON**)🤫

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

## Site compatibility

### ✅ Working Sites
| Site | Status |
|------|--------|
| https://bscscan.com/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://etherscan.io/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://optimistic.etherscan.io/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://sepolia-optimism.etherscan.io/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://polygonscan.com/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://sepolia.arbiscan.io/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://arbiscan.io/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://testnet.bscscan.com/ | ![Intermittent](https://img.shields.io/badge/Status-⚡-yellow) |
| https://nova.arbiscan.io/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://basescan.org/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://zkevm.polygonscan.com/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://lineascan.build/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://sepolia.basescan.org/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |
| https://sepolia.etherscan.io/ | ![Working](https://img.shields.io/badge/Status-✅-brightgreen) |

### ❌ Non-Working Sites
| Site | Status |
|------|--------|
| https://opbnb.bscscan.com/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://celoscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://testnet.bttcscan.com/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://bttcscan.com/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://moonriver.moonscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://moonscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://moonbase.moonscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://alfajores.celoscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://gnosisscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://uniscan.xyz/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://sepolia.blastscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://scrollscan.com/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://fraxscan.com/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://blastscan.io/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://holesky.fraxscan.com/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
| https://sepolia.scrollscan.com/ | ![Fail](https://img.shields.io/badge/Status-❌-red) |
