//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const TOTAL1: i32 = -3;
const ANNLR1: i32 = -2;
const PARTL1: i32 = -1;
const NOOCC: i32 = 0;
const PARTL2: i32 = 1;
const ANNLR2: i32 = 2;
const TOTAL2: i32 = 3;
const DSK: &[u8] = b"occult_nat.bds";
const PCK: &[u8] = b"nat.tpc";
const PCK2: &[u8] = b"generic.tpc";
const SPK1: &[u8] = b"nat.bsp";
const SPK2: &[u8] = b"generic.bsp";
const TIMSIZ: i32 = 20;
const NUMTIM: i32 = 5;

struct SaveVars {
    TIMES: ActualCharArray,
    RESULT: StackArray<i32, 5>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TIMES = ActualCharArray::new(TIMSIZ, 1..=NUMTIM);
        let mut RESULT = StackArray::<i32, 5>::new(1..=NUMTIM);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"2011-JAN-02 19:00:00")].into_iter();
            fstr::assign(TIMES.get_mut(1), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"2011-JAN-02 21:00:00")].into_iter();
            fstr::assign(TIMES.get_mut(2), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"2011-JAN-03 00:00:00")].into_iter();
            fstr::assign(TIMES.get_mut(3), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"2011-JAN-03 09:00:00")].into_iter();
            fstr::assign(TIMES.get_mut(4), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"2011-JAN-03 11:00:00")].into_iter();
            fstr::assign(TIMES.get_mut(5), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::I(-1), Val::I(-3), Val::I(1), Val::I(2)].into_iter();
            RESULT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { TIMES, RESULT }
    }
}

//$Procedure      F_OCCULT ( OCCULT family tests )
pub fn F_OCCULT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ABCORR = [b' '; TIMSIZ as usize];
    let mut AFRAME = [b' '; TIMSIZ as usize];
    let mut BFRAME = [b' '; TIMSIZ as usize];
    let mut FRAME = [b' '; TIMSIZ as usize];
    let mut FRAME1 = [b' '; TIMSIZ as usize];
    let mut FRAME2 = [b' '; TIMSIZ as usize];
    let mut OBSRVR = [b' '; TIMSIZ as usize];
    let mut SHAPE1 = [b' '; TIMSIZ as usize];
    let mut SHAPE2 = [b' '; TIMSIZ as usize];
    let mut TARG1 = [b' '; TIMSIZ as usize];
    let mut TARG2 = [b' '; TIMSIZ as usize];
    let mut ARAD = StackArray::<f64, 3>::new(1..=3);
    let mut C: f64 = 0.0;
    let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
    let mut BOUNDS = StackArray2D::<f64, 4>::new(1..=2, 1..=2);
    let mut ET: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut PMCOEF = StackArray::<f64, 3>::new(1..=3);
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut LAST: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut BODYID: i32 = 0;
    let mut CORSYS: i32 = 0;
    let mut HAN1: i32 = 0;
    let mut HAN2: i32 = 0;
    let mut MLTFAC: i32 = 0;
    let mut N: i32 = 0;
    let mut NLAT: i32 = 0;
    let mut NLON: i32 = 0;
    let mut OCLTID: i32 = 0;
    let mut SURFID: i32 = 0;
    let mut FOUND: bool = false;
    let mut MAKVTL: bool = false;
    let mut USEPAD: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Save variables
    //

    //
    // Initialize
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_OCCULT", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    testutil::NATSPK(SPK1, true, &mut HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK2, true, &mut HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(PCK2, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000-JAN-01 00:00:00 (TDB)", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Invalid first shape", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"PNT",
        b"ALPHAFIXED",
        b"BETA",
        b"POINT",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    // Missing "unprioritized" keyword:
    //
    spicelib::OCCULT(
        b"ALPHA",
        b"DSK",
        b"ALPHAFIXED",
        b"BETA",
        b"POINT",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADPRIORITYSPEC)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Invalid second shape", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"PNT",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    // Missing "unprioritized" keyword:
    //
    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"DSK",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADPRIORITYSPEC)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Verify both targets cannot be points", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"POINT",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSHAPECOMBO)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Verify both targets cannot be identical", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"ALPHA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Verify the first target and observer cannot be identical.",
        ctx,
    )?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"NONE",
        b"ALPHA",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Verify the second target and observer cannot be identical.",
        ctx,
    )?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"NONE",
        b"BETA",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Note:  This is a test for three cases:
    //     1)  If an unrecognized target is elliptical and the frame
    //         cannot be found.
    //     2)  If an unrecognized target is a point and the ID code is
    //         not found in the call tree of ZZGFOCIN.
    //     3)  If an observer is unrecognized.
    //
    testutil::TCASE(
        b"Verify an error if the targets or observer are unrecognized",
        ctx,
    )?;

    spicelib::OCCULT(
        b"ALPH",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BTA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"NONE",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"NONE",
        b"SUNN",
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Verify aberration correction inputs", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"S",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"XS",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"RLT",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"XRLT",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"z",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for observer", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"LT",
        b"GASPRA",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for first target", ctx)?;

    spicelib::OCCULT(
        b"GASPRA",
        b"POINT",
        b"IAU_GASPRA",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for second target", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"GASPRA",
        b"POINT",
        b"IAU_GASPRA",
        b"LT",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for first target; both targets are ellipsoids.",
        ctx,
    )?;

    spicelib::OCCULT(
        b"EARTH",
        b"ellipsoid",
        b"ITRF93",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"LT",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for second target; both targets are ellipsoids.",
        ctx,
    )?;

    spicelib::OCCULT(
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"EARTH",
        b"ellipsoid",
        b"ITRF93",
        b"LT",
        b"SUN",
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for front target; front target is ellipsoid; back is point.",
        ctx,
    )?;

    //
    // Capture Beta's prime meridian data; delete it from
    // the kernel pool.
    //
    spicelib::GDPOOL(
        b"BODY2000_PM",
        1,
        3,
        &mut N,
        PMCOEF.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::DVPOOL(b"BODY2000_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::OCCULT(
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"ALPHA",
        b"point",
        b" ",
        b"NONE",
        b"SUN",
        250.0,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // Restore the PM data.
    //
    spicelib::PDPOOL(b"BODY2000_PM", 3, PMCOEF.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for back target; front target is point; back is ellipsoid.",
        ctx,
    )?;

    //
    // Because the frame ALPHAFIXED is a TK frame defined
    // relative to the PCK frame BETAAFIXED, we can make
    // the SINCPT call, which accesses the ALPHAFIXED frame,
    // fail by damaging the data for the for BETAFIXED frame.
    //
    // This scheme depends on the Nat's solar system PCK
    // implementation.
    //
    // Capture Beta's prime meridian data; delete it from
    // the kernel pool.
    //
    spicelib::GDPOOL(
        b"BODY2000_PM",
        1,
        3,
        &mut N,
        PMCOEF.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::DVPOOL(b"BODY2000_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::OCCULT(
        b"BETA",
        b"point",
        b" ",
        b"ALPHA",
        b"ellipsoid",
        b"alphafixed",
        b"NONE",
        b"SUN",
        62.0,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // Restore the PM data.
    //
    spicelib::PDPOOL(b"BODY2000_PM", 3, PMCOEF.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************
    //
    //     When TARG1 is GAMMA (abcorr = none):
    //
    // 2000-JAN-02 19 GAMMA  not occulted by        ALPHA  as seen by SUN   0
    // 2000-JAN-02 21 GAMMA  partially occulted by  ALPHA  as seen by SUN  -1
    // 2000-JAN-03 00 GAMMA  totally occulted by    ALPHA  as seen by SUN  -3
    // 2000-JAN-03 07 GAMMA  not occulted by        ALPHA  as seen by SUN   0
    // 2000-JAN-03 09 ALPHA  partially occulted by  GAMMA  as seen by SUN   1
    // 2000-JAN-03 11 ALPHA  transited by           GAMMA  as seen by SUN   2
    // 2000-JAN-03 16 GAMMA  not occulted by        ALPHA  as seen by SUN   0
    //
    //     When TARG1 is ALPHA (abcorr = none):
    //
    // 2000-JAN-02 19 ALPHA not occulted by         GAMMA  as seen by SUN   0
    // 2000-JAN-02 21 GAMMA partially occulted by   ALPHA  as seen by SUN   1
    // 2000-JAN-03 00 GAMMA totally occulted by     ALPHA  as seen by SUN   3
    // 2000-JAN-03 07 ALPHA not occulted by         GAMMA  as seen by SUN   0
    // 2000-JAN-03 09 ALPHA partially occulted by   GAMMA  as seen by SUN  -1
    // 2000-JAN-03 11 ALPHA transited by            GAMMA  as seen by SUN  -2
    // 2000-JAN-03 16 ALPHA not occulted by         GAMMA  as seen by SUN   0
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (none),
    //                  occultation type (any)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-02 20:41:15
    //                     Stop time:  2000-JAN-03 03:51:58
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (none),
    //                  occultation type (partial)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-02 20:41:15
    //                     Stop time:  2000-JAN-02 22:52:53
    //                  Interval            2
    //                     Start time: 2000-JAN-03 02:28:28
    //                     Stop time:  2000-JAN-03 03:51:58
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (none),
    //                  occultation type (annular)
    //
    //                  No occultation was found.
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (none),
    //                  occultation type (full, total)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-02 22:52:53
    //                     Stop time:  2000-JAN-03 02:28:28
    //
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (none),
    //                  occultation type (any)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-03 08:24:46
    //                     Stop time:  2000-JAN-03 14:57:23
    //
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (none),
    //                  occultation type (partial)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-03 08:24:46
    //                     Stop time:  2000-JAN-03 09:58:08
    //                  Interval            2
    //                     Start time: 2000-JAN-03 12:34:35
    //                     Stop time:  2000-JAN-03 14:57:23
    //
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (none),
    //                  occultation type (annular)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-03 09:58:08
    //                     Stop time:  2000-JAN-03 12:34:35
    //
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (none),
    //                  occultation type (full, total)
    //
    //                  No occultation was found.
    //
    //           -   -   -   -   -   -   -   -   -   -   -   -
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (xcn),
    //                  occultation type (any)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-03 08:24:39
    //                     Stop time:  2000-JAN-03 14:57:17
    //
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (xcn),
    //                  occultation type (partial)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-03 08:24:39
    //                     Stop time:  2000-JAN-03 09:57:59
    //                  Interval            2
    //                     Start time: 2000-JAN-03 12:34:32
    //                     Stop time:  2000-JAN-03 14:57:17
    //
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (xcn),
    //                  occultation type (annular transit)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-03 09:57:59
    //                     Stop time:  2000-JAN-03 12:34:32
    //
    //     From GFOCLT, Front (gamma), back (alpha), abcorr (xcn),
    //                  occultation type (full)
    //
    //                  No occultation was found.
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (xcn),
    //                  occultation type (any)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-02 20:41:09
    //                     Stop time:  2000-JAN-03 03:51:51
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (xcn),
    //                  occultation type (partial)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-02 20:41:09
    //                     Stop time:  2000-JAN-02 22:52:49
    //                  Interval            2
    //                     Start time: 2000-JAN-03 02:28:19
    //                     Stop time:  2000-JAN-03 03:51:51
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (xcn),
    //                  occultation type (annular)
    //
    //                  No occultation was found.
    //
    //     From GFOCLT, Front (alpha), back (gamma), abcorr (xcn),
    //                  occultation type (full)
    //
    //                  Interval            1
    //                     Start time: 2000-JAN-02 22:52:49
    //                     Stop time:  2000-JAN-03 02:28:19
    //
    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Verify the occultation types for specific times are correct.",
        ctx,
    )?;

    //
    // In order to test all occultation ID codes, first make GAMMA the
    // first target, and then make ALPHA the first target.  The same
    // configuration will have an occultation ID of -1*result if the
    // target 1 and target 2 bodies are reversed.  This is why MLTFAC
    // exists.
    //

    for I in 1..=2 {
        if (I == 1) {
            fstr::assign(&mut TARG1, b"GAMMA");
            fstr::assign(&mut SHAPE1, b"ELLIPSOID");
            fstr::assign(&mut FRAME1, b"GAMMAFIXED");
            fstr::assign(&mut TARG2, b"ALPHA");
            fstr::assign(&mut SHAPE2, b"ELLIPSOID");
            fstr::assign(&mut FRAME2, b"ALPHAFIXED");
            fstr::assign(&mut OBSRVR, b"SUN");
            fstr::assign(&mut ABCORR, b"NONE");
            MLTFAC = 1;
        } else {
            fstr::assign(&mut TARG1, b"ALPHA");
            fstr::assign(&mut FRAME1, b"ALPHAFIXED");
            fstr::assign(&mut TARG2, b"GAMMA");
            fstr::assign(&mut FRAME2, b"GAMMAFIXED");
            MLTFAC = -1;
        }
        //
        // For each test, convert the time to ET, call OCCULT, and verify
        // the OCLTID matches with the desired result.
        //
        for J in 1..=NUMTIM {
            spicelib::STR2ET(&save.TIMES[J], &mut ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::OCCULT(
                &TARG1,
                &SHAPE1,
                &FRAME1,
                &TARG2,
                &SHAPE2,
                &FRAME2,
                &ABCORR,
                &OBSRVR,
                ET,
                &mut OCLTID,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                b"OCLTID",
                OCLTID,
                b"=",
                (MLTFAC * save.RESULT[J]),
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"OCLTID",
                OCLTID,
                b"=",
                (MLTFAC * save.RESULT[J]),
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //     At TIMES(3) with Gamma as the first target (point), the
    //     occultation ID should be -3, representing that Gamma is completely
    //     occulted by Alpha.
    //
    testutil::TCASE(b"Point case:  Point totally occulted by ellipsoid", ctx)?;

    spicelib::STR2ET(&save.TIMES[3], &mut ET, ctx)?;

    spicelib::OCCULT(
        b"gamma",
        b"point",
        b" ",
        b"alpha",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", save.RESULT[3], 0, OK, ctx)?;

    spicelib::OCCULT(
        b"alpha",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"gamma",
        b"point",
        b" ",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", -(1 * save.RESULT[3]), 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     At TIMES(5) with Gamma as the first target (point), the
    //     occultation ID should be 2 (alpha transited by gamma).
    //
    testutil::TCASE(b"Point case: Point transiting ellipsoid", ctx)?;

    spicelib::STR2ET(&save.TIMES[5], &mut ET, ctx)?;

    spicelib::OCCULT(
        b"GAMMA",
        b"POINT",
        b" ",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", save.RESULT[5], 0, OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"GAMMA",
        b"POINT",
        b" ",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", -(1 * save.RESULT[5]), 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     At the time below, if the abcorr of 'none' is used, a partial
    //     occultation is reported.  If 'xcn' is used, an annular transit
    //     is reported.  The time and occultation types were calculated
    //     using GFOCLT.
    //
    testutil::TCASE(b"Verify results with different abcorr", ctx)?;

    spicelib::STR2ET(b"2000-JAN-03 09:58:02 (TDB)", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Calculate the occultation type with no aberration correction.
    // Check that the result is 'partial'.
    //
    spicelib::OCCULT(
        b"GAMMA",
        b"ELLIPSOID",
        b"GAMMAFIXED",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", 1, 0, OK, ctx)?;
    //
    // Calculate the occultation type with the aberration correction set
    // to 'xcn'.  Check that the result is 'annular'.
    //
    spicelib::OCCULT(
        b"GAMMA",
        b"ELLIPSOID",
        b"GAMMAFIXED",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"XCN",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", 2, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     DSK tests
    //
    //***********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a DSK containing shape models for bodies Alpha and Beta.",
        ctx,
    )?;

    //
    // Make sure the generic SPK is unloaded! We need the sun ephemeris
    // from Nat's SPK.
    //
    spicelib::SPKUEF(HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //
    // This block is suitable for use in F_GFOCLT. We don't actually
    // have to have the high-resolution DSK patches to test OCCULT.
    //
    // We'll enhance the generic DSK models for Alpha and Beta by
    // appending segments containing small patches of high-resolution
    // data for the surface regions that participate in computation of
    // accurate occultation ingress and egress times. We use small
    // patches because the size and run time needed to create full
    // models at this resolution would be prohibitive.
    //
    // Start by creating the basic DSK.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        spicelib::UNLOAD(DSK, ctx)?;
        spicelib::DELFIL(DSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Use low resolution tessellations for the main models.
    //
    NLON = 20;
    NLAT = 10;

    fstr::assign(&mut AFRAME, b"ALPHAFIXED");
    fstr::assign(&mut BFRAME, b"BETAFIXED");

    testutil::NATDSK(DSK, &AFRAME, NLON, NLAT, &BFRAME, NLON, NLAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the -Y patch for body Alpha. The patch covers
    // the lon/lat rectangle
    //
    //    -92 deg. <= lon <= -88 deg.
    //      0 deg. <= lat <=   4 deg.
    //
    // Note that Alpha' body-fixed Z axis lies in Alpha's orbital
    // plane.
    //
    fstr::assign(&mut FRAME, &AFRAME);
    BODYID = 1000;
    SURFID = 2;
    FIRST = -((100 as f64) * spicelib::JYEAR());
    LAST = ((100 as f64) * spicelib::JYEAR());

    spicelib::BODVCD(BODYID, b"RADII", 3, &mut N, ARAD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make the patch spherical, using the ellipsoid's Z
    // semi-axis length.
    //
    C = ARAD[3];

    CORSYS = LATSYS;
    spicelib::CLEARD(NSYPAR, CORPAR.as_slice_mut());

    MAKVTL = false;

    BOUNDS[[1, 1]] = (spicelib::RPD(ctx) * -92.0);
    BOUNDS[[2, 1]] = (spicelib::RPD(ctx) * -88.0);
    BOUNDS[[1, 2]] = (spicelib::RPD(ctx) * 0.0);
    BOUNDS[[2, 2]] = (spicelib::RPD(ctx) * 4.0);

    NLAT = 200;
    NLON = 200;

    USEPAD = true;

    //
    // Append the patch segment to the existing DSK.
    //
    testutil::T_SECDS2(
        BODYID,
        SURFID,
        &FRAME,
        FIRST,
        LAST,
        CORSYS,
        CORPAR.as_slice(),
        BOUNDS.as_slice(),
        C,
        C,
        C,
        NLON,
        NLAT,
        MAKVTL,
        USEPAD,
        DSK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the +Y patch for body Alpha. The patch covers
    // the lon/lat rectangle
    //
    //     88 deg. <= lon <=  92 deg.
    //      0 deg. <= lat <=   4 deg.
    //

    SURFID = 3;

    BOUNDS[[1, 1]] = (spicelib::RPD(ctx) * 88.0);
    BOUNDS[[2, 1]] = (spicelib::RPD(ctx) * 92.0);
    BOUNDS[[1, 2]] = (spicelib::RPD(ctx) * 0.0);
    BOUNDS[[2, 2]] = (spicelib::RPD(ctx) * 4.0);

    NLAT = 200;
    NLON = 200;

    USEPAD = true;

    //
    // Append the patch segment to the existing DSK.
    //
    testutil::T_SECDS2(
        BODYID,
        SURFID,
        &FRAME,
        FIRST,
        LAST,
        CORSYS,
        CORPAR.as_slice(),
        BOUNDS.as_slice(),
        C,
        C,
        C,
        NLON,
        NLAT,
        MAKVTL,
        USEPAD,
        DSK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Check for non-occultation just prior to transit start, for point BETA and DSK ALPHA. BETA is body 1.", ctx)?;

    testutil::NATPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"ALPHA",
        0.0,
        b"J2000",
        b"NONE",
        b"sun",
        POS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use a time 2 microseconds before the nominal start of occultation.
    //
    spicelib::STR2ET(b"2000 JAN 1 12:00:59.999998 TDB", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::OCCULT(
        b"BETA",
        b"POINT",
        b" ",
        b"ALPHA",
        b"DSK/UNPRIORITIZED",
        &FRAME,
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", NOOCC, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Check for non-occultation just prior to transit start, for point BETA and DSK ALPHA. BETA is body 2.", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"DSK/UNPRIORITIZED",
        &FRAME,
        b"BETA",
        b"POINT",
        b" ",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", NOOCC, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Check for annular transit just after transit start, for point BETA and DSK ALPHA. BETA is body 1.", ctx)?;

    //
    // Use a time 2 microseconds after the nominal start of occultation.
    //
    spicelib::STR2ET(b"2000 JAN 1 12:01:00.000002 TDB", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::OCCULT(
        b"BETA",
        b"POINT",
        b" ",
        b"ALPHA",
        b"DSK/UNPRIORITIZED",
        &FRAME,
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", ANNLR2, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Check for annular transit just after transit start, for point BETA and DSK ALPHA. BETA is body 2.", ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"DSK/UNPRIORITIZED",
        &FRAME,
        b"BETA",
        b"POINT",
        b" ",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", ANNLR1, 0, OK, ctx)?;

    //
    // In the tests below, BETA is modeled by a DSK and ALPHA is
    // treated as a point.
    //

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Check for full occultation at the midpoint of the occultation interval, for DSK BETA and point ALPHA. BETA is body 1.", ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 12:05:00 TDB", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::OCCULT(
        b"BETA",
        b"DSK/UNPRIORITIZED",
        &BFRAME,
        b"ALPHA",
        b"POINT",
        b" ",
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", TOTAL2, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Check for full occultation at the midpoint of the occultation interval, for DSK BETA and point ALPHA. BETA is body 2.", ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 12:05:00 TDB", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::OCCULT(
        b"ALPHA",
        b"POINT",
        b" ",
        b"BETA",
        b"DSK/UNPRIORITIZED",
        &BFRAME,
        b"NONE",
        &OBSRVR,
        ET,
        &mut OCLTID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"OCLTID", OCLTID, b"=", TOTAL1, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Clean up. Unload and delete kernels.", ctx)?;

    //
    // Clean up SPK and DSK files.
    //
    spicelib::SPKUEF(HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
