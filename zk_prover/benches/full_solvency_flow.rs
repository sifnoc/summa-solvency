#![feature(generic_const_exprs)]
use criterion::{criterion_group, criterion_main, Criterion};
use halo2_proofs::plonk::{keygen_pk, keygen_vk};
use rand::rngs::OsRng;
use rand::Rng;
use summa_solvency::{
    circuits::merkle_sum_tree::MstInclusionCircuit,
    circuits::{
        utils::{full_prover, full_verifier, generate_setup_artifacts},
        WithInstances,
    },
    merkle_sum_tree::{utils::generate_dummy_entries, Cryptocurrency, MerkleSumTree, Tree},
};

fn bench_mst<
    const K: u32,
    const LEVELS: usize,
    const N_USERS: usize,
    const N_BYTES: usize,
    const N_CURRENCIES: usize,
>(
    name: &str,
) where
    [(); N_CURRENCIES + 1]: Sized,
    [(); N_CURRENCIES + 2]: Sized,
{
    let mut c = Criterion::default().sample_size(10);

    let entries = generate_dummy_entries::<N_USERS, N_CURRENCIES>().unwrap();

    let cryptocurrencies: [Cryptocurrency; N_CURRENCIES] =
        std::array::from_fn(|_| Cryptocurrency {
            name: "ETH".to_string(),
            chain: "ETH".to_string(),
        });

    c.bench_function(&format!("{name} - build merkle sum tree"), |b| {
        b.iter_batched(
            || (entries.clone(), cryptocurrencies.to_vec()),
            |(entries, cryptocurrencies)| {
                let _ = MerkleSumTree::<N_CURRENCIES, N_BYTES>::from_entries(
                    entries,
                    cryptocurrencies,
                    false,
                )
                .unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function(&format!("{name} - build sorted merkle sum tree"), |b| {
        b.iter_batched(
            || (entries.clone(), cryptocurrencies.to_vec()),
            |(entries, cryptocurrencies)| {
                let _ = MerkleSumTree::<N_CURRENCIES, N_BYTES>::from_entries(
                    entries,
                    cryptocurrencies,
                    true,
                )
                .unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Generate a random user index
    let get_random_user_index = || {
        let user_range: std::ops::Range<usize> = 0..N_USERS;
        OsRng.gen_range(user_range)
    };

    let merkle_sum_tree = MerkleSumTree::<N_CURRENCIES, N_BYTES>::from_entries(
        entries,
        cryptocurrencies.to_vec(),
        false,
    )
    .unwrap();

    c.bench_function(&format!("{name} - generate merkle proof"), |b| {
        b.iter_batched(
            || (get_random_user_index()),
            |user_index| {
                let _ = merkle_sum_tree.generate_proof(user_index).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });

    let user_index = get_random_user_index();

    let merkle_proof = merkle_sum_tree.generate_proof(user_index).unwrap();

    let circuit = MstInclusionCircuit::<LEVELS, N_CURRENCIES, N_BYTES>::init(merkle_proof);

    let empty_circuit = MstInclusionCircuit::<LEVELS, N_CURRENCIES, N_BYTES>::init_empty();

    let (params, pk, vk) = generate_setup_artifacts(K, None, empty_circuit.clone()).unwrap();

    c.bench_function(&format!("{name} - gen verification key"), |b| {
        b.iter_batched(
            || empty_circuit.clone(),
            |empty_circuit| {
                keygen_vk(&params, &empty_circuit).expect("vk generation should not fail");
            },
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{name} - gen proving key"), |b| {
        b.iter_batched(
            || (vk.clone(), empty_circuit.clone()),
            |(vk, empty_circuit)| {
                keygen_pk(&params, vk, &empty_circuit).expect("pk generation should not fail");
            },
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{name} - generate inclusion zk proof"), |b| {
        b.iter_batched(
            || (pk.clone(), vk.clone(), circuit.clone(), circuit.instances()),
            |(pk, vk, circuit, instances)| {
                full_prover(&params, &pk, circuit, instances);
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // verify inclusion zk proof
    let proof = full_prover(&params, &pk, circuit.clone(), circuit.instances());

    c.bench_function(&format!("{name} - verify inclusion zk proof"), |b| {
        b.iter_batched(
            || {
                (
                    vk.clone(),
                    proof.clone(),
                    circuit.clone(),
                    circuit.instances(),
                )
            },
            |(vk, proof, circuit, instances)| {
                full_verifier(&params, &vk, proof, instances);
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn criterion_benchmark(_c: &mut Criterion) {
    const K: u32 = 13;
    {
        const N_CURRENCIES: usize = 1;
        const LEVELS: usize = 18;
        const N_USERS: usize = 2usize.pow(LEVELS as u32) - 6;
        bench_mst::<K, LEVELS, N_USERS, 8, N_CURRENCIES>(
            format!("K = {K}, N_USERS = {N_USERS}, N_CURRENCIES = {N_CURRENCIES}").as_str(),
        );
    }
    {
        const N_CURRENCIES: usize = 350;
        const LEVELS: usize = 18;
        const N_USERS: usize = 2usize.pow(LEVELS as u32) - 6;
        bench_mst::<K, LEVELS, N_USERS, 8, N_CURRENCIES>(
            format!("K = {K}, N_USERS = {N_USERS}, N_CURRENCIES = {N_CURRENCIES}").as_str(),
        );
    }
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
