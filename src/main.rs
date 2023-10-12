use std::env;
mod process_fasta;
mod process_fastq;


fn main() {
    let args: Vec<String> = env::args().collect();

    let experiment = &args[1];

    match experiment.as_str() {
        "fasta" => { 
            if args.len() < 2 {
                panic!("Fasta is not provided.");
            }
            process_fasta::process_fasta(&args[2]);
        },
        "fastq" => {
            if args.len() < 2 {
                panic!("Fastq is not provided.");
            }
            process_fastq::process_fastq(&args[2]);
        },
        _ => panic!("Invalid argument provided!")
    }
}
