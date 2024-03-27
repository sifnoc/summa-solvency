# summa-solvency

This section describes how to use [`summa-solvency`](https://github.com/summa-dev/summa-solvency/tree/master/backend) backend to perform a full [Protocol Flow](../introduction/protocol-flow.md).

## Setup

### 1. Generic Parameters

First of all, it is essential to determine two key parameters:&#x20;

* The number of currencies that will be handled simultaneously in a single Proof of Solvency - `N_CURRENCIES`
* The number of users included in a proof of solvency - `N_USERS`

Take, for example, a scenario where a Custodian opts to include ten currencies and 64 users in one proof. This setup enables the commitment to cover 64 users. Furthermore, it allows users to simultaneously verify their inclusion proof for all ten currencies in one go, rather than having to verify ten separate proofs.

Therefore, before starting, Custodians need to thoughtfully determine the `N_CURRENCIES` and `N_USERS` values.

### 2. Powers Of Tau&#x20;

[Zero-knowledge-proof](../cryptographic-primitives/zero-knowledge-proof.md) systems require a Trusted Setup. Summa uses the [Halo2 proving system](https://github.com/privacy-scaling-explorations/halo2) with the [KZG](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html) polynomial commitment scheme. The `UnivariateGrandSum` circuit requires a one-time, universal trusted setup, commonly known as a powers-of-tau ceremony.

To generate proving artifacts, custodians must first download the `.ptau` file that corresponds with their circuit's size, denoted as `K`.

Identifying the correct `K` value is essential for Summa's halo2 circuit, as it is for any halo2 circuit. The minimum circuit size can be accurately determined based on the number of users by applying the following formula:

$$
K = MAX(\log_2({N\_USERS} + 6), 17)
$$

This formula ensures that the chosen `K` value satisfies both the prerequisite condition `N_USERS < 2^K - 6` and the minimum rows required for the range check circuit.

Pre-existing trusted setup files can be found at [`halo2-kzg-srs`](https://github.com/han0110/halo2-kzg-srs). To download a specific `.ptau` file, utilize the following command:

```
wget https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-17
```

This action downloads a `.ptau` file for a circuit size of `k=17`.

### 3. Generate SNARK Verifier Contract

In this example, we'll work under the assumption that `N_CURRENCIES` is set to 2, and `N_USERS` is 16. Additionally, it's presumed that the .ptau file is located at `backend/ptau/hermez-raw-17`. Custodians have the flexibility to adjust these parameters and the .ptau file path within the `kzg_prover/bin/gen_verifier.rs` file:

<pre class="language-rust"><code class="lang-rust">// gen_verifier.rs 
const K: u32 = 17;
<strong>const N_CURRENCIES: usize = 2;
</strong>const N_USERS: usize = 16;

let (params, pk, _) = generate_setup_artifacts(K, Some("../backend/ptau/hermez-raw-17"), circuit.clone())
</code></pre>

The `gen_verifier.rs` script assists in generating a Solidity verifier contract. It initializes the circuit and generates the verifier contract using the [`halo2-solidity-verifier`](https://github.com/summa-dev/halo2-solidity-verifier).

To begin, clone the Summa Solvency repository:

```
git clone --branch v2 https://github.com/summa-dev/summa-solvency.git
```

Then, execute the script with the following commands:

<pre><code><strong>cd kgz_prover
</strong><strong>cargo run --release --bin generate_verifier
</strong></code></pre>

This process yields new contracts: `SnarkVerifier.sol` and `VerifyingKey.sol` in `contracts/src`. If you haven't calculated the `K` correctly in [step 2](summa-solvency.md#id-2.-powers-of-tau), you may encounter the following error:

```
thread 'main' panicked at 'NotEnoughRowsAvailable { current_k: 17 }', 
```

This error indicates that the circuit size is too small. To ensure `K` is correctly determined to match the requirements, please revisit the section[#id-2.-powers-of-tau](summa-solvency.md#id-2.-powers-of-tau "mention"). To solve that, a `.ptau` file with a correct `K` must be [downloaded](summa-solvency.md#2.-ptau), and the function must be adjusted accordingly:

```rust
let (params, pk, _) = generate_setup_artifacts(18, Some("../backend/ptau/hermez-raw-18"), circuit.clone())
```

### 4. Deploy Summa Smart Contract

Following the generation of the SNARK verifier contract, "[SnarkVerifier.sol](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/SnarkVerifier.sol)", it is now necessary to deploy the main Summa Smart Contract, "[Summa.sol](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/Summa.sol)". The deployment process requires the addresses of three verifier contracts and verifying key contract —"[SnarkVerifier.sol](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/SnarkVerifier.sol)", "[GrandSumVerifier.sol](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/GrandSumVerifier.sol)", "[InclusionVerifier.sol](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/InclusionVerifier.sol)" and "[VerifyingKey.sol](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/VerifyingKey.sol)"—to be provided to the "[Summa.sol](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/Summa.sol)" constructor.

* **Modify Key Parameter**

Custodians must update specific parameters within the deployment script located at "[contracts/scripts/deploy.ts](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/scripts/deploy.ts)". These parameters should match the `N_CURRENCIES` value specified earlier.

```typescript
// The number of cryptocurrencies in the balance polynomials
const currenciesCount = 2;
```

* **Updating Hardhat Configuration**

The Summa-solvency project includes a Hardhat configuration file "hardhat.config.ts", primarily intended for testing. Custodians are required to modify the configuration to deploy the contract using their own account and node settings. For detailed instructions on configuring the Hardhat file, visit [Hardhat Configuration Documentation](https://hardhat.org/hardhat-runner/docs/config).

* Running Deploy Script

Execute the command below with the network name set in the previous step from the `contracts` folder

To deploy the contract, execute the following command from the contracts directory, ensuring the network name matches the one set in the Hardhat configuration:

<pre><code><strong>cd contracts
</strong><strong>npx hardhat run scripts/deploy.ts --network [network_name]
</strong></code></pre>

Successful execution of this script will deploy the Summa contract to the specified network, and generate a "deployments.json" file. This file contains the address of the deployed Summa Contract and is saved in "backend/src/contracts/deployments.json".

## Interact with the Backend

Once the setup is completed, the Custodian can utilize summa-solvency [backend](https://github.com/summa-dev/summa-solvency/tree/v2/backend) to perform a [full Proof of Solvency flow](../introduction/protocol-flow.md).

The  [Summa Solvency Flow Example](https://github.com/summa-dev/summa-solvency/blob/v2/backend/examples/summa\_solvency\_flow.rs) illustrates the process for performing the necessary steps, as outlined below.

#### 1. Address Ownership Proof

Custodians are able to provide proofs of ownership for their addresses to the Summa contract using the `AddressOwnership` feature. To accomplish this, they must first compile a CSV file (`signature_csv`) containing the account addresses, chain names, signatures of the messages, and the messages themselves. Following this preparation, custodians can submit ownership proofs by executing the following transaction:

```rust
let mut address_ownership_client = AddressOwnership::new(&signer, signature_csv_path).unwrap();
address_ownership_client.dispatch_proof_of_address_ownership().await?;
```

The `signer` in the above code is an instance of `SummaSigner`, which includes LocalWallet from ethers-rs, and is essential for signing Ethereum transactions for the Summa contract. It is used to sign transactions that send ownership proofs to the Summa contract. However, one of its internal components, `LocalWallet`, is considered an insecure wallet type for production use in ethers-rs.&#x20;

```rust
impl SummaSigner {
    /// Creates a new SummaSigner instance
    /// # Arguments
    /// * `signer_key` - The private key of wallet that will interact with the chain on behalf of the exchange
    /// * `url` -  The endpoint for connecting to the node
    /// * `address` - The address of the Summa contract
    pub async fn new(
        signer_key: &str,
        url: &str,
        address_input: AddressInput,
    ) -> Result<Self, Box<dyn Error>> {
        let wallet: LocalWallet = LocalWallet::from_str(signer_key).unwrap();

        let provider = Arc::new(Provider::try_from(url)?);
        let chain_id = provider.get_chainid().await?.as_u64();
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(chain_id),
        ));
    
```

For enhanced security, custodians are encouraged to explore more secure signer structures available in ethers-rs. A variety of options, including AwsSigner and hardware wallets, offer increased security measures. Details on these and other signers can be found in the ethers-rs documentation: [https://docs.rs/ethers/latest/ethers/signers/index.html](https://docs.rs/ethers/latest/ethers/signers/index.html). To switch from using LocalWallet to a hardware wallet, the SummaSigner configuration would need to be modified accordingly.

The `signer` is only used to submit proofs and commitments to Summa contract so it does not have to hold any of the Custodian funds. Note that since Summa contract is an `Ownable` contract, the `signer` has to be the same used to deploy the Summa contract. Otherwise, transactions will be reverted.

#### 2. Liabilities Commitment

To generate the liabilities commitment and user inclusion proofs a custodian has to prepare a CSV file, referred to as "entry.csv" , which contains all usernames and their `N_CURRENCIES` balances. The csv file format looks like this:

```
username,balance_ETH_ETH,balance_DAI_ETH
dxGaEAii,11888,41163
```

The first column header must be the username, and the rest of the columns should follow the pattern _balance\_\[Currency]\_\[Chain]_. Assuming the CSV file is named as `entry.csv`, initialize Univariate Grand Sum circuit as follows:

```rust
let entry_csv = "entry.csv";
let mut entries: Vec<Entry<N_CURRENCIES>> = vec![Entry::init_empty(); N_USERS];
let mut cryptos = vec![Cryptocurrency::init_empty(); N_CURRENCIES];
parse_csv_to_entries::<&str, N_CURRENCIES>(entry_csv, &mut entries, &mut cryptos).unwrap();

let univariate_grand_sum_circuit = UnivariateGrandSum::<
    N_USERS,
    N_CURRENCIES,
    UnivariateGrandSumConfig<N_CURRENCIES, N_USERS>,
>::init(entries.to_vec());

```

Initializing a round requires multiple parameters: `zk_snark_proof`, `advice_polys`, `params`, `vk`, and `timestamp`:

```rust
let mut round = Round::<N_CURRENCIES, N_POINTS, N_USERS>::new(
    &signer,
    zk_snark_proof,
    advice_polys,
    params,
    vk,
    timestamp,
);
```

* `params`: An instance of `ParamsKZG` from halo2, initialized as in the Generate SNARK Verifier Contract section.
* `zk_snark_proof`: The proof generated from the `UnivariateGrandSum` circuit.
* `advice_polys`: The advice polynomials that were used as witness data to create the zk\_snark\_proof.
* `vk`: The verifying key for the `zk_snark_proof`.
* `timestamp`: A UNIX timestamp marking the moment the liabilities snapshot (reflected in the "entry.csv" file) was taken.

Now, the custodian can send the commitment transaction using the `dispatch_commitment` method:

```rust
round.dispatch_commitment().await?;
```

Internally, `dispatch_commitment` submits a transaction to the Summa Contract using the `submitCommitment` function, which includes the `zk_snark_proof`, `grand_sum_proof`, and the total balance of the KZG polynomials.

#### 3. Generate Inclusion Proofs

Now, the custodian can generate inclusion proofs for their users using the `round`, Initialized from the previous step.

The `get_proof_of_inclusion` method requires a `user_index`, which corresponds to the index of the user's data within the Entry CSV file ("entry.csv"), as established in the prior section.

```rust
let inclusion_proof = round.get_proof_of_inclusion(user_index).unwrap();
```

The custodian can then provide these inclusion proofs to their users.

{% hint style="info" %}
Generating proofs for all the users at once can present some [scaling issues](../open-issues.md#timing-of-proofs-of-inclusion-generation). Alternatively, the custodian can generate such proof only if the user queries it. Please refer to the [benchmarks](../benchmarks.md).
{% endhint %}

#### 4. Verify Inclusion Proof

Users who have received a proof for a specific round can utilize the `verify_inclusion_proof` API provided by the backend. The term `downloaded_inclusion_proof` denotes the proof users have obtained from custodians. \
For guidance on processing `downloaded_inclusion_proof` at the code level, please check the [Summa Solvency Flow Example](https://github.com/summa-dev/summa-solvency/blob/66ebc918f1fb8d6d1a8c0fc4cbc8ac56c34c23b2/backend/examples/summa\_solvency\_flow.rs#L137-L140).

To perform the verification, users must also provide the `snapshot_time` corresponding to the timestamp when the Liabilities Commitment was registered with the smart contract.

```rust
let inclusion_proof = downloaded_inclusion_proof.get_proof().clone();
let challenge_s_g2 = downloaded_inclusion_proof.get_challenge().clone().unwrap();
let input_values = downloaded_inclusion_proof.get_input_values().clone();

// Validate the inclusion proof using the contract verifier.
let verification_result = summa_contract
    .verify_inclusion_proof(snapshot_time, inclusion_proof, challenge_s_g2, input_values)
    .await?;
```

Note that the `verify_inclusion_proof` function in the Summa contract is a view function that does not consume gas nor alters the state of the blockchain, and therefore, is free to call. This allows users to independently verify their proofs using public frontends like Etherscan, ensuring added transparency and trust without incurring any cost.

Alternatively, custodians may offer an application similar to the demonstrated above for users to verify their inclusion proofs locally.
