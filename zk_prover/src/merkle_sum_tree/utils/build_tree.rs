use crate::merkle_sum_tree::utils::create_middle_node::create_middle_node;
use crate::merkle_sum_tree::{Entry, Node};
use halo2_proofs::halo2curves::bn256::Fr as Fp;
use rayon::prelude::*;

pub fn build_merkle_tree_from_entries<const N_ASSETS: usize>(
    entries: &[Entry<N_ASSETS>],
    depth: usize,
    nodes: &mut Vec<Vec<Node<N_ASSETS>>>,
) -> Result<Node<N_ASSETS>, Box<dyn std::error::Error>>
where
    [usize; N_ASSETS + 1]: Sized,
    [usize; 2 * (1 + N_ASSETS)]: Sized,
{
    let n = entries.len();

    let mut tree: Vec<Vec<Node<N_ASSETS>>> = Vec::with_capacity(depth + 1);

    tree.push(vec![
        Node {
            hash: Fp::from(0),
            balances: [Fp::from(0); N_ASSETS]
        };
        n
    ]);

    for _ in 1..=depth {
        let previous_level = tree.last().unwrap();
        let nodes_in_level = (previous_level.len() + 1) / 2;

        tree.push(vec![
            Node {
                hash: Fp::from(0),
                balances: [Fp::from(0); N_ASSETS]
            };
            nodes_in_level
        ]);
    }

    build_leaves_level(entries, &mut tree);

    for level in 1..=depth {
        build_middle_level(level, &mut tree, n)
    }

    let root = tree[depth][0].clone();
    *nodes = tree;
    Ok(root)
}

fn build_leaves_level<const N_ASSETS: usize>(
    entries: &[Entry<N_ASSETS>],
    tree: &mut [Vec<Node<N_ASSETS>>],
) where
    [usize; N_ASSETS + 1]: Sized,
{
    let results = entries
        .par_iter()
        .map(|entry| entry.compute_leaf())
        .collect::<Vec<_>>();

    for (index, node) in results.into_iter().enumerate() {
        tree[0][index] = node;
    }
}

fn build_middle_level<const N_ASSETS: usize>(
    level: usize,
    tree: &mut [Vec<Node<N_ASSETS>>],
    n: usize,
) where
    [usize; 2 * (1 + N_ASSETS)]: Sized,
{
    let nodes_in_level = (n + (1 << level) - 1) / (1 << level);

    let results: Vec<_> = tree[level - 1]
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            if index % 2 == 0 && index + 1 < nodes_in_level {
                Some(create_middle_node(node, &tree[level - 1][index + 1]))
            } else {
                None
            }
        })
        .par_bridge() // Convert the standard iterator to a parallel iterator
        .collect();

    for (index, new_node) in results.into_iter().enumerate() {
        tree[level][index] = new_node;
    }
}
