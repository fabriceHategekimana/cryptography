use rand::Rng;

type Lsfr = Vec<u8>;

fn xor(b1: u8, b2: u8) -> u8 {
    match (b1, b2) {
        (0, 0) => 0,
        (1, 0) => 1,
        (0, 1) => 1,
        (1, 1) => 1,
        _ => panic!()
    }
}

impl Vec<u8> {
    fn new(size: i8) -> Vec<u8> {
        (0..n).iter().map(|x| rand::thread_rng().gen_range(0..=1)).collect()
    }

    fn predictible() -> Vec<u8> {
        [1, 0, 0, 1].iter().collect()
    }

    fn generate(&self) -> (Vec<u8>, u8) {
        let random_number = self[ self.len()-1 ];
        let new_bit = xor(self[self.len()-1], self[self.len()-2]);
        ([new_bit].iter().chain(self.iter()).take(self.len()).collect(),
        random_number)
    }
}
