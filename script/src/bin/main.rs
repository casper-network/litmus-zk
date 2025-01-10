mod fixtures;
mod utils;
use camino::Utf8PathBuf;
use clap::Parser;
use fixtures::types::WrappedBlockV2WithProofs;
use lkernel::Kernel;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::{fs, path::PathBuf};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const _ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(short, long, default_value = "content")]
    path_to_config: Utf8PathBuf,

    #[clap(long)]
    prove: bool,
}

fn main() {
    // Set args.
    let args = Args::parse();
    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }
    if args.path_to_config.exists() == false {
        eprintln!("Error: Invalid config file path.");
    }

    // Set logger.
    sp1_sdk::utils::setup_logger();

    // Set kernel.
    let kernel = Kernel::new(&args.path_to_config);
    kernel.init();

    // Set stdin set ... i.e. a sequence of ZK-VM prover inputs.
    let mut set_of_stdin = Vec::<SP1Stdin>::new();

    let g = kernel.get_block_with_proofs(None);
    let g = match g {
        Some(block) => WrappedBlockV2WithProofs(block, kernel.get_chain_name_digest()),
        None => panic!("Invalid trusted hash"),
    };
    set_of_stdin.push(SP1Stdin::try_from(&g).unwrap());

    // Invoke stdin set.
    for stdin in set_of_stdin {
        if args.execute {
            do_pgm_execute(&args, &stdin);
        } else {
            do_pgm_prove(&args, &stdin);
        }
    }
}

fn do_pgm_execute(args: &Args, stdin: &SP1Stdin) {
    // Set VM client.
    let client = ProverClient::new();
    let (_, report) = client.execute(_ELF, stdin.clone()).run().unwrap();

    // Render report.
    println!(
        "EXECUTION: # vm cycles   : {}",
        report.total_instruction_count()
    );
    println!(
        "EXECUTION: # calls to sys: {}",
        report.total_syscall_count()
    );
}

fn do_pgm_prove(_: &Args, stdin: &SP1Stdin) {
    // Set VM client.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(_ELF);

    // Set proof.
    let proof = client
        .prove(&pk, stdin.clone())
        .run()
        .expect("failed to generate proof");
    println!("PROOF: generation complete");

    // Verify proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("PROOF: verification complete");
}
