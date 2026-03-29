# forceweb 

> Web Login Brute Forcer — written in Rust

![Rust](https://img.shields.io/badge/Rust-edition_2024-orange?logo=rust)
![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
```
    ______                    _       __     __
   / ____/___  _____________  | |     / /__  / /_
  / /_  / __ \/ ___/ ___/ _ \ | | /| / / _ \/ __ \
 / __/ / /_/ / /  / /__/  __/ | |/ |/ /  __/ /_/ /
/_/    \____/_/   \___/\___/  |__/|__/\___/_.___/
        Web Login Brute Forcer | by buda-sys
```


## What is it?

**forceweb** is a brute force tool for web login forms. It tests passwords from a wordlist against any HTTP/HTTPS endpoint using POST form authentication, detecting success based on a configurable failure string.


## Installation

**Requirements:** [Rust](https://rustup.rs/) installed.
```bash
git clone https://github.com/buda-sys/forceweb
cd forceweb
cargo build --release
```

Binary will be at `target/release/forceweb`.


## Usage
```bash
./forceweb -u <URL> -U <user> -w <wordlist> -f <fail_text> [options]
```

### Arguments

| Flag | Long | Default | Description |
|------|------|---------|-------------|
| `-u` | `--url` | required | Target login endpoint URL |
| `-U` | `--usuario` | required | Username to attack |
| `-w` | `--wordlist` | required | Path to password wordlist file |
| `-f` | `--fail` | required | Text present in the response when login **fails** |
| `-p` | `--param_user` | `username` | HTML field name for the username |
| `-P` | `--param_pass` | `password` | HTML field name for the password |

### Example
```bash
./forceweb \
  -u http://target.local/login \
  -U admin \
  -w /usr/share/wordlists/rockyou.txt \
  -f "Invalid username or password" \
  -p username \
  -P password
```

### Successful output
```
╔══════════════════════════════════════╗
║       PASSWORD FOUND                 ║
╚══════════════════════════════════════╝
[+] URL:      http://target.local/login
[+] User:     admin
[+] Password: letmein123
```

<img width="928" height="472" alt="ng6" src="https://github.com/user-attachments/assets/64f31968-3520-46c5-bd0c-18b9dee52f20" />



## How it works

forceweb sends POST requests to the target endpoint with each password from the wordlist. It detects success by checking whether the HTTP response **does not contain** the string specified in `--fail`.
```
wordlist → POST /login → response contains --fail? → no: password found ✓
                                                    → yes: try next password
```

## ⚠️ Legal Disclaimer

This tool is intended for **ethical pentesting and controlled environments** (CTFs, labs, systems you own). Only use it against systems you have explicit permission to test. The author is not responsible for any misuse.

## License

MIT License — Copyright (c) 2026 **buda-sys**

Permission is granted to use, copy, modify and distribute this software freely,
as long as the original author credit is maintained.

See the [LICENSE](LICENSE) file for details.


## Author

**buda-sys** — built with Rust 🦀
