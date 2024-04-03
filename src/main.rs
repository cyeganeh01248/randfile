use std::io::{stdout, BufWriter, Write};

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command()]
/// A program to output a random stream of text
struct Args {
    #[arg(short='s', long, default_value = None)]
    /// The [OPTIONAL] target file size to try and generate
    file_size: Option<usize>,

    #[arg(short='n', long="no-regen_buffer", default_value_t = true, action = ArgAction::SetFalse)]
    /// Flag to not regenerate the contents of the random buffer on each loop
    regen_buffer: bool,

    #[arg(short, long, default_value_t = 1024 * 32)]
    /// The size of the buffer of random characters to write
    buffer_size: usize,
}

fn main() {
    let args = Args::parse();

    let mut rng = fastrand::Rng::new();
    let mut _i = 0;

    let alpha = "0123456789abcdef"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let alphas: &[u8] = alpha.as_slice();

    let mut buff = vec![0u8; args.buffer_size];
    let mut std = BufWriter::new(stdout());

    (0..buff.len()).for_each(|j| {
        buff[j] = rng.choice(alphas.iter()).unwrap().to_owned();
    });

    loop {
        if args.regen_buffer {
            (0..buff.len()).for_each(|j| {
                buff[j] = rng.choice(alphas.iter()).unwrap().to_owned();
            });
        }
        std.write_all(buff.as_slice())
            .expect("Unable to write to file");
        // buff_writer.flush();
        _i += buff.len();
        if let Some(max_size) = args.file_size {
            if _i >= max_size {
                break;
            }
        }
    }
}
