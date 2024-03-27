---
description: >-
  This section lists a series of open problems and challenges regarding the
  design of Summa.
---

# Open Issues

### Dispute Resolution

As described in [Provisions: Privacy-preserving proofs of solvency for Bitcoin exchanges](https://eprint.iacr.org/2015/1008.pdf), the current version of Summa does not provide dispute resolution.&#x20;

> _If a user finds their account missing or balance incorrect, they do not have sufficient cryptographic evidence that this is the case. The issue appears unsolvable cryptographically. Recall that the primary motivation for users keeping funds with an exchange is to avoid needing to remember long-term cryptographic secrets, therefore exchanges must be able to execute user orders and change their balance without cryptographic authentication from the user (e.g., password authentication). Resolving this will likely require legal regulation. Users who dislike an exchange may also falsely claim that verification of their accounts failed, and it is not possible to judge if the user or the exchange is correct in this case based on a Provisions transcript alone._

### Timing of Proofs of Inclusion Generation&#x20;

At the state of the art, generating a single [Proof of Inclusion](introduction/protocol-flow.md#3.-inclusion-proof) takes between 100ms and 500ms, considering different machine powers and different generic parameters of the proof. Given that a custodial can potentially have millions of users, generating all these proofs at once would be very time and resource-consuming. Let's do some paper math.&#x20;

$$
250ms * 10.000.000= 2.500.000.000 ms = 28,93518518519 days
$$

For a custodial with a userbase of 10M users, generating Proofs of Inclusion for the entire userbase would take 28 days. The process can be parallelized efficiently across many machines, but it would still represent a major infrastructure cost. And this has to be performed for every round!

Summa solves that by advising custodials to generate the proof only whenever a user requests it. But this gives an information advantage to the custodial. Knowing which users requested their proof (and therefore performed the verification) across multiple rounds allows them to forecast which users are likely never to request their proof of inclusion. This information can be used to exclude these "lazy" users from liability accounting and get away with it.&#x20;

This attack is described in [Chalkias, Chatzigiannis, Ji - "Broken Proofs of Solvency in Blockchain Custodial Wallets and Exchanges." (2022)](https://eprint.iacr.org/2022/043.pdf) - section 4.6

### Interactivity&#x20;

The reliability of a Proof of Solvency depends on the number of users correctly verifying their correct inclusion in the liability commitment. Because of that, we define the process as interactive. The user who verifies their [Proof of Inclusion](introduction/protocol-flow.md#3.-inclusion-proof) can only tell that their balance was accounted for correctly, but this doesn't tell anything about other users. Furthermore, individual users don't know the percentage of users who correctly verified their inclusion, making it harder to assess the actual solvency of the custodial.

In an ideal world, the custodial would only generate a single Proof of Solvency that, if verified, tells with no doubts that the custodial is solvent. Without having to depend on an interactive phase of individual user verification.&#x20;

One way to go is to let users sign their trades and commit to these signed trades on a timestamped public ledger. But this is exactly what a Decentralized Exchange already does, bringing issues such as key management, privacy, high latency, and transaction fees, which are exactly the problems that custodial aims to solve.

Another, more sophisticated, solution is to force the custodial to generate state transition proofs for every update of their database. The constraints are set such that:&#x20;

* For every user deposit, a balance must be added to the database&#x20;
* For every trade, the same amount must be subtracted from a user's balance and added to another user's balance
* For every user withdrawal, a balance must be subtracted from the database

### Asset Privacy

The original goal of Summa was to provide a fully private Proof of Solvency protocol. A custodial should generate a valid Proof of Solvency without having to reveal sensitive data such as:&#x20;

* The number of users&#x20;
* The balances of users&#x20;
* The amount of liabilities
* The amount of assets&#x20;
* The addresses they use to custody such funds

Summa achieves such a level of privacy regarding the users' side but doesn't protect any information regarding the assets (and the addresses) side. The solution would be the following:&#x20;

* Committing to address data either via a custom Merkle Tree, as suggested by [Provisions: Privacy-preserving proofs of solvency for Bitcoin exchanges](https://eprint.iacr.org/2015/1008.pdf), or leveraging existing data structure such as Patricia Merkle Trie for Ethereum
* Generating a [Zero Knowledge Proof](cryptographic-primitives/zero-knowledge-proof.md) that:
  1. The custodial controls an address
  2. This address owns an amount of coins&#x20;
  3. The tuple `(address, amount)` is included in the commitment
  4. This amount is greater than the liabilities

To prove 1, the custodial must provide the private key or a digital signature. For practical and security reasons, the custodians won't likely use their private key as input for the proof generation, so we default to digital signature. Given that the Zero Knowledge Proof is hiding the address, we need a way to make sure that different custodials claim ownership over the same address. We need a way to [nullify](https://blog.aayushg.com/nullifier/#one-address-one-nullifier) the address. Unfortunately, ECDSA signatures are [malleable](https://twitter.com/0xPARC/status/1493704577002049537?s=20\&t=X-5Bs1oWNjmbTASp2T82DA), so you cannot rely on them to build a secure nullifier scheme. You may probably find more luck with deterministic signature schemes such as EDDSA. You can find a more detailed analysis of the problem [here](https://hackmd.io/iezJ3Z0dQQmgHO3RicNxzg).
