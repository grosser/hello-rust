#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[cfg(test)]
mod tests {
	pub use std::collections::HashMap;

	describe! stainless {
		it "can use HashMap" {
			let map = HashMap::new();
		}
	}
}
