use rsspice::*;

use clap::Parser;

// The name of the meta-kernel that lists the kernels
// to load into the program.
const METAKR: &str = "subpts.tm";

#[derive(Parser)]
struct Args {
    /// Input UTC time
    utc_time: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut spice = SpiceContext::new();

    // Load the kernels that this program requires.  We
    // will need:
    //
    //    A leapseconds kernel
    //    The necessary ephemerides
    //    A planetary constants file (PCK)
    //    A DSK file containing Phoebe shape data
    spice.furnsh(METAKR)?;

    println!("Converting UTC Time: {}", args.utc_time);

    // Convert UTCTIM to ET.
    let et = spice.str2et(&args.utc_time)?;

    println!("   ET seconds past J2000: {et:16.3}");

    for method in [
        // Use the "near point" sub-point definition
        // and an ellipsoidal model.
        "NEAR POINT/Ellipsoid",
        // Use the "nadir" sub-point definition and a
        // DSK model.
        "NADIR/DSK/Unprioritized",
    ] {
        println!();
        println!("Sub-point/target shape model: {method}");
        println!();

        // Compute the apparent sub-observer point of CASSINI
        // on Phoebe.
        let (spoint, _trgepc, srfvec) =
            spice.subpnt(method, "PHOEBE", et, "IAU_PHOEBE", "LT+S", "CASSINI")?;

        println!("   Apparent sub-observer point of CASSINI on Phoebe in the");
        println!("   IAU_PHOEBE frame (km):");
        println!("      X = {:16.3}", spoint[0]);
        println!("      Y = {:16.3}", spoint[1]);
        println!("      Z = {:16.3}", spoint[2]);
        println!("    ALT = {:16.3}", spice.vnorm(&srfvec));

        // Compute the apparent sub-solar point on Phoebe as seen
        // from CASSINI.
        let (spoint, _trgepc, _srfvec) =
            spice.subslr(method, "PHOEBE", et, "IAU_PHOEBE", "LT+S", "CASSINI")?;

        println!("   Apparent sub-solar point on Phoebe as seen from CASSINI in");
        println!("   the IAU_PHOEBE frame (km):");
        println!("      X = {:16.3}", spoint[0]);
        println!("      Y = {:16.3}", spoint[1]);
        println!("      Z = {:16.3}", spoint[2]);
    }

    Ok(())
}
