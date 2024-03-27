# Verifiers

Summa utilizes three verifier contracts and one key contract: [**SnarkVerifier**](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/SnarkVerifier.sol), [**GrandSumVerifier**](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/GrandSumVerifier.sol), [**InclusionVerifier**](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/InclusionVerifier.sol), and [**VerifyingKey**](https://github.com/summa-dev/summa-solvency/blob/v2/contracts/src/VerifyingKey.sol) . The **SnarkVerifier** and **VerifyingKey** contracts must be regenerated whenever custodians redeploy the Summa contract with new configurations, as they are specifically generated to a particular circuit configuration by the [`halo2-solidity-verifier`](https://github.com/summa-dev/halo2-solidity-verifier).

For example, let's assume the Summa contract is initially deployed with the configuration to support two cryptocurrencies as specified by the custodian. This setup allows users to verify their inclusion proofs containing two currencies within the Summa contract. If the custodian decides to accommodate three cryptocurrencies for user convenience, they must redeploy the Summa contract with newly generated SnarkVerifier and VerifyingKey contracts configured for three cryptocurrencies.

In contrast, the **GrandSumVerifier** and **InclusionVerifier** contracts do not need to be regenerated for new deployments. These contracts are capable of verifying proofs regardless of size, including any number of cryptocurrencies included in the proof.

This means that custodians might not need to deploy these contracts themselves if they are already deployed on EVM.

{% hint style="info" %}
**Note:** Generating a `SnarkVerifier` for more than 9 currencies may exceed the Ethereum mainnet contract size limit.
{% endhint %}
