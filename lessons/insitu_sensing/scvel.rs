use rsspice::*;

fn main() -> Result<()> {
    let mut spice = SpiceContext::new();

    spice.furnsh("scvel.tm")?;

    let utc = "2004-06-11T19:32:00";
    let et = spice.str2et(utc)?;

    println!("UTC       = {utc}");
    println!("ET        = {et:20.6}");

    let scid = -82;
    let sclk = "1465674964.105";
    let et = spice.scs2e(scid, sclk)?;

    println!("SCLK      = {sclk}");
    println!("ET        = {et:20.6}");

    let target = "CASSINI";
    let frame = "ECLIPJ2000";
    let corrtn = "NONE";
    let observ = "SUN";
    let (state, _lt) = spice.spkezr(target, et, frame, corrtn, observ)?;

    println!(" X        = {:20.6}", state[0]);
    println!(" Y        = {:20.6}", state[1]);
    println!(" Z        = {:20.6}", state[2]);
    println!("VX        = {:20.6}", state[3]);
    println!("VY        = {:20.6}", state[4]);
    println!("VZ        = {:20.6}", state[5]);

    let target = "SUN";
    let frame = "CASSINI_INMS";
    let corrtn = "LT+S";
    let observ = "CASSINI";
    let (mut sundir, _lt) = spice.spkpos(target, et, frame, corrtn, observ)?;
    spice.vhatip(&mut sundir);

    println!("SUNDIR(X) = {:20.6}", sundir[0]);
    println!("SUNDIR(Y) = {:20.6}", sundir[1]);
    println!("SUNDIR(Z) = {:20.6}", sundir[2]);

    let method = "NEAR POINT: ELLIPSOID";
    let target = "PHOEBE";
    let frame = "IAU_PHOEBE";
    let corrtn = "NONE";
    let observ = "CASSINI";
    let (spoint, _trgepc, srfvec) = spice.subpnt(method, target, et, frame, corrtn, observ)?;
    let (_srad, slon, slat) = spice.reclat(&spoint);

    let fromfr = "IAU_PHOEBE";
    let tofr = "CASSINI_INMS";
    let m2imat = spice.pxform(fromfr, tofr, et)?;

    let mut sbpdir = spice.mxv(&m2imat, &srfvec);
    spice.vhatip(&mut sbpdir);

    println!("LON       = {:20.6}", slon * spice.dpr());
    println!("LAT       = {:20.6}", slat * spice.dpr());
    println!("SBPDIR(X) = {:20.6}", sbpdir[0]);
    println!("SBPDIR(Y) = {:20.6}", sbpdir[1]);
    println!("SBPDIR(Z) = {:20.6}", sbpdir[2]);

    let target = "CASSINI";
    let frame = "J2000";
    let corrtn = "NONE";
    let observ = "PHOEBE";
    let (state, _lt) = spice.spkezr(target, et, frame, corrtn, observ)?;
    let scvdir = spice.vpack(state[3], state[4], state[5]);

    let fromfr = "J2000";
    let tofr = "CASSINI_INMS";
    let j2imat = spice.pxform(fromfr, tofr, et)?;

    let tmpdir = spice.mxv(&j2imat, &scvdir);
    let scvdir = spice.vhat(&tmpdir);

    println!("SCVDIR(X) = {:20.6}", scvdir[0]);
    println!("SCVDIR(Y) = {:20.6}", scvdir[1]);
    println!("SCVDIR(Z) = {:20.6}", scvdir[2]);

    Ok(())
}
