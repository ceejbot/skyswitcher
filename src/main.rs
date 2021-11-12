use blake2::{Blake2b, Digest};
use structopt::clap::AppSettings::*;
use structopt::StructOpt;
use subprocess::{Exec, NullFile};

#[derive(Clone, StructOpt)]
#[structopt(
    name = "skyswitcher",
    about = "swap versions of skyrim & skse on the fly"
)]
#[structopt(global_setting(ColoredHelp), global_setting(ColorAuto))]
pub struct Flags {
    #[structopt(
        short,
        long,
        parse(from_occurrences),
        help = "Pass -v or -vv to increase verbosity"
    )]
    verbose: u64,
    #[structopt(subcommand)]
    edition: Edition,
}

#[derive(Clone, StructOpt, PartialEq)]
enum Edition {
    /// Run the Anniversary Edition
    Anniversary,
    /// Run the legacy Special Edition
    Legacy,
}

fn main() -> anyhow::Result<(), anyhow::Error> {
    let flags = Flags::from_args();

    loggerv::Logger::new()
        .verbosity(flags.verbose)
        .line_numbers(false)
        .module_path(false)
        .colors(true)
        .init()
        .unwrap();

    let source_dir = if flags.edition == Edition::Legacy {
        log::info!("running the legacy special edition...");
        "./SLL/Legacy"
    } else {
        log::info!("running the anniversary edition...");
        "./SLL/Anniversary"
    };

    let target_exec = format!("{}/SkyrimSE.exe", source_dir);
    let target_skse = format!("{}/skse64_loader.exe", source_dir);

    // get shasums of desired files, compare to shasums of destination files;
    // do no work if none needed
    {
        let buf = std::fs::read(&target_exec)?;
        let target_sum = Blake2b::digest(&buf);
        let buf = std::fs::read("SkyrimSE.exe")?;
        let existing_sum = Blake2b::digest(&buf);
        if target_sum != existing_sum {
            log::info!("   copying SkyrimSE.exe into place...")
            std::fs::copy(target_exec, "SkyrimSE.exe")?;
        }
    }

    {
        let buf = std::fs::read(&target_skse)?;
        let target_sum = Blake2b::digest(&buf);
        let buf = std::fs::read("skse64_loader.exe")?;
        let existing_sum = Blake2b::digest(&buf);
        if target_sum != existing_sum {
            log::info!("   copying skse64_loader.exe into place...")
            std::fs::copy(target_skse, "SkyrimSE.exe")?;
        }
    }

    Exec::cmd("skse64_loader.exe").stderr(NullFile).join()?;

    Ok(())
}
