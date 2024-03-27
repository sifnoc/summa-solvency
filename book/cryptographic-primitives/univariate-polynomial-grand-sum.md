# Univariate Polynomial Grand Sum

## Preliminaries

### Modular Arithmetic

An intuitive example of modular arithmetic from our daily life is the "clock arithmetic". When we see "19:00" boarding time on a boarding pass, we know that it corresponds to "7" on the clock face. Formally, in this case we perform the modular reduction by the modulus 12:

$$
19 \equiv 7\pmod{12}
$$

because the clock face only has twelve hours marked on it.

### Roots Of Unity In Modular Arithmetic

An integer $$\omega$$ is an $$n$$-th root of unity modulo $$p$$ if $$\omega^n \equiv 1 \pmod{p}$$

In other words, $$\omega^n$$ "wraps around" due to modular reduction to yield exactly $$1$$. A particular case is when no integers smaller than $$n$$ yield $$1$$ modulo $$p$$. In this case, $$\omega$$ is called a _primitive_ root of unity: $$\omega^k \not\equiv 1 \pmod{p}$$ for any $$0 < k < n$$.

### Roots Of Unity Example

Let's observe a finite field of order $$p = 7$$: $$\mathbb{F}_7 = {0, 1,2,3,4,5,6}$$. Let's see that $$2$$ and $$4$$ are the 3rd roots of unity in such a field:

* $$2^3 \equiv 8 \equiv 1 \pmod{7}$$, so 2 is a 3rd root of unity modulo 7.
* $$4^3 \equiv 64 \equiv 1 \pmod{7}$$, so 4 is another 3rd root of unity modulo 7.

$$1$$ itself is a root of unity, too, and is called a trivial root of unity.

### Special Property Of The Sum Of Roots of Unity

Let's consider a finite field $$\mathbb{F}_p$$ that has $$n$$-th roots of unity. Let $$\omega$$ be a primitive $$n$$-th root of unity in $$\mathbb{F}_p$$, which means $$\omega^n = 1$$ and no smaller positive power of $$\omega$$ equals $$1$$.

The $$n$$-th roots of unity in $$\mathbb{F}_p$$ are $$1, \omega, \omega^2, \ldots, \omega^{n-1}$$.

**Claim**: $$1 + \omega + \omega^2 + \ldots + \omega^{n-1} = 0$$ (**the sum of all the roots of unity in a finite field is equal to zero**).

**Proof**:

Consider the sum $$S = 1 + \omega + \omega^2 + \ldots + \omega^{n-1}$$. We can multiply $$S$$ by $$\omega - 1$$, noting that $$\omega - 1 \neq 0$$ so that such a multiplication preserves the equality:

$$
(\omega - 1)S = (\omega - 1)(1 + \omega + \omega^2 + \ldots + \omega^{n-1})
$$

Expanding the right hand side, we get:

$$
\omega + \omega^2 + \omega^3 + \ldots + \omega^n - (1 + \omega + \omega^2 + \ldots + \omega^{n-1})
$$

Notice that if we were to expand further, every term except $$\omega^n$$ and $$-1$$ would cancel out:&#x20;

$$
(\omega - 1)S = \omega^n - 1
$$

Since $$\omega$$ is a primitive $$n$$-th root of unity, $$\omega^n = 1$$. So, $$\omega^n - 1 = 0$$. Therefore:

$$
(\omega - 1)S = 0
$$

If the product of two factors is zero, at least one of them must be zero. We already established that $$\omega - 1 \neq 0$$, thus $$S$$ must be zero: $$S = 0$$. Therefore,&#x20;

$$
\boxed{1 + \omega + \omega^2 + \ldots + \omega^{n-1} = 0}
$$

Let's also check it on our previous toy example of $$\mathbb{F}_7$$ and $$n = 3$$: $$1 + 2 + 4 = 7 \equiv 0 \pmod{7}$$.

Let's see how we can take advantage of _the sum of all roots of unity being zero_ when applied to the proof of solvency.

## Data Structure & Commitment Scheme

The desired commitment scheme for Summa should have the following properties:

* Committing to the total liabilities of the Custodian that is the sum of all user balances;
* Allowing to publicly reveal the total liabilities without revealing any information about user balances;
* Allowing to prove the individual user inclusion into the commitment without revealing any information about other user balances;
* Preserving the user privacy and hiding the user data (namely the user cryptocurrency balances);
* Outperform the Merkle sum tree in both commitment phase and proving phase.

We will demonstrate how a polynomial commitment can be used to achieve all of these properties.

Let's consider a polynomial $$B(X)$$ that evaluates to an $$i$$-th user balance $$b_i$$ at a "user's point" - some value $$x_i$$ that is designated for this specific user:

$$B(x_i) = b_i$$.

We can call it a user balance polynomial. It is quite easy to construct such a polynomial using the Lagrange interpolation. The formula for the polynomial that interpolates these data points is:

$$
B(X) = \sum_{i=1}^{n} b_i \cdot L_i(X)
$$

Where $$L_i(X)$$ is the Lagrange basis polynomial defined as a product:

$$
L_i(X) = \prod_{\substack{j=1 \ j \neq i}}^{n} \frac{X - x_j}{x_i - x_j}
$$

A polynomial constructed using the Lagrange interpolation is known to have the degree $$d = n - 1$$ where $$n$$ is the number of users (and the number of the corresponding balance evaluation points). The resulting polynomial should look like the following:

$$
\boxed{B(X) = a_0 + a_1x + a_2x^2 + ... + a_{n-1} x^{n-1}}
$$

Let's choose the $$x_i$$ values as the $$i$$-th degrees of an $$n$$-th root of unity (assuming that we are performing all the calculations in the prime field with a sufficiently large modulus):

$$\boxed{B(\omega^i) = b_i}$$ where $$\omega$$ is the $$n$$-th primitive root of unity and $$i\in0..n-1$$.

## Grand Sum Of The Polynomial Evaluations

To prove the solvency of the Custodian, we need to find its total liabilities by summing up all the user balances and to prove to the public that the sum is less than the assets owned by the Custodian. An individual $$i$$-th user balance is the evaluation of the polynomial at the $$\omega^i$$ value corresponding to the user:

$$
B(\omega^i) = b_i =a_0 + a_1(\omega^i)^1 + a_2(\omega^i)^2 + ... + a_{n-1} (\omega^i)^{n-1}
$$

Let's calculate the sum $$S$$ of all the user balances as the sum of the polynomial evaluations:

$$
\begin{align*}
S = \sum\limits_{i=0..n-1} B(\omega^i)& = &a_0\quad& + &a_1\omega^{0\phantom{-1}}\quad & + & a_2(\omega^{0\phantom{-1}})^2\quad & + & \cdots\quad & + & a_{n-1} (\omega^{0\phantom{-1}})^{n-1} +\\
& + &a_0\quad& + &a_1\omega^{1\phantom{-1}}\quad & + & a_2(\omega^{1\phantom{-1}})^2\quad & + & \cdots\quad & + & a_{n-1} (\omega^{1\phantom{-1}})^{n-1} +\\
& + &a_0\quad& + &a_1\omega^{2\phantom{-1}}\quad & + & a_2(\omega^{2\phantom{-1}})^2\quad & + & \cdots\quad & + & a_{n-1} (\omega^{2\phantom{-1}})^{n-1} +\\
&&&&&\vdots \\
& + &a_0\quad& + &a_1\omega^{n-1}\quad & + & a_2(\omega^{n-1})^2\quad & + & \cdots\quad & + & a_{n-1} (\omega^{n-1})^{n-1} =\\
\\
\rlap{\text{(let's factor out each $a_i$)}}
\end{align*}
$$

$$
\begin{align*}&=n a_0 + a_1(\underbrace{\omega^0 + \omega^1 + \omega^2 + \cdots +\omega^{n-1}}_{=0}) + \cdots + a_{n-1}(\underbrace{\omega^0 + \omega^1 + \omega^2 +\cdots+ \omega^{n-1} }_{=0})^{n-1} =
\\
&\rlap{\text{(using the property of the sum of all roots of unity inside the parentheses being zero)}}
\\\quad
\\&= n a_0
\end{align*}
$$

Therefore, the grand sum of the user balances is simply the constant coefficient of the polynomial times the number of users:

$$
\boxed{S = \sum\limits_{i} B(\omega_i) = n a_0}
$$

As it turns out, the Halo2 proving system is internally using the roots of unity as $$X$$ coordinates for the polynomial construction, and we will later see how we can take advantage of that.
