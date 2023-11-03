#[derive(Debug, PartialEq)]
struct Poop {
    amount: u32,
}

struct Baby {
    milk_in_belly: u32,
    poop: u32,
}

impl Default for Baby {
    fn default() -> Self {
        Baby {
            milk_in_belly: 0,
            poop: 0,
        }
    }
}

impl Baby {
    pub fn feed_milk(&mut self, milk_mml: u32) {
        self.milk_in_belly += milk_mml;
    }

    pub fn digest_milk(&mut self) {
        self.poop = 0.max(self.poop + self.milk_in_belly);
        self.milk_in_belly = 0.max(self.milk_in_belly - 10);
    }

    pub fn open_windle(&self) -> Option<Poop> {
        // If there is no poop - return None
        // If there is poop - return the amount as Some(amount)
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::option_poop::{Baby, Poop};

    #[test]
    fn when_the_baby_has_not_eaten_there_is_no_poop() {
        // given
        let baby = Baby::default();

        // when
        let maybe_poop = baby.open_windle();

        // then
        assert_eq!(maybe_poop, None);
    }

    #[test]
    fn when_feeding_the_baby_and_letting_it_digest_then_it_poops_alot() {
        // given
        let mut baby = Baby::default();
        baby.feed_milk(10);
        baby.digest_milk();

        // when
        let maybe_poop = baby.open_windle();

        // then
        assert_eq!(maybe_poop, Some(Poop { amount: 10 }));
    }

    #[test]
    fn get_the_raw_value_out_of_the_optional_poop() {
        let maybe_poop = Some(Poop{amount: 121});

        // Pull out the value of the maybe_poop!
        // Save the amount in a variable called poop_amount

        let poop_amount = 42;

        assert_eq!(poop_amount, 121);
    }
}