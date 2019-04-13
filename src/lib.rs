#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn simple_test() {
        let mut random = i8::I8BasedRandom::new(Box::new(TestI8Random {}));
        println!("Bool value is {}", random.next_bool());
        println!("Int8 value is {}", random.next_i8());
    }
}

pub mod i8;

pub trait Random {

    fn next_bool(&mut self) -> bool;

    fn next_u8(&mut self) -> u8;

    fn next_i8(&mut self) -> i8;

    fn next_u16(&mut self) -> u16;

    fn next_i16(&mut self) -> i16;

    fn next_u32(&mut self) -> u32;

    fn next_i32(&mut self) -> i32;
}



pub struct TestI8Random {

}

impl i8::I8RNG for TestI8Random {

    fn next_i8(&mut self) -> i8 {
        return 32;
    }
}