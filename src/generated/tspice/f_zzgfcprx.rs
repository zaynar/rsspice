//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK: &[u8] = b"zzgfcprx.tpc";
const LNSIZE: i32 = 200;
const EPSLAT: f64 = 0.1;
const TWEAK: f64 = 0.001;
const NSYS: i32 = 7;
const SYSLEN: i32 = 32;
const CRDLEN: i32 = 32;
const NLATSM: i32 = 5;
const NLONSM: i32 = 8;

struct SaveVars {
    CRDNMS: ActualCharArray2D,
    CRDSYS: Vec<u8>,
    SYSNMS: ActualCharArray,
    TITLE: Vec<u8>,
    QNAME: Vec<u8>,
    DELTLA: f64,
    DELTLO: f64,
    DLAT: f64,
    DLON: f64,
    DP: f64,
    F: f64,
    JACOBI: StackArray2D<f64, 9>,
    LAT: f64,
    LON: f64,
    RE: f64,
    STATE: StackArray<f64, 6>,
    XVEL: StackArray<f64, 3>,
    CDSIGN: StackArray<i32, 3>,
    CI: i32,
    SENSE: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CRDNMS = ActualCharArray2D::new(CRDLEN, 1..=3, 1..=NSYS);
        let mut CRDSYS = vec![b' '; CRDLEN as usize];
        let mut SYSNMS = ActualCharArray::new(SYSLEN, 1..=NSYS);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut DELTLA: f64 = 0.0;
        let mut DELTLO: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut DP: f64 = 0.0;
        let mut F: f64 = 0.0;
        let mut JACOBI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut RE: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut XVEL = StackArray::<f64, 3>::new(1..=3);
        let mut CDSIGN = StackArray::<i32, 3>::new(1..=3);
        let mut CI: i32 = 0;
        let mut SENSE: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(RECSYS),
                Val::C(LATSYS),
                Val::C(RADSYS),
                Val::C(SPHSYS),
                Val::C(CYLSYS),
                Val::C(GEOSYS),
                Val::C(PGRSYS),
            ]
            .into_iter();
            SYSNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(XCRD),
                Val::C(YCRD),
                Val::C(ZCRD),
                Val::C(RADCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(RNGCRD),
                Val::C(RACRD),
                Val::C(DECCRD),
                Val::C(RADCRD),
                Val::C(CLTCRD),
                Val::C(LONCRD),
                Val::C(RADCRD),
                Val::C(LONCRD),
                Val::C(ZCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(ALTCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(ALTCRD),
            ]
            .into_iter();
            CRDNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CRDNMS,
            CRDSYS,
            SYSNMS,
            TITLE,
            QNAME,
            DELTLA,
            DELTLO,
            DLAT,
            DLON,
            DP,
            F,
            JACOBI,
            LAT,
            LON,
            RE,
            STATE,
            XVEL,
            CDSIGN,
            CI,
            SENSE,
        }
    }
}

//$Procedure      F_ZZGFCPRX ( Test coordinate derivative proxy routine )
pub fn F_ZZGFCPRX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Small delta value used for near 0 and pi
    // latitudes.
    //

    //
    // Number of recognized coordinate systems.
    //

    //
    // Maximum length of a coordinate system name.
    //

    //
    // Maximum length of a coordinate name.
    //

    //
    // Numbers of latitude and longitude samples.
    //

    //
    // Local Variables
    //

    //
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Names of supported coordinate systems.
    //
    // The Ith coordinate system in the array SYSNMS has coordinates
    // in the Ith row of the array CRDNMS. This association must be
    // preserved when this routine is updated.
    //

    //
    // Names of coordinate triples for the supported coordinate
    // systems.
    //
    // The order of the coordinate names in the Ith row of this array
    // matches the order of the outputs of the corresponding
    // SPICELIB routine REC*, which maps a Cartesian vector to
    // the Ith coordinate system in the array SYSNMS. Again, this
    // order must be preserved.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFCPRX", ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create and load PCK file.", ctx)?;
    //
    // Load a PCK.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate system", ctx)?;

    //
    // Set shape and spin parameters.
    //
    save.RE = 100000.0;
    save.F = 0.5;
    save.SENSE = -1;

    spicelib::FILLD(1.0, 6, save.STATE.as_slice_mut());

    spicelib::ZZGFCPRX(
        save.STATE.as_slice(),
        b"GAUSSIAN",
        save.RE,
        save.F,
        save.SENSE,
        save.CDSIGN.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad longitude sense", ctx)?;

    //
    // Set shape and spin parameters.
    //
    save.RE = 100000.0;
    save.F = 0.5;
    save.SENSE = -2;

    spicelib::FILLD(1.0, 6, save.STATE.as_slice_mut());

    spicelib::ZZGFCPRX(
        save.STATE.as_slice(),
        b"PLANETOGRAPHIC",
        save.RE,
        save.F,
        save.SENSE,
        save.CDSIGN.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad equatorial radius", ctx)?;

    //
    // Set shape and spin parameters.
    //
    save.RE = -100000.0;
    save.F = 0.5;
    save.SENSE = 1;

    spicelib::FILLD(1.0, 6, save.STATE.as_slice_mut());

    spicelib::ZZGFCPRX(
        save.STATE.as_slice(),
        b"PLANETOGRAPHIC",
        save.RE,
        save.F,
        save.SENSE,
        save.CDSIGN.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::ZZGFCPRX(
        save.STATE.as_slice(),
        b"GEODETIC",
        save.RE,
        save.F,
        save.SENSE,
        save.CDSIGN.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad flattening coefficient", ctx)?;

    //
    // Set shape and spin parameters.
    //
    save.RE = 100000.0;
    save.F = 1000000.0;
    save.SENSE = 1;

    spicelib::FILLD(1.0, 6, save.STATE.as_slice_mut());

    spicelib::ZZGFCPRX(
        save.STATE.as_slice(),
        b"PLANETOGRAPHIC",
        save.RE,
        save.F,
        save.SENSE,
        save.CDSIGN.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::ZZGFCPRX(
        save.STATE.as_slice(),
        b"GEODETIC",
        save.RE,
        save.F,
        save.SENSE,
        save.CDSIGN.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Non-error Exceptional cases
    //*
    //*********************************************************************

    //
    // Test handling of zero velocity vector.
    //
    testutil::TCASE(b"Input velocity is zero; position is off Z-axis.", ctx)?;

    spicelib::VPACK(1.0, 1.0, 1.0, save.STATE.as_slice_mut());
    spicelib::CLEARD(3, save.STATE.subarray_mut(4));

    for SI in 1..=NSYS {
        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Zero velocity; system is #");

        spicelib::REPMC(
            &save.TITLE.to_vec(),
            b"#",
            &save.SYSNMS[SI],
            &mut save.TITLE,
        );

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::ZZGFCPRX(
            save.STATE.as_slice(),
            &save.SYSNMS[SI],
            1.0,
            0.0,
            1,
            save.CDSIGN.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.CI = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut save.QNAME, b"sign of d(");
                spicelib::SUFFIX(&save.CRDNMS[[save.CI, SI]], 0, &mut save.QNAME);
                spicelib::SUFFIX(b")/dt", 0, &mut save.QNAME);

                testutil::CHCKSI(&save.QNAME, save.CDSIGN[save.CI], b"=", 0, 0, OK, ctx)?;

                save.CI += m3__;
            }
        }
    }

    // Test handling of zero velocity vector.
    //
    testutil::TCASE(b"Input velocity is zero; position is on Z-axis.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.STATE.as_slice_mut());
    spicelib::CLEARD(3, save.STATE.subarray_mut(4));

    for SI in 1..=NSYS {
        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Zero velocity; system is #");

        spicelib::REPMC(
            &save.TITLE.to_vec(),
            b"#",
            &save.SYSNMS[SI],
            &mut save.TITLE,
        );

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::ZZGFCPRX(
            save.STATE.as_slice(),
            &save.SYSNMS[SI],
            1.0,
            0.0,
            1,
            save.CDSIGN.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.CI = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut save.QNAME, b"sign of d(");
                spicelib::SUFFIX(&save.CRDNMS[[save.CI, SI]], 0, &mut save.QNAME);
                spicelib::SUFFIX(b")/dt", 0, &mut save.QNAME);

                testutil::CHCKSI(&save.QNAME, save.CDSIGN[save.CI], b"=", 0, 0, OK, ctx)?;

                save.CI += m3__;
            }
        }
    }

    //
    //     Test results for positions on the Z-axis, with velocity
    //     orthogonal to Z. Applies only to the cylindrical system.
    //
    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"velocity orthogonal to Z; system is #");

    spicelib::REPMC(&save.TITLE.to_vec(), b"#", CYLSYS, &mut save.TITLE);

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.STATE.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.STATE.subarray_mut(4));

    spicelib::ZZGFCPRX(
        save.STATE.as_slice(),
        CYLSYS,
        1.0,
        0.0,
        1,
        save.CDSIGN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.CI = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Note: the index of the cylindrical system is 5.
            //
            fstr::assign(&mut save.QNAME, b"sign of d(");
            spicelib::SUFFIX(&save.CRDNMS[[save.CI, 5]], 0, &mut save.QNAME);
            spicelib::SUFFIX(b")/dt", 0, &mut save.QNAME);

            testutil::CHCKSI(&save.QNAME, save.CDSIGN[save.CI], b"=", 0, 0, OK, ctx)?;

            save.CI += m3__;
        }
    }

    //
    // Test results for range or altitude coordinates for positions on
    // the Z-axis, with velocity orthogonal to Z. Applies to all but
    // rectangular systems.
    //
    // The rectangular system has index 1, so start at index 2.
    //
    for SI in 2..=NSYS {
        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.CRDSYS, save.SYSNMS.get(SI));

        fstr::assign(&mut save.TITLE, b"velocity orthogonal to Z; system is #");

        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::VPACK(0.0, 0.0, 1.0, save.STATE.as_slice_mut());
        spicelib::VPACK(1.0, 0.0, 0.0, save.STATE.subarray_mut(4));

        spicelib::ZZGFCPRX(
            save.STATE.as_slice(),
            CYLSYS,
            1.0,
            0.0,
            1,
            save.CDSIGN.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set CI to the index of the radius or altitude coordinate
        // of the current system.
        //
        if fstr::eq(&save.CRDSYS, LATSYS) {
            save.CI = 1;
        } else if fstr::eq(&save.CRDSYS, GEOSYS) {
            save.CI = 3;
        } else if fstr::eq(&save.CRDSYS, RADSYS) {
            save.CI = 1;
        } else if fstr::eq(&save.CRDSYS, PGRSYS) {
            save.CI = 3;
        } else if fstr::eq(&save.CRDSYS, CYLSYS) {
            save.CI = 1;
        } else if fstr::eq(&save.CRDSYS, SPHSYS) {
            save.CI = 1;
        } else {
            spicelib::SETMSG(b"Unexpected system: #", ctx);
            spicelib::ERRCH(b"#", &save.CRDSYS, ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        fstr::assign(&mut save.QNAME, b"sign of d(");
        spicelib::SUFFIX(&save.CRDNMS[[save.CI, SI]], 0, &mut save.QNAME);
        spicelib::SUFFIX(b")/dt", 0, &mut save.QNAME);

        testutil::CHCKSI(&save.QNAME, save.CDSIGN[save.CI], b"=", 0, 0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // Test results for positions on the Z-axis.
    //
    for NLAT in 1..=3 {
        //
        // Generate Z values corresponding to
        //
        //    +1, 0, -1
        //
        // positions, respectively.
        //
        save.STATE[1] = 0.0;
        save.STATE[2] = 0.0;
        save.STATE[3] = (2.0 - (NLAT as f64));

        for NDLAT in 1..=2 {
            //
            // Generate latitude values corresponding to
            //
            //    +Pi/4, -Pi/4
            //
            // positions, respectively.
            //
            save.DLAT = ((((3 - (2 * NDLAT)) as f64) * spicelib::PI(ctx)) / 4.0);
            save.DLON = (spicelib::PI(ctx) / 4.0);

            spicelib::LATREC(1.0, save.DLON, save.DLAT, save.STATE.subarray_mut(4));

            save.DP = spicelib::VDOT(save.STATE.as_slice(), save.STATE.subarray(4));
            //
            // Check the coordinate derivative signs for
            // each coordinate system.
            //
            for SI in 1..=NSYS {
                //
                //---- Case -------------------------------------------------------------
                //
                fstr::assign(&mut save.TITLE, b"#; Z #; dlat #(deg); dlon #(deg)");

                spicelib::REPMC(
                    &save.TITLE.to_vec(),
                    b"#",
                    &save.SYSNMS[SI],
                    &mut save.TITLE,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.STATE[3],
                    3,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    (spicelib::DPR(ctx) * save.DLAT),
                    3,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    (spicelib::DPR(ctx) * save.DLON),
                    3,
                    &mut save.TITLE,
                    ctx,
                );

                testutil::TCASE(&save.TITLE, ctx)?;

                if fstr::eq(save.SYSNMS.get(SI), RECSYS) {
                    for I in 1..=3 {
                        save.XVEL[I] = save.STATE[(3 + I)];
                    }
                } else if fstr::eq(save.SYSNMS.get(SI), LATSYS) {
                    save.XVEL[1] = save.DP;
                    save.XVEL[2] = 0.0;
                    save.XVEL[3] = 0.0;
                } else if fstr::eq(save.SYSNMS.get(SI), RADSYS) {
                    save.XVEL[1] = save.DP;
                    save.XVEL[2] = 0.0;
                    save.XVEL[3] = 0.0;
                } else if fstr::eq(save.SYSNMS.get(SI), SPHSYS) {
                    save.XVEL[1] = save.DP;
                    save.XVEL[2] = 0.0;
                    save.XVEL[3] = 0.0;
                } else if fstr::eq(save.SYSNMS.get(SI), CYLSYS) {
                    save.XVEL[1] = 0.0;
                    save.XVEL[2] = 0.0;
                    save.XVEL[3] = save.STATE[6];
                } else if fstr::eq(save.SYSNMS.get(SI), GEOSYS) {
                    save.XVEL[1] = 0.0;
                    save.XVEL[2] = 0.0;
                    save.XVEL[3] = save.DP;
                } else if fstr::eq(save.SYSNMS.get(SI), PGRSYS) {
                    save.XVEL[1] = 0.0;
                    save.XVEL[2] = 0.0;
                    save.XVEL[3] = save.DP;
                } else {
                    spicelib::SETMSG(b"Bad coordinate system #", ctx);
                    spicelib::ERRCH(b"#", &save.SYSNMS[SI], ctx);
                    spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Set shape and spin parameters.
                //
                save.RE = 100000.0;
                save.F = 0.5;
                save.SENSE = -1;

                spicelib::ZZGFCPRX(
                    save.STATE.as_slice(),
                    &save.SYSNMS[SI],
                    save.RE,
                    save.F,
                    save.SENSE,
                    save.CDSIGN.as_slice_mut(),
                    ctx,
                )?;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 3;
                    let m3__: i32 = 1;
                    save.CI = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        fstr::assign(&mut save.QNAME, b"sign of d(");
                        spicelib::SUFFIX(&save.CRDNMS[[save.CI, SI]], 0, &mut save.QNAME);
                        spicelib::SUFFIX(b")/dt", 0, &mut save.QNAME);

                        if (save.XVEL[save.CI] > 0.0) {
                            testutil::CHCKSI(
                                &save.QNAME,
                                save.CDSIGN[save.CI],
                                b"=",
                                1,
                                0,
                                OK,
                                ctx,
                            )?;
                        } else if (save.XVEL[save.CI] < 0.0) {
                            testutil::CHCKSI(
                                &save.QNAME,
                                save.CDSIGN[save.CI],
                                b"=",
                                -1,
                                0,
                                OK,
                                ctx,
                            )?;
                        } else {
                            testutil::CHCKSI(
                                &save.QNAME,
                                save.CDSIGN[save.CI],
                                b"=",
                                0,
                                0,
                                OK,
                                ctx,
                            )?;
                        }

                        save.CI += m3__;
                    }
                }
                //
                // End of coordinate system loop.
                //
            }
            //
            // End of coordinate loop.
            //
        }
        //
        // End of derivative direction loop.
        //
    }
    //
    // End of Z-axis position loop.
    //

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    Rectangular coordinate system cases
    //*
    //*********************************************************************

    //
    // Since outputs for the rectangular coordinate system require
    // special handling by ZZGFCPRX, we test the routine's processing
    // for this case separately.
    //
    // We try cases where each velocity component is negative, 0, or
    // positive.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.STATE.as_slice_mut());

    for I in 1..=3 {
        save.STATE[4] = ((I - 2) as f64);

        for J in 1..=3 {
            save.STATE[5] = ((J - 2) as f64);

            for K in 1..=3 {
                save.STATE[6] = ((K - 2) as f64);
                //
                //---- Case -------------------------------------------------------------
                //
                fstr::assign(&mut save.TITLE, b"Velocity (# # #)");

                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.STATE[4],
                    3,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.STATE[5],
                    3,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.STATE[6],
                    3,
                    &mut save.TITLE,
                    ctx,
                );

                testutil::TCASE(&save.TITLE, ctx)?;

                save.RE = 1.0;
                save.F = 0.0;
                save.SENSE = 1;

                spicelib::ZZGFCPRX(
                    save.STATE.as_slice(),
                    RECSYS,
                    save.RE,
                    save.F,
                    save.SENSE,
                    save.CDSIGN.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    b"X vel sign",
                    save.CDSIGN[1],
                    b"=",
                    (save.STATE[4] as i32),
                    0,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //*********************************************************************
    //*
    //*    Non-rectangular coordinate system cases
    //*
    //*********************************************************************

    //
    // We're going to loop over a variety of state vectors
    // and all coordinate system/coordinate combinations.
    // For each test, the derivative signs we find must match
    // those of the corresponding coordinate derivative.
    //

    //
    // DELTLA and DELTLO are, respectively the latitude and longitude
    // increments we use to create different direction vectors.
    //
    save.DELTLA = (spicelib::PI(ctx) / (NLATSM - 1) as f64);
    save.DELTLO = (((2 as f64) * spicelib::PI(ctx)) / NLONSM as f64);

    for NLON in 1..=NLONSM {
        for NLAT in 1..=NLATSM {
            //
            // Create the latitudinal coordinates of a position vector.
            //
            save.LON = (((NLON - 1) as f64) * save.DELTLO);

            save.LAT = ((spicelib::PI(ctx) / 2 as f64) - (((NLAT - 1) as f64) * save.DELTLA));

            //
            // Don't pick a singular latitude value.
            //
            save.LAT = intrinsics::DMIN1(&[save.LAT, ((spicelib::PI(ctx) / 2 as f64) - EPSLAT)]);
            save.LAT = intrinsics::DMAX1(&[save.LAT, (EPSLAT - (spicelib::PI(ctx) / 2 as f64))]);

            //
            // Fill in the position portion of the state vector.
            //
            spicelib::LATREC(1.0, save.LON, save.LAT, save.STATE.as_slice_mut());

            //
            // Loop over the velocity directions.
            //
            for NDLON in 1..=NLONSM {
                for NDLAT in 1..=NLATSM {
                    //
                    // Create a velocity direction. Tweak the direction
                    // so it doesn't differ from the position direction
                    // only by round-off; this prevents situations where
                    // we get the wrong derivative sign due to noise.
                    //
                    save.DLON = ((((NDLON - 1) as f64) * save.DELTLO) + TWEAK);
                    save.DLAT = (((spicelib::PI(ctx) / 2 as f64)
                        - (((NDLAT - 1) as f64) * save.DELTLA))
                        + TWEAK);

                    //
                    // Fill in the position portion of the state vector.
                    //
                    spicelib::LATREC(1.0, save.DLON, save.DLAT, save.STATE.subarray_mut(4));

                    for SI in 1..=NSYS {
                        //
                        //---- Case -------------------------------------------------------------
                        //
                        fstr::assign(
                            &mut save.TITLE,
                            b"#; lat #(deg); lon #(deg); dlat #(deg); dlon #(deg)",
                        );

                        spicelib::REPMC(
                            &save.TITLE.to_vec(),
                            b"#",
                            &save.SYSNMS[SI],
                            &mut save.TITLE,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#",
                            (spicelib::DPR(ctx) * save.LAT),
                            3,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#",
                            (spicelib::DPR(ctx) * save.LON),
                            3,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#",
                            (spicelib::DPR(ctx) * save.DLAT),
                            3,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#",
                            (spicelib::DPR(ctx) * save.DLON),
                            3,
                            &mut save.TITLE,
                            ctx,
                        );

                        testutil::TCASE(&save.TITLE, ctx)?;

                        //
                        // Set shape and spin parameters.
                        //
                        save.RE = 100000.0;
                        save.F = 0.5;
                        save.SENSE = -1;

                        //
                        // Look up the Jacobian matrix for the current system
                        // and state. Find the expected coordinate rates.
                        //
                        if fstr::eq(save.SYSNMS.get(SI), RECSYS) {
                            spicelib::IDENT(save.JACOBI.as_slice_mut());
                        } else if fstr::eq(save.SYSNMS.get(SI), LATSYS) {
                            spicelib::DLATDR(
                                save.STATE[1],
                                save.STATE[2],
                                save.STATE[3],
                                save.JACOBI.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        } else if fstr::eq(save.SYSNMS.get(SI), RADSYS) {
                            spicelib::DLATDR(
                                save.STATE[1],
                                save.STATE[2],
                                save.STATE[3],
                                save.JACOBI.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        } else if fstr::eq(save.SYSNMS.get(SI), SPHSYS) {
                            spicelib::DSPHDR(
                                save.STATE[1],
                                save.STATE[2],
                                save.STATE[3],
                                save.JACOBI.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        } else if fstr::eq(save.SYSNMS.get(SI), CYLSYS) {
                            spicelib::DCYLDR(
                                save.STATE[1],
                                save.STATE[2],
                                save.STATE[3],
                                save.JACOBI.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        } else if fstr::eq(save.SYSNMS.get(SI), GEOSYS) {
                            spicelib::DGEODR(
                                save.STATE[1],
                                save.STATE[2],
                                save.STATE[3],
                                save.RE,
                                save.F,
                                save.JACOBI.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        } else if fstr::eq(save.SYSNMS.get(SI), PGRSYS) {
                            spicelib::DPGRDR(
                                b"Mars",
                                save.STATE[1],
                                save.STATE[2],
                                save.STATE[3],
                                save.RE,
                                save.F,
                                save.JACOBI.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        } else {
                            spicelib::SETMSG(b"Bad coordinate system #", ctx);
                            spicelib::ERRCH(b"#", &save.SYSNMS[SI], ctx);
                            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }

                        spicelib::MXV(
                            save.JACOBI.as_slice(),
                            save.STATE.subarray(4),
                            save.XVEL.as_slice_mut(),
                        );

                        spicelib::ZZGFCPRX(
                            save.STATE.as_slice(),
                            &save.SYSNMS[SI],
                            save.RE,
                            save.F,
                            save.SENSE,
                            save.CDSIGN.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 3;
                            let m3__: i32 = 1;
                            save.CI = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                fstr::assign(&mut save.QNAME, b"sign of d(");
                                spicelib::SUFFIX(&save.CRDNMS[[save.CI, SI]], 0, &mut save.QNAME);
                                spicelib::SUFFIX(b")/dt", 0, &mut save.QNAME);

                                if (save.XVEL[save.CI] > 0.0) {
                                    testutil::CHCKSI(
                                        &save.QNAME,
                                        save.CDSIGN[save.CI],
                                        b"=",
                                        1,
                                        0,
                                        OK,
                                        ctx,
                                    )?;
                                } else if (save.XVEL[save.CI] < 0.0) {
                                    testutil::CHCKSI(
                                        &save.QNAME,
                                        save.CDSIGN[save.CI],
                                        b"=",
                                        -1,
                                        0,
                                        OK,
                                        ctx,
                                    )?;
                                } else {
                                    testutil::CHCKSI(
                                        &save.QNAME,
                                        save.CDSIGN[save.CI],
                                        b"=",
                                        0,
                                        0,
                                        OK,
                                        ctx,
                                    )?;
                                }

                                save.CI += m3__;
                            }
                        }
                        //
                        // End of coordinate loop.
                        //
                    }
                    //
                    // End of system loop.
                    //
                }
                //
                // End of latitude rate loop.
                //
            }
            //
            // End of longitude rate loop.
            //
        }
        //
        // End of latitude loop.
        //
    }
    //
    // End of longitude loop.
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
