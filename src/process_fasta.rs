use std::fs::File;
// use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;
use seq_io::fasta::Record;
use rust_lcp;


fn mean(data: &[usize]) -> Option<f32> {
    let sum = data.iter().sum::<usize>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}


fn std_deviation(data: &[usize]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}

  
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

    let mut chrom_index = 0;

    while let Some(record) = reader.next() {

        if chrom_index == 23 {
            break;
        }
        chrom_index += 1;

        let record = record.unwrap();
        let id = record.id().unwrap();
        println!("Processing: {}", id);

        let mut lcp_str = rust_lcp::String::from_u8(record.seq());

        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
        println!("End of creating string for id: {}", id);
        println!("Total number of cores: {}", lcp_str.cores.len());

        for i in 1..5 {
            let start_level = Instant::now();
            
            lcp_str.deepen();

            println!("End of deepeneing to level: {}", i);

            if lcp_str.cores.len() > 0 {
                let mut prev_index_end: usize = lcp_str.cores[0].end;
                let mut prev_index_start: usize = lcp_str.cores[0].start;
                let mut overlapping_count: usize = 0;
                let mut distances_pos = [0 as usize; 10000];
                let mut distances = [0 as usize; 10000];
                let mut lengths = [0 as usize; 10000];
                for (index, core) in lcp_str.cores.iter().enumerate().skip(1) {                
                    if core.start < prev_index_end {
                        overlapping_count += 1;
                    } else {
                        if core.start-prev_index_end < 10000 {
                            distances[core.start-prev_index_end] += 1;
                        } else {
                            // println!("Found large distance in level {}, core index: {}, core.start: {}, core.end: {}, prev_core.start: {}, prev_core.end: {}", i, index, core.start, core.end, prev_index_start, prev_index_end)
                        }
                    }
                    
                    if core.start-prev_index_start < 10000 {
                        distances_pos[core.start-prev_index_start] += 1;
                    } else {
                        // println!("Found large distance between start pos in level {}, core index: {}, core.start: {}, core.end: {}, prev_core.start: {}, prev_core.end: {}", i, index, core.start, core.end, prev_index_start, prev_index_end)
                    }
                    
                    if core.end-core.start < 10000 {
                        lengths[core.end-core.start] += 1;
                    } else {
                        // println!("Found large length in level {}, core index: {}, core.start: {}, core.end: {}", i, index, core.start, core.end)
                    }

                    prev_index_end = core.end;
                    prev_index_start = core.start;
                }

                println!("Level: {}", i);
                println!("Overlapping core counts: {}", overlapping_count);
                println!("Std of distances btw cores: {:?}", std_deviation(&distances));
                println!("Mean of disntances btw cores: {:?}", mean(&distances));
                println!("Std of distances btw starts: {:?}", std_deviation(&distances_pos));
                println!("Mean of disntances btw starts: {:?}", mean(&distances_pos));
                println!("Std of lengths: {:?}", std_deviation(&lengths));
                println!("Mean of lengths: {:?}", mean(&lengths));
                // println!("Distances: {:?}", distances);
                // println!("Lengths: {:?}", lengths);
            }
            
            let duration_level = start_level.elapsed();
            println!("Total execution time: {:?}", duration_level);
            println!("Total number of cores: {}", lcp_str.cores.len());
            println!();
        }

        let total_duration = start.elapsed();
        println!("Total execution time of the program: {:?}", total_duration);
    }
}
