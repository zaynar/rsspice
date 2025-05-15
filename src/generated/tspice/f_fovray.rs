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
const PCK: &[u8] = b"fovray.tpc";
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const NIK: &[u8] = b"nat.ti";
const SPK: &[u8] = b"fovray.bsp";
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const KVARLN: i32 = 32;
const MAXWIN: i32 = 20000;
const LBCELL: i32 = -5;
const NINST: i32 = 4;
const LBLSIZ: i32 = 40;
const NCORS: i32 = 3;

struct SaveVars {
    ABCORR: Vec<u8>,
    ABCORS: ActualCharArray,
    INST: Vec<u8>,
    INSTRS: ActualCharArray,
    KVNAME: Vec<u8>,
    OBSRVR: Vec<u8>,
    RFRAME: Vec<u8>,
    TITLE: Vec<u8>,
    BADBND: ActualArray2D<f64>,
    BSITE: StackArray<f64, 3>,
    ET: f64,
    FOVBND: ActualArray2D<f64>,
    RAYDIR: StackArray<f64, 3>,
    TIMES: StackArray<f64, 2>,
    HAN2: i32,
    HANDLE: i32,
    INSTID: i32,
    N: i32,
    FOUND: bool,
    RESULT: StackArray<bool, 2>,
    VISIBL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut ABCORS = ActualCharArray::new(CORLEN, 1..=NCORS);
        let mut INST = vec![b' '; BDNMLN as usize];
        let mut INSTRS = ActualCharArray::new(BDNMLN, 1..=NINST);
        let mut KVNAME = vec![b' '; KVARLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut RFRAME = vec![b' '; FRNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut BADBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut BSITE = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut FOVBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut TIMES = StackArray::<f64, 2>::new(1..=2);
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut INSTID: i32 = 0;
        let mut N: i32 = 0;
        let mut FOUND: bool = false;
        let mut RESULT = StackArray::<bool, 2>::new(1..=2);
        let mut VISIBL: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"NONE"), Val::C(b"S"), Val::C(b"XS")].into_iter();
            ABCORS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALPHA_CIRCLE_NONE"),
                Val::C(b"ALPHA_ELLIPSE_NONE"),
                Val::C(b"ALPHA_RECTANGLE_NONE"),
                Val::C(b"ALPHA_DIAMOND_NONE"),
            ]
            .into_iter();
            INSTRS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            ABCORS,
            INST,
            INSTRS,
            KVNAME,
            OBSRVR,
            RFRAME,
            TITLE,
            BADBND,
            BSITE,
            ET,
            FOVBND,
            RAYDIR,
            TIMES,
            HAN2,
            HANDLE,
            INSTID,
            N,
            FOUND,
            RESULT,
            VISIBL,
        }
    }
}

//$Procedure F_FOVRAY ( FOVRAY family tests )
pub fn F_FOVRAY(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //
    //  DOUBLE PRECISION      SPD

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved Variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //

    testutil::TOPEN(b"F_FOVRAY", ctx)?;

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
    // Create and load Nat's solar system SPK, PCK/FK, and IK
    // files.
    //
    testutil::NATPCK(NPCK, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATSPK(NSPK, true, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATIK(NIK, NSPK, NPCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;
    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad observer name.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"BET");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad reference frame RFRAME", ctx)?;

    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"XCN+S");
    fstr::assign(&mut save.RFRAME, b"EME2000");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-existent reference frame.", ctx)?;

    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");
    fstr::assign(&mut save.RFRAME, b"EME2000");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction for ray target.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.RFRAME, b"BETAFIXED");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No ID code for instrument.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Instrument parameters missing from kernel pool.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");

    spicelib::BODS2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_SHAPE");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    //
    // Delete the instrument shape from the kernel pool.
    //
    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SHAPEMISSING)", OK, ctx)?;

    //
    // Restore the instrument shape value.
    //
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"ELLIPSE"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Degenerate FOV ellipse.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");

    spicelib::BODS2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Fetch the instrument boresight vector from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.BSITE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the instrument boundary vectors from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        (3 * MAXVRT),
        &mut save.N,
        save.FOVBND.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the first boundary vector equal to the boresight vector.
    //
    spicelib::MOVED(save.FOVBND.as_slice(), save.N, save.BADBND.as_slice_mut());
    spicelib::MOVED(save.BSITE.as_slice(), 3, save.BADBND.subarray_mut([1, 1]));

    spicelib::PDPOOL(&save.KVNAME, save.N, save.BADBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Restore the first boundary vector and set the second
    // equal to the boresight vector.
    //
    spicelib::MOVED(
        save.FOVBND.subarray([1, 1]),
        3,
        save.BADBND.subarray_mut([1, 1]),
    );
    spicelib::MOVED(save.BSITE.as_slice(), 3, save.BADBND.subarray_mut([1, 2]));

    spicelib::PDPOOL(&save.KVNAME, save.N, save.BADBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Restore the instrument FOV boundary vectors.
    //
    spicelib::PDPOOL(&save.KVNAME, save.N, save.FOVBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"FOV boundary vector has excessive angular separation from boresight.",
        ctx,
    )?;

    fstr::assign(&mut save.INST, b"ALPHA_RECTANGLE_NONE");

    spicelib::BODS2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Fetch the instrument boresight vector from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.BSITE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the instrument boundary vectors from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        (3 * MAXVRT),
        &mut save.N,
        save.FOVBND.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the third boundary vector equal to the *negative*
    // of the boresight vector.
    //
    spicelib::MOVED(save.FOVBND.as_slice(), save.N, save.BADBND.as_slice_mut());
    spicelib::VMINUS(save.BSITE.as_slice(), save.BADBND.subarray_mut([1, 3]));

    spicelib::PDPOOL(&save.KVNAME, save.N, save.BADBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FACENOTFOUND)", OK, ctx)?;

    //
    // Restore the instrument FOV boundary vectors.
    //
    spicelib::PDPOOL(&save.KVNAME, save.N, save.FOVBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No observer ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.RFRAME, b"IAU_SUN");
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    fstr::assign(&mut save.ABCORR, b"S");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No RFRAME orientation data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.RFRAME, b"ITRF93");

    spicelib::FOVRAY(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.ET,
        &mut save.VISIBL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // We'll start out with cases using an ellipsoidal target and
    // geometric states. We'll use the four instruments defined
    // in nat.ti:
    //
    //    ALPHA_CIRCLE_NONE
    //    ALPHA_ELLIPSE_NONE
    //    ALPHA_RECTANGLE_NONE
    //    ALPHA_DIAMOND_NONE
    //
    // These have the FOV shapes
    //
    //    ELLIPSE
    //    CIRCLE
    //    RECTANGLE
    //    POLYGON
    //
    // and track body Alpha so body Beta's FOV entry and
    // exit times match the start and stop times of Beta's
    // transit across Alpha.
    //
    save.RESULT[1] = true;
    save.RESULT[2] = false;

    //
    // The TIMES variable is indexed according to the expected RESULT,
    // so the first index in TIMES corresponds to a .TRUE. visibility
    // case, while the second corresponds to a .FALSE. visibility case.
    //
    save.TIMES[1] = 61.5;
    save.TIMES[2] = 540.5;

    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.RFRAME, b"BETA_VIEW_XY");

    //
    // Set the ray's direction vector: BETA_VIEW_XY +X.
    //
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Loop through the instruments.
    //
    for INS in 1..=NINST {
        fstr::assign(&mut save.INST, save.INSTRS.get(INS));
        //
        // Loop through the ABCORR values (5 of them)
        //
        for NC in 1..=NCORS {
            //
            // --- Case: ------------------------------------------------------
            //
            //
            fstr::assign(&mut save.ABCORR, save.ABCORS.get(NC));

            //
            // Set up the TCASE call.
            //
            fstr::assign(&mut save.TITLE, b"Inst. #; #; inst frame #");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.INST, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.RFRAME, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for TMP in 1..=2 {
                testutil::TCASE(&save.TITLE, ctx)?;
                //
                // Is it visible?
                //
                spicelib::FOVRAY(
                    &save.INST,
                    save.RAYDIR.as_slice(),
                    &save.RFRAME,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.TIMES[TMP],
                    &mut save.VISIBL,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"VISIBL", save.VISIBL, save.RESULT[TMP], OK, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(NSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
