use crate::args::{
    Args, BalancesArgs, Command, DistributeStakeArgs, DistributeTokensArgs, PrintDbArgs,
};
use clap::{value_t, value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use solana_clap_utils::input_validators::{is_valid_pubkey, is_valid_signer};
use solana_cli_config::CONFIG_FILE;
use std::ffi::OsString;
use std::process::exit;

fn get_matches<'a, I, T>(args: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let default_config_file = CONFIG_FILE.as_ref().unwrap();
    App::new("solana-tokens")
        .about("about")
        .version("version")
        .arg(
            Arg::with_name("config_file")
                .long("config")
                .takes_value(true)
                .value_name("FILEPATH")
                .default_value(default_config_file)
                .help("Config file"),
        )
        .arg(
            Arg::with_name("url")
                .long("url")
                .global(true)
                .takes_value(true)
                .value_name("URL")
                .help("RPC entrypoint address. i.e. http://devnet.solana.com"),
        )
        .subcommand(
            SubCommand::with_name("distribute-tokens")
                .about("Distribute tokens")
                .arg(
                    Arg::with_name("transactions_db")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Transactions database file"),
                )
                .arg(
                    Arg::with_name("from_bids")
                        .long("from-bids")
                        .help("Input CSV contains bids in dollars, not allocations in SOL"),
                )
                .arg(
                    Arg::with_name("input_csv")
                        .long("input-csv")
                        .required(true)
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Input CSV file"),
                )
                .arg(
                    Arg::with_name("dollars_per_sol")
                        .long("dollars-per-sol")
                        .required(true)
                        .takes_value(true)
                        .value_name("NUMBER")
                        .help("Dollars per SOL, if input CSV contains bids"),
                )
                .arg(
                    Arg::with_name("dry_run")
                        .long("dry-run")
                        .help("Do not execute any transfers"),
                )
                .arg(
                    Arg::with_name("no_wait")
                        .long("no-wait")
                        .help("Don't wait for transaction confirmations"),
                )
                .arg(
                    Arg::with_name("sender_keypair")
                        .long("from")
                        .takes_value(true)
                        .value_name("SENDING_KEYPAIR")
                        .validator(is_valid_signer)
                        .help("Keypair to fund accounts"),
                )
                .arg(
                    Arg::with_name("fee_payer")
                        .long("fee-payer")
                        .takes_value(true)
                        .value_name("KEYPAIR")
                        .validator(is_valid_signer)
                        .help("Fee payer"),
                )
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .help("Do not block transfers is recipients have a non-zero balance"),
                ),
        )
        .subcommand(
            SubCommand::with_name("distribute-stake")
                .about("Distribute stake accounts")
                .arg(
                    Arg::with_name("transactions_db")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Transactions database file"),
                )
                .arg(
                    Arg::with_name("allocations_csv")
                        .long("allocations-csv")
                        .required(true)
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Allocations CSV file"),
                )
                .arg(
                    Arg::with_name("dry_run")
                        .long("dry-run")
                        .help("Do not execute any transfers"),
                )
                .arg(
                    Arg::with_name("no_wait")
                        .long("no-wait")
                        .help("Don't wait for transaction confirmations"),
                )
                .arg(
                    Arg::with_name("stake_account_address")
                        .required(true)
                        .long("stake-account-address")
                        .takes_value(true)
                        .value_name("ACCOUNT_ADDRESS")
                        .validator(is_valid_pubkey)
                        .help("Stake Account Address"),
                )
                .arg(
                    Arg::with_name("sol_for_fees")
                        .default_value("1.0")
                        .long("sol-for-fees")
                        .takes_value(true)
                        .value_name("SOL_AMOUNT")
                        .help("Amount of SOL to put in system account to pay for fees"),
                )
                .arg(
                    Arg::with_name("stake_authority")
                        .long("stake-authority")
                        .takes_value(true)
                        .value_name("KEYPAIR")
                        .validator(is_valid_signer)
                        .help("Stake Authority Keypair"),
                )
                .arg(
                    Arg::with_name("withdraw_authority")
                        .long("withdraw-authority")
                        .takes_value(true)
                        .value_name("KEYPAIR")
                        .validator(is_valid_signer)
                        .help("Withdraw Authority Keypair"),
                )
                .arg(
                    Arg::with_name("fee_payer")
                        .long("fee-payer")
                        .takes_value(true)
                        .value_name("KEYPAIR")
                        .validator(is_valid_signer)
                        .help("Fee payer"),
                ),
        )
        .subcommand(
            SubCommand::with_name("balances")
                .about("Balance of each account")
                .arg(
                    Arg::with_name("input_csv")
                        .long("input-csv")
                        .required(true)
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Bids CSV file"),
                )
                .arg(
                    Arg::with_name("from_bids")
                        .long("from-bids")
                        .help("Input CSV contains bids in dollars, not allocations in SOL"),
                )
                .arg(
                    Arg::with_name("dollars_per_sol")
                        .long("dollars-per-sol")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .help("Dollars per SOL"),
                ),
        )
        .subcommand(
            SubCommand::with_name("print-database")
                .about("Print the database to a CSV file")
                .arg(
                    Arg::with_name("transactions_db")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Transactions database file"),
                )
                .arg(
                    Arg::with_name("output_path")
                        .long("output-path")
                        .required(true)
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Output file"),
                ),
        )
        .get_matches_from(args)
}

fn parse_distribute_tokens_args(matches: &ArgMatches<'_>) -> DistributeTokensArgs<String, String> {
    DistributeTokensArgs {
        input_csv: value_t_or_exit!(matches, "input_csv", String),
        from_bids: matches.is_present("from_bids"),
        transactions_db: value_t_or_exit!(matches, "transactions_db", String),
        dollars_per_sol: value_t!(matches, "dollars_per_sol", f64).ok(),
        dry_run: matches.is_present("dry_run"),
        no_wait: matches.is_present("no_wait"),
        sender_keypair: value_t!(matches, "sender_keypair", String).ok(),
        fee_payer: value_t!(matches, "fee_payer", String).ok(),
        force: matches.is_present("force"),
        stake_args: None,
    }
}

fn parse_distribute_stake_args(matches: &ArgMatches<'_>) -> DistributeStakeArgs<String, String> {
    DistributeStakeArgs {
        allocations_csv: value_t_or_exit!(matches, "allocations_csv", String),
        transactions_db: value_t_or_exit!(matches, "transactions_db", String),
        dry_run: matches.is_present("dry_run"),
        no_wait: matches.is_present("no_wait"),
        stake_account_address: value_t_or_exit!(matches, "stake_account_address", String),
        sol_for_fees: value_t_or_exit!(matches, "sol_for_fees", f64),
        stake_authority: value_t!(matches, "stake_authority", String).ok(),
        withdraw_authority: value_t!(matches, "withdraw_authority", String).ok(),
        fee_payer: value_t!(matches, "fee_payer", String).ok(),
    }
}

fn parse_balances_args(matches: &ArgMatches<'_>) -> BalancesArgs {
    BalancesArgs {
        input_csv: value_t_or_exit!(matches, "input_csv", String),
        from_bids: matches.is_present("from_bids"),
        dollars_per_sol: value_t!(matches, "dollars_per_sol", f64).ok(),
    }
}

fn parse_print_db_args(matches: &ArgMatches<'_>) -> PrintDbArgs {
    PrintDbArgs {
        transactions_db: value_t_or_exit!(matches, "transactions_db", String),
        output_path: value_t_or_exit!(matches, "output_path", String),
    }
}

pub fn parse_args<I, T>(args: I) -> Args<String, String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = get_matches(args);
    let config_file = matches.value_of("config_file").unwrap().to_string();
    let url = matches.value_of("url").map(|x| x.to_string());

    let command = match matches.subcommand() {
        ("distribute-tokens", Some(matches)) => {
            Command::DistributeTokens(parse_distribute_tokens_args(matches))
        }
        ("distribute-stake", Some(matches)) => {
            Command::DistributeStake(parse_distribute_stake_args(matches))
        }
        ("balances", Some(matches)) => Command::Balances(parse_balances_args(matches)),
        ("print-database", Some(matches)) => Command::PrintDb(parse_print_db_args(matches)),
        _ => {
            eprintln!("{}", matches.usage());
            exit(1);
        }
    };
    Args {
        config_file,
        url,
        command,
    }
}
