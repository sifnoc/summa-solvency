# Summa.sol

The Summa smart contract acts as a registry for the Custodian to commit to their liabilities by submitting a Merkle sum tree (MST) root. Users can then verify their inclusion into the liabilities commitment and compare the committed total liability sums with the assets owned by the Custodian on-chain. While the Custodian can still use other methods to distribute the commitment, the proofs, and verification tools to the users, having it inside the smart contract gives the following advantages:

* Single source of truth: the smart contract guarantees that the commitments submitted by the Custodian are immutable and the same for each verifying user - users don't need to trust the Custodian with hosting the data;
* Simple verifier UX: users can verify the proof using any established block explorer web UI (e.g., Etherscan). To verify the proof, users should simply paste the verification function arguments into the webpage fields and observe the result after calling the function - no need to install any developer tools

{% hint style="info" %}
While the smart contract registry is the recommended solution, it is not essential for the proof of solvency protocol to function correctly. The Custodian can choose to self-host the liability commitments and ask the users to run the Rust verifier locally on their machines after installing the necessary tooling
{% endhint %}

### Summa.sol Functionality

Summa smart contract provides the following features:

* **Address Ownership Proofs**: Custodians should submit the proof of address ownership for all addresses that hold the assets included in the commitment by using `submitProofOfAddressOwnership` function. The proofs are accepted optimistically and subject to off-chain verification by any interested party.
* **Liabilities Commitments**: Custodians can submit commitments to their liabilities in the form of MST roots and the corresponding total sums that represent the snapshots of the liabilities at a given timestamp by using `submitCommitment` function.
* **Inclusion Verification**: Users can verify the zero-knowledge proof of inclusion of their balances into the MST using `verifyInclusionProof` function. The function is calling the underlying smart contract Verifier. The verifier is generated from the `zk_prover` module as a standalone contract and passed into Summa.sol as a constructor argument.

### Verifier Contract Validity

When verifying the user's inclusion proof, Summa smart contract is internally calling a verifier contract that contains the cryptographic verification algorithm.

{% hint style="danger" %}
As a user, you are inherently trusting that the verifier contract accurately represents the verifier for the given ZK circuit. This trust is not trivial, as any discrepancy in the verifier's implementation could lead to incorrect verification. Therefore, it is essential for users to actively engage in _**validating the verifier**_. This section will explain how you can generate the verifier contract code from the open source circuit code and check that it matches the deployed verifier.
{% endhint %}

Follow the steps below to validate the verifier:

1. Install Rust & Solc

On OSX and Unix:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run [https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

{% hint style="info" %}
In case of any issues, please refer to the official documentation for the up-to-date instructions on how to install Rust here: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
{% endhint %}

Also, it is required to have `solc` to compile the verifier contract in the following steps. Use this command to install `solc` through `svm`, which is the Solc Version Manager:

```
cargo install --version 0.2.23 svm-rs
svm install 0.8.23
```

2. Clone the Summa repository:

This step assumes that you have Git installed in your system. If not, please refer to installation instructions here: [https://git-scm.com/book/en/v2/Getting-Started-Installing-Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git). After you have Git installed, run

<pre><code><strong>git clone https://github.com/summa-dev/summa-solvency.git
</strong></code></pre>

3. Locate the setup parameters that the Custodian used during the deployment of Summa.

As described in the [setup ](../../backend/summa-solvency/#setup)section, the Custodian has to set generic parameters describing the structure of the circuit.&#x20;

The values of these parameters are also recorded in `Summa.sol` upon its deployment. They are stored in the `config` public field as members of the `SummaConfig` struct.&#x20;

Furthermore, a trusted setup `ptau` file must be used to generate the verifier contract. The Custodian should make this file available to the public

4. Generate the inclusion verifier contract code

The user who wants to check the correct deployment of the verifier contract code has to go through the same [setup process](../../backend/summa-solvency/#3.-generate-verifier-contract) that the Custodian has already gone through. The setup parameters fetched from the previous step must be used here as input for the `gen_inclusion_verifier` script

```
cd zk_prover
cargo run --release --example gen_inclusion_verifier
```

{% hint style="warning" %}
Certain combinations of LEVELS and N\_BYTES can lead to arithmetic overflow during the proof generation. A malicious Custodian may be willing to take advantage of the overflow to understate the liabilities (see a detailed technical description [here](https://github.com/summa-dev/summa-solvency/issues/167)). The verifier generation script mentioned performs the necessary check that the combination of LEVELS and N\_BYTES used for deployment cannot lead to overflow.&#x20;
{% endhint %}

In the case of a combination of `LEVELS` and `N_BYTES` can lead to arithmetic overflow during the proof generation, the following error will appear.&#x20;

```
thread 'main' panicked at 'There is a risk of balance overflow in the Merkle Root, given the combination of `N_BYTES` and `LEVELS`', examples/gen_inclusion_verifier.rs:29:5
```

The script will generate  `InclusionVerifier.sol` contract in `contracts/src`.

5. Go to the current Summa.sol deployment on Etherscan and locate the Verifier contract. Check that the code displayed at [https://etherscan.io/address/\<verifier address>#code](https://etherscan.io/address/0xd90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b#code) matches the code in the generated `InclusionVerifier.sol.`

