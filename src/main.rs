use structopt::StructOpt;
use ternary_cpu::TernaryCPU;

/// CLI options for the Ternary CPU
#[derive(StructOpt)]
struct Cli {
    /// The opcodes to run
    #[structopt(short, long)]
    opcodes: Vec<u8>,
}

fn main() {
    // Parse CLI arguments
    let args = Cli::from_args();

    // Initialize the CPU with the provided opcodes
    let mut cpu = TernaryCPU::new(args.opcodes);

    // Run the CPU
    let result = cpu.run();

    // Display the result
    println!("Final Memory State: {:?}", cpu.get_memory());
    println!("Execution Log:\n{}", result);
}

