# ZK Prover

This directory contains the Halo2 ZK circuit implementation for the Summa Proof of Solvency protocol.

## Usage

To build, test and print the circuits, execute

```
cargo build
cargo test --release --features dev-graph
```

## Documentation

The documentation for the circuits can be generated by running

```
cargo doc --no-deps --open
```

## Powers of Tau Trusted Setup

For testing purposes, it's not necessary to download the `ptau` file. The `generate_setup_artifacts` function can manage this by generating a new setup from a randomly generated value. This automated generation process is intended for testing and development convenience, and it should not be used in production.
For real-world situations, you must provide the path of a specific `ptau` file to the `generate_setup_artifacts`. The circuit will use the randomness from the given file. You can find an example that initializes a `Snapshot` instance [here](https://github.com/summa-dev/summa-solvency/blob/11d4fce5d18f6175804aa792fc9fc5ac27bf5c00/backend/src/apis/snapshot.rs#L115-L116) in the backend.

## Build a Commitment

A `gen_commitment.rs` script is provided to generate a commitment out of a Merkle Sum Tree. In particular, the example takes a csv file located in "../csv/entry_16.csv", build a Merkle Sum Tree and extract a commitment out it. The commitment is made of the `root_hash` and the `root_balances`. 

The script will eventually generate a `commitment_solidity_calldata.json` file that contains some testing calldata to be used within `contracts` and `backend` to test the publishing of the commitment to the Summa Smart Contract.

The script can be run as follows:

```
cargo run --release --example gen_commitment
```

Note that the generic parameters of the Merkle Sum Tree `N_CURRENCIES` and `N_BYTES` are set to `2` and `14`. This means that this should go in pair with a Inclusion Verifier Circuit tuned to the same generic parameters.

## Build an Inclusion Verifier Contract

A `gen_inclusion_verifier.rs` script is provided to generate a solidity contract that can be used to verify the proof of user inclusion into CEX liabilites. The script can be run as follows:

```
cargo run --release --example gen_inclusion_verifier
```

The script will generate a new `InclusionVerifier.sol` and `InclusionVerifier.yul` contracts in `contracts/src`.

Note that the generic parameters of the circuits `LEVELS`, `N_CURRENCIES` and `N_BYTES` are set to `4`, `2` and `14`. This means that the circuit is tuned to verify the proof of inclusion for an exchange with a userbase of 4 levels (2^4 = 16 users), 2 currencies and a balances in a range of 14 bytes. These parameters can be changed in the script.

The verifier are generated based on an unsafe setup. For a production ready verifier, the setup should be generated by providing a `ptau` file generated after a trusted setup ceremony to `generate_setup_artifacts` function.

On top of that the script will also generate a `inclusion_proof_solidity_calldata.json` file that contains some testing calldata to be used within `contracts` and `backend` to test the verifier. Again, in the example, the proof is generated based on the `../csv/entry_16.csv` file for a specific `user_index`, which is set to 0 by default. If you want to generate a testing proof for a different file, you can change the path in the script. If you want to generate a proof for a different `user_index`, you can change the `user_index` in the script.

## Incremental Nova Verifier 

The Incremental Nova Verifier is an experimental feature that allows a user to verify a sequence of proofs of inclusion in one shot. More details can be found in the [write up](https://hackmd.io/@summa/HkGMF4Ovn).

We provide an example to test the Nova verifier. The build folder already contains the artifacts generated by the circuit compilation. If you want to recompile the circuit, you can run the following command:

```
$ cd src/circom
$ npm install
$ circom incremental_mst_inclusion.circom  --r1cs --wasm  -o ../../examples/build --prime bn128
```

To run the Nova Incremental Verifier example run:

```
cargo run --release --example nova_incremental_verifier
```

## Benches

The benchmarking included the following areas:

- Merkle Sum Tree Generation
- Verification Key Gen for MstInclusion Circuit
- Proving Key Gen for MstInclusion Circuit
- ZK Proof Generation for MstInclusion Circuit
- ZK Proof Verification for MstInclusion Circuit

In order to run the benchmarking, we provide a set of dummy `username, balances` entries formatted in csv files. The csv files can be downloaded as follows

```
cd benches
mkdir csv
cd csv
wget https://summa-solvency.s3.eu-central-1.amazonaws.com/csv_files.tar.bz2
tar -xjf csv_files.tar.bz2
```

The csv folder will contain two subfolder namely `one_asset` and `two_assets`. Each folders will contain files named as `one_asset_entry_2_17.csv` or `two_assets_entry_2_5.csv`. 2^17 or 2^5 is the number of entries in the file that will be used to feed the merkle sum tree and, eventually, the zk prover. These entries represent the number of users of the exchange.

To run the benches

`cargo bench`

You can set the following parameters to run the benches:

- `LEVELS` -> the number of entries in the merkle sum tree. By default it is set to 20, which means that the benches will run for 2^20 entries.
- `SAMPLE_SIZE` -> the number of samples to run for each bench. By default it is set to 10, which is the minimum allowed by criterion.rs
- `N_CURRENCIES and PATH_NAME` -> the number of currencies to be used in the benchmarking. By default it is set to 1. For now you can only switch it between 1 and 2 as these are the only csv folder available. More will be added soon.

Note that the `k` of the circuit may vary based on the LEVELS

Furthermore the benchmarking function `verify_zk_proof_benchmark` will also print out the proof size in bytes.

## Current Benches

Run on MacBook Pro 2023, M2 Pro, 32GB RAM, 12 cores

2^20 entries (1048576) users, 1 currency

| MST init              |
| --------              |
| 73.695 s              |

| MST init (sorted)     |
| --------              |
| 73.847 s              |

For Merkle Sum Tree Proof of Inclusion circuit

| VK Gen             | Pk Gen              | Proof Generation    | Proof Verification  | Proof Size (bytes) |
| ------------------ | ------------------- | ------------------- | ------------------- | ------------------ |
| 183.25 ms          | 116.32 ms           | 517.98 ms           | 3.3291 ms           | 1632               |

