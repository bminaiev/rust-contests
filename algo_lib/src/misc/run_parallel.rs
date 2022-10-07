// Partially copied from [https://github.com/EgorKulikov/rust_algo/blob/master/algo_lib/src/misc/run_parallel.rs]
use crate::io::input::Input;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use std::sync::atomic::AtomicUsize;
use std::time::Instant;

pub trait ParallelJob: Sync + Send + Default + Clone {
    type Context: Sync;

    fn read_input(&mut self, input: &mut Input);
    fn solve(&mut self, context: &Self::Context);
    fn write_output(&mut self, test_case: usize);
}

pub fn run_parallel<J: ParallelJob>(
    input: &mut Input,
    num_threads: Option<usize>,
    context: &J::Context,
) {
    let start = Instant::now();

    let t = input.read();
    let mut jobs = vec![J::default(); t];
    for job in jobs.iter_mut() {
        job.read_input(input);
    }
    let mut thread_pool = ThreadPoolBuilder::new().stack_size(1000000000);
    if let Some(num_threads) = num_threads {
        thread_pool = thread_pool.num_threads(num_threads);
    };
    thread_pool.build_global().unwrap();
    let rem = AtomicUsize::new(t);
    jobs.par_iter_mut().enumerate().for_each(|(test, job)| {
        job.solve(context);
        eprintln!(
            "Test {} done, {} remaining",
            test,
            rem.fetch_sub(1, std::sync::atomic::Ordering::Relaxed) - 1
        );
    });
    for (i, mut job) in jobs.into_iter().enumerate() {
        job.write_output(i + 1);
    }

    eprintln!("Finished in {:?}", start.elapsed());
}
