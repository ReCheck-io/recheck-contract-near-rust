# recheck-contract-near-rust

ReCheck's Smart Contract for NEAR using Rust

# What This Contract Does

Stores unique records for usage in all of ReCheck's blockchain [**solutions**](https://recheck.io).
<br />

# Quickstart

Clone this repository locally or [**open it in GitHub**](https://github.com/ReCheck-io/recheck-contract-near-rust).

```bash
git clone git@github.com:ReCheck-io/recheck-contract-near-rust.git
```

Then follow these steps inside the repo directory:

### 1. Install Dependencies

Install Rust from the [**installer script**](https://rustup.rs).

```bash
curl https://sh.rustup.rs/ -sSf | sh
```

Set the required target.

```bash
rustup target add wasm32-unknown-unknown
```

### 2. Build the Contract

Build the contract.

```bash
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

Run contract tests and verify they pass.

```bash
cargo test
```

### 3. Deploy the Contract using NEAR CLI

Install [**NEAR CLI**](https://github.com/near/near-cli)

```bash
npm install -g near-cli
```

By default, it is set for "testnet". For "mainnet" set it like this.

```bash
export NEAR_ENV=mainnet
```

You can verify it to be sure.

```bash
echo $NEAR_ENV
```

Login with your NEAR wallet.

```bash
near login
```

Deploy the contract using a new testnet account.

```bash
near dev-deploy ./target/wasm32-unknown-unknown/release/recheck_near.wasm
```

For mainnet you can create a sub account first.

```bash
near create-account SUB-ACCOUNT.YOUR-WALLET-ID.near --masterAccount YOUR-WALLET-ID.near --initialBalance DESIRED-AMMOUNT
```

And then deploy with the sub account.

```bash
near deploy YOUR-NEW-ACCOUNT.near ./target/wasm32-unknown-unknown/release/recheck_near.wasm
```

Any sub account can be added to your wallet with its private key.

```bash
https://wallet.near.org/auto-import-secret-key#YOUR_ACCOUNT_ID/YOUR_PRIVATE_KEY
```

All account keys are located here.

```bash
cd ~/.near-credentials
```

If any of the steps fails due to low balance use this formula to convert yocto to near.

```bash
X yocto / 10^24 = Y NEAR
```

### 4. Interact with the Contract using NEAR CLI

Execute change method (*you have to be logged in with the **same** NEAR wallet used for deployment*)

```bash
near call --accountId YOUR-WALLET-ID.TESTNET DEV-ACCOUNT-USED-FOR-DEPLOYMENT createSubRecordWithExtras2 '{"recordIdStr":"SET_HASH_VALUE","parentRecordIdStr":"SET_HASH_VALUE","trailStr":"SET_HASH_VALUE","trailSignatureStr":"SET_HASH_VALUE","extra0Str":"SET_HASH_VALUE","extra1Str":"SET_HASH_VALUE"}'
```

Execute view method (*with **any** logged in wallet*)

```bash
near call --accountId ANY-WALLET-ID.TESTNET ANY-ACCOUNT records '{"record_id_str":"SET_HASH_VALUE"}'
```

---

# Learn More

1. Learn more about the contract through its [**README**](./README.md).
2. Test [**our solution**](https://beta.recheck.io/) which uses this contract.
3. Check our [**GitHub**](https://github.com/ReCheck-io/) for more.
