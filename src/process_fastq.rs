use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;
use flate2::read::GzDecoder;
use seq_io::fastq::Record;


pub fn process_fastq(fasta_path: &str) {
    
    let start = Instant::now();
    
    let _ = match File::open(fasta_path) {
        Ok(file) => file,
        Err(error) => panic!("Error opening fastq file: {:?}.", error),
    };

    let file = File::open("path").unwrap();
    let file = BufReader::new(file);
    let file = GzDecoder::new(file);
    
    let buf: Box<dyn BufRead + Send> = Box::new(BufReader::new(file));
    let mut reader  = seq_io::fastq::Reader::new(buf);

    let mut num_reads = 0;
    while let Some(record) = reader.next() {

        num_reads += 1;

        let record = record.unwrap();
        println!("{:?}", record.id());
    }

    let total_duration = start.elapsed();
    println!("Total execution time of the program: {:?}", total_duration);
    println!("number of reads: {}", num_reads) ;
}
