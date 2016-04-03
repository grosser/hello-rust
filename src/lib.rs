#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

pub struct Monster {
  health: usize
}

impl Monster {
  pub fn new(health: usize) -> Monster {
    Monster {
      health: health
    }
  }

  pub fn healthy(&self) -> bool {
    if self.health >= 50 {
      true
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  pub use super::Monster;

  describe! monster {
    describe! healthy {
      it "is healthy when above 50" {
        let monster = Monster::new(50);
        assert_eq!(true, monster.healthy());
      }

      it "is unhealthy when below 50" {
        let monster = Monster::new(49);
        assert_eq!(false, monster.healthy());
      }
    }
  }
}
