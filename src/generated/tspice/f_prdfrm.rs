//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FK0: &[u8] = b"PRDFRM_test0.tf";
const PCK0: &[u8] = b"PRDFRM_test0.tpc";
const SPK0: &[u8] = b"PRDFRM_test0.bsp";
const VTIGHT: f64 = 0.00000000000001;
const LNSIZE: i32 = 80;
const TXTSIZ: i32 = 300;

struct SaveVars {
    TXTBUF: ActualCharArray,
    ET: f64,
    ET0: f64,
    ET1: f64,
    IDNT33: StackArray2D<f64, 9>,
    IDNT66: StackArray2D<f64, 36>,
    RMAT: StackArray2D<f64, 9>,
    STEP: f64,
    TOL: f64,
    XFORM: StackArray2D<f64, 36>,
    XRMAT: StackArray2D<f64, 9>,
    XXFORM: StackArray2D<f64, 36>,
    HANDLE: i32,
    N: i32,
    NLINES: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TXTBUF = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut IDNT33 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut IDNT66 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XRMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XXFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut NLINES: i32 = 0;

        Self {
            TXTBUF,
            ET,
            ET0,
            ET1,
            IDNT33,
            IDNT66,
            RMAT,
            STEP,
            TOL,
            XFORM,
            XRMAT,
            XXFORM,
            HANDLE,
            N,
            NLINES,
        }
    }
}

//$Procedure F_PRDFRM ( Product frame tests )
pub fn F_PRDFRM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test utility functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_PRDFRM", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Setup: create and load CK0, SCLK0, FK0, PCK0, SPK0, and LSK file.",
        ctx,
    )?;

    //
    // Create a text kernel containing the following frame
    // specifications:
    //
    // FRAME_SINGLETON_PRODUCT     =  1400000
    // FRAME_1400000_NAME          = 'SINGLETON_PRODUCT'
    // FRAME_1400000_CLASS         =  5
    // FRAME_1400000_CLASS_ID      =  1400000
    // FRAME_1400000_CENTER        =  499
    // FRAME_1400000_RELATIVE      = 'J2000'
    // FRAME_1400000_DEF_STYLE     = 'PARAMETERIZED'
    // FRAME_1400000_FAMILY        = 'PRODUCT'
    // FRAME_1400000_FROM_FRAMES   = 'IAU_EARTH'
    // FRAME_1400000_TO_FRAMES     = 'IAU_JUPITER'
    //
    // FRAME_IAU_MARS_PRODUCT      =  1400499
    // FRAME_1400499_NAME          = 'IAU_MARS_PRODUCT'
    // FRAME_1400499_CLASS         =  5
    // FRAME_1400499_CLASS_ID      =  1400499
    // FRAME_1400499_CENTER        =  499
    // FRAME_1400499_RELATIVE      = 'J2000'
    // FRAME_1400499_DEF_STYLE     = 'PARAMETERIZED'
    // FRAME_1400499_FAMILY        = 'PRODUCT'
    // FRAME_1400499_FROM_FRAMES   = ( 'IAU_EARTH'
    //                                 'IAU_JUPITER'
    //                                 'J2000' )
    // FRAME_1400499_TO_FRAMES     = ( 'IAU_MARS'
    //                                 'IAU_EARTH'
    //                                 'IAU_JUPITER')
    //
    // FRAME_MARS_EQUATOR_SOLAR_PROD   =  1400001
    // FRAME_1400001_NAME              =  'MARS_EQUATOR_SOLAR_PROD'
    // FRAME_1400001_CLASS             =  5
    // FRAME_1400001_CLASS_ID          =  1400001
    // FRAME_1400001_CENTER            =  'EARTH'
    // FRAME_1400001_RELATIVE          =  'J2000'
    // FRAME_1400001_DEF_STYLE         =  'PARAMETERIZED'
    // FRAME_1400001_FAMILY            =  'TWO-VECTOR'
    // FRAME_1400001_PRI_AXIS          =  'Z'
    // FRAME_1400001_PRI_VECTOR_DEF    =  'CONSTANT'
    // FRAME_1400001_PRI_FRAME         =  'IAU_MARS_PRODUCT'
    // FRAME_1400001_PRI_SPEC          =  'RECTANGULAR'
    // FRAME_1400001_PRI_VECTOR        =  ( 0.0 0.0 1.0 )
    // FRAME_1400001_SEC_AXIS          =  'Y'
    // FRAME_1400001_SEC_VECTOR_DEF    =  'OBSERVER_TARGET_VELOCITY'
    // FRAME_1400001_SEC_OBSERVER      =  'MARS'
    // FRAME_1400001_SEC_TARGET        =  'SUN'
    // FRAME_1400001_SEC_ABCORR        =  'NONE'
    // FRAME_1400001_SEC_FRAME         =  'J2000'
    //
    // FRAME_MARS_EQUATOR_SOLAR        =  1400002
    // FRAME_1400002_NAME              =  'MARS_EQUATOR_SOLAR'
    // FRAME_1400002_CLASS             =  5
    // FRAME_1400002_CLASS_ID          =  1400002
    // FRAME_1400002_CENTER            =  'EARTH'
    // FRAME_1400002_RELATIVE          =  'J2000'
    // FRAME_1400002_DEF_STYLE         =  'PARAMETERIZED'
    // FRAME_1400002_FAMILY            =  'TWO-VECTOR'
    // FRAME_1400002_PRI_AXIS          =  'Z'
    // FRAME_1400002_PRI_VECTOR_DEF    =  'CONSTANT'
    // FRAME_1400002_PRI_FRAME         =  'IAU_MARS'
    // FRAME_1400002_PRI_SPEC          =  'RECTANGULAR'
    // FRAME_1400002_PRI_VECTOR        =  ( 0.0 0.0 1.0 )
    // FRAME_1400002_SEC_AXIS          =  'Y'
    // FRAME_1400002_SEC_VECTOR_DEF    =  'OBSERVER_TARGET_VELOCITY'
    // FRAME_1400002_SEC_OBSERVER      =  'MARS'
    // FRAME_1400002_SEC_TARGET        =  'SUN'
    // FRAME_1400002_SEC_ABCORR        =  'NONE'
    // FRAME_1400002_SEC_FRAME         =  'J2000'
    //
    //
    //
    //
    //
    testutil::BEGDAT(&mut save.TXTBUF[1]);
    fstr::assign(save.TXTBUF.get_mut(2), b" ");
    fstr::assign(save.TXTBUF.get_mut(3), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(4),
        b"      FRAME_SINGLETON_PRODUCT     =  1400000",
    );
    fstr::assign(
        save.TXTBUF.get_mut(5),
        b"      FRAME_1400000_NAME          = \'SINGLETON_PRODUCT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(6),
        b"      FRAME_1400000_CLASS         =  5",
    );
    fstr::assign(
        save.TXTBUF.get_mut(7),
        b"      FRAME_1400000_CLASS_ID      =  1400000",
    );
    fstr::assign(
        save.TXTBUF.get_mut(8),
        b"      FRAME_1400000_CENTER        =  499",
    );
    fstr::assign(
        save.TXTBUF.get_mut(9),
        b"      FRAME_1400000_RELATIVE      = \'J2000\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(10),
        b"      FRAME_1400000_DEF_STYLE     = \'PARAMETERIZED\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(11),
        b"      FRAME_1400000_FAMILY        = \'PRODUCT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(12),
        b"      FRAME_1400000_FROM_FRAMES   = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(13),
        b"      FRAME_1400000_TO_FRAMES     = \'IAU_JUPITER\'",
    );
    fstr::assign(save.TXTBUF.get_mut(14), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(15),
        b"      FRAME_IAU_MARS_PRODUCT      =  1400499",
    );
    fstr::assign(
        save.TXTBUF.get_mut(16),
        b"      FRAME_1400499_NAME          = \'IAU_MARS_PRODUCT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(17),
        b"      FRAME_1400499_CLASS         =  5",
    );
    fstr::assign(
        save.TXTBUF.get_mut(18),
        b"      FRAME_1400499_CLASS_ID      =  1400499",
    );
    fstr::assign(
        save.TXTBUF.get_mut(19),
        b"      FRAME_1400499_CENTER        =  499",
    );
    fstr::assign(
        save.TXTBUF.get_mut(20),
        b"      FRAME_1400499_RELATIVE      = \'J2000\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(21),
        b"      FRAME_1400499_DEF_STYLE     = \'PARAMETERIZED\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(22),
        b"      FRAME_1400499_FAMILY        = \'PRODUCT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(23),
        b"      FRAME_1400499_FROM_FRAMES   = ( \'IAU_EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(24),
        b"                                      \'IAU_JUPITER\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(25),
        b"                                      \'J2000\' )",
    );
    fstr::assign(
        save.TXTBUF.get_mut(26),
        b"      FRAME_1400499_TO_FRAMES     = ( \'IAU_MARS\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(27),
        b"                                      \'IAU_EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(28),
        b"                                      \'IAU_JUPITER\')",
    );
    fstr::assign(save.TXTBUF.get_mut(29), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(30),
        b"      FRAME_MARS_EQUATOR_SOLAR_PROD   =  1400001",
    );
    fstr::assign(
        save.TXTBUF.get_mut(31),
        b"      FRAME_1400001_NAME              =  \'MARS_EQUATOR_SOLAR_PROD\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(32),
        b"      FRAME_1400001_CLASS             =  5",
    );
    fstr::assign(
        save.TXTBUF.get_mut(33),
        b"      FRAME_1400001_CLASS_ID          =  1400001",
    );
    fstr::assign(
        save.TXTBUF.get_mut(34),
        b"      FRAME_1400001_CENTER            =  \'EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(35),
        b"      FRAME_1400001_RELATIVE          =  \'J2000\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(36),
        b"      FRAME_1400001_DEF_STYLE         =  \'PARAMETERIZED\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(37),
        b"      FRAME_1400001_FAMILY            =  \'TWO-VECTOR\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(38),
        b"      FRAME_1400001_PRI_AXIS          =  \'Z\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(39),
        b"      FRAME_1400001_PRI_VECTOR_DEF    =  \'CONSTANT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(40),
        b"      FRAME_1400001_PRI_FRAME         =  \'IAU_MARS_PRODUCT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(41),
        b"      FRAME_1400001_PRI_SPEC          =  \'RECTANGULAR\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(42),
        b"      FRAME_1400001_PRI_VECTOR        =  ( 0.0 0.0 1.0 )",
    );
    fstr::assign(
        save.TXTBUF.get_mut(43),
        b"      FRAME_1400001_SEC_AXIS          =  \'Y\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(44),
        b"      FRAME_1400001_SEC_VECTOR_DEF    =  \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(45),
        b"      FRAME_1400001_SEC_OBSERVER      =  \'MARS\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(46),
        b"      FRAME_1400001_SEC_TARGET        =  \'SUN\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(47),
        b"      FRAME_1400001_SEC_ABCORR        =  \'NONE\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(48),
        b"      FRAME_1400001_SEC_FRAME         =  \'J2000\'",
    );
    fstr::assign(save.TXTBUF.get_mut(49), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(50),
        b"      FRAME_MARS_EQUATOR_SOLAR        =  1400002",
    );
    fstr::assign(
        save.TXTBUF.get_mut(51),
        b"      FRAME_1400002_NAME              =  \'MARS_EQUATOR_SOLAR\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(52),
        b"      FRAME_1400002_CLASS             =  5",
    );
    fstr::assign(
        save.TXTBUF.get_mut(53),
        b"      FRAME_1400002_CLASS_ID          =  1400002",
    );
    fstr::assign(
        save.TXTBUF.get_mut(54),
        b"      FRAME_1400002_CENTER            =  \'EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(55),
        b"      FRAME_1400002_RELATIVE          =  \'J2000\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(56),
        b"      FRAME_1400002_DEF_STYLE         =  \'PARAMETERIZED\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(57),
        b"      FRAME_1400002_FAMILY            =  \'TWO-VECTOR\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(58),
        b"      FRAME_1400002_PRI_AXIS          =  \'Z\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(59),
        b"      FRAME_1400002_PRI_VECTOR_DEF    =  \'CONSTANT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(60),
        b"      FRAME_1400002_PRI_FRAME         =  \'IAU_MARS\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(61),
        b"      FRAME_1400002_PRI_SPEC          =  \'RECTANGULAR\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(62),
        b"      FRAME_1400002_PRI_VECTOR        =  ( 0.0 0.0 1.0 )",
    );
    fstr::assign(
        save.TXTBUF.get_mut(63),
        b"      FRAME_1400002_SEC_AXIS          =  \'Y\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(64),
        b"      FRAME_1400002_SEC_VECTOR_DEF    =  \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(65),
        b"      FRAME_1400002_SEC_OBSERVER      =  \'MARS\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(66),
        b"      FRAME_1400002_SEC_TARGET        =  \'SUN\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(67),
        b"      FRAME_1400002_SEC_ABCORR        =  \'NONE\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(68),
        b"      FRAME_1400002_SEC_FRAME         =  \'J2000\'",
    );
    fstr::assign(save.TXTBUF.get_mut(69), b" ");
    testutil::BEGTXT(&mut save.TXTBUF[70]);
    fstr::assign(save.TXTBUF.get_mut(71), b" ");

    if spicelib::EXISTS(FK0, ctx)? {
        testutil::KILFIL(FK0, ctx)?;
    }

    save.NLINES = 71;
    testutil::TSTTXT(FK0, save.TXTBUF.as_arg(), save.NLINES, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(FK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(PCK0, ctx)? {
        testutil::KILFIL(PCK0, ctx)?;
    }

    testutil::TSTPCK(PCK0, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK0, false, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Also load LSK to support time conversions.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create identity matrices.
    //
    spicelib::IDENT(save.IDNT33.as_slice_mut());

    spicelib::CLEARD(36, save.IDNT66.as_slice_mut());

    for I in 1..=6 {
        save.IDNT66[[I, I]] = 1.0;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Get the state transformation from J2000 to SINGLETON_PRODUCT. This should match the transformation from IAU_EARTH to IAU_JUPITER at the same epoch.", ctx)?;

    spicelib::STR2ET(b"2020 OCT 1", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"SINGLETON_PRODUCT",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"IAU_EARTH",
        b"IAU_JUPITER",
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XFORM a",
        save.XFORM.as_slice(),
        b"~",
        save.XXFORM.as_slice(),
        36,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Get the position transformation from J2000 to SINGLETON_PRODUCT. This should match the transformation from IAU_EARTH to IAU_JUPITER at the same epoch.", ctx)?;

    spicelib::STR2ET(b"2020 OCT 1", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        b"J2000",
        b"SINGLETON_PRODUCT",
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        b"IAU_EARTH",
        b"IAU_JUPITER",
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"RMAT",
        save.RMAT.as_slice(),
        b"~",
        save.XRMAT.as_slice(),
        9,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Get the state transformation from J2000 to IAU_MARS_PRODUCT. This should match the transformation from J2000 to IAU_MARS at the same epoch.", ctx)?;

    spicelib::STR2ET(b"2020 OCT 1", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_MARS_PRODUCT",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_MARS",
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XFORM a",
        save.XFORM.as_slice(),
        b"~",
        save.XXFORM.as_slice(),
        36,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Get the position transformation from J2000 to IAU_MARS_PRODUCT. This should match the transformation from J2000 to IAU_MARS at the same epoch.", ctx)?;

    spicelib::STR2ET(b"2020 OCT 1", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        b"J2000",
        b"IAU_MARS_PRODUCT",
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        b"J2000",
        b"IAU_MARS",
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"RMAT",
        save.RMAT.as_slice(),
        b"~",
        save.XRMAT.as_slice(),
        9,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Get the state transformation from J2000 to MARS_EQUATOR_SOLAR_PROD. This should match the transformation from J2000 to MARS_EQUATOR_SOLAR at the same epoch. Compute transformations for a sequence of times.", ctx)?;

    spicelib::STR2ET(b"2005 OCT 1", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2005 OCT 30", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 100;
    save.STEP = ((save.ET1 - save.ET0) / (save.N - 1) as f64);

    save.TOL = VTIGHT;

    for I in 1..=save.N {
        save.ET = (save.ET0 + (((I - 1) as f64) * save.STEP));

        spicelib::SXFORM(
            b"J2000",
            b"MARS_EQUATOR_SOLAR_PROD",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SXFORM(
            b"J2000",
            b"MARS_EQUATOR_SOLAR",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Get the position transformation from J2000 to MARS_EQUATOR_SOLAR_PROD. This should match the transformation from J2000 to MARS_EQUATOR_SOLAR at the same epoch. Compute transformations for a sequence of times.", ctx)?;

    spicelib::STR2ET(b"2005 OCT 1", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2005 OCT 30", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 100;
    save.STEP = ((save.ET1 - save.ET0) / (save.N - 1) as f64);

    save.TOL = VTIGHT;

    for I in 1..=save.N {
        save.ET = (save.ET0 + (((I - 1) as f64) * save.STEP));

        spicelib::PXFORM(
            b"J2000",
            b"MARS_EQUATOR_SOLAR_PROD",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PXFORM(
            b"J2000",
            b"MARS_EQUATOR_SOLAR",
            save.ET,
            save.XRMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.XRMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;
    }
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::KILFIL(FK0, ctx)?;
    testutil::KILFIL(PCK0, ctx)?;
    testutil::KILFIL(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
