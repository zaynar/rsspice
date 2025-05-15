//! SIMPLE - A simple program to demonstrate the NAIF toolkit.
//!
//! This 'cookbook' program demonstrates the use of NAIF SPK
//! Ephemeris files and software.
//!
//! Although this program is not extremely sophisticated, it can
//! serve as a starting point from which you could build your own
//! program.

//! The program calculates the angular separation of the two
//! target bodies as seen from the observing body for 10 equally
//! spaced times in the time interval defined by the beginning
//! UTC time and the ending UTC time. The ephemeris times (ET)
//! and the angular separation for the two target bodies are
//! displayed on the screen.

use rsspice::*;

use clap::Parser;

const MAXPTS: usize = 10;

/// This program calculates the angular separation of two
/// target bodies as seen from an observing body.
///
/// The angular separations are calculated for each of 10
/// equally spaced times in a given time interval. A table
/// of the results is presented.
#[derive(Parser)]
struct Args {
    /// Leapseconds kernel file
    lsk: String,

    /// Binary SPK ephemeris file
    ephem: String,

    /// Observing body
    observer: String,

    /// First target body
    first: String,

    /// Second target body
    second: String,

    /// Beginning UTC time
    begin: String,

    /// Ending UTC time
    end: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut spice = SpiceContext::new();

    // Load the leapseconds information into the SPICELIB kernel pool
    // so that the time conversion routines can gain access to it.
    spice.furnsh(&args.lsk)?;

    // Load the SPK ephemeris file.
    spice.furnsh(&args.ephem)?;

    // Convert the UTC times to ephemeris seconds past J2000 (ET),
    // since that is what the SPICELIB readers are expecting.
    let et_beg = spice.str2et(&args.begin)?;
    let et_end = spice.str2et(&args.end)?;
    let utc_beg = spice.et2utc(et_beg, "C", 0)?;
    let utc_end = spice.et2utc(et_end, "C", 0)?;

    // Calculate the difference between evaluation times.
    let delta = (et_end - et_beg) / (MAXPTS - 1) as f64;

    // For each time, get the apparent states of the two target
    // bodies as seen from the observer.
    let mut state = vec![];
    for i in 0..MAXPTS {
        let et = et_beg + delta * i as f64;

        let (state1, _lt) = spice.spkezr(&args.first, et, "J2000", "LT+S", &args.observer)?;
        let (state2, _lt) = spice.spkezr(&args.second, et, "J2000", "LT+S", &args.observer)?;

        // State includes position and velocity. Extract the position,
        // and convert from slice to array
        let pos1 = state1[0..3].try_into().unwrap();
        let pos2 = state2[0..3].try_into().unwrap();

        // Save the time and the separation between the target bodies
        // (in degrees), as seen from the observer, for output to the
        // screen.
        state.push((et, spice.vsep(&pos1, &pos2) * spice.dpr()));
    }

    // Display the time and angular separation of the desired
    // target bodies for the requested observer for each of the
    // equally spaced evaluation times in the given time interval.
    //
    // If you have a graphics package, you may wish to write the
    // time and angular separation data to a file, and then plot
    // them for added effect.

    println!(
        "The angular separation between bodies {} and {},",
        args.first, args.second
    );
    println!("as seen from body {}.", args.observer);
    println!();
    println!("From: {utc_beg}");
    println!("To  : {utc_end}");
    println!();
    println!("       UTC Time                 Separation");
    println!("----------------------------------------------");

    for (et, sep) in state {
        let utc_tim = spice.et2utc(et, "C", 0)?;
        println!("  {utc_tim:20}     {sep:15.8} deg");
    }

    Ok(())
}
