use std::char;
use std::collections::HashMap;

use std::thread::{JoinHandle, spawn};

type CharCountsMap = HashMap<char, usize>;

fn count(s: &str) -> CharCountsMap {
    let letters: Vec<char> = s.chars()
        .filter(|&c| char::is_alphabetic(c))
        .map(|c| char::to_lowercase(c).next().unwrap())
        .collect();

    let mut count_map = HashMap::new();
    for c in letters {
        let count = count_map.entry(c).or_insert(0);
        *count += 1;
    }

    count_map
}

fn merge_counts(count_maps: &[CharCountsMap]) -> CharCountsMap {
    count_maps.iter()
        .fold(HashMap::new(), |mut acc, map| {
            for (&c, &count) in map.iter() {
                let acc_count = acc.entry(c).or_insert(0);
                *acc_count += count;
            }
            acc
        })
}

fn counts(strs: Vec<String>) -> CharCountsMap {
    let counts = strs.iter().map(|s| count(s)).collect::<Vec<CharCountsMap>>();
    merge_counts(&counts)
}

fn spawn_counters(strs: &[&str], workers_count: usize) -> Vec<JoinHandle<CharCountsMap>> {
    let jobs_count = strs.len();
    assert!(workers_count > 0);
    assert!(jobs_count >= workers_count);

    let jobs_per_worker = jobs_count / workers_count;
    let mut workers = Vec::new();

    for i in 0..workers_count {
        let first = i * jobs_per_worker;
        let job_range = first .. first+jobs_per_worker;
        let worker_strings: Vec<String> = strs[job_range].iter().map(|&s| s.to_string()).collect();

        let worker = spawn(|| {
            counts(worker_strings)
        });
        workers.push(worker);
    }

    workers
}

fn get_workers_counts(workers: Vec<JoinHandle<CharCountsMap>>) -> CharCountsMap {
    let mut vec_counts = Vec::new();

    for worker in workers {
        let counts = worker.join().expect("A worker thread has failed");
        vec_counts.push(counts)
    }

    merge_counts(&vec_counts)
}

fn main_counter(strs: &[&str]) -> CharCountsMap {
    let strings: Vec<String> = strs.iter().map(|&s| s.to_string()).collect();
    counts(strings)
}

pub fn frequency(strs: &[&str], workers_count: usize) -> HashMap<char, usize> {
    let jobs_count = strs.len();
    if jobs_count < workers_count || workers_count <= 1 {
        main_counter(strs)
    } else {
        let jobs_per_worker = jobs_count / workers_count;
        let workers_jobs = 0 .. jobs_count-jobs_per_worker;
        let main_jobs = jobs_count-jobs_per_worker .. jobs_count;

        let workers = spawn_counters(&strs[workers_jobs], workers_count-1);
        let main_counts = main_counter(&strs[main_jobs]);
        let workers_counts = get_workers_counts(workers);

        merge_counts(&[main_counts, workers_counts])
    }
}
