# summa-solvency

This section describes how to use [`summa-solvency`](https://github.com/summa-dev/summa-solvency/tree/master/backend) backend to perform a full [Protocol Flow](../../introduction/protocol-flow.md).

## Setup

### 1. Generic Parameters

First of all, it is essential to determine three key parameters:&#x20;

* The number of currencies that will be handled simultaneously in a single Proof of Solvency - `N_CURRENCIES`
* The byte range in which the balances part of the liabilities - `N_BYTES`&#x20;
* The number of users to be included in the Proof of Solvency expressed as the number of levels in the Merkle Sum Tree -`LEVELS`&#x20;

These parameters are crucial as they significantly influence the proof size and the computation time required.

For instance, if a Custodian decides to handle ten currencies in a single proof, the proof size will be larger, and generating each proof will take more time than a single currency proof. However, this approach enhances user experience since users won't need to verify ten different proofs. In the backend, this translates into the number of currencies supported in a single Merkle Sum Tree.

The byte range parameter also influences the proof size and generation time. However, there is one key difference: [proof generation](broken-reference) will fail if balances, input to the proof, exceeds the predefined byte range limit.&#x20;

The number of users refers to the magnitude of the user base of the Custodian. This number has to be expressed as a power of two since the users are aggregated inside a Merkle Tree-like data structure.&#x20;

{% hint style="info" %}
16 users can be represented in a Merkle Sum Tree of 4 levels. 17 users can only be represented in a Merkle Sum Tree of 5. The extra 15 entries are padded with zeroes in the latter case.
{% endhint %}

So, before starting, the Custodian has to choose the generic parameters `N_CURRENCIES`, `N_BYTES` and `LEVELS`.&#x20;

### 2. Powers Of Tau

[Zero-knowledge-proof](../../cryptographic-primitives/zero-knowledge-proof.md) systems require a Trusted Setup. Summa uses the [Halo2 proving system](https://github.com/privacy-scaling-explorations/halo2) with the [KZG](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html) polynomial commitment scheme. All Summa circuits must rely on a one-time universal [trusted setup](https://vitalik.ca/general/2022/03/14/trustedsetup.html) (also known as a powers-of-tau ceremony).

To generate the proving artifacts, the Custodian must download the appropriate `.ptau` file depending on their circuit size.&#x20;

You can find pre-existing trusted setup files at [halo2-kzg-srs](https://github.com/han0110/halo2-kzg-srs). To download a specific file, you can use:

```
wget https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-11
```

In this case, we are fetching a `.ptau`for a circuit size equal to `k=11`. If this doesn't make sense to you yet, this is completely fine. In the following step, you will learn how to set the appropriate circuit size.&#x20;

### 3. Generate Verifier Contract

For the sake of the current example, let's assume that `N_CURRENCIES` is 2, `N_BYTES` is 8, and `LEVELS` is 4 (namely, a maximum user base of 16 users). Also, let's consider that a `ptau` file placed in the path[`backend/ptau/hermez-raw-11`](https://github.com/summa-dev/summa-solvency/blob/master/backend/ptau/hermez-raw-11)

Custodians can manually modify these parameters and `ptau` file path in [`zk_prover/examples/gen_inclusion_verifier.rs`](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/examples/gen\_inclusion\_verifier.rs)&#x20;

<pre class="language-rust"><code class="lang-rust">// gen_inclusion_verifier.rs 
const LEVELS: usize = 4;
<strong>const N_CURRENCIES: usize = 2;
</strong>const N_BYTES: usize = 8;

let (params, pk, _) = generate_setup_artifacts(11, Some("../backend/ptau/hermez-raw-11"), circuit.clone())
</code></pre>

The [`gen_inclusion_verifier.rs`](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/examples/gen\_inclusion\_verifier.rs) script is provided to generate a solidity contract used by the users to verify their inclusion inside the liabilities Merkle Sum Tree.

After cloning the [`summa-solvency`](https://github.com/summa-dev/summa-solvency) Repo&#x20;

```
git clone https://github.com/summa-dev/summa-solvency.git
```

The script can be run as follows:

<pre><code><strong>cd zk_prover
</strong><strong>cargo run --release --example gen_inclusion_verifier
</strong></code></pre>

The script will generate a new `InclusionVerifier.sol` and `InclusionVerifier.yul` contracts in [`contracts/src`](https://github.com/summa-dev/summa-solvency/tree/master/contracts/src).

Setting different (and higher) generic parameters might cause the following error

```
thread 'main' panicked at 'vk generation should not fail: NotEnoughRowsAvailable { current_k: 11 }', 
```

Given the generics parameters set, this error indicates that the circuit size is too small. To solve that, a `.ptau` file with a higher `k` must be [downloaded](./#2.-ptau), and the function must be adjusted accordingly.

```rust
let (params, pk, _) = generate_setup_artifacts(12, Some("../backend/ptau/hermez-raw-12"), circuit.clone())
```

The best way to find out the smallest available size fits your circuit is through trial and error.

### 4. Deploy Summa Smart Contract

In the previous section, the verifier contract files are generated, which are `InclusionVerifier.sol` and `InclusionVerifier.yul`. Both are needed to deploy the main [Summa Smart Contract](broken-reference), in particular, the `InclusionVerifier.sol` is passed to the constructor of `Summa.sol`

Follow the steps below to deploy the Summa contract:

* Modify Key Parameters

Custodians need to modify certain parameters in the deploy script located at [`contracts/scripts/deploy.ts`](https://github.com/summa-dev/summa-solvency/blob/master/contracts/scripts/deploy.ts). These parameters must be equal to `LEVELS`, `N_CURRENCIES`, and `N_BYTES` set above.

```typescript
  // The number of levels of the Merkle sum tree
  const mstLevels = 4;
  //The number of cryptocurrencies supported by the Merkle sum tree
  const currenciesCount = 2;
  // The number of bytes used to represent the balance of a cryptocurrency in the Merkle sum tree
  const balanceByteRange = 8;
```

* Update Hardhat Configuration&#x20;

The Summa-solvency project provides a hardhat configuration file primarily for testing purposes. Custodians must update `hardhat.config.ts` for deploying the contract with their own account and node. For guidance on configuring the hardhat config file, refer to [https://hardhat.org/hardhat-runner/docs/config](https://hardhat.org/hardhat-runner/docs/config).&#x20;

* Run Deploy Script

Execute the command below with the network name set in the previous step from the `contracts` folder

<pre><code><strong>cd contracts
</strong><strong>npx hardhat run scripts/deploy.ts --network [network_name]
</strong></code></pre>

As a result of running this script, Summa contract is deployed to `network_name`and a `deployments.json` file is created. This file includes the address of the deployed Summa Contract and is located at `backend/src/contracts/deployments.json`.

## Interact with the Backend

Once the setup is completed, the Custodian can utilize `summa-solvency` [backend](https://github.com/summa-dev/summa-solvency/tree/master/backend) to perform a [full Proof of Solvency ](../../introduction/protocol-flow.md)[flow.](../../introduction/protocol-flow.md)

The following steps can be performed via a working example in [Summa Solvency Flow Example](https://github.com/summa-dev/summa-solvency/blob/master/backend/examples/summa\_solvency\_flow.rs).

#### 1. Address Ownership Proof

Custodians can send ownership proofs of their address to the Summa contract using `AddressOwnership`. To do this, the Custodian must create a CSV file containing their account addresses, asset types, and signed messages. This file is referred to as `signature_csv`. Then, custodians can send a transaction containing ownership proofs as follows:

```rust
let address_ownership_client = AddressOwnership::new(&signer, signature_csv_path).unwrap();
address_ownership_client.dispatch_proof_of_address_ownership().await?;
```

The `signer` in the above code is an instance of `SummaSigner`, which includes LocalWallet from ethers-rs, and is essential for signing Ethereum transactions for the Summa contract. It is used to sign transactions that send ownership proofs to the Summa contract. However, one of its internal components, LocalWallet, is considered an insecure wallet type for production use in ethers-rs.&#x20;

More secure signer structures, such as [AwsSigner](https://docs.rs/ethers/latest/ethers/signers/struct.AwsSigner.html) and [Ledger](https://docs.rs/ethers/latest/ethers/signers/struct.Ledger.html), are available in ethers-rs, and custodians are advised to use these for enhanced security. For instance, to use a Ledger wallet instead of LocalWallet, modify the [`SummaSigner`](https://github.com/summa-dev/summa-solvency/blob/master/backend/src/contracts/signer.rs#L20) as shown below:

```rust
impl SummaSigner {
    pub async fn new(
        address_input: AddressInput,
    ) -> Result<Self, Box<dyn Error>> {
       let ledger_wallet = Ledger::new(HDPath::LedgerLive(0), 1).await?;

        let provider = Arc::new(Provider::try_from(url)?);
        let chain_id = provider.get_chainid().await?.as_u64();
        let client = Arc::new(SignerMiddleware::new(
            provider,
            ledger_wallet.with_chain_id(chain_id),
        ));
    
```

The `signer` is only used to submit proofs and commitments to Summa contract so it does not have to hold any of the Custodian funds. Note that since Summa contract is a `Ownable` contract, the `signer` has to be the same used to deploy the Summa contract. Otherwise, transactions will be reverted.

#### 2. Liabilities Commitment

For generating liabilities commitment and user inclusion proofs, a custodian has to prepare a CSV file, referred to as `entry.csv` , which contains all usernames and their `N_CURRENCIES` balances. The csv file format looks like this:

```
username,balance_ETH_ETH,balance_USDT_ETH
dxGaEAii,11888,41163
```

The first column header must be the username, and the rest of the columns should follow the pattern balance\_\[Currency]\_\[Chain]. Assuming the CSV file is named as`entry.csv`, build the Merkle Sum Tree as follows:

```rust
let entry_csv_path = "csv/entry.csv";
let mst = MerkleSumTree::new(entry_csv_path).unwrap();
```

To initiate `Round`, two additional parameters are needed: `params_path` and `timestamp`. `params_path` is the path to the .ptau file, which should be the same as used in the [Generate Verifier Contract section](./#3.-generate-verifier-contract). `timestamp` is a UNIX timestamp that indicates the time when the liabilities snapshot (reflected in the `entry.csv` file) was taken.

```rust
let params_path = "backend/ptau/hermez-raw-11"
let timestamp = 1701666053u64;
let mut round = Round::<LEVELS, N_CURRENCIES, N_BYTES>::new(&signer, Box::new(mst), params_path, timestamp).unwrap();
```

Now, the custodian can send the commitment transaction using the `dispatch_commitment` method:

```rust
round.dispatch_commitment().await
```

Internally, `dispatch_commitment` sends a transaction that includes the `mstRoot` and `rootBalance` of the Merkle Sum Tree to the `submitCommitment` function in the Summa Contract.

{% hint style="info" %}
Building a Merkle Sum Tree on a single machine is a heavy task that can be parallelized and, therefore, sped up, using [`summa-aggregation`](../summa-aggregation/)
{% endhint %}

#### 3. Generate Inclusion Proofs

Now, custodians can generate inclusion proofs for their users using the `round`, initiated from the previous step.

The `get_proof_of_inclusion` method requires a `user_index`, which corresponds to the index of the user's data in the Entry CSV file (`entry.csv` from the previous section).

```rust
let inclusion_proof = round.get_proof_of_inclusion(user_index).unwrap();
```

Custodians can then provide these inclusion proofs to their users.

{% hint style="info" %}
Generating proofs for all the users at once can present some [scaling issues](../../open-issues.md#timing-of-proofs-of-inclusion-generation). Alternatively, the custodian can generate such proof only if the user queries it. Have a look at the [benchmarks](benchmarks.md) for this!
{% endhint %}

#### 4. Verify Inclusion Proof

Users who receive a proof for a specific round can use the `verify_inclusion_proof` API provided by the backend. The term `downloaded_inclusion_proof` refers to the proof that users have downloaded from custodians.\
For details on handling `downloaded_inclusion_proof` at the code level, please refer to the [Summa Solvency Flow Example](https://github.com/summa-dev/summa-solvency/blob/master/backend/examples/summa\_solvency\_flow.rs).

The verification function also requires the user to pass the `snapshot_time` which must match the `timestamp` at which the Liabilities Commitment was submitted to the smart contract.

```rust
let proof = downloaded_inclusion_proof.get_proof()
let public_input = downloaded_inclusion_proof.get_public_inputs();
let verify_result = summa_contract.verify_inclusion_proof(proof, public_inputs, snapshot_time).await?
```

As demonstrated in the example above, custodians can provide an application to users to verify their inclusion proof locally on their machines.

Note that the `verify_inclusion_proof` function in the Summa contract is a view function. This means that it does not consume gas or alter the state of the blockchain. Additionally, users can independently verify their proofs using public interfaces, such as Etherscan, for added transparency and trust.

<figure><img src="../../.gitbook/assets/Screenshot 2023-12-04 at 11.55.20.png" alt=""><figcaption><p>An example Etherscan interface to let users verify their Inclusion Proof</p></figcaption></figure>

When receiving the proof, the user should also verify that the `leaf_hash` being verified matches the combination of their username and balances. `balances` represent the balances of the user at `snapshot_time`

```rust
let user_name = "dxGaEAii".to_string();
let balances = vec!["11888".to_string(), "41163".to_string()];

let leaf_hash = public_inputs[0];
assert_eq!(
    leaf_hash,
    leaf_hash_from_inputs::<N_CURRENCIES>(user_name.clone(), balances.clone())
); 
```
