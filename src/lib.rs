#![feature(once_cell)]

use std::{collections::BTreeSet, fs, path::Path, sync::Arc};

use crate::options::RuntimeOptions;
use clap::StructOpt;
use rand::{distributions::Standard, prelude::*, SeedableRng};

mod coverage;
mod driver;
mod mutations;
mod options;
mod signal;
mod weak;

// extern "C" {
//     #[linkage = "weak"]
//     fn LLVMFuzzerTestOneInput(data: *const u8, size: usize) -> std::os::raw::c_int;
// }

#[derive(Debug)]
pub struct Fazi<R: Rng> {
    rng: R,
    input: Arc<Vec<u8>>,
    dictionary: Vec<Vec<u8>>,
    corpus: Vec<Arc<Vec<u8>>>,
    options: RuntimeOptions,
}

impl Default for Fazi<StdRng> {
    fn default() -> Self {
        Fazi {
            rng: StdRng::from_entropy(),
            input: Default::default(),
            dictionary: vec![],
            corpus: Default::default(),
            options: Default::default(),
        }
    }
}

impl<R: Rng + SeedableRng> Fazi<R> {
    pub fn new() -> Self {
        Fazi {
            rng: R::from_entropy(),
            input: Default::default(),
            dictionary: vec![],
            corpus: Default::default(),
            options: Default::default(),
        }
    }

    pub fn new_from_seed(seed: R::Seed) -> Self {
        Fazi {
            rng: R::from_seed(seed),
            input: Default::default(),
            dictionary: vec![],
            corpus: Default::default(),
            options: Default::default(),
        }
    }

    pub fn options_from_os_args(&mut self) {
        self.options = RuntimeOptions::parse();
    }

    pub fn restore_inputs(&mut self) {
        let input_paths: [&Path; 1] = [self.options.corpus_dir.as_ref()];
        for &path in &input_paths {
            if !path.exists() || !path.is_dir() {
                continue;
            }

            for dirent in fs::read_dir(path).expect("failed to read input directory") {
                if let Ok(dirent) = dirent {
                    let input_file_path = dirent.path();
                    if input_file_path.is_dir() {
                        continue;
                    }

                    self.corpus.push(Arc::new(
                        fs::read(input_file_path).expect("failed to read input file"),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut fazi = Fazi::default();
        for i in 0..30 {
            fazi.mutate_input();
            println!("{:?}", fazi.input);
        }
    }
}
