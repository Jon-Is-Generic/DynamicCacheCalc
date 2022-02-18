use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use rand::prelude::ThreadRng;
use rand::Rng;

struct Access {
    eviction_time : usize,
    hit : bool,
}

struct Sampler {
    r : RefCell<ThreadRng>,
}

impl Sampler {

    fn new<T : Iterator<Item=(u64, f64)>>(t : T) -> Sampler {
        let mut r = rand::thread_rng();
        Sampler { r : RefCell::new(r) }
    }

    fn sample(&self) -> u64 {
        self.r.borrow_mut().gen_range(1..=100)
    }
}

struct Simulator {
    size: u64,
    tracker : HashMap<u64, u64>,
    step : u64,
}

impl Simulator {
    fn init() -> Simulator {
        Simulator { size: 0, tracker : HashMap::new(), step : 0}
    }

    fn add_tenancy(&mut self, tenancy : u64) {
        self.update();
        self.size += 1;
        let target = tenancy + self.step;
        let leases_at_step = self.tracker.get(&target).copied().unwrap_or(0);
        self.tracker.insert(target, leases_at_step +1);
    }

    fn update(&mut self) {
        self.step += 1;
        self.size -= self.tracker.remove(&self.step).unwrap_or(0);
    }

    fn get_excess(&self, fixed : u64) -> u64 {
        if self.size <= fixed {
            0
        } else {
            self.size - fixed
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
}



fn main() {

    let (over_alloc, trace_len) = caching(Sampler::new(None.into_iter()), 10, 0.05);

    println!("over_alloc: {}, trace_len: {}, div : {}", over_alloc, trace_len, over_alloc/trace_len);
}
