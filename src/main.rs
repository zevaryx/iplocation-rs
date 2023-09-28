use clap::Parser;
use serde::Deserialize;
use std::fmt;

#[derive(Parser)]
struct Cli {
    ip: String,
}

#[derive(Deserialize)]
struct IpLocation {
    ip: String,
    ip_number: String,
    ip_version: i8,
    country_name: String,
    country_code2: String,
    isp: String,
    response_code: String,
    response_message: String,
}

impl fmt::Display for IpLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IPv{}: {}\nIP Decimal: {}\nCountry: {} ({})\nISP: {}",
            self.ip_version,
            self.ip,
            self.ip_number,
            self.country_code2,
            self.country_name,
            self.isp
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let resp = reqwest::get(format!("https://api.iplocation.net?ip={}", cli.ip))
        .await?
        .json::<IpLocation>()
        .await?;

    match resp.response_code.as_str() {
        "200" => {
            println!("{resp}");
            Ok(())
        }
        _ => {
            let err_msg = format!(
                "Error Code {}: {}",
                resp.response_code, resp.response_message
            );
            Err(err_msg.into())
        }
    }
}
