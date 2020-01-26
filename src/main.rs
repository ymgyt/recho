use std::env;
use log::error;

mod tcp_client;
mod tcp_server;
//mod udp_client;
//mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}",e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}",e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                // TODO: call udp server
            },
            "client" => {
                // TODO: call udp client
            }
            _ => {
                missing_role();
            }
        }
        _ => {
            error!("specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("specify server or client on the 2nd argument.");
    std::process::exit(1);
}

