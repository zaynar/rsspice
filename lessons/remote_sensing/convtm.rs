use rsspice::*;

use clap::Parser;

// The name of the meta-kernel that lists the kernels
// to load into the program.
const METAKR: &str = "convtm.tm";

// The spacecraft clock ID code for CASSINI.
const SCLKID: i32 = -82;

#[derive(Parser)]
struct Args {
    /// Input UTC time
    utc_time: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut spice = SpiceContext::new();

    // Load the kernels this program requires.
    // Both the spacecraft clock kernel and a
    // leapseconds kernel should be listed
    // in the meta-kernel.
    spice.furnsh(METAKR)?;

    println!("Converting UTC Time: {}", args.utc_time);

    // Convert UTCTIM to ET.
    let et = spice.str2et(&args.utc_time)?;

    println!("   ET Seconds Past J2000: {et:16.3}");

    // Now convert ET to a formal calendar time
    // string.  This can be accomplished in two
    // ways.
    let calet = spice.etcal(et);
    println!("   Calendar ET (ETCAL): {calet}");

    // Or use TIMOUT for finer control over the
    // output format.  The picture below was built
    // by examining the header of TIMOUT.
    let calet = spice.timout(et, "YYYY-MON-DDTHR:MN:SC ::TDB")?;
    println!("   Calendar ET (TIMOUT): {calet}");

    // Convert ET to spacecraft clock time.
    let sclkst = spice.sce2s(SCLKID, et)?;

    println!("   Spacecraft Clock Time: {sclkst}");

    Ok(())
}
