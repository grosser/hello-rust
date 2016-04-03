#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

pub struct Monster {
  health: usize,
  eyes: usize
}

impl Monster {
  pub fn new(health: usize, eyes: usize) -> Monster {
    Monster {
      health: health,
      eyes: eyes
    }
  }

  pub fn healthy(&self) -> bool {
    if self.health >= 50 {
      true
    } else {
      false
    }
  }

  pub fn friendly(&self) -> &str {
    match self.eyes {
      1 => "evil",
      2 => "normal",
      _ => "cute"
    }
  }
}

#[cfg(test)]
mod tests {
  pub use super::Monster;

  describe! monster {
    describe! healthy {
      it "is healthy when above 50" {
        let monster = Monster::new(50, 1);
        assert_eq!(true, monster.healthy());
      }

      it "is unhealthy when below 50" {
        let monster = Monster::new(49, 1);
        assert_eq!(false, monster.healthy());
      }
    }

    describe! friendly {
      it "is evil with 1 eye" {
        let monster = Monster::new(49, 1);
        assert_eq!("evil", monster.friendly());
      }

      it "is normal with 2 eyes" {
        let monster = Monster::new(49, 2);
        assert_eq!("normal", monster.friendly());
      }

      it "is cute with 3 eyes" {
        let monster = Monster::new(49, 3);
        assert_eq!("cute", monster.friendly());
      }
    }
  }
}
