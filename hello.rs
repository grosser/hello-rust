#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[cfg(test)]
mod tests {
	pub use std::collections::HashMap;

	describe! stainless {
		it "can use HashMap" {
			let mut map = HashMap::new();
			map.insert(1, 1);
			assert!(map.contains_key(&1));
		}
	}
}
