use rand::distributions::Distribution;
use rand::{rngs::ThreadRng, thread_rng, Error, Rng, RngCore};
use rand_distr::StandardNormal;

struct BoxStdNormD;

impl Distribution<Box<f64>> for BoxStdNormD {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<f64> {
        Box::new(StandardNormal.sample(rng))
    }
}

const N: u32 = 20;
struct PanicRng {
    counter: u32,
    rng: ThreadRng,
}

impl RngCore for PanicRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        dbg!(self.counter);
        if self.counter < N {
            self.counter += 1;
            self.rng.next_u64()
        } else {
            panic!("PanicRng only knows how to panic.")
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dbg!(self.counter);
        if self.counter < N {
            self.counter += 1;
            self.rng.fill_bytes(dest)
        } else {
            panic!("PanicRng only knows how to panic.")
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn main() {
    let mut rng = PanicRng {
        counter: 0,
        rng: thread_rng(),
    };
    let distr = rand_array_iid::IIDDistr::new(BoxStdNormD);
    let x: [Box<f64>; 31] = distr.sample(&mut rng);
    println!("{:?}", &x)
}
