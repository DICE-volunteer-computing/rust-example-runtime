use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

#[derive(Deserialize)]
struct InputData {
    num: usize,
}

fn main() {
    // ------ read arguments ------

    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].parse::<String>().unwrap();

    // ------ load data from input file ------

    let inputs_dir = Path::new("./data/inputs");

    let data_file = File::open(inputs_dir.join(input_filename)).unwrap();
    let data_file_reader = BufReader::new(data_file);
    let input_data: InputData = serde_json::from_reader(data_file_reader).unwrap();

    // ------ execute computation ------

    let n = input_data.num;
    let nruns = 100;
    const CHUNKSIZE: usize = 32;

    let mut a: Vec<f64> = vec![0.0; n];
    let mut b: Vec<f64> = vec![0.0; n];
    let mut c: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        a[i] = (i as f64).sin().abs() + 0.00001;
        b[i] = (i as f64).cos();
        c[i] = (i as f64).cos();
    }

    let mut count: usize = 0;
    let now = Instant::now();

    let mut beta_vec: [f64; CHUNKSIZE] = [0.0; CHUNKSIZE];
    let mut r_vec: [f64; CHUNKSIZE] = [0.0; CHUNKSIZE];
    while now.elapsed().as_secs_f64() <= 1.0 {
        count += 1;
        for _ in 0..nruns {
            //Initialize partial reduction arrays
            for bv in beta_vec.iter_mut() {
                *bv = 0.0;
            }
            for rv in (r_vec).iter_mut() {
                *rv = 0.0;
            }

            //Form iterator over chunks of
            //input arrays
            let outer_iter = (&a)
                .chunks_exact(CHUNKSIZE)
                .zip((&b).chunks_exact(CHUNKSIZE))
                .zip((&c).chunks_exact(CHUNKSIZE));
            //Get remainder iterator
            let outer_iter_remainder = (&a)
                .chunks_exact(CHUNKSIZE)
                .remainder()
                .iter()
                .zip((&b).chunks_exact(CHUNKSIZE).remainder().iter())
                .zip((&c).chunks_exact(CHUNKSIZE).remainder().iter());

            //Loop over all chunks and form partial reductions
            for ((avec, bvec), cvec) in outer_iter {
                let inner_itter = avec
                    .iter()
                    .zip(bvec.iter())
                    .zip(cvec.iter())
                    .zip(beta_vec.iter_mut())
                    .zip(r_vec.iter_mut());

                for ((((ai, bi), ci), betai), ri) in inner_itter {
                    let res = ci - ai * bi;
                    let ares = ai * res;
                    *betai += ares * ares;
                    *ri += res * ares;
                }
            }
            //Form remainder reduction
            let mut beta = 0.0;
            let mut r = 0.0;
            for ((ai, bi), ci) in outer_iter_remainder {
                let res = ci - ai * bi;
                let ares = ai * res;
                beta += ares * ares;
                r += res * ares;
            }
            //Loop over partial reductions to form final reduction
            beta += beta_vec.iter().fold(0.0, |acc, x| acc + x);
            r += r_vec.iter().fold(0.0, |acc, x| acc + x);

            let rinvbeta = r / beta;

            for ((ai, bi), ci) in (&a).iter().zip(b.iter_mut()).zip(&c) {
                *bi = *bi + rinvbeta * (ci - ai * (*bi));
            }
        }
    }

    println!(
        "Normalized Average time = {}",
        now.elapsed().as_secs_f64() / ((count as f64) * (n as f64) * (nruns as f64))
    );

    let mut sumb = 0.0;
    for i in 0..n {
        sumb += b[i];
    }

    println!("sumb={}", sumb);
}
