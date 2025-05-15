use rsspice::*;

const METAKR: &str = "erotat.tm";
const X: [f64; 3] = [1.0, 0.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];

fn main() -> Result<()> {
    let mut spice = SpiceContext::new();

    // Load the kernels this program requires.
    spice.furnsh(METAKR)?;

    // Convert our UTC string to seconds past J2000 TDB.
    let timstr = "2007 JAN 1 00:00:00";
    let et = spice.str2et(timstr)?;

    // Look up the apparent position of the Moon relative
    // to the Earth's center in the IAU_EARTH frame at ET.
    let (lmoonv, _ltime) = spice.spkpos("MOON", et, "IAU_EARTH", "LT+S", "EARTH")?;

    // Express the Moon direction in terms of longitude
    // and latitude in the IAU_EARTH frame.
    let (_r, lon, lat) = spice.reclat(&lmoonv);

    println!();
    println!("Earth-Moon direction using low accuracy");
    println!("PCK and IAU_EARTH frame:");
    println!("Moon lon (deg):        {:15.6}", lon * spice.dpr());
    println!("Moon lat (deg):        {:15.6}", lat * spice.dpr());
    println!();

    // Look up the apparent position of the Moon relative
    // to the Earth's center in the ITRF93 frame at ET.
    let (hmoonv, _ltime) = spice.spkpos("MOON", et, "ITRF93", "LT+S", "EARTH")?;

    // Express the Moon direction in terms of longitude
    // and latitude in the ITRF93 frame.
    let (_r, lon, lat) = spice.reclat(&hmoonv);

    println!("Earth-Moon direction using high accuracy");
    println!("PCK and ITRF93 frame:");
    println!("Moon lon (deg):        {:15.6}", lon * spice.dpr());
    println!("Moon lat (deg):        {:15.6}", lat * spice.dpr());
    println!();

    // Find the angular separation of the Moon position
    // vectors in degrees.
    let sep = spice.dpr() * spice.vsep(&lmoonv, &hmoonv);

    println!("Earth-Moon vector separation angle (deg):     {sep:15.6}");
    println!();

    // Next, express the +Z and +X axes of the ITRF93 frame in
    // the IAU_EARTH frame. We'll do this for two times: ET
    // and ET + 100 days.
    for i in 0..2 {
        // Set the time, expressing the time delta in
        // seconds.
        let t = et + i as f64 * spice.spd() * 100.0;

        // Convert the TDB time T to a string for output.
        let outstr = spice.timout(t, "YYYY-MON-DD HR:MN:SC.### (UTC)")?;

        println!("Epoch: {outstr}");

        // Find the rotation matrix for conversion of position
        // vectors from the IAU_EARTH to the ITRF93 frame.
        let rmat = spice.pxform("IAU_EARTH", "ITRF93", t)?;

        let itrfx = [rmat[0][0], rmat[1][0], rmat[2][0]];
        let itrfz = [rmat[0][2], rmat[1][2], rmat[2][2]];

        // Display the angular offsets of the ITRF93 +X and +Z
        // axes from their IAU_EARTH counterparts.
        let sep = spice.vsep(&itrfx, &X);
        println!(
            "ITRF93 - IAU_EARTH +X axis separation angle (deg): {:13.6}",
            sep * spice.dpr()
        );

        let sep = spice.vsep(&itrfz, &Z);
        println!(
            "ITRF93 - IAU_EARTH +Z axis separation angle (deg): {:13.6}",
            sep * spice.dpr()
        );
        println!();
    }

    // Find the azimuth and elevation of the apparent
    // position of the Moon in the local topocentric
    // reference frame at the DSN station DSS-13.
    // First look up the Moon's position relative to the
    // station in that frame.
    let (topov, _ltime) = spice.spkpos("MOON", et, "DSS-13_TOPO", "LT+S", "DSS-13")?;

    // Express the station-Moon direction in terms of
    // longitude and latitude in the DSS-13_TOPO frame.
    let (_r, lon, lat) = spice.reclat(&topov);

    // Convert to azimuth/elevation.
    let az = (-lon).rem_euclid(std::f64::consts::TAU);
    let el = lat;

    println!("DSS-13-Moon az/el using high accuracy PCK and DSS-13_TOPO frame:");
    println!("Moon Az (deg):         {:15.6}", az * spice.dpr());
    println!("Moon El (deg):         {:15.6}", el * spice.dpr());
    println!();

    // Find the sub-solar point on the Earth at ET using the
    // Earth body-fixed frame IAU_EARTH. Treat the Sun as
    // the observer.
    let (lsub, _trgepc, _srfvec) = spice.subslr(
        "NEAR POINT: ELLIPSOID",
        "EARTH",
        et,
        "IAU_EARTH",
        "LT+S",
        "SUN",
    )?;

    // Display the sub-point in latitudinal coordinates.
    let (_r, lon, lat) = spice.reclat(&lsub);

    println!("Sub-Solar point on Earth using low accuracy");
    println!("PCK and IAU_EARTH frame:");
    println!("Sub-Solar lon (deg):   {:15.6}", lon * spice.dpr());
    println!("Sub-Solar lat (deg):   {:15.6}", lat * spice.dpr());
    println!();

    // Find the sub-solar point on the Earth at ET using the
    // Earth body-fixed frame ITRF93. Treat the Sun as
    // the observer.
    let (hsub, _trgepc, _srfvec) = spice.subslr(
        "NEAR POINT: ELLIPSOID",
        "EARTH",
        et,
        "ITRF93",
        "LT+S",
        "SUN",
    )?;

    // Display the sub-point in latitudinal coordinates.
    let (_r, lon, lat) = spice.reclat(&hsub);

    println!("Sub-Solar point on Earth using high accuracy");
    println!("PCK and ITRF93 frame:");
    println!("Sub-Solar lon (deg):   {:15.6}", lon * spice.dpr());
    println!("Sub-Solar lat (deg):   {:15.6}", lat * spice.dpr());
    println!();

    // Find the distance between the sub-solar point
    // vectors in km.
    let dist = spice.vdist(&lsub, &hsub);

    println!("Distance between sub-solar points (km): {dist:15.6}");

    Ok(())
}
