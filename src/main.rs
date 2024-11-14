use std::io::Read;
use reqwest::blocking::Client;
use reqwest::Url;
use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    url: String,
}

fn main() {
    // let input_url = std::env::args().nth(1).unwrap_or_else(|| {
    //     eprintln!("Usage: {} <url>", std::env::args().nth(0).unwrap());
    //     std::process::exit(1);
    // });

    let args = Opt::from_args();
    let input_url = args.url;
    println!("Requesting URL: {}", input_url);
    println!("Method: GET");
        
    let splits = input_url.split("://").collect::<Vec<&str>>();
    let parsed_url = Url::parse(&input_url);
    match &parsed_url {
        Ok(_) => {
            let base_protocol = splits[0];
            if base_protocol != "http" && base_protocol != "https" {
                eprintln!("Error: The URL does not have a valid base protocol.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            if e.to_string().contains("relative URL without a base") {
                eprintln!("Error: The URL does not have a valid base protocol.");
                std::process::exit(1);
            }
            if e.to_string().contains("invalid IPv6") {
                eprintln!("Error: The URL contains an invalid IPv6 address.");
                std::process::exit(1);
            }
            if e.to_string().contains("invalid IPv4") {
                eprintln!("Error: The URL contains an invalid IPv4 address.");
                std::process::exit(1);
            }
            if e.to_string().contains("invalid port number") {
                eprintln!("Error: The URL contains an invalid port number.");
                std::process::exit(1);
            }
            eprintln!("Unhandled Error: {}", e);
            std::process::exit(1);
        }
    }

    // URL is ok
    let client = Client::new();
    let parsed_url = parsed_url.unwrap();
    let response = client.get(parsed_url).send().map_err(|e| e.to_string());
    match response {
        Ok(response) => println!("Response body:\n{}", response.text().unwrap()),
        Err(e) => eprintln!("Error: {}", e),
    }
    std::process::exit(0);
}
