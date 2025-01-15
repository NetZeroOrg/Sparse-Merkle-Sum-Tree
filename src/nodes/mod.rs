use crate::{record::Record, secret::Secret};

pub mod node;
pub mod partial;

// Common trait for both partial and full node

pub trait TreeNode {
    fn new_leaf<const N_CURR: usize>(
        blinding_factor: Secret,
        record: Record<N_CURR>,
        user_salt: Secret,
    ) -> Self;

    fn new_pad(blinding_factor: Secret, hieght: u8, level_offset: u64, user_salt: Secret) -> Self;

    fn merge(left_child: &Self, right_child: &Self) -> Self;
}
