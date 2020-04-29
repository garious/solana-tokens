# Reconcile Solana token distributions

The terms of stake and token distributions are sometimes managed off-chain.
For example, a third-party may host a dutch auction and then provide the
tokenholder with a spreadsheet detailing who bought what. The automation
here, a command-line tool called `solana-tokens`, reconciles spreadsheet
data with Solana's on-chain state. It caches records of each on-chain
transaction, so that the spreadsheet can be updated over time. You can
then run `solana-tokens` with the `--dry-run` option to see what distributions
have not yet been performed. If you re-run the commend without the `--dry-run`
option, those same distributions are executed.

## Reconcile auction results

Send tokens to the recipients in `<BIDS_CSV>`.

Example bids.csv:

```text
primary_address,bid_amount_dollars
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv,6.6
```

```bash
solana-reconcile auction --from <KEYPAIR> --dollars-per-sol <NUMBER> --bids-csv <BIDS_CSV> <TRANSACTION_LOG> --fee-payer <KEYPAIR>
```

Example transaction log before:

```text
recipient,amount,signature
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv,30,1111111111111111111111111111111111111111111111111111111111111111
```

Send tokens to the recipients in `<BIDS_CSV>` if the distribution is
not already recorded in the transaction log.

```bash
solana-reconcile auction --from <KEYPAIR> --dollars-per-sol <NUMBER> --bids-csv <BIDS_CSV> <TRANSACTION_LOG> --fee-payer <KEYPAIR>
```

Example output:

```text
Recipient                                     Amount
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv  70
3ihfUy1n9gaqihM5bJCiTAGLgWc5zo3DqVUS6T736NLM  42
UKUcTXgbeTYh65RaVV5gSf6xBHevqHvAXMo3e8Q6np8k  43
```


Example transaction log after:

```text
recipient,amount,signature
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv,30,1111111111111111111111111111111111111111111111111111111111111111
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv,70,1111111111111111111111111111111111111111111111111111111111111111
3ihfUy1n9gaqihM5bJCiTAGLgWc5zo3DqVUS6T736NLM,42,1111111111111111111111111111111111111111111111111111111111111111
UKUcTXgbeTYh65RaVV5gSf6xBHevqHvAXMo3e8Q6np8k,43,1111111111111111111111111111111111111111111111111111111111111111
```

## Calculate what tokens should be sent

List the differences between a list of expected distributions and the record of what
transactions have already been sent.

```bash
solana-reconcile auction --dollars-per-sol <NUMBER> --dry-run --bids-csv <BIDS_CSV> <TRANSACTION_LOG>
```

Example bids.csv:

```text
primary_address,bid_amount_dollars
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv,6.6
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv,15.4
3ihfUy1n9gaqihM5bJCiTAGLgWc5zo3DqVUS6T736NLM,9.24
UKUcTXgbeTYh65RaVV5gSf6xBHevqHvAXMo3e8Q6np8k,9.46
```

Example output:

```text
Recipient                                     Amount
6Vo87BaDhp4v4GHwVDhw5huhxVF8CyxSXYtkUwVHbbPv  70
3ihfUy1n9gaqihM5bJCiTAGLgWc5zo3DqVUS6T736NLM  42
UKUcTXgbeTYh65RaVV5gSf6xBHevqHvAXMo3e8Q6np8k  43
```

