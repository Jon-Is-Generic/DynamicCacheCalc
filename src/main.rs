use std::collections::{HashMap, HashSet};

struct Access {
    eviction_time : usize,
    hit : bool,
}

fn init_cache() -> Simulator {
    todo!()
}


fn oracle(i : i32) -> usize {
    todo!()
}

struct Sampler {

}

impl Sampler {
    fn new<T : Iterator<Item=(u64, f64)>>(t : T) -> Sampler {
        todo!()
    }

    fn sample(&self) -> u64 {
        todo!()
    }
}

struct Simulator {
    cur : u64,
    tracker : HashMap<u64, u64>,
    step : u64,
}

impl Simulator {
    fn init() -> Simulator {
        Simulator {cur : 0, tracker : HashMap::new(), step : 0}
    }

    fn add_tenancy(&mut self, tenancy : u64) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn get_excess(&self, fixed : u64) -> u64 {
        if self.cur <= fixed {
            0
        } else {
            fixed - self.cur
        }
    }
}

fn caching(ten_dist : Sampler, cache_size : u64, delta : f64) -> (u64, u64) {
    let mut cache = Simulator::init();
    let mut trace_len: u64 = 0;
    let mut samples_to_issue: u64 = 1024;
    let mut prev_output : Option<f64> = None;
    let mut total_overalloc : u64 = 0;
    loop {
        for i in 0..samples_to_issue {
            trace_len += 1;
            let tenancy = ten_dist.sample();
            cache.add_tenancy(tenancy);
            total_overalloc += cache.get_excess(cache_size);
        }
        if prev_output.is_some() && ((total_overalloc as f64) / (trace_len as f64) - prev_output.unwrap()) < delta {
            return (total_overalloc, trace_len)
        }
        prev_output = Some((total_overalloc as f64) / (trace_len as f64));
        samples_to_issue *= 2;
    }

    todo!()
}



fn main() {
    println!("Hello, world!");
}
