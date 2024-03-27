# Zero Knowledge Proof

A core concept that often helps understand zero knowledge proof, or, more specifically, [zkSNARKs](https://vitalik.ca/general/2021/01/26/snarks.html) is **Computational Integrity Guarantee**.

A computation is any set of rules (or **constraints**) that can be encoded into a computer program.

A computation can be as simple as performing the sum of 2 numbers.

<figure><img src="https://mirror.xyz/_next/image?url=https%3A%2F%2Fimages.mirror-media.xyz%2Fpublication-images%2FbNUf8fJ3hhNZUC0Fjo31z.png&#x26;w=3840&#x26;q=75" alt="" height="540" width="960"><figcaption></figcaption></figure>

A more complex computation is validating blockchain transactions and bundling them into a block.

<figure><img src="https://mirror.xyz/_next/image?url=https%3A%2F%2Fimages.mirror-media.xyz%2Fpublication-images%2FnxXaAGAgoGQ3wUs1x_YxZ.png&#x26;w=3840&#x26;q=75" alt="" height="540" width="960"><figcaption></figcaption></figure>

You can see that a computation is made of a list of inputs, a program that sets the constraints of the computation, and a list of outputs (it can be more than one).

Most of the time, after an actor performs the computation, other people must verify that the computation was done correctly. This is especially relevant in zero-trust contexts like block building (or mining).

More formally, given a computation with constraints known by everyone, a Prover wants to prove to a Verifier that the output is the result of running a computation on certain inputs.

The naive way to achieve such **Computational Integrity Guarantee** is for the Verifier to rerun the same computation with the same inputs and check that the output matches.

<figure><img src="https://mirror.xyz/_next/image?url=https%3A%2F%2Fimages.mirror-media.xyz%2Fpublication-images%2FYyQ7OXyLwYeS9LoD0rjN9.png&#x26;w=3840&#x26;q=75" alt="" height="939" width="600"><figcaption></figcaption></figure>

Such an approach has two main issues:

* The verification time is exactly the same as the time it takes to perform the computation. In order to achieve consensus to a new block header, every node has to perform this computationally intensive operation, which is the main bottleneck to blockchain scalability.
* To achieve computational integrity guarantee the list of inputs and outputs has to be public.

zkSNARKs elegantly solve these two issues by providing a new protocol to run any arbitrary computation that, together with the output, also returns a proof π. Such proof, despite being very tiny and faster to verify than running the original computation, carries enough information to provide the **Computational Integrity Guarantee**.

<figure><img src="https://mirror.xyz/_next/image?url=https%3A%2F%2Fimages.mirror-media.xyz%2Fpublication-images%2FdNRWawjIRWEOoFXhV8uQK.png&#x26;w=3840&#x26;q=75" alt="" height="540" width="960"><figcaption></figcaption></figure>

The Verifier doesn't need to re-run the whole algorithm again but only needs to run a lightweight program using π as input. While the time required by the original computation grows proportionally to its complexity or the size of the inputs, the time to verify a zkSNARK proof grows logarithmically with the complexity/input size, or is even constant.

A further characteristic of such protocols is that the prover can selectively decide whether to keep an input of the computation private or public. The proof provides the verifier with **zero knowledge** of potentially any of the inputs of the computation.

Summa leverages the properties of zkSNARKs to allow an Exchange to generate a Proof of Solvency that:

* Provides a cryptographic-based guarantee that the statement is satisfied.
* It can be verified quickly on any consumer device
* Keeps the sensitive data of the Exchange (and its users) private

As with anything in engineering, switching to a zkSNARK Protocol comes with trade-offs:

* **Trusted Setup**: each zkSNARK protocol relies on a [trusted setup](https://vitalik.ca/general/2022/03/14/trustedsetup.html). You can think of the setup as the parameters that guarantee the integrity of a protocol. These parameters result from a ceremony in which many parties contribute random inputs. If these parties get together and recompose the whole input used to create the parameters, they can potentially attack a ZK protocol and generate valid proofs even without performing the computation that follows the pre-defined rules.
* **Prover Overhead**: the reduction of the verification time comes at the cost of proving time. In fact, running the same computation inside a ZK circuit takes, on average, > 100 times more than performing it without generating a ZK proof.

Summa uses [Halo2](https://github.com/privacy-scaling-explorations/halo2) PSE fork, a proving system that ZCash originally built. Beyond high proving speed, Halo2 allows the reuse of existing and reputable trusted setups, such as the [Hermez 1.0 Trusted Setup](https://docs.hermez.io/Hermez\_1.0/about/security/#multi-party-computation-for-the-trusted-setup), for any application-specific circuit.
