//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const KVNMLN: i32 = 32;
const KVLEN: i32 = 80;
const FRNMLN: i32 = 32;
const BDNMLN: i32 = 36;
const MAXCOF: i32 = 20;
const MXNFAC: i32 = 10;
const LBSEP: f64 = 0.001;
const QEXP: i32 = -27;
const KWBFRM: &[u8] = b"RELATIVE";
const KWSTYL: &[u8] = b"DEF_STYLE";
const KVPARM: &[u8] = b"PARAMETERIZED";
const KWFREZ: &[u8] = b"FREEZE_EPOCH";
const KWRSTA: &[u8] = b"ROTATION_STATE";
const KVROTG: &[u8] = b"ROTATING";
const KVINRT: &[u8] = b"INERTIAL";
const KWFFAM: &[u8] = b"FAMILY";
const KVMEQT: &[u8] = b"MEAN_EQUATOR_AND_EQUINOX_OF_DATE";
const KVMECL: &[u8] = b"MEAN_ECLIPTIC_AND_EQUINOX_OF_DATE";
const KVTEQT: &[u8] = b"TRUE_EQUATOR_AND_EQUINOX_OF_DATE";
const KV2VEC: &[u8] = b"TWO-VECTOR";
const KVEULR: &[u8] = b"EULER";
const KVPROD: &[u8] = b"PRODUCT";
const KWPRCM: &[u8] = b"PREC_MODEL";
const KWNUTM: &[u8] = b"NUT_MODEL";
const KWOBQM: &[u8] = b"OBLIQ_MODEL";
const KVM001: &[u8] = b"EARTH_IAU_1976";
const KVM002: &[u8] = b"EARTH_IAU_1980";
const KVM003: &[u8] = b"EARTH_IAU_1980";
const KWVAXI: &[u8] = b"AXIS";
const KVX: &[u8] = b"X";
const KVY: &[u8] = b"Y";
const KVZ: &[u8] = b"Z";
const KWPRI: &[u8] = b"PRI_";
const KWSEC: &[u8] = b"SEC_";
const KWVCDF: &[u8] = b"VECTOR_DEF";
const KVPOSV: &[u8] = b"OBSERVER_TARGET_POSITION";
const KVVELV: &[u8] = b"OBSERVER_TARGET_VELOCITY";
const KVNEAR: &[u8] = b"TARGET_NEAR_POINT";
const KVCONS: &[u8] = b"CONSTANT";
const KWVOBS: &[u8] = b"OBSERVER";
const KWVTRG: &[u8] = b"TARGET";
const KWVFRM: &[u8] = b"FRAME";
const KWVABC: &[u8] = b"ABCORR";
const KWVSPC: &[u8] = b"SPEC";
const KVRECC: &[u8] = b"RECTANGULAR";
const KVLATC: &[u8] = b"LATITUDINAL";
const KVRADC: &[u8] = b"RA/DEC";
const KWVECT: &[u8] = b"VECTOR";
const KWLAT: &[u8] = b"LATITUDE";
const KWLON: &[u8] = b"LONGITUDE";
const KWRA: &[u8] = b"RA";
const KWDEC: &[u8] = b"DEC";
const KWATOL: &[u8] = b"ANGLE_SEP_TOL";
const KWEPOC: &[u8] = b"EPOCH";
const KWEUAX: &[u8] = b"AXES";
const KWEAC1: &[u8] = b"ANGLE_1_COEFFS";
const KWEAC2: &[u8] = b"ANGLE_2_COEFFS";
const KWEAC3: &[u8] = b"ANGLE_3_COEFFS";
const KWFFRM: &[u8] = b"FROM_FRAMES";
const KWTFRM: &[u8] = b"TO_FRAMES";
const KWUNIT: &[u8] = b"UNITS";
const KVRADN: &[u8] = b"RADIANS";
const KVDEGR: &[u8] = b"DEGREES";
const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const PCK: &[u8] = b"test_dyn.tpc";
const SPK: &[u8] = b"test_dyn.bsp";
const LNSIZE: i32 = 80;
const MAXDEF: i32 = 50;
const TIMLEN: i32 = 50;

//$Procedure F_DYN07 ( Dynamic Frame Test Family 07 )
pub fn F_DYN07(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DEFTX2 = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut ERRMSG = [b' '; LMSGLN as usize];
    let mut GSETXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut GSMTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut KEYWRD = [b' '; LNSIZE as usize];
    let mut MARTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut MCQTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut MQQTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut TQQTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut DX: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut J: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Local Parameters
    //

    //
    // Parameters controlling frame definitions:
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DYN07", ctx)?;

    // **************************************************************
    // **************************************************************
    // **************************************************************
    //     TWO-VECTOR CASES
    // **************************************************************
    // **************************************************************
    // **************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create and load kernels.", ctx)?;

    //
    // Create and load kernels.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_PCK08(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the GSE frame.
    //
    fstr::assign(
        GSETXT.get_mut(1),
        b"FRAME_GSE                        =  2399001",
    );
    fstr::assign(
        GSETXT.get_mut(2),
        b"FRAME_2399001_NAME               = \'GSE\'",
    );
    fstr::assign(GSETXT.get_mut(3), b"FRAME_2399001_CLASS              =  5");
    fstr::assign(
        GSETXT.get_mut(4),
        b"FRAME_2399001_CLASS_ID           =  2399001",
    );
    fstr::assign(
        GSETXT.get_mut(5),
        b"FRAME_2399001_CENTER             =  399",
    );
    fstr::assign(
        GSETXT.get_mut(6),
        b"FRAME_2399001_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        GSETXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSTYL), b"       = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2399001_", KWFFAM),
                    b"             = \'",
                ),
                KV2VEC,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVAXI),
                    b"       = \'",
                ),
                KVX,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVCDF),
                    b"       = \'",
                ),
                KVPOSV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVOBS),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVTRG),
            b"         = \'SUN\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVAXI),
                    b"       = \'-",
                ),
                KVY,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVCDF),
                    b"       =  \'",
                ),
                KVVELV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(16),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVOBS),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVTRG),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVFRM),
            b"          = \'J2000\'",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(b"FRAME_2399001_", KWATOL),
            b"       =  1.E-04",
        ),
    );
    fstr::assign(
        GSETXT.get_mut(21),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWRSTA), b"       =  \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    DX = 21;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check handling of missing keywords in GSE (two-vector) frame definition.",
        ctx,
    )?;
    //
    // First set of tests:  delete required variables from the GSE
    // definition.  Make sure we get the expected error messages
    // when we refer to the GSE frame in an SXFORM or PXFORM call.
    // We currently restrict the testing to the keywords applicable
    // to dynamic frames.  These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the GSE definition.
        //
        spicelib::MOVEC(GSETXT.as_arg(), DX, DEFTX2.as_arg_mut());
        //
        // Lose the Ith element of the definition.
        //
        J = intrinsics::INDEX(&DEFTX2[I], b"=");
        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        //
        // Load the modified GSE frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Delete the Ith element of the frame definition from
        // the kernel pool.
        //
        spicelib::DVPOOL(&KEYWRD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSE SXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"GSE", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;

        if ((I < DX) && (I != 20)) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION STATE keyword is optional.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSE PXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"GSE", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;

        if ((I < DX) && (I != 20)) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION STATE keyword is optional.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Second set of tests:  assign bogus values to variables from the
    // GSE definition.  Make sure we get the expected error messages
    // when we refer to the GSE frame in an SXFORM or PXFORM call. We
    // currently restrict the testing to the keywords applicable to
    // dynamic frames. These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the GSE definition.
        //
        spicelib::MOVEC(GSETXT.as_arg(), DX, DEFTX2.as_arg_mut());

        J = intrinsics::INDEX(&DEFTX2[I], b"=");

        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSE SXFORM case: Changing RHS value for keyword ", &KEYWRD),
            ctx,
        )?;

        fstr::assign(fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..), b"\'ABC\'");

        //
        // Load the modified GSE frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set the expected error message resulting from an SXFORM
        // call relying on this frame definition.
        //
        // Start with the most common error message.
        //
        fstr::assign(&mut ERRMSG, b"SPICE(NOTSUPPORTED)");

        //
        // Handle body name errors.
        //
        if ((intrinsics::INDEX(&KEYWRD, KWVTRG) > 0) || (intrinsics::INDEX(&KEYWRD, KWVOBS) > 0)) {
            fstr::assign(&mut ERRMSG, b"SPICE(NOTRANSLATION)");
        }

        //
        // Handle aberration correction errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWVABC) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(INVALIDOPTION)");
        }

        //
        // Handle axis name errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWVAXI) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(INVALIDAXIS)");
        }

        //
        // Handle velocity frame name errors.  (Careful, the
        // substring 'FRAME' is at the start of each keyword.)
        //
        if (intrinsics::INDEX(&KEYWRD, &fstr::concat(b"_", KWVFRM)) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(NOTRANSLATION)");
        }

        //
        // Handle minimum anglular separation errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWATOL) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"GSE", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSE PXFORM case: changing value for keyword ", &KEYWRD),
            ctx,
        )?;

        //
        // Try a PXFORM call.
        //
        //
        // Handle aberration correction errors.
        //
        if ((intrinsics::INDEX(&KEYWRD, KWVABC) > 0) && (intrinsics::INDEX(&KEYWRD, b"PRI") > 0)) {
            //
            // For this PXFORM case, the error is caught in ZZSPKPA0,
            // which uses the following short error message:
            //
            fstr::assign(&mut ERRMSG, b"SPICE(SPKINVALIDOPTION)");
        }

        spicelib::PXFORM(b"GSE", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;
    }

    // **************************************************************
    // **************************************************************
    // **************************************************************
    //     TWO-VECTOR CASE:  GSM
    // **************************************************************
    // **************************************************************
    // **************************************************************

    //
    // Define the GSM frame.
    //
    fstr::assign(
        GSMTXT.get_mut(1),
        b"FRAME_GSM                        =  2399002",
    );
    fstr::assign(
        GSMTXT.get_mut(2),
        b"FRAME_2399002_NAME               = \'GSM\'",
    );
    fstr::assign(GSMTXT.get_mut(3), b"FRAME_2399002_CLASS              =  5");
    fstr::assign(
        GSMTXT.get_mut(4),
        b"FRAME_2399002_CLASS_ID           =  2399002",
    );
    fstr::assign(
        GSMTXT.get_mut(5),
        b"FRAME_2399002_CENTER             =  399",
    );
    fstr::assign(
        GSMTXT.get_mut(6),
        b"FRAME_2399002_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        GSMTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSTYL), b"       = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2399002_", KWFFAM),
                    b"             = \'",
                ),
                KV2VEC,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVAXI),
                    b"       = \'",
                ),
                KVX,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVCDF),
                    b"       =  \'",
                ),
                KVPOSV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVOBS),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVTRG),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVAXI),
                    b"       = \'",
                ),
                KVZ,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVCDF),
                    b"       = \'",
                ),
                KVCONS,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(16),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVSPC),
                    b"         = \'",
                ),
                KVLATC,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWUNIT),
                    b"         = \'",
                ),
                KVDEGR,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWLON),
            b"          =   288.43",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWLAT),
            b"          =    79.54",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVFRM),
            b"          = \'J2000\'",
        ),
    );
    fstr::assign(
        GSMTXT.get_mut(21),
        &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWATOL), b"       =  1.E-4"),
    );

    DX = 21;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check handling of missing keywords in GSM (two-vector) frame definition.",
        ctx,
    )?;
    //
    // First set of tests:  delete required variables from the GSM
    // definition.  Make sure we get the expected error messages
    // when we refer to the GSM frame in an SXFORM or PXFORM call.
    // We currently restrict the testing to the keywords applicable
    // to dynamic frames.  These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the GSM definition.
        //
        spicelib::MOVEC(GSMTXT.as_arg(), DX, DEFTX2.as_arg_mut());
        //
        // Lose the Ith element of the definition.
        //
        J = intrinsics::INDEX(&DEFTX2[I], b"=");
        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        //
        // Load the modified GSM frame definition.  We don't
        // want to clear the kernel pool here because we'd have
        // to keep re-loading the PCK.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Delete the Ith element of the frame definition from
        // the kernel pool.
        //
        spicelib::DVPOOL(&KEYWRD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSM SXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"GSM", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;

        if ((I < DX) && (I != 21)) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The MIN_ANG_SEP keyword is optional.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSM PXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"GSM", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;

        if ((I < DX) && (I != 21)) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The MIN_ANG_SEP keyword is optional.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Second set of tests:  assign bogus values to variables from the
    // GSM definition.  Make sure we get the expected error messages
    // when we refer to the GSM frame in an SXFORM or PXFORM call. We
    // currently restrict the testing to the keywords applicable to
    // dynamic frames. These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the GSM definition.
        //
        spicelib::MOVEC(GSMTXT.as_arg(), DX, DEFTX2.as_arg_mut());

        J = intrinsics::INDEX(&DEFTX2[I], b"=");

        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSM SXFORM case: Changing RHS value for keyword ", &KEYWRD),
            ctx,
        )?;

        fstr::assign(fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..), b"\'ABC\'");

        //
        // Load the modified GSM frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set the expected error message resulting from an SXFORM
        // call relying on this frame definition.
        //
        // Start with the most common error message.
        //
        fstr::assign(&mut ERRMSG, b"SPICE(NOTSUPPORTED)");

        //
        // Handle body name errors.
        //
        if ((intrinsics::INDEX(&KEYWRD, KWVTRG) > 0) || (intrinsics::INDEX(&KEYWRD, KWVOBS) > 0)) {
            fstr::assign(&mut ERRMSG, b"SPICE(NOTRANSLATION)");
        }

        //
        // Handle aberration correction errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWVABC) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(INVALIDOPTION)");
        }

        //
        // Handle axis name errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWVAXI) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(INVALIDAXIS)");
        }

        //
        // Handle constant frame name errors.
        //
        if (intrinsics::INDEX(&KEYWRD, &fstr::concat(b"_", KWVFRM)) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(NOTRANSLATION)");
        }

        //
        // Handle minimum anglular separation errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWATOL) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        //
        // Handle latitude and longitude errors.
        //
        if (intrinsics::INDEX(&KEYWRD, b"LATITUDE") > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        if (intrinsics::INDEX(&KEYWRD, b"LONGITUDE") > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        //
        // Handle unit errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWUNIT) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(UNITSNOTREC)");
        }

        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"GSM", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"GSM PXFORM case: changing value for keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        //
        // Handle aberration correction errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWVABC) > 0) {
            //
            // For PXFORM, the error is caught in ZZSPKPA0,
            // which uses the following short error message:
            //
            fstr::assign(&mut ERRMSG, b"SPICE(SPKINVALIDOPTION)");
        }

        spicelib::PXFORM(b"GSM", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;
    }

    // **************************************************************
    // **************************************************************
    // **************************************************************
    //     MEAN EQUATOR AND EQUINOX OF DATE CASES
    // **************************************************************
    // **************************************************************
    // **************************************************************

    //
    // First set of tests:  delete required variables from the MQQ
    // definition.  Make sure we get the expected error messages
    // when we refer to the MQQ frame in an SXFORM or PXFORM call.
    // We currently restrict the testing to the keywords applicable
    // to dynamic frames.  These are keywords 7 through DX.
    //
    fstr::assign(
        MQQTXT.get_mut(1),
        b"FRAME_MQQ                        =  2399003",
    );
    fstr::assign(
        MQQTXT.get_mut(2),
        b"FRAME_2399003_NAME               = \'MQQ\'",
    );
    fstr::assign(MQQTXT.get_mut(3), b"FRAME_2399003_CLASS              =  5");
    fstr::assign(
        MQQTXT.get_mut(4),
        b"FRAME_2399003_CLASS_ID           =  2399003",
    );
    fstr::assign(
        MQQTXT.get_mut(5),
        b"FRAME_2399003_CENTER             =  399",
    );
    fstr::assign(
        MQQTXT.get_mut(6),
        b"FRAME_2399003_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        MQQTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSTYL), b"   = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MQQTXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2399003_", KWFFAM),
                    b"             = \'",
                ),
                KVMEQT,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MQQTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRCM), b"   = \'"),
                KVM001,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MQQTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWRSTA), b"    = \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    DX = 10;

    for I in 7..=DX {
        //
        // Make a copy of the MQQ definition.
        //
        spicelib::MOVEC(MQQTXT.as_arg(), DX, DEFTX2.as_arg_mut());
        //
        // Lose the Ith element of the definition.
        //
        J = intrinsics::INDEX(&DEFTX2[I], b"=");
        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        //
        // Load the modified MQQ frame definition.  We don't
        // want to clear the kernel pool here because we'd have
        // to keep re-loading the PCK.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Delete the Ith element of the frame definition from
        // the kernel pool.
        //
        spicelib::DVPOOL(&KEYWRD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MQQ SXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"MQQ", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION_STATE keyword or the FREEZE_EPOCH keyword
            // must be present.
            //
            testutil::CHCKXC(true, b"SPICE(FRAMEDEFERROR)", OK, ctx)?;
        }

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MQQ PXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"MQQ", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION_STATE keyword or the FREEZE_EPOCH keyword
            // must be present.
            //
            testutil::CHCKXC(true, b"SPICE(FRAMEDEFERROR)", OK, ctx)?;
        }
    }

    //
    // Second set of tests:  assign bogus values to variables from the
    // MQQ definition.  Make sure we get the expected error messages
    // when we refer to the MQQ frame in an SXFORM or PXFORM call. We
    // currently restrict the testing to the keywords applicable to
    // dynamic frames. These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the MQQ definition.
        //
        spicelib::MOVEC(MQQTXT.as_arg(), DX, DEFTX2.as_arg_mut());

        J = intrinsics::INDEX(&DEFTX2[I], b"=");

        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MQQ SXFORM case: Changing RHS value for keyword ", &KEYWRD),
            ctx,
        )?;

        fstr::assign(fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..), b"\'ABC\'");

        //
        // Load the modified MQQ frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set the expected error message resulting from an SXFORM
        // call relying on this frame definition.
        //
        // Start with the most common error message.
        //
        fstr::assign(&mut ERRMSG, b"SPICE(NOTSUPPORTED)");

        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"MQQ", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MQQ PXFORM case: changing value for keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"MQQ", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;
    }

    // **************************************************************
    // **************************************************************
    // **************************************************************
    //     TRUE EQUATOR AND EQUINOX OF DATE CASES
    // **************************************************************
    // **************************************************************
    // **************************************************************
    //
    //     First set of tests:  delete required variables from the TQQ
    //     definition.  Make sure we get the expected error messages
    //     when we refer to the TQQ frame in an SXFORM or PXFORM call.
    //     We currently restrict the testing to the keywords applicable
    //     to dynamic frames.  These are keywords 7 through DX.
    //

    fstr::assign(
        TQQTXT.get_mut(1),
        b"FRAME_TQQ                        =  2399004",
    );
    fstr::assign(
        TQQTXT.get_mut(2),
        b"FRAME_2399004_NAME               = \'TQQ\'",
    );
    fstr::assign(TQQTXT.get_mut(3), b"FRAME_2399004_CLASS              =  5");
    fstr::assign(
        TQQTXT.get_mut(4),
        b"FRAME_2399004_CLASS_ID           =  2399004",
    );
    fstr::assign(
        TQQTXT.get_mut(5),
        b"FRAME_2399004_CENTER             =  399",
    );
    fstr::assign(
        TQQTXT.get_mut(6),
        b"FRAME_2399004_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        TQQTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399004_", KWSTYL), b"   = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        TQQTXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2399004_", KWFFAM),
                    b"             = \'",
                ),
                KVTEQT,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        TQQTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399004_", KWPRCM), b"   = \'"),
                KVM001,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        TQQTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399004_", KWNUTM), b"   = \'"),
                KVM002,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        TQQTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399004_", KWRSTA), b"    = \'"),
                KVINRT,
            ),
            b"\'",
        ),
    );

    DX = 11;

    for I in 7..=DX {
        //
        // Make a copy of the TQQ definition.
        //
        spicelib::MOVEC(TQQTXT.as_arg(), DX, DEFTX2.as_arg_mut());
        //
        // Lose the Ith element of the definition.
        //
        J = intrinsics::INDEX(&DEFTX2[I], b"=");
        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        //
        // Load the modified TQQ frame definition.  We don't
        // want to clear the kernel pool here because we'd have
        // to keep re-loading the PCK.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Delete the Ith element of the frame definition from
        // the kernel pool.
        //
        spicelib::DVPOOL(&KEYWRD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"TQQ SXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"TQQ", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION STATE keyword or the FREEZE EPOCH keyword
            // must be present.
            //
            testutil::CHCKXC(true, b"SPICE(FRAMEDEFERROR)", OK, ctx)?;
        }

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"TQQ PXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"TQQ", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION_STATE keyword or the FREEZE_EPOCH keyword
            // must be present.
            //
            testutil::CHCKXC(true, b"SPICE(FRAMEDEFERROR)", OK, ctx)?;
        }
    }

    //
    // Second set of tests:  assign bogus values to variables from the
    // TQQ definition.  Make sure we get the expected error messages
    // when we refer to the TQQ frame in an SXFORM or PXFORM call. We
    // currently restrict the testing to the keywords applicable to
    // dynamic frames. These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the TQQ definition.
        //
        spicelib::MOVEC(TQQTXT.as_arg(), DX, DEFTX2.as_arg_mut());

        J = intrinsics::INDEX(&DEFTX2[I], b"=");

        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"TQQ SXFORM case: Changing RHS value for keyword ", &KEYWRD),
            ctx,
        )?;

        fstr::assign(fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..), b"\'ABC\'");

        //
        // Load the modified TQQ frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set the expected error message resulting from an SXFORM
        // call relying on this frame definition.
        //
        // Start with the most common error message.
        //
        fstr::assign(&mut ERRMSG, b"SPICE(NOTSUPPORTED)");

        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"TQQ", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"TQQ PXFORM case: changing value for keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"TQQ", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;
    }

    // **************************************************************
    // **************************************************************
    // **************************************************************
    //     MEAN ECLIPTIC AND EQUINOX OF DATE CASES
    // **************************************************************
    // **************************************************************
    // **************************************************************
    //
    //     First set of tests:  delete required variables from the MCQ
    //     definition.  Make sure we get the expected error messages
    //     when we refer to the MCQ frame in an SXFORM or PXFORM call.
    //     We currently restrict the testing to the keywords applicable
    //     to dynamic frames.  These are keywords 7 through DX.
    //

    fstr::assign(
        MCQTXT.get_mut(1),
        b"FRAME_MCQ                        =  2399005",
    );
    fstr::assign(
        MCQTXT.get_mut(2),
        b"FRAME_2399005_NAME               = \'MCQ\'",
    );
    fstr::assign(MCQTXT.get_mut(3), b"FRAME_2399005_CLASS              =  5");
    fstr::assign(
        MCQTXT.get_mut(4),
        b"FRAME_2399005_CLASS_ID           =  2399005",
    );
    fstr::assign(
        MCQTXT.get_mut(5),
        b"FRAME_2399005_CENTER             =  399",
    );
    fstr::assign(
        MCQTXT.get_mut(6),
        b"FRAME_2399005_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        MCQTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399005_", KWSTYL), b"   = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MCQTXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2399005_", KWFFAM),
                    b"             = \'",
                ),
                KVMECL,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MCQTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399005_", KWPRCM), b"  = \'"),
                KVM001,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MCQTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399005_", KWOBQM), b"    = \'"),
                KVM002,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MCQTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(b"FRAME_2399005_", KWFREZ),
            b"       = @2005-JAN-1/00:00:00",
        ),
    );

    DX = 11;

    for I in 7..=DX {
        //
        // Make a copy of the MCQ definition.
        //
        spicelib::MOVEC(MCQTXT.as_arg(), DX, DEFTX2.as_arg_mut());
        //
        // Lose the Ith element of the definition.
        //
        J = intrinsics::INDEX(&DEFTX2[I], b"=");
        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        //
        // Load the modified MCQ frame definition.  We don't
        // want to clear the kernel pool here because we'd have
        // to keep re-loading the PCK.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Delete the Ith element of the frame definition from
        // the kernel pool.
        //
        spicelib::DVPOOL(&KEYWRD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MCQ SXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"MCQ", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION_STATE keyword or the FREEZE_EPOCH keyword
            // must be present.
            //
            testutil::CHCKXC(true, b"SPICE(FRAMEDEFERROR)", OK, ctx)?;
        }

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MCQ PXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"MCQ", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION STATE keyword or the FREEZE EPOCH keyword
            // must be present.
            //
            testutil::CHCKXC(true, b"SPICE(FRAMEDEFERROR)", OK, ctx)?;
        }
    }

    //
    // Second set of tests:  assign bogus values to variables from the
    // MCQ definition.  Make sure we get the expected error messages
    // when we refer to the MCQ frame in an SXFORM or PXFORM call. We
    // currently restrict the testing to the keywords applicable to
    // dynamic frames. These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the MCQ definition.
        //
        spicelib::MOVEC(MCQTXT.as_arg(), DX, DEFTX2.as_arg_mut());

        J = intrinsics::INDEX(&DEFTX2[I], b"=");

        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MCQ SXFORM case: Changing RHS value for keyword ", &KEYWRD),
            ctx,
        )?;

        fstr::assign(fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..), b"\'ABC\'");

        //
        // Load the modified MCQ frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set the expected error message resulting from an SXFORM
        // call relying on this frame definition.
        //
        // Start with the most common error message.
        //
        fstr::assign(&mut ERRMSG, b"SPICE(NOTSUPPORTED)");

        //
        // Handle freeze epoch errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWFREZ) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"MCQ", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"MCQ PXFORM case: changing value for keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"MCQ", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;
    }

    // **************************************************************
    // **************************************************************
    // **************************************************************
    //     EULER FRAME CASES
    // **************************************************************
    // **************************************************************
    // **************************************************************
    //
    //     Define the pseudo IAU_MARS frame.
    //
    fstr::assign(
        MARTXT.get_mut(1),
        b"FRAME_IAU_MARS2                  =  2499000",
    );
    fstr::assign(
        MARTXT.get_mut(2),
        b"FRAME_2499000_NAME               = \'IAU_MARS2\'",
    );
    fstr::assign(MARTXT.get_mut(3), b"FRAME_2499000_CLASS              =  5");
    fstr::assign(
        MARTXT.get_mut(4),
        b"FRAME_2499000_CLASS_ID           =  2499000",
    );
    fstr::assign(
        MARTXT.get_mut(5),
        b"FRAME_2499000_CENTER             =  499",
    );
    fstr::assign(
        MARTXT.get_mut(6),
        b"FRAME_2499000_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        MARTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSTYL), b"   = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MARTXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2499000_", KWFFAM),
                    b"             = \'",
                ),
                KVEULR,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        MARTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(b"FRAME_2499000_", KWEPOC),
                b"              =  ",
            ),
            b"@2000-JAN-1/12:00",
        ),
    );
    fstr::assign(
        MARTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(b"FRAME_2499000_", KWEUAX),
            b"               = ( 3  1  3 )",
        ),
    );
    fstr::assign(
        MARTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(b"FRAME_2499000_", KWEAC1),
                b"     = ( -47.68143 ",
            ),
            b"0.33621061170684714E-10 )",
        ),
    );
    fstr::assign(
        MARTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(b"FRAME_2499000_", KWEAC2),
                b"     = ( -37.1135 ",
            ),
            b"-0.19298045478743630E-10 )",
        ),
    );
    fstr::assign(
        MARTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(b"FRAME_2499000_", KWEAC3),
                b"     = (-176.630  ",
            ),
            b"-0.40612497946759260E-02 )",
        ),
    );
    fstr::assign(
        MARTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2499000_", KWUNIT),
                    b"              =  \'",
                ),
                KVDEGR,
            ),
            b"\'",
        ),
    );

    fstr::assign(
        MARTXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWRSTA), b"    = \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    DX = 15;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check handling of missing keywords in IAU_MARS2 (Euler) frame definition.",
        ctx,
    )?;
    //
    // First set of tests:  delete required variables from the GSE
    // definition.  Make sure we get the expected error messages
    // when we refer to the GSE frame in an SXFORM or PXFORM call.
    // We currently restrict the testing to the keywords applicable
    // to dynamic frames.  These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // Make a copy of the IAU_MARS2 definition.
        //
        spicelib::MOVEC(MARTXT.as_arg(), DX, DEFTX2.as_arg_mut());
        //
        // Lose the Ith element of the definition.
        //
        J = intrinsics::INDEX(&DEFTX2[I], b"=");
        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));
        //
        //
        // Load the modified IAU_MARS2 frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Delete the Ith element of the frame definition from
        // the kernel pool.
        //
        spicelib::DVPOOL(&KEYWRD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"IAU_MARS2 SXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"IAU_MARS2", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION_STATE keyword is optional.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(b"IAU_MARS2 PXFORM case: deleting keyword ", &KEYWRD),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"IAU_MARS2", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;

        if (I < DX) {
            //
            // We've deleted a required keyword.
            //
            testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;
        } else {
            //
            // The ROTATION_STATE keyword is optional.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Second set of tests:  assign bogus values to variables from the
    // IAU_MARS2 definition. Make sure we get the expected error
    // messages when we refer to the IAU_MARS2 frame in an SXFORM or
    // PXFORM call. We currently restrict the testing to the keywords
    // applicable to dynamic frames. These are keywords 7 through DX.
    //
    for I in 7..=DX {
        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(
                b"IAU_MARS2 SXFORM case: Changing RHS value for keyword ",
                &KEYWRD,
            ),
            ctx,
        )?;

        //
        // Make a copy of the IAU_MARS2 definition.
        //
        spicelib::MOVEC(MARTXT.as_arg(), DX, DEFTX2.as_arg_mut());

        J = intrinsics::INDEX(&DEFTX2[I], b"=");

        fstr::assign(&mut KEYWRD, fstr::substr(DEFTX2.get(I), 1..=(J - 1)));

        if spicelib::EQSTR(&KEYWRD, &fstr::concat(b"FRAME_2499000_", KWEUAX)) {
            fstr::assign(
                fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..),
                b"( 1, 0, 1 )",
            );
        } else if spicelib::EQSTR(&KEYWRD, &fstr::concat(b"FRAME_2499000_", KWEPOC)) {
            fstr::assign(
                fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..),
                b"@2004-ABC/12:00:00",
            );
        } else {
            fstr::assign(fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..), b"\'ABC\'");
        }

        //
        // Load the modified IAU_MARS2 frame definition.
        //
        spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;

        if spicelib::EQSTR(&KEYWRD, &fstr::concat(b"FRAME_2499000_", KWEPOC)) {
            //
            // LMPOOL catches malformed time tokens of the form @*.
            //
            fstr::assign(&mut ERRMSG, b"SPICE(BADTIMESPEC)");

            testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;

            //
            // Re-set the time token to a string not recognized
            // as a time token.
            //
            fstr::assign(fstr::substr_mut(DEFTX2.get_mut(I), (J + 1)..), b"\'ABC\'");

            spicelib::LMPOOL(DEFTX2.as_arg(), DX, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        } else {
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Set the expected error message resulting from an SXFORM
        // call relying on this frame definition.
        //
        // Start with the most common error message.
        //
        fstr::assign(&mut ERRMSG, b"SPICE(NOTSUPPORTED)");

        //
        // Handle epoch errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWEPOC) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        //
        // Handle units errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWUNIT) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(UNITSNOTREC)");
        }

        //
        // Handle axis sequence errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWEUAX) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADAXISNUMBERS)");
        }

        //
        // Handle angle 1 coefficient errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWEAC1) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        //
        // Handle angle 2 coefficient errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWEAC2) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }
        //
        // Handle angle 3 coefficient errors.
        //
        if (intrinsics::INDEX(&KEYWRD, KWEAC3) > 0) {
            fstr::assign(&mut ERRMSG, b"SPICE(BADVARIABLETYPE)");
        }

        //
        // Try an SXFORM call.
        //
        spicelib::SXFORM(b"IAU_MARS2", b"J2000", 1000000.0, XFORM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(
            &fstr::concat(
                b"IAU_MARS2 PXFORM case: changing value for keyword ",
                &KEYWRD,
            ),
            ctx,
        )?;
        //
        // Try a PXFORM call.
        //
        spicelib::PXFORM(b"IAU_MARS2", b"J2000", 1000000.0, R.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, &ERRMSG, OK, ctx)?;
    }

    //
    // Clean up the SPK file.
    //
    testutil::TCASE(b"File clean-up.", ctx)?;
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
