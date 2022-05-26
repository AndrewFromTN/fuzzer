use rand::{rngs::StdRng, Rng, SeedableRng};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let prng_seed = args[1]
        .parse::<i32>()
        .expect("prng_seed is not a valid number");
    let num_iterations = args[2]
        .parse::<i32>()
        .expect("num_iterations is not a valid number");

    let file = File::open("seed").expect("Failed to open seed file");
    let reader = BufReader::new(file);
    let seed = reader
        .lines()
        .next()
        .expect("Seed file not valid")
        .expect("Seed file not valid");

    let mut output = seed.as_bytes().to_vec();
    let mut rng = StdRng::seed_from_u64(prng_seed as u64);
    (0..num_iterations).for_each(|index| {
        output.iter_mut().for_each(|byte| {
            let chance = rng.gen_range(0, 100);
            if chance >= 0 && chance <= 13 {
                let random_byte = rng.gen_range(0, 255);
                *byte = random_byte;
            }
        });

        if (index + 1) % 500 == 0 {
            let mut new_bytes = (0..10).map(|_i| rng.gen_range(0, 255)).collect::<Vec<u8>>();

            output.append(&mut new_bytes);
		}
		
		let mut out = std::io::stdout();
		out.write_all(output.as_slice())
			.expect("Failed to write to stdout");
		out.flush().expect("Failed to flush stdout");
    });
}
