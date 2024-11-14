use std::collections::HashMap;
use std::io::Read;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The URL to request
    url: String,

    /// The HTTP method to use (GET or POST)
    #[structopt(short = "X", long, default_value = "GET")]
    method: String,

    /// The data to send in a POST request
    #[structopt(short = "d", long)]
    data: Option<String>,

    /// The JSON data to send in a POST request
    #[structopt(long)]
    json: Option<String>,
}

fn main() {
    let args = Opt::from_args();

    let input_url = args.url;
    if args.data.is_some() && args.json.is_some() {
        eprintln!("Error: The -d and --json options cannot be used together.");
        std::process::exit(1);
    }
    
    let method = if (args.method.to_uppercase() == "POST" && args.data.is_some()) || args.json.is_some() { "POST" } else { "GET" };

    println!("Requesting URL: {}", input_url);
    println!("Method: {}", method);

    if let Some(ref print_data) = args.data {
        println!("Data: {}", print_data);
    }

    if let Some(ref print_data) = args.json {
        println!("JSON: {}", print_data);
    }

    let splits = input_url.split("://").collect::<Vec<&str>>();
    let parsed_url = Url::parse(&input_url);
    match parsed_url {
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
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    // URL is ok
    let client = Client::new();
    let parsed_url = parsed_url.unwrap();
    let response = match method {
        "POST" => {
            if let Some(ref json_data) = args.json {
                // Check if the JSON data is valid
                serde_json::from_str::<serde_json::Value>(&json_data).expect("Invalid JSON");
                client.post(parsed_url).header(CONTENT_TYPE, "application/json").body(json_data.clone()).send().map_err(|e| e.to_string())
            } else if let Some(ref data) = args.data {
                let params: HashMap<&str, &str> = data.split('&')
                .map(|s| {
                    let mut split = s.splitn(2, '=');
                    (split.next().unwrap(), split.next().unwrap_or(""))
                })
                .collect();
                client.post(parsed_url).form(&params).send().map_err(|e| e.to_string())
            } else {
                eprintln!("Error: The POST method requires data to be sent.");
                std::process::exit(1);
            }
        },
        _ => client.get(parsed_url).send().map_err(|e| e.to_string()),
    };

    match response {
        Ok(response) => {
            if !response.status().is_success() {
                eprintln!("Error: Request failed with status code: {}.", response.status().as_u16());
                std::process::exit(1);
            }
            let text = response.text().map_err(|e| e.to_string()).unwrap();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                let sorted_json = serde_json::to_string_pretty(&json).unwrap();
                println!("Response body (JSON with sorted keys):\n{}", sorted_json);
            } else {
                println!("Response body:\n{}", text);
            }
        }
        Err(e) => {
            if e.contains("error sending request for url") {
                eprintln!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
                std::process::exit(1);
            }
            eprintln!("Error: {}", e);
        }
    }
}
