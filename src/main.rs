mod request;

use request::multi_requests;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, BufWriter, Write, BufReader, BufRead};
use std::env;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let (ids, verbose) = parse_and_load_ids(&args);

    // this stores the requested (ID, information) pairs
    let mut infos = HashMap::new();

    // perform requests to the API
    let _ = multi_requests(&ids, &mut infos, verbose);

    // Write requested (ID, information) pairs to infos.txt
    let f = OpenOptions::new().write(true).open("infos.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("infos.txt").unwrap()
        } else {
            panic!("{}", e);
        }
    });
    let mut f = BufWriter::new(f);
    for (k, v) in infos.iter() {
        f.write(format!("{}: {}\n", k, v).as_ref()).unwrap();
    }

    Ok(())
}

// Generate ids from 0 to n.
fn generate_ids(n: usize) -> Vec<String> {
    let mut ids = vec!["".to_string(); n];
    for i in 0..n {
        ids[i] = i.to_string();
    }
    ids
}

// Parse environment variables and load ids from file if exists.
fn parse_and_load_ids(args: &Vec<String>) -> (Vec<String>, bool) {
    let mut ids = vec![];
    let mut load = false;
    let mut verbose = false;
    let mut ids_file_name = "";
    for i in 0..args.len() {
        if args[i] == "load" && i + 1 < args.len() {
            load = true;
            ids_file_name = &args[i + 1];
        } else if args[i] == "verbose" {
            verbose = true;
        }
    }

    if load {
        let f = File::open(ids_file_name).unwrap();
        let f = BufReader::new(f);
        for line in f.lines() {
            if let Ok(s) = line {
                if s != "" {
                    ids.push(s);
                }
            }
        }
    } else {
        // generate ids if ID file doesn't exist
        ids = generate_ids(200);
    }

    (ids, verbose)
}
