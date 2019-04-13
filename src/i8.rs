extern crate bit_helper;

use bit_helper::converter::*;

pub struct I8BasedRandom {

    backing_random: Box<I8RNG>
}

impl I8BasedRandom {

    pub fn new(backing_random: Box<I8RNG>) -> I8BasedRandom {
        I8BasedRandom {
            backing_random: backing_random
        }
    }
}

pub trait I8RNG {

    fn next_i8(&mut self) -> i8;
}

impl crate::Random for I8BasedRandom {

    fn next_bool(&mut self) -> bool {
        self.next_i8() >= 0
    }

    fn next_u8(&mut self) -> u8 {
        self.next_i8() as u8
    }

    fn next_i8(&mut self) -> i8 {
        self.backing_random.next_i8()
    }

    fn next_u16(&mut self) -> u16 {
        i8s_to_u16(self.next_i8(), self.next_i8())
    }

    fn next_i16(&mut self) -> i16 {
        i8s_to_i16(self.next_i8(), self.next_i8())
    }

    fn next_u32(&mut self) -> u32 {
        i8s_to_u32(self.next_i8(), self.next_i8(), self.next_i8(), self.next_i8())
    }

    fn next_i32(&mut self) -> i32 {
        i8s_to_i32(self.next_i8(), self.next_i8(), self.next_i8(), self.next_i8())
    }
}