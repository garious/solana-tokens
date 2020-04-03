mod arg_parser;
mod args;
mod thin_client;
mod tokens;

use crate::arg_parser::parse_args;
use crate::args::{resolve_command, Command};
use crate::thin_client::ThinClient;
use crate::tokens::process_distribute;
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let command_args = parse_args(env::args_os());
    let config = Config::load(&command_args.config_file)?;

    match resolve_command(&command_args.command)? {
        Command::Distribute(args) => {
            let client = if args.dry_run {
                ()
            } else {
                let json_rpc_url = command_args.url.unwrap_or(config.json_rpc_url);
                RpcClient::new(json_rpc_url);
            };
            let thin_client = ThinClient(client);
            process_distribute(&thin_client, &args)?;
        }
    }
    Ok(())
}
