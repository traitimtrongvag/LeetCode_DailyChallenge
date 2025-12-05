// 219. Contains Duplicate II
use std::{
	collections::HashSet,
	hash::{BuildHasher, Hasher},
};

#[derive(Default, Clone, Copy)]
struct IdentityHash(i32);
impl Hasher for IdentityHash {
	fn finish(&self) -> u64 {
		self.0 as u64
	}

	fn write(&mut self, _: &[u8]) {
		unreachable!()
	}

	fn write_i32(&mut self, i: i32) {
		self.0 = i
	}
}
impl BuildHasher for IdentityHash {
	type Hasher = Self;

	fn build_hasher(&self) -> Self::Hasher {
		*self
	}
}

impl Solution {
	pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
		let k = k as usize;
		let mut seen = HashSet::with_capacity_and_hasher(nums.len(), IdentityHash::default());
		nums.iter().enumerate().any(|(i, &num)| {
			if !seen.insert(num) {
				true
			} else {
				if seen.len() > k {
					seen.remove(&nums[i - k]);
				}
				false
			}
		})
	}
}