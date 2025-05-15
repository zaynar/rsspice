//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const CKER: &[u8] = b"zznofcon.bc";
const CLKKER: &[u8] = b"zznofcon.tsc";
const LNSIZE: i32 = 80;
const MSGLEN: i32 = 600;
const NCASE: i32 = 5;
const BUFSIZ: i32 = 82;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 35;

struct SaveVars {
    BASES: ActualCharArray,
    FRAMES: ActualCharArray,
    KERBUF: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASES = ActualCharArray::new(FRNMLN, 1..=NCASE);
        let mut FRAMES = ActualCharArray::new(FRNMLN, 1..=NCASE);
        let mut KERBUF = ActualCharArray::new(LNSIZE, 1..=BUFSIZ);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"BOGUS_CK_-9999"),
                Val::C(b"BOGUS_CK_-9999"),
                Val::C(b"BOGUS_CK_-10000"),
                Val::C(b"J2000"),
            ]
            .into_iter();
            BASES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"BOGUS_TK_1"),
                Val::C(b"BOGUS_TK_2"),
                Val::C(b"BOGUS_CK_-9999"),
                Val::C(b"BOGUS_CK_-10000"),
                Val::C(b"BOGUS_TK_3"),
            ]
            .into_iter();
            FRAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BASES,
            FRAMES,
            KERBUF,
        }
    }
}

//$Procedure F_ZZNOFCON ( ZZNOFCON tests )
pub fn F_ZZNOFCON(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CKWARN = [b' '; LMSGLN as usize];
    let mut CLOSNG = [b' '; LMSGLN as usize];
    let mut ETSTR = [b' '; TIMLEN as usize];
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ERRMSG = [b' '; LMSGLN as usize];
    let mut TIMSTR = [b' '; LMSGLN as usize];
    let mut INSUFF = [b' '; LMSGLN as usize];
    let mut SCWARN = [b' '; LMSGLN as usize];
    let mut TRANS = [b' '; LMSGLN as usize];
    let mut ET: f64 = 0.0;
    let mut BASEID = StackArray::<i32, 2>::new(1..=2);
    let mut CENTER: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLKID: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRAMID = StackArray::<i32, 2>::new(1..=2);
    let mut HANDLE: i32 = 0;
    let mut K: i32 = 0;
    let mut L: i32 = 0;
    let mut NOSCLK: i32 = 0;
    let mut FOUND: bool = false;
    let mut ISCK: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local Parameters
    //
    //
    // Note: the name "CK" is already in use; it's a parameter
    // in frmtyp.inc.
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
    // The simple test cases.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZNOFCON", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Load kernels and POOL data", ctx)?;

    //
    // Initialize ET.
    //
    ET = 0.0;

    //
    // Initialize the frame kernel buffer.
    //
    // Note that the CK frame BOGUS_CK_-10000 has been altered
    // so that it *doesn't* refer to the CK and SCLK IDs -10000
    // and -9, which are used by TSTCK3. This CK frame never
    // has available pointing or SCLK data.
    //
    fstr::assign(
        save.KERBUF.get_mut(1),
        b"      FRAME_BOGUS_TK_1      =  1400000",
    );
    fstr::assign(
        save.KERBUF.get_mut(2),
        b"      FRAME_1400000_NAME     = \'BOGUS_TK_1\'",
    );
    fstr::assign(save.KERBUF.get_mut(3), b"      FRAME_1400000_CLASS    =  4");
    fstr::assign(
        save.KERBUF.get_mut(4),
        b"      FRAME_1400000_CLASS_ID =  1400000",
    );
    fstr::assign(
        save.KERBUF.get_mut(5),
        b"      FRAME_1400000_CENTER   =   400000",
    );
    fstr::assign(save.KERBUF.get_mut(6), b" ");
    fstr::assign(
        save.KERBUF.get_mut(7),
        b"      OBJECT_400000_FRAME    = \'BOGUS_TK_1\'",
    );
    fstr::assign(save.KERBUF.get_mut(8), b" ");
    fstr::assign(
        save.KERBUF.get_mut(9),
        b"      TKFRAME_BOGUS_TK_1_RELATIVE = \'BOGUS_CK_-9999\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(10),
        b"      TKFRAME_BOGUS_TK_1_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(11),
        b"      TKFRAME_BOGUS_TK_1_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(12),
        b"      TKFRAME_BOGUS_TK_1_AXES     = ( 3, 2, 3 )",
    );
    fstr::assign(
        save.KERBUF.get_mut(13),
        b"      TKFRAME_BOGUS_TK_1_ANGLES   = ( -243.126496675,",
    );
    fstr::assign(
        save.KERBUF.get_mut(14),
        b"                                        -54.657822839,",
    );
    fstr::assign(
        save.KERBUF.get_mut(15),
        b"                                        180.0 )",
    );
    fstr::assign(save.KERBUF.get_mut(16), b" ");
    fstr::assign(save.KERBUF.get_mut(17), b" ");
    fstr::assign(
        save.KERBUF.get_mut(18),
        b"      FRAME_BOGUS_TK_2      =  1400001",
    );
    fstr::assign(
        save.KERBUF.get_mut(19),
        b"      FRAME_1400001_NAME     = \'BOGUS_TK_2\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(20),
        b"      FRAME_1400001_CLASS    =  4",
    );
    fstr::assign(
        save.KERBUF.get_mut(21),
        b"      FRAME_1400001_CLASS_ID =  1400001",
    );
    fstr::assign(
        save.KERBUF.get_mut(22),
        b"      FRAME_1400001_CENTER   =   400001",
    );
    fstr::assign(save.KERBUF.get_mut(23), b" ");
    fstr::assign(
        save.KERBUF.get_mut(24),
        b"      OBJECT_400001_FRAME    = \'BOGUS_TK_2\'",
    );
    fstr::assign(save.KERBUF.get_mut(25), b" ");
    fstr::assign(
        save.KERBUF.get_mut(26),
        b"      TKFRAME_BOGUS_TK_2_RELATIVE = \'BOGUS_TK_1\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(27),
        b"      TKFRAME_BOGUS_TK_2_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(28),
        b"      TKFRAME_BOGUS_TK_2_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(29),
        b"      TKFRAME_BOGUS_TK_2_AXES     = ( 3, 2, 3 )",
    );
    fstr::assign(
        save.KERBUF.get_mut(30),
        b"      TKFRAME_BOGUS_TK_2_ANGLES   = ( -243.126496675,",
    );
    fstr::assign(
        save.KERBUF.get_mut(31),
        b"                                        -54.657822839,",
    );
    fstr::assign(
        save.KERBUF.get_mut(32),
        b"                                        180.0 )",
    );
    fstr::assign(save.KERBUF.get_mut(33), b" ");
    fstr::assign(
        save.KERBUF.get_mut(34),
        b"      FRAME_BOGUS_CK_-9999        = -9999",
    );
    fstr::assign(
        save.KERBUF.get_mut(35),
        b"      FRAME_-9999_NAME            = \'BOGUS_CK_-9999\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(36),
        b"      FRAME_-9999_CLASS           =  3",
    );
    fstr::assign(
        save.KERBUF.get_mut(37),
        b"      FRAME_-9999_CLASS_ID        = -9999",
    );
    fstr::assign(
        save.KERBUF.get_mut(38),
        b"      FRAME_-9999_CENTER          = -9999",
    );
    fstr::assign(
        save.KERBUF.get_mut(39),
        b"      CK_-9999_SCLK               = -9",
    );
    fstr::assign(
        save.KERBUF.get_mut(40),
        b"      OBJECT_-9999_FRAME          = \'BOGUS_CK_-9999\'",
    );
    fstr::assign(save.KERBUF.get_mut(41), b" ");
    fstr::assign(save.KERBUF.get_mut(42), b" ");
    fstr::assign(
        save.KERBUF.get_mut(43),
        b"      FRAME_BOGUS_CK_-10000       = -10000",
    );
    fstr::assign(
        save.KERBUF.get_mut(44),
        b"      FRAME_-10000_NAME           = \'BOGUS_CK_-10000\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(45),
        b"      FRAME_-10000_CLASS          =  3",
    );
    fstr::assign(
        save.KERBUF.get_mut(46),
        b"      FRAME_-10000_CLASS_ID       = -10",
    );
    fstr::assign(
        save.KERBUF.get_mut(47),
        b"      FRAME_-10000_CENTER         = -10000",
    );
    fstr::assign(
        save.KERBUF.get_mut(48),
        b"      CK_-10_SCLK                 = -1",
    );
    fstr::assign(
        save.KERBUF.get_mut(49),
        b"      OBJECT_-10000_FRAME         = \'BOGUS_CK_-10000\'",
    );
    fstr::assign(save.KERBUF.get_mut(50), b" ");
    fstr::assign(save.KERBUF.get_mut(51), b" ");
    fstr::assign(
        save.KERBUF.get_mut(52),
        b"      FRAME_BOGUS_TK_3      =  1400002",
    );
    fstr::assign(
        save.KERBUF.get_mut(53),
        b"      FRAME_1400002_NAME     = \'BOGUS_TK_3\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(54),
        b"      FRAME_1400002_CLASS    =  4",
    );
    fstr::assign(
        save.KERBUF.get_mut(55),
        b"      FRAME_1400002_CLASS_ID =  1400002",
    );
    fstr::assign(
        save.KERBUF.get_mut(56),
        b"      FRAME_1400002_CENTER   =   400002",
    );
    fstr::assign(save.KERBUF.get_mut(57), b" ");
    fstr::assign(
        save.KERBUF.get_mut(58),
        b"      OBJECT_400002_FRAME    = \'BOGUS_TK_3\'",
    );
    fstr::assign(save.KERBUF.get_mut(59), b" ");
    fstr::assign(
        save.KERBUF.get_mut(60),
        b"      TKFRAME_BOGUS_TK_3_RELATIVE = \'J2000\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(61),
        b"      TKFRAME_BOGUS_TK_3_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(62),
        b"      TKFRAME_BOGUS_TK_3_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(63),
        b"      TKFRAME_BOGUS_TK_3_AXES     = ( 3, 2, 3 )",
    );
    fstr::assign(
        save.KERBUF.get_mut(64),
        b"      TKFRAME_BOGUS_TK_3_ANGLES   = ( -243.126496675,",
    );
    fstr::assign(
        save.KERBUF.get_mut(65),
        b"                                        -54.657822839,",
    );
    fstr::assign(
        save.KERBUF.get_mut(66),
        b"                                        180.0 )",
    );
    fstr::assign(save.KERBUF.get_mut(67), b" ");
    fstr::assign(
        save.KERBUF.get_mut(68),
        b"      FRAME_BOGUS_TK_4      =  1400003",
    );
    fstr::assign(
        save.KERBUF.get_mut(69),
        b"      FRAME_1400003_NAME     = \'BOGUS_TK_4\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(70),
        b"      FRAME_1400003_CLASS    =  4",
    );
    fstr::assign(
        save.KERBUF.get_mut(71),
        b"      FRAME_1400003_CLASS_ID =  1400003",
    );
    fstr::assign(
        save.KERBUF.get_mut(72),
        b"      FRAME_1400003_CENTER   =   400003",
    );
    fstr::assign(save.KERBUF.get_mut(73), b" ");
    fstr::assign(
        save.KERBUF.get_mut(74),
        b"      OBJECT_400003_FRAME    = \'BOGUS_TK_4\'",
    );
    fstr::assign(save.KERBUF.get_mut(75), b" ");
    fstr::assign(
        save.KERBUF.get_mut(76),
        b"      TKFRAME_BOGUS_TK_4_RELATIVE = \'J2000\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(77),
        b"      TKFRAME_BOGUS_TK_4_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(78),
        b"      TKFRAME_BOGUS_TK_4_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.KERBUF.get_mut(79),
        b"      TKFRAME_BOGUS_TK_4_AXES     = ( 3, 2, 3 )",
    );
    fstr::assign(
        save.KERBUF.get_mut(80),
        b"      TKFRAME_BOGUS_TK_4_ANGLES   = ( -243.126496675,",
    );
    fstr::assign(
        save.KERBUF.get_mut(81),
        b"                                        -54.657822839,",
    );
    fstr::assign(
        save.KERBUF.get_mut(82),
        b"                                        180.0 )",
    );

    spicelib::LMPOOL(save.KERBUF.as_arg(), BUFSIZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create CK and SCLK kernels. Don't load the kernels; keep
    // the SCLK kernel.
    //
    testutil::TSTCK3(CKER, CLKKER, false, false, true, &mut HANDLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NCASE {
        //
        // Frame and base I map to the first frame ID, base ID pair.
        //
        spicelib::NAMFRM(&save.FRAMES[I], &mut FRAMID[1], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::NAMFRM(&save.BASES[I], &mut BASEID[1], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=NCASE {
            //
            // Frame and base J map to the second frame ID, base ID pair.
            //
            spicelib::NAMFRM(&save.FRAMES[J], &mut FRAMID[2], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::NAMFRM(&save.BASES[J], &mut BASEID[2], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set ET.
            //
            ET = ((100000000.0 * I as f64) + J as f64);

            for M in 1..=2 {
                //
                // Load SCLK kernel if M is 2.
                //
                if (M == 2) {
                    spicelib::FURNSH(CLKKER, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::UNLOAD(CLKKER, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::LMPOOL(save.KERBUF.as_arg(), BUFSIZ, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // --- Case: ------------------------------------------------------
                //

                if (J != I) {
                    fstr::assign(&mut TITLE, b"Fr 1: #, Bs 1: #, Fr 2: #, Bs 2: #, SCLK:#");

                    spicelib::REPMC(&TITLE.clone(), b"#", &save.FRAMES[1], &mut TITLE);
                    spicelib::REPMC(&TITLE.clone(), b"#", &save.BASES[1], &mut TITLE);
                    spicelib::REPMC(&TITLE.clone(), b"#", &save.FRAMES[2], &mut TITLE);
                    spicelib::REPMC(&TITLE.clone(), b"#", &save.BASES[2], &mut TITLE);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (M == 1) {
                        spicelib::REPMC(&TITLE.clone(), b"#", b"N", &mut TITLE);
                    } else {
                        spicelib::REPMC(&TITLE.clone(), b"#", b"Y", &mut TITLE);
                    }

                    testutil::TCASE(&TITLE, ctx)?;

                    spicelib::ZZNOFCON(
                        ET,
                        FRAMID[1],
                        BASEID[1],
                        FRAMID[2],
                        BASEID[2],
                        &mut ERRMSG,
                        ctx,
                    )?;

                    //
                    // Indicate this is not a CK case to start. We're not
                    // yet missing SCLK or CK data.
                    //
                    ISCK = false;
                    NOSCLK = 0;

                    //
                    // Create the expected time string; if this
                    // string is at the start of the message,
                    // subtract it from the message.
                    //
                    fstr::assign(&mut TIMSTR, b"At epoch # TDB (# TDB),");

                    spicelib::REPMD(&TIMSTR.clone(), b"#", ET, 14, &mut TIMSTR, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::ETCAL(ET, &mut ETSTR, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMC(&TIMSTR.clone(), b"#", &ETSTR, &mut TIMSTR);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    L = spicelib::RTRIM(&TIMSTR);

                    if fstr::eq(fstr::substr(&ERRMSG, 1..=L), &TIMSTR) {
                        spicelib::REMSUB(&ERRMSG.clone(), 1, L, &mut ERRMSG, ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Create the "insufficient data" string; if
                    // string is at the start of the message,
                    // subtract it from the message.
                    //
                    fstr::assign(&mut INSUFF, b" there is insufficient information available to transform from reference frame # (#) to reference frame # (#).");

                    spicelib::REPMI(&INSUFF.clone(), b"#", FRAMID[1], &mut INSUFF, ctx);
                    spicelib::REPMC(&INSUFF.clone(), b"#", &save.FRAMES[I], &mut INSUFF);
                    spicelib::REPMI(&INSUFF.clone(), b"#", FRAMID[2], &mut INSUFF, ctx);
                    spicelib::REPMC(&INSUFF.clone(), b"#", &save.FRAMES[J], &mut INSUFF);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    L = spicelib::RTRIM(&INSUFF);

                    if fstr::eq(fstr::substr(&ERRMSG, 1..=L), &INSUFF) {
                        spicelib::REMSUB(&ERRMSG.clone(), 1, L, &mut ERRMSG, ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Now clean up the path-dependent message portions.
                    //
                    for PATH in 1..=2 {
                        //
                        // Get info for the base frame.
                        //
                        spicelib::FRINFO(
                            BASEID[PATH],
                            &mut CENTER,
                            &mut CLASS,
                            &mut CLSSID,
                            &mut FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSL(b"FRINFO found", FOUND, true, OK, ctx)?;

                        //
                        // Indicate we've got at least one CK base if the
                        // current base is type CK.
                        //
                        if (CLASS == CK) {
                            ISCK = true;
                        }

                        //
                        // Let K point to the index I or J, depending
                        // on which path we're looking at.
                        //
                        if (PATH == 1) {
                            K = I;
                        } else {
                            K = J;
                        }

                        //
                        // If the frame doesn't equal its base, create the
                        // transformation phrase for the first path.
                        //
                        if (FRAMID[PATH] != BASEID[PATH]) {
                            //
                            // The path contains more than one node.
                            //
                            fstr::assign(
                                &mut TRANS,
                                b" Frame # could be transformed to frame # (#).",
                            );

                            spicelib::REPMC(&TRANS.clone(), b"#", &save.FRAMES[K], &mut TRANS);
                            spicelib::REPMI(&TRANS.clone(), b"#", BASEID[PATH], &mut TRANS, ctx);
                            spicelib::REPMC(&TRANS.clone(), b"#", &save.BASES[K], &mut TRANS);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            L = spicelib::RTRIM(&TRANS);

                            if fstr::eq(fstr::substr(&ERRMSG, 1..=L), &TRANS) {
                                spicelib::REMSUB(&ERRMSG.clone(), 1, L, &mut ERRMSG, ctx)?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;
                            }

                            if (CLASS == CK) {
                                //
                                // If the base frame is a CK frame, create
                                // the CK warning for this frame.
                                //

                                fstr::assign(&mut CKWARN, b" The latter is a CK frame; a CK file containing data for instrument or structure # at the epoch shown above, as well as a corresponding SCLK kernel, must be loaded in order to use this frame.");

                                spicelib::REPMI(&CKWARN.clone(), b"#", CLSSID, &mut CKWARN, ctx);

                                L = spicelib::RTRIM(&CKWARN);

                                if fstr::eq(fstr::substr(&ERRMSG, 1..=L), &CKWARN) {
                                    spicelib::REMSUB(&ERRMSG.clone(), 1, L, &mut ERRMSG, ctx)?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                }
                            }
                        } else {
                            //
                            // The path is a singleton.
                            //
                            if (CLASS == CK) {
                                //
                                // If the base frame is a CK frame, create
                                // the CK warning for this frame.
                                //
                                fstr::assign(&mut CKWARN, b" # is a CK frame; a CK file containing data for instrument or structure # at the epoch shown above, as well as a corresponding SCLK kernel, must be loaded in order to use this frame.");

                                spicelib::REPMC(&CKWARN.clone(), b"#", &save.BASES[K], &mut CKWARN);
                                spicelib::REPMI(&CKWARN.clone(), b"#", CLSSID, &mut CKWARN, ctx);

                                L = spicelib::RTRIM(&CKWARN);

                                if fstr::eq(fstr::substr(&ERRMSG, 1..=L), &CKWARN) {
                                    spicelib::REMSUB(&ERRMSG.clone(), 1, L, &mut ERRMSG, ctx)?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                }
                            }

                            //
                            // End of CK base case.
                            //
                        }

                        //
                        // End path length case.
                        //
                        // Now, if the base frame is of CK type,
                        // and if there is no SCLK data for this frame,
                        // generate the SCLK warning message.
                        //
                        if (CLASS == CK) {
                            spicelib::CKMETA(CLSSID, b"SCLK", &mut CLKID, ctx)?;

                            if !spicelib::ZZSCLK(CLSSID, CLKID, ctx)? {
                                //
                                // Increment the count of bases missing
                                // SCLK data.
                                //
                                NOSCLK = (NOSCLK + 1);

                                fstr::assign(&mut SCWARN, b" No SCLK kernel for instrument or structure #, with corresponding SCLK ID #, is currently loaded.");

                                spicelib::REPMI(&SCWARN.clone(), b"#", CLSSID, &mut SCWARN, ctx);
                                spicelib::REPMI(&SCWARN.clone(), b"#", CLKID, &mut SCWARN, ctx);

                                L = spicelib::RTRIM(&SCWARN);

                                if fstr::eq(fstr::substr(&ERRMSG, 1..=L), &SCWARN) {
                                    spicelib::REMSUB(&ERRMSG.clone(), 1, L, &mut ERRMSG, ctx)?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                }
                            }
                        }
                    }

                    //
                    // End of path loop.
                    //
                    // Create the closing message, if needed.
                    //
                    if (ISCK && (NOSCLK < 2)) {
                        if (NOSCLK == 0) {
                            //
                            // We have at least one CK base frame
                            // but no missing SCLK data.
                            //
                            fstr::assign(&mut CLOSNG, b" Failure to find required CK data could be due to one or more CK files not having been loaded, or to the epoch shown above lying within a coverage gap or beyond the coverage bounds of the loaded CK files. It is also possible that no loaded CK file has required angular velocity data for the input epoch, even if a loaded CK does have attitude data for that epoch. You can use CKBRIEF with the -dump option to display coverage intervals of a CK file.");
                        } else {
                            //
                            // We're missing one set of SCLK data.
                            //
                            fstr::assign(&mut CLOSNG, b" For a CK frame for which the corresponding SCLK kernel has been loaded, failure to find required CK data could be due to one or more CK files not having been loaded, or to the epoch shown above lying within a coverage gap or beyond the coverage bounds of the loaded CK files. It is also possible that no loaded CK file has required angular velocity data for the input epoch, even if a loaded CK does have attitude data for that epoch. You can use CKBRIEF with the -dump option to display coverage intervals of a CK file.");
                        }

                        //
                        // Remove the closing message.
                        //
                        L = spicelib::RTRIM(&CLOSNG);

                        if fstr::eq(fstr::substr(&ERRMSG, 1..=L), &CLOSNG) {
                            spicelib::REMSUB(&ERRMSG.clone(), 1, L, &mut ERRMSG, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }
                    }

                    //
                    // Ok, here's the test. ERRMSG should be blank.
                    //
                    testutil::CHCKSC(b"ERRMSG post deletion", &ERRMSG, b"=", b" ", OK, ctx)?;
                }

                //
                // End of J != I case. This is the block in which a
                // test message is created.
                //
            }
            //
            // End of SCLK load loop.
            //
        }
        //
        // End of second path loop.
        //
    }
    //
    // End of first path loop.
    //

    //
    // Close out the test family.
    //

    //
    // Clean-up all kernels.
    //
    spicelib::KCLEAR(ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
