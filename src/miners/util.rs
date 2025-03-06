use reqwest::StatusCode;
use reqwest::header::HeaderMap;
use std::error::Error;
use std::net::IpAddr;
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub(crate) async fn send_rpc_command(
    ip: &IpAddr,
    command: &'static str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let mut stream = tokio::net::TcpStream::connect(format!("{}:4028", ip))
        .await
        .unwrap();

    let command = String::from(format!("{{\"command\":\"{command}\"}}"));

    stream.write_all(command.as_bytes()).await.unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await.unwrap();

    let response = String::from_utf8_lossy(&buffer)
        .into_owned()
        .replace('\0', "");
    let parsed = parse_rpc_result(&response);
    parsed
}

pub(crate) async fn send_web_command(
    ip: &IpAddr,
    command: &'static str,
    https: bool,
) -> Result<(String, HeaderMap, StatusCode), Box<dyn Error>> {
    let scheme = if https { "https" } else { "http" };
    let resp = reqwest::get(format!("{}://{}{}", scheme, ip.to_string(), command)).await?;
    let resp_headers = &resp.headers().to_owned();
    let resp_status = &resp.status().to_owned();
    let resp_text = &resp.text().await?;
    Ok((resp_text.clone(), resp_headers.clone(), *resp_status))
}

fn parse_rpc_result(response: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let parsed: serde_json::Value = serde_json::from_str(response)?;

    let success_codes = ["S", "I"];
    let command_status = parsed["STATUS"][0]["STATUS"].as_str().unwrap();

    if success_codes.contains(&command_status) {
        Ok(parsed)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "RPC command failed",
        )))
    }
}
