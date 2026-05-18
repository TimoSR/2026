use serde::Serialize;

use std::io::{Read, Write};
use std::net::TcpListener;

use crate::cli::HeadlessServerArgs;
use crate::commands::CommandResponse;
use crate::error::AppError;

#[derive(Debug, Serialize)]
struct HeadlessServerReport {
    command: &'static str,
    mode: &'static str,
    address: String,
    max_clients: u32,
    handled_clients: u32,
}

pub fn execute(args: HeadlessServerArgs) -> Result<CommandResponse, AppError> {
    let address = format!("{}:{}", args.bind, args.port);

    if !args.live {
        let report = HeadlessServerReport {
            command: "headless-server",
            mode: "simulated",
            address,
            max_clients: args.max_clients,
            handled_clients: 0,
        };

        let human_body = format!(
            "[headless-server] simulated mode address={} max_clients={}",
            report.address, report.max_clients
        );

        return CommandResponse::from_payload(args.json, human_body, &report);
    }

    let listener = TcpListener::bind(&address)
        .map_err(|err| AppError::server(format!("failed to bind `{address}`: {err}")))?;

    let mut handled_clients = 0_u32;

    while handled_clients < args.max_clients {
        let (mut stream, peer) = listener
            .accept()
            .map_err(|err| AppError::server(format!("accept failed: {err}")))?;

        let mut buffer = [0_u8; 1024];
        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|err| AppError::server(format!("failed to read request: {err}")))?;

        let request = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_lowercase();
        let response = if request == "ping" {
            "pong\n"
        } else {
            "unknown\n"
        };

        stream
            .write_all(response.as_bytes())
            .map_err(|err| AppError::server(format!("failed to write response: {err}")))?;

        handled_clients += 1;
        eprintln!(
            "[headless-server] handled client {} from {}",
            handled_clients, peer
        );
    }

    let report = HeadlessServerReport {
        command: "headless-server",
        mode: "live",
        address,
        max_clients: args.max_clients,
        handled_clients,
    };

    let human_body = format!(
        "[headless-server] live mode address={} handled_clients={}",
        report.address, report.handled_clients
    );

    CommandResponse::from_payload(args.json, human_body, &report)
}
