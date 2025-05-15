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
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK: &[u8] = b"zzgfssob.tpc";
const SPK: &[u8] = b"zzgfssob.bsp";
const MEDABS: f64 = 0.00005;
const MEDREL: f64 = 0.00000001;
const TIGHT: f64 = 0.0000000001;
const TDELTA: f64 = 10.0;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const NMETH: i32 = 2;
const NSAMP: i32 = 10;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    FIXREF: Vec<u8>,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    TIMSTR: Vec<u8>,
    TITLE: Vec<u8>,
    METH: ActualCharArray,
    METHOD: Vec<u8>,
    DELTA: f64,
    ET: f64,
    ET0: f64,
    RADII: StackArray<f64, 3>,
    SPNT: StackArray2D<f64, 6>,
    SRFVEC: StackArray<f64, 3>,
    STATE: StackArray<f64, 6>,
    T: f64,
    TOL: f64,
    TRGEPC: f64,
    XPNT: StackArray<f64, 3>,
    XVEL: StackArray<f64, 3>,
    HANDLE: i32,
    N: i32,
    TRGID: i32,
    OBSID: i32,
    ATTBLK: StackArray<bool, 6>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut METH = ActualCharArray::new(LNSIZE, 1..=NMETH);
        let mut METHOD = vec![b' '; LNSIZE as usize];
        let mut DELTA: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut SPNT = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut T: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut XPNT = StackArray::<f64, 3>::new(1..=3);
        let mut XVEL = StackArray::<f64, 3>::new(1..=3);
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut TRGID: i32 = 0;
        let mut OBSID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"lt"),
                Val::C(b" lt+s"),
                Val::C(b" cn"),
                Val::C(b" cn + s"),
                Val::C(b"XLT"),
                Val::C(b"XLT + S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Near point: ellipsoid"),
                Val::C(b"Intercept:  ellipsoid"),
            ]
            .into_iter();
            METH.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORR,
            FIXREF,
            OBSRVR,
            TARGET,
            TIMSTR,
            TITLE,
            METH,
            METHOD,
            DELTA,
            ET,
            ET0,
            RADII,
            SPNT,
            SRFVEC,
            STATE,
            T,
            TOL,
            TRGEPC,
            XPNT,
            XVEL,
            HANDLE,
            N,
            TRGID,
            OBSID,
            ATTBLK,
            FOUND,
        }
    }
}

//$Procedure      F_ZZGFSSOB ( Test sub-observer state computation )
pub fn F_ZZGFSSOB(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Local parameters
    //

    //
    // Time delta used for discrete derivatives. The 10-second
    // value used here was found by trial and error: it yields
    // better approximations than does the "standard" 1-second
    // value.
    //

    //
    // Local variables
    //

    //
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFSSOB", ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load a PCK.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Actual DE-based ephemerides yield better comparisons,
    // since these ephemerides have less noise than do those
    // produced by TSTSPK.
    //
    //  CALL FURNSH ( 'de421.bsp' )
    //  CALL FURNSH ( 'jup230.bsp' )
    //

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None.");
    fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSOB(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        save.OBSID,
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad computation method", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSOB(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        save.OBSID,
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target orientation data available", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"ITRF93");

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSOB(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        save.OBSID,
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Reference frame not centered on target", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
    fstr::assign(&mut save.TARGET, b"Mars");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSOB(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        save.OBSID,
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target ephemeris data available", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
    fstr::assign(&mut save.TARGET, b"GASPRA");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_GASPRA");

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSOB(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        save.OBSID,
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No observer ephemeris data available", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSOB(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        save.OBSID,
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // We're going to loop over all aberration corrections and
    // computation methods.
    //

    for NC in 1..=NCORR {
        for NM in 1..=NMETH {
            // Loop over the set of sample times.
            //
            fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

            spicelib::STR2ET(&save.TIMSTR, &mut save.ET0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DELTA = ((100 as f64) * spicelib::SPD());

            for NS in 1..=NSAMP {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut save.ABCORR, save.CORR.get(NC));
                fstr::assign(&mut save.METHOD, save.METH.get(NM));
                fstr::assign(&mut save.TARGET, b"EARTH");
                fstr::assign(&mut save.OBSRVR, b"MOON");
                fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

                fstr::assign(&mut save.TITLE, b"sub-# point on #. #; #; #.");

                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

                save.ET = (save.ET0 + (((NS - 1) as f64) * save.DELTA));
                spicelib::ETCAL(save.ET, &mut save.TIMSTR, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TIMSTR, &mut save.TITLE);

                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::BODVRD(
                    &save.TARGET,
                    b"RADII",
                    3,
                    &mut save.N,
                    save.RADII.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
                spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

                spicelib::ZZGFSSOB(
                    &save.METHOD,
                    save.TRGID,
                    save.ET,
                    &save.FIXREF,
                    &save.ABCORR,
                    save.OBSID,
                    save.RADII.as_slice(),
                    save.STATE.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Estimate the sub-point velocity numerically.
                //
                for I in 1..=2 {
                    save.T = (save.ET + ((((2 * I) - 3) as f64) * TDELTA));

                    spicelib::SUBPNT(
                        &save.METHOD,
                        &save.TARGET,
                        save.T,
                        &save.FIXREF,
                        &save.ABCORR,
                        &save.OBSRVR,
                        save.SPNT.subarray_mut([1, I]),
                        &mut save.TRGEPC,
                        save.SRFVEC.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::QDERIV(
                    3,
                    save.SPNT.subarray([1, 1]),
                    save.SPNT.subarray([1, 2]),
                    TDELTA,
                    save.XVEL.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the velocity from ZZGFSSOB. We expect an error of
                // less than 1 cm/s.
                //
                testutil::CHCKAD(
                    b"Sub-point vel",
                    save.STATE.subarray(4),
                    b"~~",
                    save.XVEL.as_slice(),
                    3,
                    MEDABS,
                    OK,
                    ctx,
                )?;

                spicelib::SUBPNT(
                    &save.METHOD,
                    &save.TARGET,
                    save.ET,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.XPNT.as_slice_mut(),
                    &mut save.TRGEPC,
                    save.SRFVEC.as_slice_mut(),
                    ctx,
                )?;

                //
                // The tolerance we use depends on the aberration correction.
                //
                spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

                if save.ATTBLK[CNVIDX] {
                    save.TOL = TIGHT;
                } else if save.ATTBLK[LTIDX] {
                    save.TOL = MEDREL;
                } else {
                    save.TOL = TIGHT;
                }

                testutil::CHCKAD(
                    b"Sub-point",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XPNT.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut save.ABCORR, save.CORR.get(NC));
                fstr::assign(&mut save.METHOD, save.METH.get(NM));
                fstr::assign(&mut save.TARGET, b"JUPITER");
                fstr::assign(&mut save.OBSRVR, b"IO");
                fstr::assign(&mut save.FIXREF, b"IAU_JUPITER");

                fstr::assign(&mut save.TITLE, b"sub-# point on #. #; #; #.");

                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

                save.ET = (save.ET0 + (((NS - 1) as f64) * save.DELTA));
                spicelib::ETCAL(save.ET, &mut save.TIMSTR, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TIMSTR, &mut save.TITLE);

                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::BODVRD(
                    &save.TARGET,
                    b"RADII",
                    3,
                    &mut save.N,
                    save.RADII.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
                spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

                spicelib::ZZGFSSOB(
                    &save.METHOD,
                    save.TRGID,
                    save.ET,
                    &save.FIXREF,
                    &save.ABCORR,
                    save.OBSID,
                    save.RADII.as_slice(),
                    save.STATE.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Estimate the sub-point velocity numerically.
                //
                for I in 1..=2 {
                    save.T = (save.ET + ((((2 * I) - 3) as f64) * TDELTA));

                    spicelib::SUBPNT(
                        &save.METHOD,
                        &save.TARGET,
                        save.T,
                        &save.FIXREF,
                        &save.ABCORR,
                        &save.OBSRVR,
                        save.SPNT.subarray_mut([1, I]),
                        &mut save.TRGEPC,
                        save.SRFVEC.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::QDERIV(
                    3,
                    save.SPNT.subarray([1, 1]),
                    save.SPNT.subarray([1, 2]),
                    TDELTA,
                    save.XVEL.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the velocity from ZZGFSSOB. We expect an error of
                // less than 1 cm/s.
                //
                testutil::CHCKAD(
                    b"Sub-point vel",
                    save.STATE.subarray(4),
                    b"~~",
                    save.XVEL.as_slice(),
                    3,
                    MEDABS,
                    OK,
                    ctx,
                )?;

                //
                // Make sure the position component of STATE matches the
                // result from SUBPNT.
                //
                spicelib::SUBPNT(
                    &save.METHOD,
                    &save.TARGET,
                    save.ET,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.XPNT.as_slice_mut(),
                    &mut save.TRGEPC,
                    save.SRFVEC.as_slice_mut(),
                    ctx,
                )?;

                //
                // The tolerance we use depends on the aberration correction.
                //
                spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

                if save.ATTBLK[CNVIDX] {
                    save.TOL = TIGHT;
                } else if save.ATTBLK[LTIDX] {
                    save.TOL = MEDREL;
                } else {
                    save.TOL = TIGHT;
                }

                testutil::CHCKAD(
                    b"Sub-point",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XPNT.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
            //
            // End of cases for the current epoch.
            //
        }
        //
        // End of cases for the current method.
        //
    }
    //
    // End of cases for the current aberration correction.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
