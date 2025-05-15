//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    CHCKPA: f64,
    CHCKPV: f64,
    CHCKTP: f64,
    COSINE: f64,
    COSINC: f64,
    DMDT: f64,
    DNODE: f64,
    DPERI: f64,
    EANOM: f64,
    ECC: f64,
    EPOCH: f64,
    GM: f64,
    MANOM: f64,
    MYPI: f64,
    NEAR: f64,
    OJ2: f64,
    P: f64,
    PA: StackArray<f64, 3>,
    PV: StackArray<f64, 3>,
    K2PI: f64,
    RPL: f64,
    SINE: f64,
    SMA: f64,
    TA: f64,
    THETA: f64,
    TP: StackArray<f64, 3>,
    TWOPIV: f64,
    TZERO: f64,
    U: StackArray<f64, 3>,
    VTEMP: StackArray<f64, 3>,
    Z: f64,
    J2FLG: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CHCKPA: f64 = 0.0;
        let mut CHCKPV: f64 = 0.0;
        let mut CHCKTP: f64 = 0.0;
        let mut COSINE: f64 = 0.0;
        let mut COSINC: f64 = 0.0;
        let mut DMDT: f64 = 0.0;
        let mut DNODE: f64 = 0.0;
        let mut DPERI: f64 = 0.0;
        let mut EANOM: f64 = 0.0;
        let mut ECC: f64 = 0.0;
        let mut EPOCH: f64 = 0.0;
        let mut GM: f64 = 0.0;
        let mut MANOM: f64 = 0.0;
        let mut MYPI: f64 = 0.0;
        let mut NEAR: f64 = 0.0;
        let mut OJ2: f64 = 0.0;
        let mut P: f64 = 0.0;
        let mut PA = StackArray::<f64, 3>::new(1..=3);
        let mut PV = StackArray::<f64, 3>::new(1..=3);
        let mut K2PI: f64 = 0.0;
        let mut RPL: f64 = 0.0;
        let mut SINE: f64 = 0.0;
        let mut SMA: f64 = 0.0;
        let mut TA: f64 = 0.0;
        let mut THETA: f64 = 0.0;
        let mut TP = StackArray::<f64, 3>::new(1..=3);
        let mut TWOPIV: f64 = 0.0;
        let mut TZERO: f64 = 0.0;
        let mut U = StackArray::<f64, 3>::new(1..=3);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut Z: f64 = 0.0;
        let mut J2FLG: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            CHCKPA,
            CHCKPV,
            CHCKTP,
            COSINE,
            COSINC,
            DMDT,
            DNODE,
            DPERI,
            EANOM,
            ECC,
            EPOCH,
            GM,
            MANOM,
            MYPI,
            NEAR,
            OJ2,
            P,
            PA,
            PV,
            K2PI,
            RPL,
            SINE,
            SMA,
            TA,
            THETA,
            TP,
            TWOPIV,
            TZERO,
            U,
            VTEMP,
            Z,
            J2FLG,
            FIRST,
        }
    }
}

//$Procedure      SPKE53 ( Evaluate a type 53 SPK data record)
pub fn SPKE53(
    ET: f64,
    RECIN: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RECIN = DummyArray::new(RECIN, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"SPKE53", ctx)?;

    if save.FIRST {
        save.FIRST = false;
        save.TWOPIV = spicelib::TWOPI(ctx);
        save.MYPI = spicelib::PI(ctx);
    }

    //
    // Fetch the various entities from the input record, first the epoch.
    //
    save.EPOCH = RECIN[1];

    //
    // The trajectory pole vector.
    //
    save.TP[1] = RECIN[2];
    save.TP[2] = RECIN[3];
    save.TP[3] = RECIN[4];

    //
    // The periapsis vector.
    //
    save.PA[1] = RECIN[5];
    save.PA[2] = RECIN[6];
    save.PA[3] = RECIN[7];

    //
    // Semi-latus rectum ( P in the P/(1 + ECC*COS(Nu)  ),
    // eccentricity, and time from periapsis at epoch.
    //
    save.P = RECIN[8];
    save.ECC = RECIN[9];
    save.TZERO = RECIN[10];

    //
    // J2 processing flag.
    //
    save.J2FLG = (RECIN[11] as i32);

    //
    // Central body pole vector.
    //
    save.PV[1] = RECIN[12];
    save.PV[2] = RECIN[13];
    save.PV[3] = RECIN[14];

    //
    // The central mass, J2 and radius of the central body.
    //
    save.GM = RECIN[15];
    save.OJ2 = RECIN[16];
    save.RPL = RECIN[17];

    //
    // Check all the inputs here for obvious failures.  Yes, perhaps
    // this is overkill.  However, there is a lot more computation
    // going on in this routine so that the small amount of overhead
    // here should not be significant.
    //
    save.CHCKPA = ((f64::abs(save.PA[1]) + f64::abs(save.PA[2])) + f64::abs(save.PA[3]));
    save.CHCKPV = ((f64::abs(save.PV[1]) + f64::abs(save.PV[2])) + f64::abs(save.PV[3]));
    save.CHCKTP = ((f64::abs(save.TP[1]) + f64::abs(save.TP[2])) + f64::abs(save.TP[3]));

    if (save.P == 0 as f64) {
        spicelib::SETMSG(b"The semi-latus rectum supplied to the SPK type 53 evaluator was zero.  This value must be non-zero. ", ctx);

        spicelib::SIGERR(b"SPICE(BADLATUSRECTUM)", ctx)?;
        spicelib::CHKOUT(b"SPKE53", ctx)?;
        return Ok(());
    } else if ((save.ECC == 1.0) || (save.ECC < 0.0)) {
        spicelib::SETMSG(b"The eccentricity supplied for a type 53 segment is out of the range of acceptable values ( 0 <= ecc < 1 or ecc >1). The value supplied to the type 53 evaluator was #. ", ctx);

        spicelib::ERRDP(b"#", save.ECC, ctx);
        spicelib::SIGERR(b"SPICE(BADECCENTRICITY)", ctx)?;
        spicelib::CHKOUT(b"SPKE53", ctx)?;
        return Ok(());
    } else if (save.CHCKTP == 0.0) {
        spicelib::SETMSG(b"The trajectory pole vector supplied to SPKE53 had length zero. The most likely cause of this problem is a corrupted SPK (ephemeris) file. ", ctx);
        spicelib::SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        spicelib::CHKOUT(b"SPKE53", ctx)?;
        return Ok(());
    } else if (save.CHCKPA == 0.0) {
        spicelib::SETMSG(b"The periapse vector supplied to SPKE53 had length zero. The most likely cause of this problem is a corrupted SPK (ephemeris) file. ", ctx);
        spicelib::SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        spicelib::CHKOUT(b"SPKE53", ctx)?;
        return Ok(());
    } else if (save.CHCKPV == 0.0) {
        spicelib::SETMSG(b"The central pole vector supplied to SPKE53 had length zero. The most likely cause of this problem is a corrupted SPK (ephemeris) file. ", ctx);
        spicelib::SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        spicelib::CHKOUT(b"SPKE53", ctx)?;
        return Ok(());
    }

    //
    // Convert TP, PV and PA to unit vectors.
    // (It won't hurt to polish them up a bit here if they are already
    //  unit vectors.)
    //
    spicelib::VHATIP(save.PA.as_slice_mut());
    spicelib::VHATIP(save.TP.as_slice_mut());
    spicelib::VHATIP(save.PV.as_slice_mut());

    //
    // Compute the semi-major axis, mean motion, and distance a periapse.
    //
    save.SMA = (save.P / (1.0 - f64::powi(save.ECC, 2)));
    save.DMDT = f64::sqrt(f64::abs((save.GM / f64::powi(save.SMA, 3))));
    save.NEAR = (save.P / (1.0 + save.ECC));

    //
    //
    // Next compute the eccentric anomaly and from that, TA, the true
    // anomaly.
    //
    save.MANOM = (((save.TZERO + ET) - save.EPOCH) * save.DMDT);

    if (save.ECC < 1.0) {
        //
        // Next compute the angle THETA such that THETA is between
        // -pi and pi and such than MANOM = THETA + K*2*pi for
        // some integer K.
        //
        save.THETA = intrinsics::DMOD(save.MANOM, save.TWOPIV);

        if (f64::abs(save.THETA) > save.MYPI) {
            save.THETA = (save.THETA - f64::copysign(save.TWOPIV, save.THETA));
        }

        save.K2PI = (save.MANOM - save.THETA);
        //
        // Compute the eccentric anomaly associated with THETA.
        //
        spicelib::ELLTOF(save.THETA, save.ECC, &mut save.EANOM, ctx)?;

        save.SINE = (f64::sin((save.EANOM / 2 as f64))
            * f64::sqrt((((1 as f64) + save.ECC) / ((1 as f64) - save.ECC))));
        save.COSINE = f64::cos((save.EANOM / 2 as f64));

        //
        // Finally, compute the accumulated true anomaly.  That is,
        // add in the accumulated angle K2PI (This works because like
        // THETA, TA is always between -PI and PI.)
        //
        save.TA = ((2.0 * f64::atan2(save.SINE, save.COSINE)) + save.K2PI);
    } else {
        spicelib::HYPTOF(save.MANOM, save.ECC, &mut save.EANOM, ctx)?;
        save.TA = (2.0
            * f64::atan(
                (f64::sqrt(((save.ECC + 1 as f64) / (save.ECC - 1 as f64)))
                    * f64::tanh((save.EANOM / 2 as f64))),
            ));
    }

    //
    // If called for, handle precession needed due to the J2 term.
    //
    if ((((save.J2FLG != 3) && (save.OJ2 != 0.0)) && (save.ECC < 1.0)) && (save.NEAR > save.RPL)) {
        //
        // Determine how far the line of nodes and periapsis have moved.
        //
        save.COSINC = spicelib::VDOT(save.PV.as_slice(), save.TP.as_slice());

        save.Z = (((save.TA * 1.5) * save.OJ2) * f64::powi((save.RPL / save.P), 2));
        save.DNODE = -(save.Z * save.COSINC);
        save.DPERI = (save.Z * ((2.5 * f64::powi(save.COSINC, 2)) - 0.5));

        //
        // Regress the line of nodes by rotating the periapsis and
        // trajectory pole vectors about the the pole of the central
        // body.
        //
        if (save.J2FLG != 2) {
            spicelib::VROTV(
                save.TP.as_slice(),
                save.PV.as_slice(),
                save.DNODE,
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VEQU(save.VTEMP.as_slice(), save.TP.as_slice_mut());

            spicelib::VROTV(
                save.PA.as_slice(),
                save.PV.as_slice(),
                save.DNODE,
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VEQU(save.VTEMP.as_slice(), save.PA.as_slice_mut());
        }

        //
        // Precess periapsis by rotating the periapsis vector about the
        // trajectory pole
        //
        if (save.J2FLG != 1) {
            spicelib::VROTV(
                save.PA.as_slice(),
                save.TP.as_slice(),
                save.DPERI,
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VEQU(save.VTEMP.as_slice(), save.PA.as_slice_mut());
        }
    }

    //
    // That's it finish the state computation.  Rotate the periapsis
    // vector by the true anomaly about the trajectory pole vector.
    // This gives a unit vector that points towards the position at ET.
    //
    spicelib::VROTV(
        save.PA.as_slice(),
        save.TP.as_slice(),
        save.TA,
        save.U.as_slice_mut(),
    );

    //
    // Compute the range from the central body and scale up the
    // position vector to get the direction vector.
    //
    save.Z = (save.P / (1.0 + (save.ECC * f64::cos(save.TA))));

    spicelib::VSCL(save.Z, save.U.as_slice(), STATE.subarray_mut(1));

    //
    //
    // Finally, the velocity. Recall that the velocity is given by
    // adding the position direction to an eccentricity vector,
    // rotating by 90 degrees in the plane of the orbit, and
    // scaling by the appropriate factor.  The "eccentricity
    // vector" is ECC times the the unit vector parallel to the
    // position at periapse.  Thus, up to scale, the velocity is:
    //
    //    TP x ( U + ECC * PA )
    //
    spicelib::VLCOM(
        1.0,
        save.U.as_slice(),
        save.ECC,
        save.PA.as_slice(),
        STATE.subarray_mut(4),
    );
    spicelib::VCRSS(
        save.TP.as_slice(),
        STATE.subarray(4),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), STATE.subarray_mut(4));

    //
    // Finally, scale up the direction of the velocity to
    // get the velocity.
    //
    save.Z = f64::sqrt((save.GM / save.P));

    spicelib::VSCLIP(save.Z, STATE.subarray_mut(4));

    spicelib::CHKOUT(b"SPKE53", ctx)?;
    Ok(())
}
