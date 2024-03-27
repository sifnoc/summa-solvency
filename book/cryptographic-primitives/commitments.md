# Commitments

[Cryptographic Commitment Schemes](https://en.wikipedia.org/wiki/Commitment\_scheme) are a fundamental component in cryptography, particularly in protocols that require a party to commit to a value without revealing it initially and then reveal the value (or just some property of the value) later.

The simplest and most popular cryptographic commitment scheme is hashing.

Let us consider a scenario in which Alice wants to publicly commit to a prediction about an upcoming event, say the first scorer in a football match, without revealing her prediction until the match has ended.

Alice can take her winner prediction "BenjaminPavard", run a hash function on top of this, and publish the resulting output (hash digest) on her Twitter profile. At this point, no one can learn Alice's prediction just from the hash digest.

<figure><img src="https://mirror.xyz/_next/image?url=https%3A%2F%2Fimages.mirror-media.xyz%2Fpublication-images%2F_uLicpcXlCPnFB2Y40Jd5.png&#x26;w=3840&#x26;q=75" alt="" height="112" width="822"><figcaption></figcaption></figure>

In fact, to decrease the likelihood that someone unmasks Alice's hash and discovers her prediction, it would be safer to add some random large number (technically known as _salt_) together with the prediction as hash input, in order to avoid brute-force and [Rainbow Table attacks](https://en.wikipedia.org/wiki/Rainbow\_table).

After the end of the event, she can reveal her prediction _"BenjaminPavard"_. Anyone can re-run the hash function on the prediction to check whether it matches the _hashDigest_ previously published on Twitter. If Alice reveals a different prediction, such as _"TheoHernandez"_, the result of hashing such a prediction will result in something different from the previously published _hashDigest_.

Cryptographic commitment schemes guarantee two very cool properties:

1. **Hiding**: The commitment hides the value, and everyone who sees the commitment cannot determine the actual value until Alice decides to reveal it.
2. **Binding**: Once Alice has made the commitment, she cannot change the value she committed to. In other words, when revealing, the committer cannot reveal different values as these wouldn’t match the original commitment. That’s because modern hash functions are computationally collision-free and we cannot find two inputs that result in the same hash digest

\
