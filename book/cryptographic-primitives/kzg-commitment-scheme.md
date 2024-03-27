# KZG Commitment Scheme

We choose a KZG commitment scheme to commit to the user balance polynomials for the compatibility with Halo2 API (more on that later). In brief, a KZG commitment is a single finite field element $$C$$ that uniquely represents a polynomial $$B(X)$$.

It is impossible to reconstruct the polynomial from the commitment, so our requirement of user privacy is satisfied because it is impossible to infer any evaluations of the polynomial from the single-value commitment $$C$$.

During the reveal (aka opening) phase, the committed value $$C$$ is used along with the claimed polynomial evaluation $$B(x)$$ to provide a succinct proof $$\pi$$ verifying that the value $$B(x)$$ is indeed an evaluation of a polynomial $$B(X)$$ at point $$x$$ and corresponds to the original commitment $$C$$. Therefore, KZG commitment allows the Custodian to individually provide the opening proofs $$\pi_i$$ to each user to prove that the polynomial $$B(X)$$ indeed evaluates to the user balance $$b_i$$ at the point $$x_i = \omega^i$$. Knowing $$\langle C, B(\omega^i),\pi\rangle$$, the user is able to verify the opening.

More broadly, the KZG commitment allows the prover to open the polynomial at _any_ point, and we will later see how it benefits our case.

## Proof Of Solvency

Using the described polynomial construction technique and the KZG commitment, it is sufficient for the Custodian to "open" the KZG commitment at $$x = 0$$:

$$
\langle C, B(0),\pi_{x=0}\rangle: B(0) = a_0 + a_10 + a_20^2 + ... + a_{n-1} 0^{n-1} = a_0
$$

The total liabilities can then be calculated by multiplying the $$a_0$$ value by the number of users:

$$
S = n a_0
$$

## Proof Of Inclusion

As described in the KZG section, individual users would receive the KZG opening proofs $$\langle C, B(\omega^i),\pi_i\rangle$$ at their specific point $$\omega^i$$ and they would be able to check that

* the opening evaluation is equal to their balance: $$B(\omega^i) = b^i$$;
* the opening proof $$\pi_i$$ corresponds to the public KZG commitment $$C$$.

The caveat is that if two or more users have the same cryptocurrency balance value, a malicious Custodian could give them the same KZG proof because the user index $$i$$ is defined by the Custodian. We will use the following technique to mitigate this:

* the Custodian has to additionally commit to another polynomial that evaluates to the hashes of user IDs at the specific user points: $$H(\omega^i) = h_i$$;
* the user ID should be known to the user (e.g, the email address used to register with the Custodian), so the user can check that the value $$h_i$$ is indeed the hash of their ID;
* the Custodian then gives two KZG commitments and two opening proofs to the user - $$\langle C_B, B(\omega^i),\pi_B\rangle$$ proving the balance inclusion into the balances polynomial, and $$\langle C_H, H(\omega^i),\pi_H\rangle$$ proving the user ID hash inclusion into the ID hash polynomial.
