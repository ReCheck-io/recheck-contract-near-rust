# recheck-contract-near-rust

ReCheck's Smart Contract for NEAR using Rust

# What This Contract Does

Stores unique records for usage in all of ReCheck's blockchain [**solutions**](https://recheck.io).
<br />

# Quickstart

Clone this repository locally or [**open it in GitHub**](https://github.com/ReCheck-io/recheck-contract-near-rust). Then
follow these steps:

### 1. Install Dependencies

```bash
npm install
```

### 2. Deploy the Contract using NEAR CLI

Install [**NEAR CLI**](https://github.com/near/near-cli)

```bash
npm install -g near-cli
```

Login with your NEAR wallet.

```bash
near login
```

Build the contract.

```bash
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

Deploy the contract using a new testnet account.

```bash
near dev-deploy ./target/wasm32-unknown-unknown/release/recheck_near.wasm
```

### 3. Interact with the Contract using NEAR CLI

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
