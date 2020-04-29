use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_reconcile::{
    arg_parser::parse_args,
    args::{resolve_command, Command},
    reconcile,
    thin_client::ThinClient,
};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let command_args = parse_args(env::args_os());
    let config = Config::load(&command_args.config_file)?;
    let json_rpc_url = command_args.url.unwrap_or(config.json_rpc_url);
    let client = RpcClient::new(json_rpc_url);
    let thin_client = ThinClient(client);

    match resolve_command(command_args.command)? {
        Command::Auction(args) => {
            reconcile::process_auction(&thin_client, &args)?;
        }
        Command::StakeAccounts(args) => {
            reconcile::process_stake_accounts(&thin_client, &args)?;
        }
        Command::Balances(args) => {
            reconcile::process_balances(&thin_client, &args)?;
        }
    }
    Ok(())
}
