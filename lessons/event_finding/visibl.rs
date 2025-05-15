use rsspice::*;

// Format string for time output:
const TDBFMT: &str = "YYYY MON DD HR:MN:SC.### TDB ::TDB";

// The meta-kernel:
const METAKR: &str = "visibl.tm";

// Maximum number of events we can handle in our event set:
const MAXEVT: usize = 1000;

// Maximum result window size:
const MAXWIN: usize = 2 * MAXEVT;

// Find and display the window of times when the MEX
// spacecraft is above a specified elevation limit in the
// topocentric reference frame of DSN station DSS-14
// and is not occulted by Mars.
fn main() -> Result<()> {
    let mut spice = SpiceContext::new();

    // Confinement window used to store interval to be searched:
    let mut cnfine = Cell::with_capacity(MAXEVT);

    // Result window used to store occultation times:
    let mut occwin = Cell::with_capacity(MAXWIN);

    // Result window used to store occultation times
    // computed using DSK data:
    let mut docwin = Cell::with_capacity(MAXWIN);

    // Result window used to store rise/set times:
    let mut riswin = Cell::with_capacity(MAXWIN);

    // Result windows used to store visibility periods:
    let mut dviswn = Cell::with_capacity(MAXWIN);
    let mut eviswn = Cell::with_capacity(MAXWIN);

    // Load the meta-kernel.
    spice.furnsh(METAKR)?;

    // Assign the inputs for our search.
    let srfpt = "DSS-14";
    let frame = "DSS-14_TOPO";
    let target = "MEX";
    let abcorr = "CN+S";
    let start = "2004 MAY 2 TDB";
    let stop = "2004 MAY 6 TDB";
    let elvlim = 6.0;

    // The elevation limit above has units of degrees; we convert
    // this value to radians for computation using SPICE routines.
    // We'll store the equivalent value in radians in REVLIM.

    let revlim = spice.rpd() * elvlim;

    // Since we're interested in the apparent location of the
    // target, we use light time and stellar aberration
    // corrections.
    //
    // Since SPICE doesn't directly support the AZ/EL coordinate
    // system, we use the equivalent constraint
    //
    //    latitude > REVLIM
    //
    // in the latitudinal coordinate system, where the reference
    // frame is topocentric and is centered at the viewing location.

    let crdsys = "LATITUDINAL";
    let coord = "LATITUDE";
    let relate = ">";

    // STEPSZ is the step size, measured in seconds, used to search
    // for times bracketing a state transition. Since we don't expect
    // any events of interest to be shorter than five minutes, and
    // since the separation between events is well over 5 minutes,
    // we'll use this value as our step size. Units are seconds.

    let stepsz = 300.0;

    // We model the target shape as a point. We either model the
    // blocking body's shape as an ellipsoid, or we represent its shape
    // using actual topographic data. No body-fixed reference frame is
    // required for the target since its orientation is not used.
    //
    // We'll set the shape specifications just before our calls to
    // the GF occultation finding routine GFOCLT.

    let back = target;
    let bshape = "POINT";
    let bframe = " ";
    let front = "MARS";
    let fframe = "IAU_MARS";

    // The occultation type should be set to 'ANY' for a point
    // target.

    let occtyp = "ANY";

    // Display a banner for the output report:
    println!();
    println!("Inputs for target visibility search:");
    println!();
    println!("   Target                       = {target}");
    println!("   Observation surface location = {srfpt}");
    println!("   Observer's reference frame   = {frame}");
    println!("   Blocking body                = {front}");
    println!("   Blocker's reference frame    = {fframe}");
    println!("   Elevation limit (degrees)    = {elvlim:.6}");
    println!("   Aberration correction        = {abcorr}");
    println!("   Step size (seconds)          = {stepsz:.6}");

    // Convert the start and stop times to ET.
    let etbeg = spice.str2et(start)?;
    let etend = spice.str2et(stop)?;

    // Display the search interval start and stop times
    // using the format shown below.
    //
    //    2004 MAY 06 20:15:00.000 (TDB)

    println!(
        "   Start time                   = {}",
        spice.timout(etbeg, TDBFMT)?
    );
    println!(
        "   Stop time                    = {}",
        spice.timout(etend, TDBFMT)?
    );
    println!();

    // Every SPICELIB window must have its size initialized.
    //
    // Initialize the 'confinement' window with the interval
    // over which we'll conduct the search.

    spice.wninsd(etbeg, etend, &mut cnfine)?;

    // The adjustment value only applies to absolute extrema
    // searches;  simply give it an initial value of zero
    // for this inequality search.
    let adjust = 0.0;

    // Note that the workspace dimensions are ( MAXWIN, NWMAX ).
    //
    // Now search for the time period, within our confinement
    // window, during which the apparent target has elevation
    // at least equal to the elevation limit.

    spice.gfposc(
        target,
        frame,
        abcorr,
        srfpt,
        crdsys,
        coord,
        relate,
        revlim,
        adjust,
        stepsz,
        &cnfine,
        MAXWIN as i32,
        &mut riswin,
    )?;

    // Now find the times when the apparent target is above
    // the elevation limit and is not occulted by the
    // blocking body (Mars). We'll find the times when the target
    // is above the elevation limit and *is* occulted, then subtract
    // that window from the view period window RISWIN found above.
    //
    // For this occultation search, we can use RISWIN as
    // the confinement window because we're not interested in
    // occultations that occur when the target is below the
    // elevation limit.
    //
    // Find occultations within the view period window.

    let fshape = "ELLIPSOID";

    println!("Searching using ellipsoid target shape model...");
    spice.gfoclt(
        occtyp,
        front,
        fshape,
        fframe,
        back,
        bshape,
        bframe,
        abcorr,
        srfpt,
        stepsz,
        &riswin,
        &mut occwin,
    )?;
    println!("Done.");

    // Subtract the occultation window from the view period
    // window: this yields the time periods when the target
    // is visible.

    spice.wndifd(&riswin, &occwin, &mut eviswn)?;

    // Repeat the search using low-resolution DSK data
    // for the front body.

    let fshape = "DSK/UNPRIORITIZED";

    println!("Searching using DSK target shape model...");
    spice.gfoclt(
        occtyp,
        front,
        fshape,
        fframe,
        back,
        bshape,
        bframe,
        abcorr,
        srfpt,
        stepsz,
        &riswin,
        &mut docwin,
    )?;
    println!("Done.");

    spice.wndifd(&riswin, &docwin, &mut dviswn)?;

    // The function WNCARD returns the number of intervals
    // in a SPICE window.

    let winsiz = spice.wncard(&eviswn)?;

    if winsiz == 0 {
        println!("No events were found.");
    } else {
        // Display the visibility time periods.
        println!();
        println!("Visibility start and stop times of {target} as seen from {srfpt}");
        println!("using both ellipsoidal and DSK target shape models:");
        println!();

        for i in 1..=winsiz {
            // Fetch the start and stop times of the Ith interval
            // from the ellipsoid search result window EVISWN.

            let (intbeg, intend) = spice.wnfetd(&eviswn, i)?;

            // Convert the start and stop times to TDB calendar
            // strings. Write the results.

            let btmstr = spice.timout(intbeg, TDBFMT)?;
            let etmstr = spice.timout(intend, TDBFMT)?;

            println!(" Ell: {btmstr} : {etmstr}");

            // Fetch the start and stop times of the Ith interval
            // from the DSK search result window DVISWN.

            let (dintbg, dinten) = spice.wnfetd(&dviswn, i)?;

            // Convert the start and stop times to TDB calendar
            // strings. Write the results.

            let btmstr = spice.timout(dintbg, TDBFMT)?;
            let etmstr = spice.timout(dinten, TDBFMT)?;

            println!(" DSK: {btmstr} : {etmstr}");
            println!();
        }
    }
    Ok(())
}
