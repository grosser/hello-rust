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

  pub fn fizzbuzz(&self) -> &str {
    if self.health % 15 == 0 {
      "FizzBuzz"
    } else if self.health % 5 == 0 {
      "Fizz"
    } else if self.health % 3 == 0 {
      "Buzz"
    } else {
      ""
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

    describe! fizzbuzz {
      it "is nothing for non-matching" {
        let monster = Monster::new(1, 1);
        assert_eq!("", monster.fizzbuzz())
      }

      it "is FizzBuzz for mod 15" {
        let monster = Monster::new(30, 1);
        assert_eq!("FizzBuzz", monster.fizzbuzz())
      }

      it "is Fizz for mod 5" {
        let monster = Monster::new(10, 1);
        assert_eq!("Fizz", monster.fizzbuzz())
      }

      it "is Buzz for mod 3" {
        let monster = Monster::new(9, 1);
        assert_eq!("Buzz", monster.fizzbuzz())
      }
    }
  }
}
