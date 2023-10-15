use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use seq_io::fastq::Record;


pub fn process_fastq(fasta_path: &str) {
    
    let file = match File::open(fasta_path) {
        Ok(file) => file,
        Err(error) => panic!("Error opening fastq file: {:?}.", error),
    };
    
    let buf: Box<dyn BufRead + Send> = Box::new(BufReader::new(file));
    let mut reader  = seq_io::fastq::Reader::new(buf);

    while let Some(record) = reader.next() {
        let record = record.unwrap();
        println!("{:?}", record.id());
    }
}
