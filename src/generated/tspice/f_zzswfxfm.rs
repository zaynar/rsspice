//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const CK0: &[u8] = b"ZZSWFXFM_test0.bc";
const CK1: &[u8] = b"ZZSWFXFM_test1.bc";
const FK0: &[u8] = b"ZZSWFXFM_test0.tf";
const PCK0: &[u8] = b"ZZSWFXFM_test0.tpc";
const SCLK0: &[u8] = b"ZZSWFXFM_test0.tsc";
const SPK0: &[u8] = b"ZZSWFXFM_test0.bsp";
const VTIGHT: f64 = 0.00000000000001;
const LBSNGL: i32 = -5;
const MAXBAS: i32 = 15000;
const MAXFRM: i32 = 1013;
const BUFSIZ: i32 = 300;
const FRNMLN: i32 = 32;
const KVNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const TXTSIZ: i32 = 300;
const NCKS: i32 = 3;
const NTICKS: i32 = 1;
const J2CODE: i32 = 1;
const NSTART: i32 = 1;
const SIDLEN: i32 = 40;

struct SaveVars {
    BASBUF: ActualCharArray,
    BASNAM: Vec<u8>,
    CKBASE: ActualCharArray,
    CKFRAM: ActualCharArray,
    FRNAME: Vec<u8>,
    KVNAME: Vec<u8>,
    LABEL: Vec<u8>,
    SEGID: Vec<u8>,
    SHRTMS: Vec<u8>,
    STPBUF: ActualCharArray,
    STRBUF: ActualCharArray,
    TXTBUF: ActualCharArray,
    AVVS: StackArray2D<f64, 3>,
    BEGTIM: f64,
    DSTPBF: ActualArray<f64>,
    DSTRBF: ActualArray<f64>,
    ENDTIM: f64,
    ET: f64,
    ET0: f64,
    ET1: f64,
    IDNT33: StackArray2D<f64, 9>,
    IDNT66: StackArray2D<f64, 36>,
    QUATS: StackArray2D<f64, 4>,
    RMAT: StackArray2D<f64, 9>,
    STARTS: StackArray<f64, 1>,
    TICKS: StackArray<f64, 1>,
    TOL: f64,
    XFORM: StackArray2D<f64, 36>,
    XRMAT: StackArray2D<f64, 9>,
    XXFORM: StackArray2D<f64, 36>,
    BASEID: i32,
    CKHAN: i32,
    CKHAN1: i32,
    CKID: i32,
    FIDLST: ActualArray<i32>,
    FRAMID: i32,
    FREE: i32,
    FRPOOL: ActualArray<i32>,
    HANDLE: i32,
    HDBASE: ActualArray<i32>,
    HDFRAM: ActualArray<i32>,
    IBASBF: ActualArray<i32>,
    N: i32,
    NBASES: i32,
    NFRAMS: i32,
    NI: i32,
    NLINES: i32,
    NREP: i32,
    OUTFRM: i32,
    PRVAT: i32,
    PRVFRM: i32,
    SCLKID: i32,
    ROOM: i32,
    XFRMID: i32,
    FOUND: bool,
    SAMFRM: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASBUF = ActualCharArray::new(FRNMLN, 1..=BUFSIZ);
        let mut BASNAM = vec![b' '; FRNMLN as usize];
        let mut CKBASE = ActualCharArray::new(FRNMLN, 1..=NCKS);
        let mut CKFRAM = ActualCharArray::new(FRNMLN, 1..=NCKS);
        let mut FRNAME = vec![b' '; FRNMLN as usize];
        let mut KVNAME = vec![b' '; KVNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut SHRTMS = vec![b' '; SMSGLN as usize];
        let mut STPBUF = ActualCharArray::new(TIMLEN, 1..=BUFSIZ);
        let mut STRBUF = ActualCharArray::new(TIMLEN, 1..=BUFSIZ);
        let mut TXTBUF = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
        let mut AVVS = StackArray2D::<f64, 3>::new(1..=3, 1..=NTICKS);
        let mut BEGTIM: f64 = 0.0;
        let mut DSTPBF = ActualArray::<f64>::new(1..=(MAXBAS + 1));
        let mut DSTRBF = ActualArray::<f64>::new(1..=(MAXBAS + 1));
        let mut ENDTIM: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut IDNT33 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut IDNT66 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut QUATS = StackArray2D::<f64, 4>::new(0..=3, 1..=NTICKS);
        let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut STARTS = StackArray::<f64, 1>::new(1..=NSTART);
        let mut TICKS = StackArray::<f64, 1>::new(1..=NTICKS);
        let mut TOL: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XRMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XXFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut BASEID: i32 = 0;
        let mut CKHAN: i32 = 0;
        let mut CKHAN1: i32 = 0;
        let mut CKID: i32 = 0;
        let mut FIDLST = ActualArray::<i32>::new(1..=MAXFRM);
        let mut FRAMID: i32 = 0;
        let mut FREE: i32 = 0;
        let mut FRPOOL = ActualArray::<i32>::new(LBSNGL..=MAXFRM);
        let mut HANDLE: i32 = 0;
        let mut HDBASE = ActualArray::<i32>::new(1..=MAXFRM);
        let mut HDFRAM = ActualArray::<i32>::new(1..=MAXFRM);
        let mut IBASBF = ActualArray::<i32>::new(1..=(MAXBAS + 1));
        let mut N: i32 = 0;
        let mut NBASES: i32 = 0;
        let mut NFRAMS: i32 = 0;
        let mut NI: i32 = 0;
        let mut NLINES: i32 = 0;
        let mut NREP: i32 = 0;
        let mut OUTFRM: i32 = 0;
        let mut PRVAT: i32 = 0;
        let mut PRVFRM: i32 = 0;
        let mut SCLKID: i32 = 0;
        let mut ROOM: i32 = 0;
        let mut XFRMID: i32 = 0;
        let mut FOUND: bool = false;
        let mut SAMFRM: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CK_-9999"),
                Val::C(b"CK_-10000"),
                Val::C(b"CK_-10001"),
            ]
            .into_iter();
            CKFRAM
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"GALACTIC"), Val::C(b"FK4"), Val::C(b"J2000")].into_iter();
            CKBASE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BASBUF,
            BASNAM,
            CKBASE,
            CKFRAM,
            FRNAME,
            KVNAME,
            LABEL,
            SEGID,
            SHRTMS,
            STPBUF,
            STRBUF,
            TXTBUF,
            AVVS,
            BEGTIM,
            DSTPBF,
            DSTRBF,
            ENDTIM,
            ET,
            ET0,
            ET1,
            IDNT33,
            IDNT66,
            QUATS,
            RMAT,
            STARTS,
            TICKS,
            TOL,
            XFORM,
            XRMAT,
            XXFORM,
            BASEID,
            CKHAN,
            CKHAN1,
            CKID,
            FIDLST,
            FRAMID,
            FREE,
            FRPOOL,
            HANDLE,
            HDBASE,
            HDFRAM,
            IBASBF,
            N,
            NBASES,
            NFRAMS,
            NI,
            NLINES,
            NREP,
            OUTFRM,
            PRVAT,
            PRVFRM,
            SCLKID,
            ROOM,
            XFRMID,
            FOUND,
            SAMFRM,
        }
    }
}

//$Procedure F_ZZSWFXFM ( ZZSWFXFM tests )
pub fn F_ZZSWFXFM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Lower bound of control area of singly linked list:
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZSWFXFM", ctx)?;

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
    // FRAME_SWITCH0               =  1400000
    // FRAME_1400000_NAME          =  'SWITCH0'
    // FRAME_1400000_CLASS         =  6
    // FRAME_1400000_CLASS_ID      =  1400000
    // FRAME_1400000_CENTER        =  0
    //
    // FRAME_1400000_ALIGNED_WITH  =  ( 'GALACTIC',
    //                                  'B1950',
    //                                  'ECLIPJ2000',
    //                                  'J2000',
    //                                  'GALACTIC',
    //                                  'B1950',
    //                                  'ECLIPJ2000',
    //                                  'J2000'      )
    //
    // FRAME_1400000_START         = ( '2001 AUG 1 00:00:00 TDB',
    //                                 '2001 SEP 1 00:00:00 TDB',
    //                                 '2001 OCT 1 00:00:00 TDB',
    //                                 '2001 NOV 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 FEB 1 00:00:00 TDB',
    //                                 '2000 JAN 1 00:00:00 TDB' )
    //
    // FRAME_1400000_STOP          = ( '2001 SEP 1 00:00:00 TDB',
    //                                 '2001 OCT 1 00:00:00 TDB',
    //                                 '2001 NOV 1 00:00:00 TDB',
    //                                 '2001 DEC 1 00:00:00 TDB',
    //                                 '2000 MAY 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 FEB 1 00:00:00 TDB' )
    //
    // FRAME_SWITCH1               =  1400001
    // FRAME_1400001_NAME          =  'SWITCH1'
    // FRAME_1400001_CLASS         =  6
    // FRAME_1400001_CLASS_ID      =  1400001
    // FRAME_1400001_CENTER        =  0
    //
    // FRAME_1400001_ALIGNED_WITH  =  ( 'J2000',
    //                                  'CK_-10001',
    //                                  'CK_-10000',
    //                                  'CK_-9999',
    //                                  'CK_-10001',
    //                                  'CK_-10000',
    //                                  'CK_-9999'  )
    //
    // FRAME_1400001_START         = ( '1990 JAN 1 00:00:00 TDB',
    //                                 '2001 JAN 1 00:00:00 TDB',
    //                                 '2001 JAN 1 00:00:00 TDB',
    //                                 '2001 JAN 1 00:00:00 TDB',
    //                                 '2000 JAN 1 00:00:00 TDB',
    //                                 '2000 JAN 1 00:00:00 TDB',
    //                                 '2000 JAN 1 00:00:00 TDB' )
    //
    // FRAME_1400001_STOP          = ( '2010 JAN 1 00:00:00 TDB',
    //                                 '2001 APR 1 00:00:00 TDB',
    //                                 '2001 MAR 1 00:00:00 TDB',
    //                                 '2001 FEB 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 FEB 1 00:00:00 TDB' )
    //
    // FRAME_SWITCH2               =  1400002
    // FRAME_1400002_NAME          =  'SWITCH2'
    // FRAME_1400002_CLASS         =  6
    // FRAME_1400002_CLASS_ID      =  1400002
    // FRAME_1400002_CENTER        =  0
    // FRAME_1400002_ALIGNED_WITH  =  ( 'J2000',
    //                                  'CK_-10000',
    //                                  'CK_-9999'  )
    //
    // FRAME_SWITCH3               =  1400003
    // FRAME_1400003_NAME          =  'SWITCH3'
    // FRAME_1400003_CLASS         =  6
    // FRAME_1400003_CLASS_ID      =  1400003
    // FRAME_1400003_CENTER        =  0
    // FRAME_1400003_ALIGNED_WITH  =  ( 'SWITCH0' )
    //
    // FRAME_SWITCH4               =  1400004
    // FRAME_1400004_NAME          =  'SWITCH4'
    // FRAME_1400004_CLASS         =  6
    // FRAME_1400004_CLASS_ID      =  1400004
    // FRAME_1400004_CENTER        =  0
    // FRAME_1400004_ALIGNED_WITH  =  ( 'IAU_EARTH',
    //                                  'TK_0',
    //                                  'SWITCH2',
    //                                  'GSE_SWITCH_0' )
    //
    // FRAME_1400004_START         = ( '2000 JAN 1 00:00:00 TDB',
    //                                 '2000 FEB 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB' )
    //
    // FRAME_1400004_STOP          = ( '2000 FEB 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB',
    //                                 '2000 MAY 1 00:00:00 TDB'  )
    //
    // FRAME_GSE_SWITCH_0              =  1400005
    // FRAME_1400005_NAME              =  'GSE_SWITCH_0'
    // FRAME_1400005_CLASS             =  5
    // FRAME_1400005_CLASS_ID          =  1400005
    // FRAME_1400005_CENTER            =  'EARTH'
    // FRAME_1400005_RELATIVE          =  'J2000'
    // FRAME_1400005_DEF_STYLE         =  'PARAMETERIZED'
    // FRAME_1400005_FAMILY            =  'TWO-VECTOR'
    // FRAME_1400005_PRI_AXIS          =  'Z'
    // FRAME_1400005_PRI_VECTOR_DEF    =  'CONSTANT'
    // FRAME_1400005_PRI_FRAME         =  'MEAN_ECL_EQ_SWITCH_0'
    // FRAME_1400005_PRI_SPEC          =  'RECTANGULAR'
    // FRAME_1400005_PRI_VECTOR        =  ( 0.0 0.0 1.0 )
    // FRAME_1400005_SEC_AXIS          =  'Y'
    // FRAME_1400005_SEC_VECTOR_DEF    =  'OBSERVER_TARGET_VELOCITY'
    // FRAME_1400005_SEC_OBSERVER      =  'EARTH'
    // FRAME_1400005_SEC_TARGET        =  'SUN'
    // FRAME_1400005_SEC_ABCORR        =  'NONE'
    // FRAME_1400005_SEC_FRAME         =  'SWITCH0'
    //
    //
    // FRAME_MEAN_ECL_EQ_SWITCH_0  = 1400006
    // FRAME_1400006_NAME          = 'MEAN_ECL_EQ_SWITCH_0'
    // FRAME_1400006_CLASS         = 5
    // FRAME_1400006_CLASS_ID      = 1400006
    // FRAME_1400006_CENTER        = 'EARTH'
    // FRAME_1400006_RELATIVE      = 'SWITCH0'
    // FRAME_1400006_DEF_STYLE     = 'PARAMETERIZED'
    // FRAME_1400006_FAMILY        = 'MEAN_ECLIPTIC_AND_EQUINOX_OF_DATE'
    // FRAME_1400006_PREC_MODEL    = 'EARTH_IAU_1976'
    // FRAME_1400006_OBLIQ_MODEL   = 'EARTH_IAU_1980'
    // FRAME_1400006_ROTATION_STATE = 'ROTATING'
    //
    //
    // FRAME_TK_0                  = 1400007
    // FRAME_1400007_NAME          = 'TK_0'
    // FRAME_1400007_CLASS         = 4
    // FRAME_1400007_CLASS_ID      = 1400007
    // FRAME_1400007_CENTER        = 499
    // TKFRAME_1400007_RELATIVE    = 'SWITCH0'
    // TKFRAME_1400007_SPEC        = 'MATRIX'
    // TKFRAME_1400007_MATRIX      = ( -1.0   0.0   0.0
    //                                  0.0  -1.0   0.0
    //                                  0.0   0.0   1.0 )
    //
    // FRAME_BOGUS_INERTIAL        =  1400008
    // FRAME_1400008_NAME          =  'BOGUS_INERTIAL'
    // FRAME_1400008_CLASS         =  1
    // FRAME_1400008_CLASS_ID      =  -77
    // FRAME_1400008_CENTER        =  0
    //
    // FRAME_SWITCH_BOGUS_INERTIAL =  1400009
    // FRAME_1400009_NAME          =  'SWITCH_BOGUS_INERTIAL'
    // FRAME_1400009_CLASS         =  6
    // FRAME_1400009_CLASS_ID      =  1400009
    // FRAME_1400009_CENTER        =  0
    // FRAME_1400009_ALIGNED_WITH  =  ( 1400008 )
    //
    // FRAME_SWITCH_SINGLE_BASE    =  1400050
    // FRAME_1400050_NAME          =  'SWITCH_SINGLE_BASE'
    // FRAME_1400050_CLASS         =  6
    // FRAME_1400050_CLASS_ID      =  1400050
    // FRAME_1400050_CENTER        =  -9
    // FRAME_1400050_ALIGNED_WITH  =  ( 'CK_-9999' )
    // FRAME_1400050_START         = '2000 JAN 1 00:00:00 TDB'
    // FRAME_1400050_STOP          = '2000 JAN 2 00:00:00 TDB'
    //
    // FRAME_SWITCH_BINARY_GAP     =  1400051
    // FRAME_1400051_NAME          =  'SWITCH_BINARY_GAP'
    // FRAME_1400051_CLASS         =  6
    // FRAME_1400051_CLASS_ID      =  1400051
    // FRAME_1400051_CENTER        =  -9
    // FRAME_1400051_ALIGNED_WITH  =  ( 'CK_-9999',
    //                                  'CK_-8888' )
    // FRAME_1400051_START         = ( '1980 JAN 1 00:00:00 TDB',
    //                                 '2011 SEP 9 01:46:40 TDB' )
    // FRAME_1400051_STOP          = ( '2011 SEP 9 01:46:40 TDB',
    //                                 '2012 JAN 1 00:00:00 TDB' )
    //
    // FRAME_CK_-8888              = -8888000
    // FRAME_-8888000_NAME         = 'CK_-8888'
    // FRAME_-8888000_CLASS        = 3
    // FRAME_-8888000_CLASS_ID     = -8888
    // FRAME_-8888000_CENTER       = -9
    // CK_-8888_SCLK               = -9
    //
    //
    testutil::BEGDAT(&mut save.TXTBUF[1]);

    fstr::assign(
        save.TXTBUF.get_mut(2),
        b"      FRAME_SWITCH0               =  1400000",
    );
    fstr::assign(
        save.TXTBUF.get_mut(3),
        b"      FRAME_1400000_NAME          =  \'SWITCH0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(4),
        b"      FRAME_1400000_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(5),
        b"      FRAME_1400000_CLASS_ID      =  1400000",
    );
    fstr::assign(
        save.TXTBUF.get_mut(6),
        b"      FRAME_1400000_CENTER        =  0",
    );
    fstr::assign(save.TXTBUF.get_mut(7), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(8),
        b"      FRAME_1400000_ALIGNED_WITH  =  ( \'GALACTIC\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(9),
        b"                                       \'B1950\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(10),
        b"                                       \'ECLIPJ2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(11),
        b"                                       \'J2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(12),
        b"                                       \'GALACTIC\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(13),
        b"                                       \'B1950\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(14),
        b"                                       \'ECLIPJ2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(15),
        b"                                       \'J2000\'      )",
    );
    fstr::assign(save.TXTBUF.get_mut(16), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(17),
        b"      FRAME_1400000_START         = ( \'2001 AUG 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(18),
        b"                                      \'2001 SEP 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(19),
        b"                                      \'2001 OCT 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(20),
        b"                                      \'2001 NOV 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(21),
        b"                                      \'2000 APR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(22),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(23),
        b"                                      \'2000 FEB 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(24),
        b"                                      \'2000 JAN 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(25), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(26),
        b"      FRAME_1400000_STOP          = ( \'2001 SEP 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(27),
        b"                                      \'2001 OCT 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(28),
        b"                                      \'2001 NOV 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(29),
        b"                                      \'2001 DEC 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(30),
        b"                                      \'2000 MAY 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(31),
        b"                                      \'2000 APR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(32),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(33),
        b"                                      \'2000 FEB 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(34), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(35),
        b"      FRAME_SWITCH1               =  1400001",
    );
    fstr::assign(
        save.TXTBUF.get_mut(36),
        b"      FRAME_1400001_NAME          =  \'SWITCH1\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(37),
        b"      FRAME_1400001_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(38),
        b"      FRAME_1400001_CLASS_ID      =  1400001",
    );
    fstr::assign(
        save.TXTBUF.get_mut(39),
        b"      FRAME_1400001_CENTER        =  0",
    );
    fstr::assign(save.TXTBUF.get_mut(40), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(41),
        b"      FRAME_1400001_ALIGNED_WITH  =  ( \'J2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(42),
        b"                                       \'CK_-10001\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(43),
        b"                                       \'CK_-10000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(44),
        b"                                       \'CK_-9999\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(45),
        b"                                       \'CK_-10001\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(46),
        b"                                       \'CK_-10000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(47),
        b"                                       \'CK_-9999\'  )",
    );
    fstr::assign(save.TXTBUF.get_mut(48), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(49),
        b"      FRAME_1400001_START         = ( \'1990 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(50),
        b"                                      \'2001 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(51),
        b"                                      \'2001 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(52),
        b"                                      \'2001 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(53),
        b"                                      \'2000 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(54),
        b"                                      \'2000 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(55),
        b"                                      \'2000 JAN 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(56), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(57),
        b"      FRAME_1400001_STOP          = ( \'2010 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(58),
        b"                                      \'2001 APR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(59),
        b"                                      \'2001 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(60),
        b"                                      \'2001 FEB 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(61),
        b"                                      \'2000 APR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(62),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(63),
        b"                                      \'2000 FEB 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(64), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(65),
        b"      FRAME_SWITCH2               =  1400002",
    );
    fstr::assign(
        save.TXTBUF.get_mut(66),
        b"      FRAME_1400002_NAME          =  \'SWITCH2\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(67),
        b"      FRAME_1400002_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(68),
        b"      FRAME_1400002_CLASS_ID      =  1400002",
    );
    fstr::assign(
        save.TXTBUF.get_mut(69),
        b"      FRAME_1400002_CENTER        =  0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(70),
        b"      FRAME_1400002_ALIGNED_WITH  =  ( \'J2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(71),
        b"                                       \'CK_-10000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(72),
        b"                                       \'CK_-9999\'  )",
    );
    fstr::assign(save.TXTBUF.get_mut(73), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(74),
        b"      FRAME_SWITCH3               =  1400003",
    );
    fstr::assign(
        save.TXTBUF.get_mut(75),
        b"      FRAME_1400003_NAME          =  \'SWITCH3\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(76),
        b"      FRAME_1400003_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(77),
        b"      FRAME_1400003_CLASS_ID      =  1400003",
    );
    fstr::assign(
        save.TXTBUF.get_mut(78),
        b"      FRAME_1400003_CENTER        =  0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(79),
        b"      FRAME_1400003_ALIGNED_WITH  =  ( \'SWITCH0\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(80), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(81),
        b"      FRAME_SWITCH4               =  1400004",
    );
    fstr::assign(
        save.TXTBUF.get_mut(82),
        b"      FRAME_1400004_NAME          =  \'SWITCH4\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(83),
        b"      FRAME_1400004_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(84),
        b"      FRAME_1400004_CLASS_ID      =  1400004",
    );
    fstr::assign(
        save.TXTBUF.get_mut(85),
        b"      FRAME_1400004_CENTER        =  0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(86),
        b"      FRAME_1400004_ALIGNED_WITH  =  ( \'IAU_EARTH\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(87),
        b"                                       \'TK_0\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(88),
        b"                                       \'SWITCH2\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(89),
        b"                                       \'GSE_SWITCH_0\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(90), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(91),
        b"      FRAME_1400004_START         = ( \'2000 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(92),
        b"                                      \'2000 FEB 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(93),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(94),
        b"                                      \'2000 APR 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(95), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(96),
        b"      FRAME_1400004_STOP          = ( \'2000 FEB 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(97),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(98),
        b"                                      \'2000 APR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(99),
        b"                                      \'2000 MAY 1 00:00:00 TDB\'  )",
    );
    fstr::assign(save.TXTBUF.get_mut(100), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(101),
        b"      FRAME_GSE_SWITCH_0              =  1400005",
    );
    fstr::assign(
        save.TXTBUF.get_mut(102),
        b"      FRAME_1400005_NAME              =  \'GSE_SWITCH_0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(103),
        b"      FRAME_1400005_CLASS             =  5",
    );
    fstr::assign(
        save.TXTBUF.get_mut(104),
        b"      FRAME_1400005_CLASS_ID          =  1400005",
    );
    fstr::assign(
        save.TXTBUF.get_mut(105),
        b"      FRAME_1400005_CENTER            =  \'EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(106),
        b"      FRAME_1400005_RELATIVE          =  \'J2000\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(107),
        b"      FRAME_1400005_DEF_STYLE         =  \'PARAMETERIZED\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(108),
        b"      FRAME_1400005_FAMILY            =  \'TWO-VECTOR\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(109),
        b"      FRAME_1400005_PRI_AXIS          =  \'Z\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(110),
        b"      FRAME_1400005_PRI_VECTOR_DEF    =  \'CONSTANT\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(111),
        b"      FRAME_1400005_PRI_FRAME         =  \'MEAN_ECL_EQ_SWITCH_0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(112),
        b"      FRAME_1400005_PRI_SPEC          =  \'RECTANGULAR\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(113),
        b"      FRAME_1400005_PRI_VECTOR        =  ( 0.0 0.0 1.0 )",
    );
    fstr::assign(
        save.TXTBUF.get_mut(114),
        b"      FRAME_1400005_SEC_AXIS          =  \'Y\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(115),
        b"      FRAME_1400005_SEC_VECTOR_DEF    =  \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(116),
        b"      FRAME_1400005_SEC_OBSERVER      =  \'EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(117),
        b"      FRAME_1400005_SEC_TARGET        =  \'SUN\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(118),
        b"      FRAME_1400005_SEC_ABCORR        =  \'NONE\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(119),
        b"      FRAME_1400005_SEC_FRAME         =  \'SWITCH0\'",
    );
    fstr::assign(save.TXTBUF.get_mut(120), b" ");
    fstr::assign(save.TXTBUF.get_mut(121), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(122),
        b"      FRAME_MEAN_ECL_EQ_SWITCH_0  = 1400006",
    );
    fstr::assign(
        save.TXTBUF.get_mut(123),
        b"      FRAME_1400006_NAME          = \'MEAN_ECL_EQ_SWITCH_0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(124),
        b"      FRAME_1400006_CLASS         = 5",
    );
    fstr::assign(
        save.TXTBUF.get_mut(125),
        b"      FRAME_1400006_CLASS_ID      = 1400006",
    );
    fstr::assign(
        save.TXTBUF.get_mut(126),
        b"      FRAME_1400006_CENTER        = \'EARTH\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(127),
        b"      FRAME_1400006_RELATIVE      = \'SWITCH0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(128),
        b"      FRAME_1400006_DEF_STYLE     = \'PARAMETERIZED\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(129),
        b"      FRAME_1400006_FAMILY        = \'MEAN_ECLIPTIC_AND_EQUINOX_OF_DATE\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(130),
        b"      FRAME_1400006_PREC_MODEL    = \'EARTH_IAU_1976\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(131),
        b"      FRAME_1400006_OBLIQ_MODEL   = \'EARTH_IAU_1980\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(132),
        b"      FRAME_1400006_ROTATION_STATE = \'ROTATING\'",
    );
    fstr::assign(save.TXTBUF.get_mut(133), b" ");
    fstr::assign(save.TXTBUF.get_mut(134), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(135),
        b"      FRAME_TK_0                  = 1400007",
    );
    fstr::assign(
        save.TXTBUF.get_mut(136),
        b"      FRAME_1400007_NAME          = \'TK_0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(137),
        b"      FRAME_1400007_CLASS         = 4",
    );
    fstr::assign(
        save.TXTBUF.get_mut(138),
        b"      FRAME_1400007_CLASS_ID      = 1400007",
    );
    fstr::assign(
        save.TXTBUF.get_mut(139),
        b"      FRAME_1400007_CENTER        = 499",
    );
    fstr::assign(
        save.TXTBUF.get_mut(140),
        b"      TKFRAME_1400007_RELATIVE    = \'SWITCH0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(141),
        b"      TKFRAME_1400007_SPEC        = \'MATRIX\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(142),
        b"      TKFRAME_1400007_MATRIX      = ( -1.0   0.0   0.0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(143),
        b"                                       0.0  -1.0   0.0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(144),
        b"                                       0.0   0.0   1.0 )",
    );
    fstr::assign(save.TXTBUF.get_mut(145), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(146),
        b"      FRAME_BOGUS_INERTIAL        =  1400008",
    );
    fstr::assign(
        save.TXTBUF.get_mut(147),
        b"      FRAME_1400008_NAME          =  \'BOGUS_INERTIAL\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(148),
        b"      FRAME_1400008_CLASS         =  1",
    );
    fstr::assign(
        save.TXTBUF.get_mut(149),
        b"      FRAME_1400008_CLASS_ID      =  -77",
    );
    fstr::assign(
        save.TXTBUF.get_mut(150),
        b"      FRAME_1400008_CENTER        =  0",
    );
    fstr::assign(save.TXTBUF.get_mut(151), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(152),
        b"      FRAME_SWITCH_BOGUS_INERTIAL =  1400009",
    );
    fstr::assign(
        save.TXTBUF.get_mut(153),
        b"      FRAME_1400009_NAME          =  \'SWITCH_BOGUS_INERTIAL\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(154),
        b"      FRAME_1400009_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(155),
        b"      FRAME_1400009_CLASS_ID      =  1400009",
    );
    fstr::assign(
        save.TXTBUF.get_mut(156),
        b"      FRAME_1400009_CENTER        =  0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(157),
        b"      FRAME_1400009_ALIGNED_WITH  =  ( 1400008 )",
    );
    fstr::assign(save.TXTBUF.get_mut(158), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(159),
        b"      FRAME_SWITCH_SINGLE_BASE    =  1400050",
    );
    fstr::assign(
        save.TXTBUF.get_mut(160),
        b"      FRAME_1400050_NAME          =  \'SWITCH_SINGLE_BASE\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(161),
        b"      FRAME_1400050_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(162),
        b"      FRAME_1400050_CLASS_ID      =  1400050",
    );
    fstr::assign(
        save.TXTBUF.get_mut(163),
        b"      FRAME_1400050_CENTER        =  -9",
    );
    fstr::assign(
        save.TXTBUF.get_mut(164),
        b"      FRAME_1400050_ALIGNED_WITH  =  ( \'CK_-9999\' )",
    );
    fstr::assign(
        save.TXTBUF.get_mut(165),
        b"      FRAME_1400050_START         = \'2000 JAN 1 00:00:00 TDB\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(166),
        b"      FRAME_1400050_STOP          = \'2000 JAN 2 00:00:00 TDB\'",
    );
    fstr::assign(save.TXTBUF.get_mut(167), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(168),
        b"      FRAME_SWITCH_BINARY_GAP     =  1400051",
    );
    fstr::assign(
        save.TXTBUF.get_mut(169),
        b"      FRAME_1400051_NAME          =  \'SWITCH_BINARY_GAP\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(170),
        b"      FRAME_1400051_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(171),
        b"      FRAME_1400051_CLASS_ID      =  1400051",
    );
    fstr::assign(
        save.TXTBUF.get_mut(172),
        b"      FRAME_1400051_CENTER        =  -9",
    );
    fstr::assign(
        save.TXTBUF.get_mut(173),
        b"      FRAME_1400051_ALIGNED_WITH  =  ( \'CK_-9999\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(174),
        b"                                       \'CK_-8888\'     )",
    );
    fstr::assign(
        save.TXTBUF.get_mut(175),
        b"      FRAME_1400051_START         = ( \'1980 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(176),
        b"                                      \'2011 SEP 9 01:46:40 TDB\' )",
    );
    fstr::assign(
        save.TXTBUF.get_mut(177),
        b"      FRAME_1400051_STOP          = ( \'2011 SEP 9 01:46:40 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(178),
        b"                                      \'2012 JAN 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(179), b" ");

    fstr::assign(
        save.TXTBUF.get_mut(180),
        b"      FRAME_CK_-8888              = -8888000",
    );
    fstr::assign(
        save.TXTBUF.get_mut(181),
        b"      FRAME_-8888000_NAME         = \'CK_-8888\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(182),
        b"      FRAME_-8888000_CLASS        = 3",
    );
    fstr::assign(
        save.TXTBUF.get_mut(183),
        b"      FRAME_-8888000_CLASS_ID     = -8888",
    );
    fstr::assign(
        save.TXTBUF.get_mut(184),
        b"      FRAME_-8888000_CENTER       = -9",
    );
    fstr::assign(
        save.TXTBUF.get_mut(185),
        b"      CK_-8888_SCLK               = -9",
    );

    testutil::BEGTXT(&mut save.TXTBUF[186]);
    fstr::assign(save.TXTBUF.get_mut(187), b" ");

    if spicelib::EXISTS(FK0, ctx)? {
        testutil::KILFIL(FK0, ctx)?;
    }

    spicelib::KCLEAR(ctx)?;

    save.NLINES = 187;
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
    // Create and load CK and SCLK kernel. The SCLK kernel includes
    // CK frame specifications.
    //
    if spicelib::EXISTS(CK0, ctx)? {
        testutil::KILFIL(CK0, ctx)?;
    }

    if spicelib::EXISTS(SCLK0, ctx)? {
        testutil::KILFIL(SCLK0, ctx)?;
    }

    testutil::TSTCK3(CK0, SCLK0, false, false, true, &mut save.CKHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(CK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SCLK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Also load LSK to support time conversions.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create and load CK with gaps. This CK uses the same SCLK
    // as CK0.
    //
    if spicelib::EXISTS(CK1, ctx)? {
        testutil::KILFIL(CK1, ctx)?;
    }

    spicelib::CKOPN(CK1, CK1, 0, &mut save.CKHAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CKID = -8888;
    fstr::assign(&mut save.SEGID, b"Type 3 segment with gap for -8888");
    save.SCLKID = -9;

    //
    // The start epoch is the end time of the segments of CK0.
    //
    spicelib::STR2ET(b"2011 JAN 01 01:46:40 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::SCE2C(save.SCLKID, save.ET0, &mut save.BEGTIM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2015 JAN 01 00:00:00 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::SCE2C(save.SCLKID, save.ET1, &mut save.ENDTIM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TICKS[1] = ((save.BEGTIM + save.ENDTIM) / 2 as f64);
    save.STARTS[1] = save.TICKS[1];

    spicelib::VPACK(0.0, 0.0, 1.0, save.AVVS.subarray_mut([1, 1]));

    spicelib::CLEARD(3, save.QUATS.subarray_mut([1, 1]));
    save.QUATS[[0, 1]] = 1.0;

    spicelib::CKW03(
        save.CKHAN1,
        save.BEGTIM,
        save.ENDTIM,
        save.CKID,
        b"J2000",
        true,
        &save.SEGID,
        NTICKS,
        save.TICKS.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        NSTART,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFCLS(save.CKHAN1, ctx)?;
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
    testutil::TCASE(
        b"Perform state transformation lookups for each base frame of frame SWITCH0",
        ctx,
    )?;

    save.NI = 8;

    spicelib::GCPOOL(
        b"FRAME_1400000_ALIGNED_WITH",
        1,
        save.NI,
        &mut save.N,
        save.BASBUF.as_arg_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"BASBUF FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::GCPOOL(
        b"FRAME_1400000_START",
        1,
        save.NI,
        &mut save.N,
        save.STRBUF.as_arg_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"STRBUF FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::GCPOOL(
        b"FRAME_1400000_STOP",
        1,
        save.NI,
        &mut save.N,
        save.STPBUF.as_arg_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"STPBUF FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We expect to get the identity matrix for each call.
    //
    for I in 1..=save.NI {
        spicelib::STR2ET(&save.STRBUF[I], &mut save.ET0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::STR2ET(&save.STPBUF[I], &mut save.ET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Use as the lookup time the midpoint of the Ith interval.
        //
        save.ET = ((save.ET0 + save.ET1) / 2 as f64);

        //
        // First try a direct call to ZZSWFXFM, then call SXFORM.
        //
        spicelib::NAMFRM(b"SWITCH0", &mut save.FRAMID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZSWFXFM(
            save.FRAMID,
            save.ET,
            6,
            save.XFORM.as_slice_mut(),
            &mut save.OUTFRM,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // The output frame is J2000 for inertial base frames.
        //
        save.XFRMID = J2CODE;
        testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", save.XFRMID, 0, OK, ctx)?;

        fstr::assign(&mut save.BASNAM, save.BASBUF.get(I));
        testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"!=", b" ", OK, ctx)?;

        spicelib::SXFORM(
            &save.BASNAM,
            b"J2000",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;

        testutil::CHCKAD(
            b"SWITCH0 to J2000",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Use SXFORM for the remaining transformations.
        //
        // Repeat the previous transformation, to test use of saved
        // values.
        //
        spicelib::SXFORM(
            &save.BASNAM,
            b"J2000",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;

        testutil::CHCKAD(
            b"SWITCH0 to J2000",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Verify that the switch frame is aligned with its base.
        //
        spicelib::SXFORM(
            b"SWITCH0",
            &save.BASNAM,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;

        testutil::CHCKAD(
            &fstr::concat(b"SWITCH0 to ", &save.BASNAM),
            save.XFORM.as_slice(),
            b"~",
            save.IDNT66.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::SXFORM(
            &save.BASNAM,
            b"SWITCH0",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &fstr::concat(b"SWITCH0 from ", &save.BASNAM),
            save.XFORM.as_slice(),
            b"~",
            save.IDNT66.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform state transformation lookups for frame SWITCH0, using times outside of the coverage intervals.", ctx)?;
    //
    // ET is less than earliest start.
    //
    spicelib::STR2ET(&save.STRBUF[save.NI], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = (save.ET0 - 1.0);

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (1)", save.FOUND, false, OK, ctx)?;

    //
    // ET is greater than 5th stop and less than 1st start.
    //
    spicelib::STR2ET(&save.STPBUF[5], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = (save.ET1 + 1.0);

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (2)", save.FOUND, false, OK, ctx)?;

    //
    // ET is past last stop time, which belongs to the 4th interval.
    //
    spicelib::STR2ET(&save.STPBUF[4], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = (save.ET1 + 1.0);

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (3)", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform state transformation lookups for frame SWITCH0, using the exact interval time bounds as request times.", ctx)?;

    //
    // Because the interval endpoint times are defined by strings,
    // expect to be able to duplicate the exact stored time bounds.
    // We'll use those bounds as the input times. We can't expect
    // transformation matrices to match expected values, however.
    //
    // Use the buffered data from the previous test case.
    //
    for I in 1..=save.NI {
        spicelib::STR2ET(&save.STRBUF[I], &mut save.ET0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::STR2ET(&save.STPBUF[I], &mut save.ET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::NAMFRM(b"SWITCH0", &mut save.FRAMID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.BASNAM, save.BASBUF.get(I));
        testutil::CHCKSC(b"(ET0) BASNAM", &save.BASNAM, b"!=", b" ", OK, ctx)?;

        save.TOL = VTIGHT;

        //
        // For the first 4 intervals, which have increasing time order,
        // and for the last one, which is the last of a set having
        // decreasing time order, the left endpoint of each interval is
        // not masked by a higher-priority interval.
        //
        if ((I <= 4) || (I == save.NI)) {
            //
            // Get state transformation to J2000 at the left endpoint.
            //
            spicelib::SXFORM(
                b"SWITCH0",
                b"J2000",
                save.ET0,
                save.XFORM.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // The expected result maps from the Ith base to J2000.
            //
            spicelib::SXFORM(
                &save.BASNAM,
                b"J2000",
                save.ET0,
                save.XXFORM.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = VTIGHT;

            fstr::assign(&mut save.LABEL, b"XFORM: SWITCH0 to J2000 (#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XFORM.as_slice(),
                b"~",
                save.XXFORM.as_slice(),
                36,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Repeat the tests for position transformations.
            //
            spicelib::PXFORM(
                b"SWITCH0",
                b"J2000",
                save.ET0,
                save.RMAT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // The expected result maps from the Ith base to J2000.
            //
            spicelib::PXFORM(
                &save.BASNAM,
                b"J2000",
                save.ET0,
                save.XRMAT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = VTIGHT;

            fstr::assign(&mut save.LABEL, b"RMAT: SWITCH0 to J2000 (#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.RMAT.as_slice(),
                b"~",
                save.XRMAT.as_slice(),
                9,
                save.TOL,
                OK,
                ctx,
            )?;
        }

        if (I >= 4) {
            //
            // For the last 4 intervals, which are in decreasing time
            // order, and for the 4th one, which is the last of a set in
            // increasing time order, the right endpoint of each interval
            // is not masked by a higher-priority interval.

            // Get state transformation to J2000 at the right endpoint.
            //
            spicelib::SXFORM(
                b"SWITCH0",
                b"J2000",
                save.ET1,
                save.XFORM.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // The expected result maps from the Ith base to J2000.
            //
            spicelib::SXFORM(
                &save.BASNAM,
                b"J2000",
                save.ET1,
                save.XXFORM.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = VTIGHT;

            fstr::assign(&mut save.LABEL, b"XFORM: SWITCH0 to J2000 (#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XFORM.as_slice(),
                b"~",
                save.XXFORM.as_slice(),
                36,
                save.TOL,
                OK,
                ctx,
            )?;
            //
            // Repeat the tests for position transformations.
            //
            spicelib::PXFORM(
                b"SWITCH0",
                b"J2000",
                save.ET1,
                save.RMAT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // The expected result maps from the Ith base to J2000.
            //
            spicelib::PXFORM(
                &save.BASNAM,
                b"J2000",
                save.ET1,
                save.XRMAT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = VTIGHT;

            fstr::assign(&mut save.LABEL, b"RMAT: SWITCH0 to J2000 (#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.RMAT.as_slice(),
                b"~",
                save.XRMAT.as_slice(),
                9,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Touch the kernel pool, then perform position transformation lookups for each base frame of frame SWITCH0.", ctx)?;

    spicelib::PDPOOL(b"X", 1, &[0.0], ctx)?;

    //
    // We expect to get the identity matrix for each call.
    //
    for I in 1..=save.NI {
        spicelib::STR2ET(&save.STRBUF[I], &mut save.ET0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::STR2ET(&save.STPBUF[I], &mut save.ET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.ET = ((save.ET0 + save.ET1) / 2 as f64);

        fstr::assign(&mut save.BASNAM, save.BASBUF.get(I));
        testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"!=", b" ", OK, ctx)?;

        spicelib::PXFORM(
            b"SWITCH0",
            &save.BASNAM,
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &fstr::concat(b"SWITCH0 to", &save.BASNAM),
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(
            &save.BASNAM,
            b"SWITCH0",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &fstr::concat(b"SWITCH0 from", &save.BASNAM),
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Perform position transformation lookups for each base frame of frame SWITCH0",
        ctx,
    )?;

    //
    // We expect to get the identity matrix for each call.
    //
    for I in 1..=save.NI {
        spicelib::STR2ET(&save.STRBUF[I], &mut save.ET0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::STR2ET(&save.STPBUF[I], &mut save.ET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.ET = ((save.ET0 + save.ET1) / 2 as f64);

        fstr::assign(&mut save.BASNAM, save.BASBUF.get(I));
        testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"!=", b" ", OK, ctx)?;

        spicelib::PXFORM(
            b"SWITCH0",
            &save.BASNAM,
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &fstr::concat(b"SWITCH0 to", &save.BASNAM),
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(
            &save.BASNAM,
            b"SWITCH0",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &fstr::concat(b"SWITCH0 from", &save.BASNAM),
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Perform state and position transformation lookups for each base frame of frame SWITCH1",
        ctx,
    )?;

    save.ROOM = BUFSIZ;

    spicelib::GCPOOL(
        b"FRAME_1400001_ALIGNED_WITH",
        1,
        save.ROOM,
        &mut save.N,
        save.BASBUF.as_arg_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"BASBUF FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"N (number of bases)", save.N, b"=", 7, 0, OK, ctx)?;

    spicelib::GCPOOL(
        b"FRAME_1400001_START",
        1,
        save.ROOM,
        &mut save.N,
        save.STRBUF.as_arg_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"STRBUF FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::GCPOOL(
        b"FRAME_1400001_STOP",
        1,
        save.ROOM,
        &mut save.N,
        save.STPBUF.as_arg_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"STPBUF FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Interval for last base:
    //
    spicelib::STR2ET(&save.STRBUF[7], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STPBUF[7], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    //
    // The last base is a CK frame having the base frame GALACTIC.
    //
    fstr::assign(&mut save.FRNAME, save.CKFRAM.get(1));
    fstr::assign(&mut save.BASNAM, save.CKBASE.get(1));

    spicelib::SXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(7) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.XXFORM.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(7) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.XRMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Interval for sixth base. Avoid the 7th interval.
    //
    spicelib::STR2ET(&save.STPBUF[7], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STPBUF[6], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    //
    // The sixth base is a CK frame having the base frame FK4.
    //
    fstr::assign(&mut save.FRNAME, save.CKFRAM.get(2));
    fstr::assign(&mut save.BASNAM, save.CKBASE.get(2));

    spicelib::SXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(6) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.XXFORM.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(6) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.XRMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Interval for 5th base. Skip over the 6th interval:
    //
    spicelib::STR2ET(&save.STPBUF[6], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STPBUF[5], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    //
    // The 5th base is a CK frame having the base frame J2000.
    //
    fstr::assign(&mut save.FRNAME, save.CKFRAM.get(3));
    fstr::assign(&mut save.BASNAM, save.CKBASE.get(3));

    spicelib::SXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(5) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.XXFORM.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(5) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.XRMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Interval for fourth base.
    //
    spicelib::STR2ET(&save.STRBUF[4], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STPBUF[4], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    //
    // The fourth base is a CK frame having the base frame GALACTIC.
    //
    fstr::assign(&mut save.FRNAME, save.CKFRAM.get(1));
    fstr::assign(&mut save.BASNAM, save.CKBASE.get(1));

    spicelib::SXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(4) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.XXFORM.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(4) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.XRMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Interval for the third base. Skip over the interval for the
    // fourth base.

    spicelib::STR2ET(&save.STPBUF[4], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STPBUF[3], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    //
    // The third base is a CK frame having the base frame FK4.
    //
    fstr::assign(&mut save.FRNAME, save.CKFRAM.get(2));
    fstr::assign(&mut save.BASNAM, save.CKBASE.get(2));

    spicelib::SXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(3) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.XXFORM.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(3) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.XRMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Interval for the second base. Skip over the interval for the
    // third base.

    spicelib::STR2ET(&save.STPBUF[3], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STPBUF[2], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    //
    // The second base is a CK frame having the base frame J2000.
    //
    fstr::assign(&mut save.FRNAME, save.CKFRAM.get(3));
    fstr::assign(&mut save.BASNAM, save.CKBASE.get(3));

    spicelib::SXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(2) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.XXFORM.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(
        b"SWITCH1",
        &save.BASNAM,
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        &save.FRNAME,
        &save.BASNAM,
        save.ET,
        save.XRMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(2) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.XRMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // First interval for the first base. Use as a stop time the
    // common start time of the other intervals.
    //
    spicelib::STR2ET(&save.STRBUF[1], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STRBUF[7], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    //
    // The first base is J2000.
    //
    spicelib::SXFORM(
        b"SWITCH1",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(1A) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.IDNT66.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(b"SWITCH1", b"J2000", save.ET, save.RMAT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(1A) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.IDNT33.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Second interval for the first base. Use as a start time the
    // latest stop time of intervals ending in 2000. Use as a stop time
    // the earliest start time of intervals starting in 2001.
    //
    spicelib::STR2ET(&save.STPBUF[5], &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.STPBUF[2], &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    spicelib::SXFORM(
        b"SWITCH1",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(1B) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.IDNT66.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(b"SWITCH1", b"J2000", save.ET, save.RMAT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(1B) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.IDNT33.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Third interval for the first base. Use as a start time the
    // latest stop time of the other intervals.
    //
    spicelib::STR2ET(&save.STPBUF[2], &mut save.ET0, ctx)?;
    spicelib::STR2ET(&save.STPBUF[1], &mut save.ET1, ctx)?;

    save.ET = ((save.ET0 + save.ET1) / 2 as f64);

    spicelib::SXFORM(
        b"SWITCH1",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(1C) XFORM",
        save.XFORM.as_slice(),
        b"=",
        save.IDNT66.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::PXFORM(b"SWITCH1", b"J2000", save.ET, save.RMAT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(1C) RMAT",
        save.RMAT.as_slice(),
        b"=",
        save.IDNT33.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform state and position transformation lookups for a switch frame that doesn\'t have time intervals associated with its bases.", ctx)?;

    save.NREP = 4;

    for I in 1..=save.NREP {
        //
        // Set ET to a value covered by the first base frame, which is a
        // CK frame.
        //
        save.ET = 0.0;

        spicelib::SXFORM(
            b"SWITCH2",
            b"GALACTIC",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SXFORM(
            b"CK_-9999",
            b"GALACTIC",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"(1A) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(
            b"SWITCH2",
            b"GALACTIC",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PXFORM(
            b"CK_-9999",
            b"GALACTIC",
            save.ET,
            save.XRMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(1A) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.XRMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Set ET to a value not covered by a CK.
        //
        spicelib::STR2ET(b"2050 JAN 1", &mut save.ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SXFORM(
            b"SWITCH2",
            b"J2000",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"(1B) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.IDNT66.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(b"SWITCH2", b"J2000", save.ET, save.RMAT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(1B) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        if (I == 2) {
            //
            // Touch the kernel pool by re-loading FK0.
            //
            spicelib::FURNSH(FK0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform state and position transformation lookups for a switch frame that has a switch frame as its base. This frame has no associated time intervals.", ctx)?;

    save.NREP = 4;

    for I in 1..=save.NREP {
        //
        // Set ET to a value covered by the first base frame, which is a
        // CK frame.
        //
        save.ET = 0.0;

        spicelib::SXFORM(
            b"SWITCH3",
            b"SWITCH0",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"(1A) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.IDNT66.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(
            b"SWITCH3",
            b"SWITCH0",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(1A) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        if (I == 2) {
            //
            // Touch the kernel pool by re-loading FK0.
            //
            spicelib::FURNSH(FK0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform state and position transformation lookups for a switch frame that has as its bases a PCK frame, a TK frame, a switch frame, and a dynamic frame. This frame has associated time intervals.", ctx)?;

    save.NREP = 4;

    //
    // Set ET to a value covered by the first base frame, which is a
    // PCK frame.
    //
    spicelib::STR2ET(b"2000 JAN 2 00:00:00 TDB", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NREP {
        spicelib::SXFORM(
            b"SWITCH4",
            b"J2000",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SXFORM(
            b"IAU_EARTH",
            b"J2000",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"(A) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(b"SWITCH4", b"J2000", save.ET, save.RMAT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PXFORM(
            b"IAU_EARTH",
            b"J2000",
            save.ET,
            save.XRMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(A) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.XRMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        if (I == 2) {
            //
            // Touch the kernel pool by re-loading FK0.
            //
            spicelib::FURNSH(FK0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Set ET to a value covered by the second base frame, which is a
    // TK frame.
    //
    spicelib::STR2ET(b"2000 FEB 2 00:00:00 TDB", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NREP {
        spicelib::SXFORM(
            b"SWITCH4",
            b"J2000",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SXFORM(b"TK_0", b"J2000", save.ET, save.XXFORM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"(B) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(b"SWITCH4", b"J2000", save.ET, save.RMAT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PXFORM(b"TK_0", b"J2000", save.ET, save.XRMAT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(B) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.XRMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        if (I == 2) {
            //
            // Touch the kernel pool by re-loading FK0.
            //
            spicelib::FURNSH(FK0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Set ET to a value covered by the third base frame, which is a
    // switch frame.
    //
    spicelib::STR2ET(b"2000 MAR 2 00:00:00 TDB", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NREP {
        spicelib::SXFORM(
            b"SWITCH4",
            b"SWITCH2",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"(1C) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.IDNT66.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::SXFORM(
            b"SWITCH4",
            b"GALACTIC",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SXFORM(
            b"CK_-9999",
            b"GALACTIC",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(2C) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::SXFORM(
            b"SWITCH4",
            b"SWITCH2",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(3C) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.IDNT66.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(
            b"SWITCH4",
            b"SWITCH2",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(1C) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(
            b"SWITCH4",
            b"GALACTIC",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PXFORM(
            b"CK_-9999",
            b"GALACTIC",
            save.ET,
            save.XRMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(2C) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.XRMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        if (I == 2) {
            //
            // Touch the kernel pool by re-loading FK0.
            //
            spicelib::FURNSH(FK0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Set ET to a value covered by the fourth base frame, which is a
    // dynamic frame.
    //
    spicelib::STR2ET(b"2000 APR 2 00:00:00 TDB", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NREP {
        spicelib::SXFORM(
            b"SWITCH4",
            b"GSE_SWITCH_0",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"(1C) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.IDNT66.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::SXFORM(
            b"SWITCH4",
            b"J2000",
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SXFORM(
            b"GSE_SWITCH_0",
            b"J2000",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(2C) XFORM",
            save.XFORM.as_slice(),
            b"~",
            save.XXFORM.as_slice(),
            36,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(
            b"SWITCH4",
            b"GSE_SWITCH_0",
            save.ET,
            save.RMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(1C) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.IDNT33.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::PXFORM(b"SWITCH4", b"J2000", save.ET, save.RMAT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PXFORM(
            b"GSE_SWITCH_0",
            b"J2000",
            save.ET,
            save.XRMAT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"(2C) RMAT",
            save.RMAT.as_slice(),
            b"~",
            save.XRMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        if (I == 2) {
            //
            // Touch the kernel pool by re-loading FK0.
            //
            spicelib::FURNSH(FK0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a switch frame having the maximum number of base frame intervals.",
        ctx,
    )?;

    //
    // Don't clear the kernel pool: force the switch frame subsystem
    // to make room.
    //
    save.NBASES = 15000;

    spicelib::PIPOOL(b"FRAME_SWITCH_MAXBASE", 1, &[1400010], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400010_NAME",
        1,
        CharArray::from_ref(b"SWITCH_MAXBASE"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400010_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400010_CLASS_ID", 1, &[1400010], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400010_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // 17 is the ID code of ECLIPJ2000.
    //
    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());

    spicelib::PIPOOL(
        b"FRAME_1400010_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of disjoint intervals.
    //
    for I in 1..=save.NBASES {
        save.DSTRBF[I] = (2.0 * ((I - 1) as f64));
        save.DSTPBF[I] = (save.DSTRBF[I] + 0.9);
    }

    spicelib::PDPOOL(
        b"FRAME_1400010_START",
        save.NBASES,
        save.DSTRBF.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400010_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up state transformations at times covered by the
    // base frame intervals.
    //
    spicelib::SXFORM(
        b"J2000",
        b"ECLIPJ2000",
        save.ET,
        save.XXFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NREP = ((100000 / save.NBASES) / 3);

    for REPS in 1..=save.NREP {
        for I in 1..=save.NBASES {
            //
            // Calls used for timing tests---do not delete.
            //
            //  CALL SXFORM ( 'IAU_EARTH', 'J2000', ET, XFORM  )
            //  CALL SXFORM ( 'J2000', 'IAU_EARTH', ET, XFORM  )
            //  CALL SXFORM ( 'ECLIPJ2000', 'J2000', ET, XFORM  )
            //  CALL SXFORM ( 'J2000', 'ECLIPJ2000', ET, XFORM  )
            //  CALL SXFORM ( 'CK_-9999', 'J2000', ET, XFORM  )
            //  CALL SXFORM ( 'J2000', 'CK_-9999', ET, XFORM  )
            //  BASE = IAU_EARTH
            //  BASE = ECLIPJ2000
            //  BASE = CK_-9999

            for J in 1..=3 {
                //
                // Perform lookups at each interval endpoint and midpoint.
                //
                if (J == 1) {
                    save.ET = save.DSTRBF[I];
                } else if (J == 2) {
                    save.ET = ((save.DSTRBF[I] + save.DSTPBF[I]) / 2.0);
                } else {
                    save.ET = save.DSTPBF[I];
                }

                spicelib::SXFORM(
                    b"J2000",
                    b"SWITCH_MAXBASE",
                    save.ET,
                    save.XFORM.as_slice_mut(),
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
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Fill the system with the maximum number of switch frames.",
        ctx,
    )?;

    //
    // Don't clear the kernel pool: force the switch frame subsystem
    // to make room.
    //
    save.NFRAMS = MAXFRM;

    for I in 1..=save.NFRAMS {
        fstr::assign(&mut save.FRNAME, b"SWITCH_MAXFRM_#");
        spicelib::REPMI(&save.FRNAME.to_vec(), b"#", I, &mut save.FRNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.FRAMID = (1500000 + I);

        spicelib::PIPOOL(
            &fstr::concat(b"FRAME_", &save.FRNAME),
            1,
            &[save.FRAMID],
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_NAME");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.FRNAME), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_CLASS");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PIPOOL(&save.KVNAME, 1, &[6], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_CLASS_ID");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRAMID], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_CENTER");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PIPOOL(&save.KVNAME, 1, &[0], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_ALIGNED_WITH");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"CK_-9999"), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Look up state transformations for each frame.
    //
    spicelib::NAMFRM(b"GALACTIC", &mut save.BASEID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NFRAMS {
        save.FRAMID = (1500000 + I);

        save.ET = 0.0;

        spicelib::ZZSWFXFM(
            save.FRAMID,
            save.ET,
            6,
            save.XFORM.as_slice_mut(),
            &mut save.OUTFRM,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", save.BASEID, 0, OK, ctx)?;

        spicelib::SXFORM(
            b"CK_-9999",
            b"GALACTIC",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
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

        spicelib::ZZSWFXFM(
            save.FRAMID,
            save.ET,
            3,
            save.RMAT.as_slice_mut(),
            &mut save.OUTFRM,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", save.BASEID, 0, OK, ctx)?;

        spicelib::PXFORM(
            b"CK_-9999",
            b"GALACTIC",
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
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create more than the maximum number of switch frames that can be buffered. It should still be possible to access all of these frames.", ctx)?;

    //
    // Don't clear the kernel pool: force the switch frame subsystem
    // to make room.
    //
    save.NFRAMS = (MAXFRM + 100);

    for I in 1..=save.NFRAMS {
        fstr::assign(&mut save.FRNAME, b"SWITCH_MAXFRM_#");
        spicelib::REPMI(&save.FRNAME.to_vec(), b"#", I, &mut save.FRNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.FRAMID = (1600000 + I);

        spicelib::PIPOOL(
            &fstr::concat(b"FRAME_", &save.FRNAME),
            1,
            &[save.FRAMID],
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_NAME");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.FRNAME), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_CLASS");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PIPOOL(&save.KVNAME, 1, &[6], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_CLASS_ID");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRAMID], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_CENTER");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PIPOOL(&save.KVNAME, 1, &[0], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"FRAME_#_ALIGNED_WITH");
        spicelib::REPMI(
            &save.KVNAME.to_vec(),
            b"#",
            save.FRAMID,
            &mut save.KVNAME,
            ctx,
        );
        spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"CK_-9999"), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Look up state transformations for each frame.
    //
    spicelib::NAMFRM(b"GALACTIC", &mut save.BASEID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NFRAMS {
        save.FRAMID = (1600000 + I);

        save.ET = 0.0;

        spicelib::ZZSWFXFM(
            save.FRAMID,
            save.ET,
            6,
            save.XFORM.as_slice_mut(),
            &mut save.OUTFRM,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", save.BASEID, 0, OK, ctx)?;

        spicelib::SXFORM(
            b"CK_-9999",
            b"GALACTIC",
            save.ET,
            save.XXFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
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

        spicelib::ZZSWFXFM(
            save.FRAMID,
            save.ET,
            3,
            save.RMAT.as_slice_mut(),
            &mut save.OUTFRM,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", save.BASEID, 0, OK, ctx)?;

        spicelib::PXFORM(
            b"CK_-9999",
            b"GALACTIC",
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
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Binary search test: create sequence of intervals with increasing start times and singleton overlap. Odd-indexed intervals map to J2000 and even-indexed intervals map to the frame GALACTIC.", ctx)?;

    //
    // Load an FK that defines switch frames, and use some of
    // these frames, so our new specification doesn't start at node 1.
    // Use of the frames must follow kernel pool updates.
    //
    spicelib::FURNSH(FK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_BINARY", 1, &[1700000], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1700000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_BINARY"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1700000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1700000_CLASS_ID", 1, &[1700000], ctx)?;
    spicelib::PIPOOL(b"FRAME_1700000_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NBASES = 250;

    for I in 1..=save.NBASES {
        if spicelib::ODD(I) {
            save.IBASBF[I] = 1;
        } else {
            save.IBASBF[I] = 13;
        }
    }

    spicelib::PIPOOL(
        b"FRAME_1700000_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of ordered intervals with singleton overlap.
    //
    for I in 1..=save.NBASES {
        save.DSTRBF[I] = ((I - 1) as f64);
        save.DSTPBF[I] = (save.DSTRBF[I] + 1.0);
    }

    spicelib::PDPOOL(
        b"FRAME_1700000_START",
        save.NBASES,
        save.DSTRBF.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1700000_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use other switch frames. This will populate the switch frame
    // database of ZZSWFXFM.
    //
    spicelib::SXFORM(b"SWITCH0", b"SWITCH1", 0.0, save.XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"SWITCH2", b"SWITCH4", 0.0, save.XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"SWITCH2",
        b"GSE_SWITCH_0",
        0.0,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up state transformations at times covered by the
    // base frame intervals.
    //
    save.NREP = 4;

    for REPS in 1..=save.NREP {
        for I in 1..=save.NBASES {
            for J in 1..=3 {
                //
                // Perform lookups at each interval endpoint and midpoint.
                //
                if (J == 1) {
                    save.ET = save.DSTRBF[I];
                } else if (J == 2) {
                    save.ET = ((save.DSTRBF[I] + save.DSTPBF[I]) / 2.0);
                } else {
                    save.ET = save.DSTPBF[I];
                }

                spicelib::SXFORM(
                    b"J2000",
                    b"SWITCH_BINARY",
                    save.ET,
                    save.XFORM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if (I == save.NBASES) {
                    //
                    // This is the highest-priority interval. We should get
                    // the same result for all three times.
                    //
                    spicelib::SXFORM(
                        b"J2000",
                        b"GALACTIC",
                        save.ET,
                        save.XXFORM.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKAD(
                        b"a XFORM",
                        save.XFORM.as_slice(),
                        b"~",
                        save.XXFORM.as_slice(),
                        36,
                        save.TOL,
                        OK,
                        ctx,
                    )?;
                } else {
                    //
                    // The right endpoint of this interval is the left
                    // endpoint of the following, higher-priority
                    // interval.
                    //
                    if (J == 3) {
                        if spicelib::ODD(I) {
                            spicelib::SXFORM(
                                b"J2000",
                                b"GALACTIC",
                                save.ET,
                                save.XXFORM.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                b"a XFORM",
                                save.XFORM.as_slice(),
                                b"~",
                                save.XXFORM.as_slice(),
                                36,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            testutil::CHCKAD(
                                b"b XFORM",
                                save.XFORM.as_slice(),
                                b"~",
                                save.IDNT66.as_slice(),
                                36,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }
                    } else {
                        if spicelib::ODD(I) {
                            testutil::CHCKAD(
                                b"c XFORM",
                                save.XFORM.as_slice(),
                                b"~",
                                save.IDNT66.as_slice(),
                                36,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            spicelib::SXFORM(
                                b"J2000",
                                b"GALACTIC",
                                save.ET,
                                save.XXFORM.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                b"d XFORM",
                                save.XFORM.as_slice(),
                                b"~",
                                save.XXFORM.as_slice(),
                                36,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }
                    }
                    //
                    // All 3 values of J have been handled.
                    //
                }
                //
                // End of the IF block for I = NBASES and I != NBASES.
                //
            }
            //
            // End of the time loop.
            //
        }

        //
        // End of the loop over all bases.
        //
        if (REPS == 2) {
            //
            // Update the kernel pool. This will force storage of the
            // frame parameters at new locations.
            //
            spicelib::PIPOOL(b"XYZ", 1, &[3], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of the repetition loop.
    //

    //
    // Try computations for times not covered by the frame's intervals.
    //
    save.ET = (save.DSTRBF[1] - 1.0);

    spicelib::SXFORM(
        b"J2000",
        b"SWITCH_BINARY",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMECONNECT)", OK, ctx)?;

    save.ET = (save.DSTPBF[save.NBASES] + 1.0);

    spicelib::SXFORM(
        b"J2000",
        b"SWITCH_BINARY",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMECONNECT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Binary search test: create sequence of disjoint intervals with increasing start times. Odd-indexed intervals map to CK_-10000 and even-indexed intervals map to the frame CK_-9999.", ctx)?;

    //
    // Load an FK that defines switch frames, and use some of
    // these frames, so our new specification doesn't start at node 1.
    // Use of the frames must follow kernel pool updates.
    //
    spicelib::FURNSH(FK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_BINARY2", 1, &[1800000], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1800000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_BINARY2"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1800000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1800000_CLASS_ID", 1, &[1800000], ctx)?;
    spicelib::PIPOOL(b"FRAME_1800000_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NBASES = 250;

    for I in 1..=save.NBASES {
        if spicelib::ODD(I) {
            save.IBASBF[I] = -10000000;
        } else {
            save.IBASBF[I] = -9999000;
        }
    }

    spicelib::PIPOOL(
        b"FRAME_1800000_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of ordered intervals separated by gaps.
    //
    for I in 1..=save.NBASES {
        save.DSTRBF[I] = ((2 as f64) * ((I - 1) as f64));
        save.DSTPBF[I] = (save.DSTRBF[I] + 1.0);
    }

    spicelib::PDPOOL(
        b"FRAME_1800000_START",
        save.NBASES,
        save.DSTRBF.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1800000_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use other switch frames. This will populate the switch frame
    // database of ZZSWFXFM.
    //
    spicelib::SXFORM(b"SWITCH0", b"SWITCH1", 0.0, save.XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"SWITCH2", b"SWITCH4", 0.0, save.XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"SWITCH2",
        b"GSE_SWITCH_0",
        0.0,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up state transformations at times covered by the
    // base frame intervals.
    //
    save.NREP = 4;

    for REPS in 1..=save.NREP {
        for I in 1..=save.NBASES {
            for J in 1..=3 {
                //
                // Perform lookups at each interval endpoint and midpoint.
                // Try a lookup in the gap preceding each interval.
                //
                if (J == 1) {
                    save.ET = save.DSTRBF[I];
                } else if (J == 2) {
                    save.ET = ((save.DSTRBF[I] + save.DSTPBF[I]) / 2.0);
                } else if (J == 3) {
                    save.ET = save.DSTPBF[I];
                } else {
                    save.ET = (save.DSTRBF[I] - 0.5);
                }

                spicelib::SXFORM(
                    b"J2000",
                    b"SWITCH_BINARY2",
                    save.ET,
                    save.XFORM.as_slice_mut(),
                    ctx,
                )?;

                if (J == 4) {
                    testutil::CHCKXC(true, b"SPICE(NOFRAMECONNECT)", OK, ctx)?;
                } else {
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if spicelib::ODD(I) {
                        spicelib::SXFORM(
                            b"J2000",
                            b"CK_-10000",
                            save.ET,
                            save.XXFORM.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKAD(
                            b"a XFORM",
                            save.XFORM.as_slice(),
                            b"~",
                            save.XXFORM.as_slice(),
                            36,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    } else {
                        spicelib::SXFORM(
                            b"J2000",
                            b"CK_-9999",
                            save.ET,
                            save.XXFORM.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKAD(
                            b"b XFORM",
                            save.XFORM.as_slice(),
                            b"~",
                            save.XXFORM.as_slice(),
                            36,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }
                }
            }
            //
            // End of the time loop.
            //
        }
        //
        // End of the loop over all bases.
        //
        if (REPS == 2) {
            //
            // Update the kernel pool. This will force storage of the
            // frame parameters at new locations.
            //
            spicelib::PIPOOL(b"XYZ", 1, &[3], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of the repetition loop.
    //

    //
    // Try a computation for the rightmost gap.
    //
    save.ET = (save.DSTPBF[save.NBASES] + 1.0);

    spicelib::SXFORM(
        b"J2000",
        b"SWITCH_BINARY2",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMECONNECT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error case: try to load frame that has too many bases to be buffered.",
        ctx,
    )?;

    spicelib::PIPOOL(b"FRAME_TOOBIG1", 1, &[1400011], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400011_NAME",
        1,
        CharArray::from_ref(b"TOOBIG1"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400011_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400011_CLASS_ID", 1, &[1400011], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400011_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NBASES = (MAXBAS + 1);

    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());

    spicelib::PIPOOL(
        b"FRAME_1400011_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"TOOBIG1",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(TOOMANYBASEFRAMES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad dimension", ctx)?;

    spicelib::ZZSWFXFM(
        1,
        0.0,
        2,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: first stop time of BADSTOP1 is invalid", ctx)?;

    spicelib::PIPOOL(b"FRAME_BADSTOP1", 1, &[1400012], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400012_NAME",
        1,
        CharArray::from_ref(b"BADSTOP1"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400012_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400012_CLASS_ID", 1, &[1400012], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400012_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NBASES = 10;

    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());

    spicelib::PIPOOL(
        b"FRAME_1400012_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NBASES {
        save.DSTRBF[I] = (2.0 * ((I - 1) as f64));
        save.DSTPBF[I] = (save.DSTRBF[I] + 1.0);
    }

    save.DSTPBF[1] = save.DSTRBF[1];

    spicelib::PDPOOL(
        b"FRAME_1400012_START",
        save.NBASES,
        save.DSTRBF.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400012_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFXFM(
        1400012,
        0.0,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTIMEBOUNDS)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Make sure the system recovers after the time bounds are
    // corrected.
    //
    save.DSTPBF[1] = (save.DSTRBF[1] + 1.0);
    spicelib::PDPOOL(
        b"FRAME_1400012_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFXFM(
        1400012,
        0.0,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error case: stop time of BADSTOP1 at index 200 is invalid.",
        ctx,
    )?;

    spicelib::PIPOOL(b"FRAME_BADSTOP1", 1, &[1400012], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400012_NAME",
        1,
        CharArray::from_ref(b"BADSTOP1"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400012_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400012_CLASS_ID", 1, &[1400012], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400012_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NBASES = 200;

    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());

    spicelib::PIPOOL(
        b"FRAME_1400012_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NBASES {
        save.DSTRBF[I] = (2.0 * ((I - 1) as f64));
        save.DSTPBF[I] = (save.DSTRBF[I] + 1.0);
    }

    save.DSTPBF[save.NBASES] = save.DSTRBF[save.NBASES];

    spicelib::PDPOOL(
        b"FRAME_1400012_START",
        save.NBASES,
        save.DSTRBF.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400012_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFXFM(
        1400012,
        0.0,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTIMEBOUNDS)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Make sure the system recovers after the time bounds are
    // corrected.
    //
    save.DSTPBF[save.NBASES] = (save.DSTRBF[save.NBASES] + 1.0);
    spicelib::PDPOOL(
        b"FRAME_1400012_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFXFM(
        1400012,
        0.0,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error case: first base frame has incomplete specification: center is missing.",
        ctx,
    )?;

    spicelib::PIPOOL(b"FRAME_BADBASE1", 1, &[1400013], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400013_NAME",
        1,
        CharArray::from_ref(b"BADBASE1"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400013_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400013_CLASS_ID", 1, &[1400013], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400013_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NBASES = 10;

    //
    // Make the first base SWITCH0; make the rest ECLIPJ2000.
    //
    save.IBASBF[1] = 1400000;
    spicelib::FILLI(17, (save.NBASES - 1), save.IBASBF.subarray_mut(2));

    spicelib::PIPOOL(
        b"FRAME_1400013_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NBASES {
        save.DSTRBF[I] = (2.0 * ((I - 1) as f64));
        save.DSTPBF[I] = (save.DSTRBF[I] + 1.0);
    }

    spicelib::PDPOOL(
        b"FRAME_1400013_START",
        save.NBASES,
        save.DSTRBF.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400013_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Remove the center from SWITCH0.
    //
    spicelib::DVPOOL(b"FRAME_1400000_CENTER", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFXFM(
        1400013,
        0.0,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Make sure the system recovers after the center is restored.
    //
    spicelib::PIPOOL(b"FRAME_1400000_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFXFM(
        1400013,
        0.0,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", 1400000, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error case: base frame has incomplete specification at index 200: center is missing.",
        ctx,
    )?;

    save.NBASES = 200;

    //
    // Make the last base SWITCH0; make the rest ECLIPJ2000.
    //
    save.IBASBF[save.NBASES] = 1400000;
    spicelib::FILLI(17, (save.NBASES - 1), save.IBASBF.as_slice_mut());

    spicelib::PIPOOL(
        b"FRAME_1400013_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NBASES {
        save.DSTRBF[I] = (2.0 * ((I - 1) as f64));
        save.DSTPBF[I] = (save.DSTRBF[I] + 1.0);
    }

    spicelib::PDPOOL(
        b"FRAME_1400013_START",
        save.NBASES,
        save.DSTRBF.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400013_STOP",
        save.NBASES,
        save.DSTPBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Remove the center from SWITCH0.
    //
    spicelib::DVPOOL(b"FRAME_1400000_CENTER", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFXFM(
        1400013,
        0.0,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Make sure the system recovers after the center is restored.
    //
    spicelib::PIPOOL(b"FRAME_1400000_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET1 = save.DSTPBF[save.NBASES];

    spicelib::ZZSWFXFM(
        1400013,
        save.ET1,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", 1400000, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: return on entry", ctx)?;

    fstr::assign(&mut save.SHRTMS, b"SPICE(TESTERROR)");

    spicelib::SIGERR(&save.SHRTMS, ctx)?;

    spicelib::ZZSWFXFM(
        1400013,
        save.ET1,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, &save.SHRTMS, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: CKFXFM failure", ctx)?;

    //
    // Try a lookup using a time that cannot be converted to the
    // SCLK associated with the CK frame for instrument -9999.
    //
    // Use frame SWITCH2.
    //
    save.FRAMID = 1400002;
    save.ET = -3000000000.0;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: CKFROT failure", ctx)?;

    //
    // Try a lookup using a time that cannot be converted to the
    // SCLK associated with the CK frame for instrument -9999.
    //
    // Use frame SWITCH2.
    //
    save.FRAMID = 1400002;
    save.ET = -3000000000.0;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: TISBOD failure", ctx)?;

    //
    // The top priority base frame of frame SWITCH4, over the
    // time interval
    //
    //    2000 JAN 1 00:00:00 TDB
    //    2000 FEB 1 00:00:00 TDB
    //
    // is a PCK frame. We'll get rid of necessary PCK data and use
    // a request time that will cause the frame subsystem to attempt
    // to compute the orientation of this frame.
    //
    spicelib::STR2ET(b"2000 JAN 2 00:00:00 TDB", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY399_POLE_RA", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FRAMID = 1400004;
    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore the PCK data.
    //
    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: TIPBOD failure", ctx)?;

    //
    // Use the same technique as was used for TISBOD to generate the
    // desired error.
    //
    spicelib::STR2ET(b"2000 JAN 2 00:00:00 TDB", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY399_POLE_RA", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FRAMID = 1400004;
    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore the PCK data.
    //
    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: TKFRAM failure", ctx)?;

    //
    // Again we'll delete data needed by frame SWITCH4.
    //
    spicelib::STR2ET(b"2000 FEB 2 00:00:00 TDB", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The frame ID 1400007 is that of frame TK_0.
    //
    spicelib::DVPOOL(b"TKFRAME_1400007_RELATIVE", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FRAMID = 1400004;
    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VARIABLENOTFOUND)", OK, ctx)?;

    //
    // Restore the frame data.
    //
    spicelib::FURNSH(FK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: IRFROT failure", ctx)?;

    //
    // We'll use a frame that has no time intervals and
    // one bogus inertial base frame.
    //
    save.FRAMID = 1400009;
    save.ET = 0.0;
    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IRFNOTREC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Non-error exception case: in binary search mode, CKFXFM cannot find data.",
        ctx,
    )?;

    //
    // Use the frame SWITCH_BINARY_GAP. The request time is
    // covered by this frame's second interval, but there's
    // no CK data available
    //
    save.FRAMID = 1400051;
    spicelib::STR2ET(b"2012 JAN 1 00:00:00 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::SXFORM(
        b"SWITCH_BINARY_GAP",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMECONNECT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Non-error exception case: in binary search mode, CKFROT cannot find data.",
        ctx,
    )?;

    //
    // Use the frame SWITCH_BINARY_GAP.
    //
    save.FRAMID = 1400051;
    spicelib::STR2ET(b"2012 JAN 1 00:00:00 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::PXFORM(
        b"SWITCH_BINARY_GAP",
        b"J2000",
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMECONNECT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-error exception case: in binary search mode, CKFXFM cannot find data at start time of interval, but data for the preceding interval with common endpoint are available.", ctx)?;

    //
    // Use the frame SWITCH_BINARY_GAP.
    //
    save.FRAMID = 1400051;

    //
    // The request time is the stop time of the first interval.
    //
    spicelib::STR2ET(b"2011 SEP 9 01:46:40 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The expected base frame is the base of CK_-9999: GALACTIC
    //
    testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", 13, 0, OK, ctx)?;

    spicelib::SXFORM(
        b"SWITCH_BINARY_GAP",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that a slightly later time lies in the gap.
    //
    save.ET = (save.ET + 0.00001);

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-error exception case: in binary search mode, CKFFOT cannot find data at stop time of interval, but data for the next interval with common endpoint are available.", ctx)?;

    //
    // Use the frame SWITCH_BINARY_GAP.
    //
    save.FRAMID = 1400051;

    //
    // The request time is the stop time of the first interval.
    //
    spicelib::STR2ET(b"2011 SEP 9 01:46:40 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.RMAT.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The expected base frame is the base of CK_-9999: GALACTIC
    //
    testutil::CHCKSI(b"OUTFRM", save.OUTFRM, b"=", 13, 0, OK, ctx)?;

    spicelib::PXFORM(
        b"SWITCH_BINARY_GAP",
        b"J2000",
        save.ET,
        save.RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that a slightly later time lies in the gap.
    //
    save.ET = (save.ET + 0.00001);

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.RMAT.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Non-error exception case: binary search on a single interval, result found, 6x6 case.",
        ctx,
    )?;

    //
    // Use frame SWITCH_SINGLE_BASE.
    //
    save.FRAMID = 1400050;

    spicelib::STR2ET(b"2000 JAN 1 18:00:00 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-error exception case: binary search on a single interval, result not found, 6x6 case.", ctx)?;

    //
    // Use frame SWITCH_SINGLE_BASE.
    //
    save.FRAMID = 1400050;

    spicelib::STR2ET(b"2000 JAN 2 00:00:00.001 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::STR2ET(b"1999 DEC 31 23:59:59.999 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        6,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Non-error exception case: binary search on a single interval, result found, 3x3 case.",
        ctx,
    )?;

    //
    // Use frame SWITCH_SINGLE_BASE.
    //
    save.FRAMID = 1400050;

    spicelib::STR2ET(b"2000 JAN 1 18:00:00 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.RMAT.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-error exception case: binary search on a single interval, result not found, 3x3 case.", ctx)?;

    //
    // Use frame SWITCH_SINGLE_BASE.
    //
    save.FRAMID = 1400050;

    spicelib::STR2ET(b"2000 JAN 2 00:00:00.001 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.RMAT.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::STR2ET(b"1999 DEC 31 23:59:59.999 TDB", &mut save.ET, ctx)?;

    spicelib::ZZSWFXFM(
        save.FRAMID,
        save.ET,
        3,
        save.XFORM.as_slice_mut(),
        &mut save.OUTFRM,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSWFINI Error case: return on entry", ctx)?;

    fstr::assign(&mut save.SHRTMS, b"SPICE(TESTERROR)");

    spicelib::SIGERR(&save.SHRTMS, ctx)?;

    spicelib::ZZSWFINI(
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.HDBASE.as_slice_mut(),
        &mut save.FREE,
        &mut save.PRVAT,
        &mut save.PRVFRM,
        &mut save.SAMFRM,
        ctx,
    )?;
    testutil::CHCKXC(true, &save.SHRTMS, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::KILFIL(FK0, ctx)?;
    testutil::KILFIL(CK0, ctx)?;
    testutil::KILFIL(CK1, ctx)?;
    testutil::KILFIL(PCK0, ctx)?;
    testutil::KILFIL(SCLK0, ctx)?;
    testutil::KILFIL(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
