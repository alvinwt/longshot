// external crates
extern crate bio;
extern crate chrono;
extern crate clap;
extern crate core;
extern crate rand;
extern crate rust_htslib;
#[macro_use]
extern crate error_chain;
extern crate fishers_exact;
extern crate hashbrown;

// import modules
pub mod call_genotypes;
pub mod call_potential_snvs;
pub mod context_model;
mod errors;
pub mod estimate_alignment_parameters;
mod estimate_read_coverage;
pub mod extract_fragments; //mod extract_fragments_debug;
pub mod genotype_probs;
pub mod haplotype_assembly;
pub mod print_output;
pub mod realignment;
pub mod util;
pub mod variants_and_fragments;
//mod spoa;

