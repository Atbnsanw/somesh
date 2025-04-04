/// cargo run --package thfhe --example thfhe --release -- -n 3
///  cargo build --package thfhe --example thfhe --release
use std::thread;

use algebra::Field;
use clap::Parser;
use mpc::{DNBackend, MPCBackend};
use network::netio::Participant;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::io::{BufWriter, Write,BufRead,BufReader};
use thfhe::{distdec, Evaluator, Fp, KeyGen, DEFAULT_128_BITS_PARAMETERS};
// const LWE_MODULUS: u64 = 4096;
const RING_MODULUS: u64 = Fp::MODULUS_VALUE;
#[derive(Parser)]
struct Args {
    /// 参数 n
    #[arg(short = 'n')]
    n: u32,
    // 参数 t
    #[arg(short = 'i')]
    id: u32,
}

fn main() {
    let args = Args::parse();
    //const NUM_PARTIES: u32 =args.n;
    let number_parties = args.n;
    let party_id = args.id;
    //let number_threshold = args.t;
    let number_threshold = (number_parties - 1) / 2;
    //const THRESHOLD: u32 = args.t;
    const BASE_PORT: u32 = 20500;
    thfhe(party_id, number_parties, number_threshold, BASE_PORT);

}

// struct Args {
//     /// 参数 n
//     #[arg(short = 'n')]
//     n: u32,
// }

// fn main() {
//     let args = Args::parse();
//     //const NUM_PARTIES: u32 =args.n;
//     let number_parties = args.n;
//     //let number_threshold = args.t;
//     let number_threshold = (number_parties - 1) / 2;
//     //const THRESHOLD: u32 = args.t;
//     const BASE_PORT: u32 = 20500;
//     // thfhe(party_id, number_parties, number_threshold, BASE_PORT);
//     let threads = (0..number_parties)
//         .map(|party_id| {
//             thread::spawn(move || thfhe(party_id, number_parties, number_threshold, BASE_PORT))
//         })
//         .collect::<Vec<_>>();

//     for handle in threads {
//         handle.join().unwrap();
//     }
// }

fn thfhe(party_id: u32, num_parties: u32, threshold: u32, base_port: u32) {
    let start = std::time::Instant::now();

    let rng = &mut rand::thread_rng();

    let parameters = &DEFAULT_128_BITS_PARAMETERS;
    let lwe_params = parameters.input_lwe_params();

    // Setup the DN backend.
    let participants = Participant::from_default(num_parties, base_port);
    let mut backend = DNBackend::<RING_MODULUS>::new(
        party_id,
        num_parties,
        threshold,
        1,
        participants,
        parameters.ring_dimension(),
        true,
        true,
    );

    let a:u64 = 1;
    let b:u64 = 2;

    println!(
        "Party {} had finished the double randoms with time {} ns,",
        party_id,
        backend.total_mul_triple_duration().as_nanos()
    );

    // let (sk, pk, evk) = KeyGen::generate_mpc_key_pair(&mut backend, **parameters, rng);

    // let evaluator = Evaluator::new(evk);
    

    // let x = pk.encrypt(a, lwe_params, rng);
    // let y = pk.encrypt(b, lwe_params, rng);

    // let res = evaluator.add(&x, &y);
    // let public_a_t = backend.sends_slice_to_all_parties(Some(res.a()), res.a().len(), 0);
    // let public_b_t = backend.sends_slice_to_all_parties(Some(&vec![res.b()]), vec![res.b()].len(), 0)[0];


    // // write cipher a, b, sk to file
    // let my_sk = sk.input_lwe_secret_key.as_ref();
    // let _ = write_cipher(&public_a_t, public_b_t, my_sk,num_parties);
    
    // ------------------------- only dd
    
    let (public_a_t,public_b_t,my_sk) = read_cipher(num_parties).unwrap();
    

    //---------------------------
    let test_num = 20000;
    let public_a = vec![public_a_t.clone();test_num];
    let public_b = vec![public_b_t;test_num];
    backend.init_z2k_triples_from_files();
    if party_id <= threshold {
        

        let (my_dd_res, (online_duration, offline_duration)) =
            distdec(&mut backend, rng, &public_a, &public_b, &my_sk);
        println!(
            "Party {} had finished the dd-online with time {} ns,",
            party_id,
            online_duration.as_nanos()
        );
        println!(
            "Party {} had finished the dd-offline with time {} ns,",
            party_id,
            offline_duration.as_nanos()
        );

        if party_id == 0 {
            println!(
                "(a + b )%4= {}, my party id: {}, my dd result: {:?}",
                (a + b) % 4,
                backend.party_id(),
                my_dd_res.unwrap()[0]
            );
        }
    }
    println!(
        "Party {} had finished the program with time {:?}",
        party_id,
        start.elapsed()
    );
}


fn write_cipher_vec(a: &Vec<Vec<u64>>, b: &Vec<u64>,num_parties: u32) -> std::io::Result<()> {
    let dir_path = Path::new("./thfhe/predata/");
    create_dir_all(dir_path)?; // 保证目录存在
    let file = File::create(dir_path.join(format!("{}party-cipherdata.txt", num_parties)))?;
    let mut writer = BufWriter::new(file);

    // 写入 a
    for row in a {
        let line = row.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(writer, "A {}", line)?;
    }

    // 写入 b
    let b_line = b.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    writeln!(writer, "B {}", b_line)?;

    Ok(())
}

fn read_cipher_vec() -> std::io::Result<(Vec<Vec<u64>>, Vec<u64>)> {
    let file = File::open("./thfhe/predata/cipherdata.txt")?;
    let reader = BufReader::new(file);

    let mut a: Vec<Vec<u64>> = Vec::new();
    let mut b: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("A ") {
            let values = line[2..].split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            a.push(values);
        } else if line.starts_with("B ") {
            b = line[2..].split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
        }
    }

    Ok((a, b))
}

/// write public_a, public_b, sk
fn write_cipher(a: &[u64], b: u64, sk: &[u64],num_parties:u32) -> std::io::Result<()> {
    let dir_path = Path::new("./thfhe/predata/");
    create_dir_all(dir_path)?;
    let file = File::create(dir_path.join(format!("{}-party-cipherdata.txt", num_parties)))?;
    let mut writer = BufWriter::new(file);

    // write a
    let a_line = a.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    writeln!(writer, "A {}", a_line)?;

    // write b
    writeln!(writer, "B {}", b)?;

    // write sk
    let sk_line = sk.iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join(" ");
    writeln!(writer, "SK {}", sk_line)?;

    Ok(())
}

/// read public_a, public_b, sk
fn read_cipher(num_parties: u32) -> std::io::Result<(Vec<u64>, u64, Vec<u64>)> {
    let file = File::open(format!("./thfhe/predata/{}-party-cipherdata.txt",num_parties))?;
    let reader = BufReader::new(file);

    let mut a: Vec<u64> = Vec::new();
    let mut sk: Vec<u64> = Vec::new();
    let mut b: u64 = 0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("A ") {
            a = line[2..].split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
        } else if line.starts_with("B ") {
            b = line[2..].trim().parse::<u64>().unwrap();
        } else if line.starts_with("SK "){
            sk = line[2..].split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        }
    }

    Ok((a, b,sk))
}
