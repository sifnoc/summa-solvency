use halo2_proofs::{
    arithmetic::{best_fft, eval_polynomial, kate_division, Field},
    halo2curves::{
        bn256::{Bn256, Fr as Fp, G1Affine, G2Affine, Gt, G1},
        group::{prime::PrimeCurveAffine, Curve, Group},
        pairing::{Engine, PairingCurveAffine},
    },
    poly::{
        commitment::{Blind, CommitmentScheme, Params, ParamsProver},
        kzg::commitment::ParamsKZG,
        Coeff, EvaluationDomain, Polynomial,
    },
};

pub fn commit_kzg(params: &ParamsKZG<Bn256>, poly: &Polynomial<Fp, Coeff>) -> G1 {
    params.commit(poly, Blind::default())
}

/// Computes the polynomial h(X) for the batch KZG algorithm.
pub fn compute_h(params: &ParamsKZG<Bn256>, f_poly: &Polynomial<Fp, Coeff>) -> Vec<G1> {
    // Double the polynomial length, thus K + 1
    let double_domain = EvaluationDomain::new(1, params.k() + 1);

    let d = f_poly.len(); // Degree of the polynomial

    // Extract s_commitments from ParamsKZG and extend with neutral elements
    let mut s_commitments_reversed = params
        .get_g()
        .iter()
        .map(PrimeCurveAffine::to_curve)
        .collect::<Vec<_>>();
    s_commitments_reversed.reverse();

    let mut y: Vec<G1> = vec![G1::identity(); 2 * d];
    y[..d].copy_from_slice(&s_commitments_reversed[..d]);

    // Prepare coefficients vector and zero-pad at the beginning
    let mut v = vec![Fp::zero(); 2 * d];
    v[d..(2 * d)].copy_from_slice(f_poly);

    let nu = double_domain.get_omega(); // 2d-th root of unity
                                        // Perform FFT on s
    best_fft(&mut y, nu, (2 * d).trailing_zeros());
    // Perform FFT on c
    best_fft(&mut v, nu, (2 * d).trailing_zeros());

    // Perform the Hadamard product
    let u: Vec<G1> = y.iter().zip(v.iter()).map(|(&y, &v)| y * v).collect();

    // Perform inverse FFT
    let nu_inv = nu.invert().unwrap(); // Inverse of 2d-th root of unity
    let mut h = u;
    // Perform inverse FFT on h
    best_fft(&mut h, nu_inv, (2 * d).trailing_zeros());

    // Scale the result by the size of the vector (part of the iFFT)
    let n_inv = Fp::from(2 * d as u64).invert().unwrap();
    h.iter_mut().for_each(|h| *h *= n_inv);

    // Truncate to get the first d coefficients
    h.truncate(d);

    h
}

//J Thaler, Proofs, Arguments, and Zero-Knowledge, 15.2
//KZG proof π is a proof of f(y) = z: π[f(y) = z] = C_Ty, where C_Ty is a commitment to a polynomial Ty(X) = (f(X)−z)/(X−y) and y is the challenge
pub fn create_standard_kzg_proof<
    Scheme: CommitmentScheme<Curve = halo2_proofs::halo2curves::bn256::G1Affine, Scalar = Fp>,
>(
    params: &ParamsKZG<Bn256>,
    domain: &EvaluationDomain<Fp>,
    f_poly: &Polynomial<<Scheme as CommitmentScheme>::Scalar, Coeff>,
    challenge: Fp,
) -> G1 {
    let z = eval_polynomial(f_poly, challenge);
    let numerator = f_poly - z;
    let mut t_y_vals = kate_division(&numerator.to_vec(), challenge);
    // The resulting degree is one less than the degree of the numerator, so we need to pad it with zeros back to the original polynomial size
    t_y_vals.resize(f_poly.len(), Fp::ZERO);
    let t_y = domain.coeff_from_vec(t_y_vals);
    commit_kzg(params, &t_y)
}

//J Thaler, Proofs, Arguments, and Zero-Knowledge, 15.2
// e(c·g^(−z),g) = e(π,g^τ ·g^(−y)), y is the challenge
pub fn verify_kzg_proof(params: &ParamsKZG<Bn256>, c: G1, pi: G1, challenge: &Fp, z: &Fp) -> bool
where
    G1Affine: PairingCurveAffine<Pair = G2Affine, PairingResult = Gt>,
{
    let g_to_minus_z = G1Affine::generator() * &(-z);
    let c_g_to_minus_z: G1 = c + g_to_minus_z;
    let left_side = Bn256::pairing(&c_g_to_minus_z.to_affine(), &G2Affine::generator());

    let g_to_minus_y = G2Affine::generator() * (-challenge);
    let g_tau = params.s_g2();
    let g_tau_g_to_minus_y = g_tau + g_to_minus_y;
    let right_side = Bn256::pairing(&pi.to_affine(), &g_tau_g_to_minus_y.to_affine());

    left_side == right_side
}
