use std::fs::File;
// use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;
use seq_io::fasta::Record;
use rust_lcp;

  
pub fn process_fasta(fasta_path: &str) {
    
    let start = Instant::now();
    
    if !fasta_path.ends_with(".fa") && !fasta_path.ends_with(".fasta") {
        panic!("fasta file has invalid extension.");
    }
    let file = match File::open(fasta_path) {
        Ok(file) => file,
        Err(error) => panic!("Error opening fasta file: {:?}.", error),
    };
    
    let buf: Box<dyn BufRead + Send> = Box::new(BufReader::new(file));
    let mut reader  = seq_io::fasta::Reader::new(buf);

    // let result_file_path = if fasta_path.ends_with(".fa") { &fasta_path[..fasta_path.len()-3] } else { &fasta_path[..fasta_path.len()-6] };

    while let Some(record) = reader.next() {
        let record = record.unwrap();
        let id = record.id().unwrap();
        println!("Processing: {}", id);

        let mut lcp_str = rust_lcp::String::from_u8(record.seq());

        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
        println!("End of creating string for id: {}", id);
        println!("Total number of cores: {}", lcp_str.cores.len());

        for i in 1..5 {
            let start = Instant::now();
            
            lcp_str.deepen();

            println!("End of deepeneing to level: {}", i);

            let mut prev_index: usize = lcp_str.cores[0].end;
            let mut overlapping_count: usize = 0;
            let mut distances = [0 as usize; 10000];
            let mut lengths = [0 as usize; 10000];
            for (index, core) in lcp_str.cores.iter().enumerate().skip(1) {                
                if core.start < prev_index {
                    overlapping_count += 1;
                } else {
                    if core.start-prev_index < 10000 {
                        distances[core.start-prev_index] += 1;
                    } else {
                        println!("Found large distance in level {}, core index: {}, core.start: {}, core.end: {}", i, index, core.start, core.end)
                    }
                }
                
                if core.end-core.start < 10000 {
                    lengths[core.end-core.start] += 1;
                } else {
                    println!("Found large length in level {}, core index: {}, core.start: {}, core.end: {}", i, index, core.start, core.end)
                }

                prev_index = core.end;
            }

            let duration = start.elapsed();
            println!("Total execution time: {:?}", duration);
            println!("Total number of cores: {}", lcp_str.cores.len());

            println!("Level: {}", i);
            println!("Overlapping core counts: {}", overlapping_count);
            println!("Distances: {:?}", distances);
            println!("Lengths: {:?}", lengths);
            println!();
        }

        let total_duration = start.elapsed();
        println!("Total execution time of the program: {:?}", total_duration);
    }
}
