//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
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
const LNSIZE: i32 = 80;
const MAXDEF: i32 = 50;
const MAXVAL: i32 = 5;

//$Procedure F_DYN08 ( Dynamic Frame Test Family 08 )
pub fn F_DYN08(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CHVALS = ActualCharArray::new(LNSIZE, 1..=MAXVAL);
    let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut EXPCHV = ActualCharArray::new(LNSIZE, 1..=MAXVAL);
    let mut FRAMNM = [b' '; FRNMLN as usize];
    let mut EXPDPV = StackArray::<f64, 5>::new(1..=MAXVAL);
    let mut DPVALS = StackArray::<f64, 5>::new(1..=MAXVAL);
    let mut EXPINV = StackArray::<i32, 5>::new(1..=MAXVAL);
    let mut FRCODE: i32 = 0;
    let mut I: i32 = 0;
    let mut INVALS = StackArray::<i32, 5>::new(1..=MAXVAL);
    let mut N: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions

    //
    // Local Parameters
    //

    //
    // Tolerance levels for various tests.
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
    testutil::TOPEN(b"F_DYN08", ctx)?;

    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    //
    //     ZZDYNVAD tests
    //
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAD, normal case:  fetch array using ID form of name.",
        ctx,
    )?;

    //
    // First load some variables into the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_DP_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_DP_ARRAY_1 = ( 1, 2, 3 )",
    );
    fstr::assign(DEFTXT.get_mut(2), b"FRAME_-1000000000_DP_ARRAY_2 = 4");

    spicelib::LMPOOL(DEFTXT.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, EXPDPV.as_slice_mut());

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (1)",
        DPVALS.as_slice(),
        b"=",
        EXPDPV.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_2",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (2)",
        DPVALS.as_slice(),
        b"=",
        &[4.0],
        1,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAD, normal case:  fetch array using character form of name.",
        ctx,
    )?;

    fstr::assign(
        DEFTXT.get_mut(3),
        b"FRAME_FRAME_DP_1_DP_ARRAY_1  = ( 5, 6, 7, 8 )",
    );
    fstr::assign(DEFTXT.get_mut(4), b"FRAME_FRAME_DP_1_DP_ARRAY_2  = 9");

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.subarray(3), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 4;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            EXPDPV[I] = (I + 4) as f64;
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (1)",
        DPVALS.as_slice(),
        b"=",
        EXPDPV.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_2",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (2)",
        DPVALS.as_slice(),
        b"=",
        &[9.0],
        1,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAD, normal case:  fetch array when ID and character form of name are both present.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, EXPDPV.as_slice_mut());

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (1)",
        DPVALS.as_slice(),
        b"=",
        EXPDPV.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_2",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (2)",
        DPVALS.as_slice(),
        b"=",
        &[4.0],
        1,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAD, exception case:  try to fetch array having excessive size.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_DP_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_DP_ARRAY_1 = ( 1, 2, 3 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        1,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAD, exception case:  try to fetch array having wrong data type.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_DP_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_DP_ARRAY_1 = ( \'1\', \'2\', \'3\' )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNVAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        3,
        &mut N,
        DPVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;

    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    //
    //     ZZDYNVAI tests
    //
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAI, normal case:  fetch array using ID form of name.",
        ctx,
    )?;

    //
    // First load some variables into the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( 1, 2, 3 )",
    );
    fstr::assign(DEFTXT.get_mut(2), b"FRAME_-1000000000_IN_ARRAY_2 = 4");

    spicelib::LMPOOL(DEFTXT.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_1",
        MAXVAL,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            EXPINV[I] = I;
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAI(
        b"INVALS (1)",
        INVALS.as_slice(),
        b"=",
        EXPINV.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_2",
        MAXVAL,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAI(b"INVALS (2)", INVALS.as_slice(), b"=", &[4], 1, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAI, normal case:  fetch array using character form of name.",
        ctx,
    )?;

    fstr::assign(
        DEFTXT.get_mut(3),
        b"FRAME_FRAME_IN_1_IN_ARRAY_1  = ( 5, 6, 7, 8 )",
    );
    fstr::assign(DEFTXT.get_mut(4), b"FRAME_FRAME_IN_1_IN_ARRAY_2  = 9");

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.subarray(3), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_1",
        MAXVAL,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 4;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            EXPINV[I] = (I + 4);
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAI(
        b"INVALS (1)",
        INVALS.as_slice(),
        b"=",
        EXPINV.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_2",
        MAXVAL,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAI(b"INVALS (2)", INVALS.as_slice(), b"=", &[9], 1, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAI, normal case:  fetch array when ID and character form of name are both present.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_1",
        MAXVAL,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            EXPINV[I] = I;
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAI(
        b"INVALS (1)",
        INVALS.as_slice(),
        b"=",
        EXPINV.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_2",
        MAXVAL,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAI(b"INVALS (2)", INVALS.as_slice(), b"=", &[4], 1, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAI, exception case:  try to fetch array that is not present.",
        ctx,
    )?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_3",
        MAXVAL,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAI, exception case:  try to fetch array having excessive size.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( 1, 2, 3 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_1",
        1,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAI, exception case:  try to fetch array having wrong data type.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( \'1\', \'2\', \'3\' )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_1",
        3,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDYNVAI, exception case:  try to fetch array having a value that causes integer overflow.", ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( 1, 1.D30 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_1",
        3,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAI, *non-error* exception case:  fetch array having a value that must be rounded.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( 1, 2.4 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the integer fetch routine for required arguments.
    //
    spicelib::ZZDYNVAI(
        &FRAMNM,
        FRCODE,
        b"IN_ARRAY_1",
        3,
        &mut N,
        INVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            EXPINV[I] = I;
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAI(
        b"INVALS (1)",
        INVALS.as_slice(),
        b"=",
        EXPINV.as_slice(),
        N,
        OK,
        ctx,
    )?;

    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    //
    //     ZZDYNVAC tests
    //
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAC, normal case:  fetch array using ID form of name.",
        ctx,
    )?;

    //
    // First load some variables into the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_CH_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_CH_ARRAY_1 = ( \'1\', \'2\', \'3\' )",
    );
    fstr::assign(DEFTXT.get_mut(2), b"FRAME_-1000000000_CH_ARRAY_2 = \'4\'");

    spicelib::LMPOOL(DEFTXT.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR(I, &mut EXPCHV[I], ctx);
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSC(b"CHVALS (1)", &CHVALS[I], b"=", &EXPCHV[I], OK, ctx)?;
            I += m3__;
        }
    }

    //
    // Repeat with a second variable.
    //
    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_2",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKSC(b"CHVALS (2)", &CHVALS[1], b"=", b"4", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAC, normal case:  fetch array using character form of name.",
        ctx,
    )?;

    fstr::assign(
        DEFTXT.get_mut(3),
        b"FRAME_FRAME_CH_1_CH_ARRAY_1  = ( \'5\', \'6\', \'7\', \'8\' )",
    );
    fstr::assign(DEFTXT.get_mut(4), b"FRAME_FRAME_CH_1_CH_ARRAY_2  = \'9\'");

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.subarray(3), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR((I + 4), &mut EXPCHV[I], ctx);
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSC(b"CHVALS (1)", &CHVALS[I], b"=", &EXPCHV[I], OK, ctx)?;
            I += m3__;
        }
    }

    //
    // Repeat with a second variable.
    //
    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_2",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKSC(b"CHVALS (2)", &CHVALS[1], b"=", b"9", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAC, normal case:  fetch array when ID and character form of name are both present.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR(I, &mut EXPCHV[I], ctx);
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSC(b"CHVALS (1)", &CHVALS[I], b"=", &EXPCHV[I], OK, ctx)?;
            I += m3__;
        }
    }

    //
    // Repeat with a second variable.
    //
    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_2",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKSC(b"CHVALS (2)", &CHVALS[1], b"=", b"4", OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAC, exception case:  try to fetch array that is not present.",
        ctx,
    )?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_3",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAC, exception case:  try to fetch array having excessive size.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_CH_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_CH_ARRAY_1 = ( \'1\', \'2\', \'3\' )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        1,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNVAC, exception case:  try to fetch array having wrong data type.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_CH_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_CH_ARRAY_1 = ( 1, 2, 3 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNVAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        3,
        &mut N,
        CHVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;

    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    //
    //     ZZDYNOAD tests
    //
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAD, normal case:  fetch array using ID form of name.",
        ctx,
    )?;

    //
    // First load some variables into the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_DP_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_DP_ARRAY_1 = ( 1, 2, 3 )",
    );
    fstr::assign(DEFTXT.get_mut(2), b"FRAME_-1000000000_DP_ARRAY_2 = 4");

    spicelib::LMPOOL(DEFTXT.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, EXPDPV.as_slice_mut());

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (1)",
        DPVALS.as_slice(),
        b"=",
        EXPDPV.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_2",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (2)",
        DPVALS.as_slice(),
        b"=",
        &[4.0],
        1,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAD, normal case:  fetch array using character form of name.",
        ctx,
    )?;

    fstr::assign(
        DEFTXT.get_mut(3),
        b"FRAME_FRAME_DP_1_DP_ARRAY_1  = ( 5, 6, 7, 8 )",
    );
    fstr::assign(DEFTXT.get_mut(4), b"FRAME_FRAME_DP_1_DP_ARRAY_2  = 9");

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.subarray(3), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 4;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            EXPDPV[I] = (I + 4) as f64;
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (1)",
        DPVALS.as_slice(),
        b"=",
        EXPDPV.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_2",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (2)",
        DPVALS.as_slice(),
        b"=",
        &[9.0],
        1,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAD, normal case:  fetch array when ID and character form of name are both present.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, EXPDPV.as_slice_mut());

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (1)",
        DPVALS.as_slice(),
        b"=",
        EXPDPV.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_2",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKAD(
        b"DPVALS (2)",
        DPVALS.as_slice(),
        b"=",
        &[4.0],
        1,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAD, exception case:  try to fetch array that is not present.",
        ctx,
    )?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_3",
        MAXVAL,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAD, exception case:  try to fetch array having excessive size.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_DP_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_DP_ARRAY_1 = ( 1, 2, 3 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        1,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAD, exception case:  try to fetch array having wrong data type.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_DP_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_DP_ARRAY_1 = ( \'1\', \'2\', \'3\' )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the d.p. fetch routine for required arguments.
    //
    spicelib::ZZDYNOAD(
        &FRAMNM,
        FRCODE,
        b"DP_ARRAY_1",
        3,
        &mut N,
        DPVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;

    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    //
    //     ZZDYNOAC tests
    //
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAC, normal case:  fetch array using ID form of name.",
        ctx,
    )?;

    //
    // First load some variables into the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_CH_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_CH_ARRAY_1 = ( \'1\', \'2\', \'3\' )",
    );
    fstr::assign(DEFTXT.get_mut(2), b"FRAME_-1000000000_CH_ARRAY_2 = \'4\'");

    spicelib::LMPOOL(DEFTXT.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR(I, &mut EXPCHV[I], ctx);
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSC(b"CHVALS (1)", &CHVALS[I], b"=", &EXPCHV[I], OK, ctx)?;
            I += m3__;
        }
    }

    //
    // Repeat with a second variable.
    //
    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_2",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKSC(b"CHVALS (2)", &CHVALS[1], b"=", b"4", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAC, normal case:  fetch array using character form of name.",
        ctx,
    )?;

    fstr::assign(
        DEFTXT.get_mut(3),
        b"FRAME_FRAME_CH_1_CH_ARRAY_1  = ( \'5\', \'6\', \'7\', \'8\' )",
    );
    fstr::assign(DEFTXT.get_mut(4), b"FRAME_FRAME_CH_1_CH_ARRAY_2  = \'9\'");

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.subarray(3), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR((I + 4), &mut EXPCHV[I], ctx);
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSC(b"CHVALS (1)", &CHVALS[I], b"=", &EXPCHV[I], OK, ctx)?;
            I += m3__;
        }
    }

    //
    // Repeat with a second variable.
    //
    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_2",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKSC(b"CHVALS (2)", &CHVALS[1], b"=", b"9", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAC, normal case:  fetch array when ID and character form of name are both present.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR(I, &mut EXPCHV[I], ctx);
            I += m3__;
        }
    }

    //
    // Check the contents of the returned array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSC(b"CHVALS (1)", &CHVALS[I], b"=", &EXPCHV[I], OK, ctx)?;
            I += m3__;
        }
    }

    //
    // Repeat with a second variable.
    //
    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_2",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check the cardinality of the returned array.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the contents of the returned array.
    //
    testutil::CHCKSC(b"CHVALS (2)", &CHVALS[1], b"=", b"4", OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAC, exception case:  try to fetch array that is not present.",
        ctx,
    )?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_3",
        MAXVAL,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the found flag.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAC, exception case:  try to fetch array having excessive size.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_CH_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_CH_ARRAY_1 = ( \'1\', \'2\', \'3\' )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        1,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNOAC, exception case:  try to fetch array having wrong data type.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_CH_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_CH_ARRAY_1 = ( 1, 2, 3 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the character fetch routine for required arguments.
    //
    spicelib::ZZDYNOAC(
        &FRAMNM,
        FRCODE,
        b"CH_ARRAY_1",
        3,
        &mut N,
        CHVALS.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;

    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    //
    //     ZZDYNBID tests
    //
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNBID, normal case:  fetch array using ID form of name.",
        ctx,
    )?;

    //
    // First load some variables into the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(DEFTXT.get_mut(1), b"FRAME_-1000000000_IN_ARRAY_1 = 199");
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_-1000000000_IN_ARRAY_2 = \'MARS BARYCENTER\'",
    );

    spicelib::LMPOOL(DEFTXT.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (1)", *INVALS.first(), b"=", 199, 0, OK, ctx)?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_2", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (2)", *INVALS.first(), b"=", 4, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNBID, normal case:  fetch array using character form of name.",
        ctx,
    )?;

    fstr::assign(DEFTXT.get_mut(3), b"FRAME_FRAME_IN_1_IN_ARRAY_1  = \'5\'");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_FRAME_IN_1_IN_ARRAY_2  = \'Pluto\'",
    );

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.subarray(3), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (1)", *INVALS.first(), b"=", 5, 0, OK, ctx)?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_2", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (2)", *INVALS.first(), b"=", 999, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNBID, normal case:  fetch array when ID and character form of name are both present.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(DEFTXT.get_mut(1), b"FRAME_-1000000000_IN_ARRAY_1 = 199");
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_-1000000000_IN_ARRAY_2 = \'MARS BARYCENTER\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_FRAME_IN_1_IN_ARRAY_1  = \'5\'");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_FRAME_IN_1_IN_ARRAY_2  = \'Pluto\'",
    );

    spicelib::LMPOOL(DEFTXT.as_arg(), 4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (1)", *INVALS.first(), b"=", 199, 0, OK, ctx)?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_2", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (2)", *INVALS.first(), b"=", 4, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNBID, exception case:  try to fetch array that is not present.",
        ctx,
    )?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_3", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNBID, exception case:  try to fetch integer array having size > 1.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( 1, 2, 3 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNBID, exception case:  try to fetch character array having size > 1.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( \'1\', \'2\' )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNBID, exception case:  try to fetch body name with no matching ID code.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 =  \'PLANET X\'",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNBID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTRANSLATION)", OK, ctx)?;

    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    //
    //     ZZDYNFID tests
    //
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************
    // ****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNFID, normal case:  fetch array using ID form of name.",
        ctx,
    )?;

    //
    // First load some variables into the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(DEFTXT.get_mut(1), b"FRAME_-1000000000_IN_ARRAY_1 = 1");
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_-1000000000_IN_ARRAY_2 = \'IAU_MARS\'",
    );

    spicelib::LMPOOL(DEFTXT.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (1)", *INVALS.first(), b"=", 1, 0, OK, ctx)?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_2", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    spicelib::NAMFRM(b"IAU_MARS", &mut I, ctx)?;
    testutil::CHCKSI(b"INVALS (2)", *INVALS.first(), b"=", I, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNFID, normal case:  fetch array using character form of name.",
        ctx,
    )?;

    fstr::assign(
        DEFTXT.get_mut(3),
        b"FRAME_FRAME_IN_1_IN_ARRAY_1  = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_FRAME_IN_1_IN_ARRAY_2  = \'B1950\'",
    );

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTXT.subarray(3), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (1)", *INVALS.first(), b"=", 1, 0, OK, ctx)?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_2", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    spicelib::NAMFRM(b"B1950", &mut I, ctx)?;
    testutil::CHCKSI(b"INVALS (2)", *INVALS.first(), b"=", I, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNFID, normal case:  fetch array when ID and character form of name are both present.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(DEFTXT.get_mut(1), b"FRAME_-1000000000_IN_ARRAY_1 = 1");
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_-1000000000_IN_ARRAY_2 = \'IAU_MARS\'",
    );
    fstr::assign(
        DEFTXT.get_mut(3),
        b"FRAME_FRAME_IN_1_IN_ARRAY_1  = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_FRAME_IN_1_IN_ARRAY_2  = \'B1950\'",
    );

    spicelib::LMPOOL(DEFTXT.as_arg(), 4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    testutil::CHCKSI(b"INVALS (1)", *INVALS.first(), b"=", 1, 0, OK, ctx)?;

    //
    // Repeat with a second variable.
    //
    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_2", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the contents of the returned ID.
    //
    spicelib::NAMFRM(b"IAU_MARS", &mut I, ctx)?;
    testutil::CHCKSI(b"INVALS (2)", *INVALS.first(), b"=", I, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNFID, exception case:  try to fetch array that is not present.",
        ctx,
    )?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_3", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNFID, exception case:  try to fetch integer array having size > 1.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( 1, 2, 3 )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNFID, exception case:  try to fetch character array having size > 1.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 = ( \'1\', \'2\' )",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDYNFID, exception case:  try to fetch frame name with no matching ID code.",
        ctx,
    )?;

    fstr::assign(&mut FRAMNM, b"FRAME_IN_1");
    FRCODE = -1000000000;

    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_-1000000000_IN_ARRAY_1 =  \'PLANET X\'",
    );

    spicelib::CLPOOL(ctx)?;

    spicelib::LMPOOL(DEFTXT.as_arg(), 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call the body ID fetch routine for required arguments.
    //
    spicelib::ZZDYNFID(&FRAMNM, FRCODE, b"IN_ARRAY_1", INVALS.first_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTRANSLATION)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
