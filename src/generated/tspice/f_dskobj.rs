//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXNSRF: i32 = 2000;
const SFNMLN: i32 = 36;
const CTRSIZ: i32 = 2;
const NROOM: i32 = 2003;
const LBSNGL: i32 = -5;
const DSK0: &[u8] = b"dskobj_test0.bds";
const DSK1: &[u8] = b"dskobj_test1.bds";
const SPK0: &[u8] = b"dskobj_test0.bsp";
const XFR0: &[u8] = b"dskobj_test0.xfr";
const EK0: &[u8] = b"dskobj_test0.bes";
const LBCELL: i32 = -5;
const FRNMLN: i32 = 32;
const NBOD: i32 = 10;
const NDSK0: i32 = 10;
const NDSK1: i32 = 1000;
const MAXSIZ: i32 = 5000;

struct SaveVars {
    FRNAME: Vec<u8>,
    BODSET: ActualArray<i32>,
    BODLST: StackArray<i32, 10>,
    BODYID: i32,
    FRAMID: i32,
    HANDLE: i32,
    J: i32,
    K: i32,
    N: i32,
    SRFSET: ActualArray<i32>,
    UNIT: i32,
    XBDSET: ActualArray<i32>,
    XN: i32,
    XSFSET: ActualArray<i32>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRNAME = vec![b' '; FRNMLN as usize];
        let mut BODSET = ActualArray::<i32>::new(LBCELL..=MAXSIZ);
        let mut BODLST = StackArray::<i32, 10>::new(1..=NBOD);
        let mut BODYID: i32 = 0;
        let mut FRAMID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut SRFSET = ActualArray::<i32>::new(LBCELL..=MAXSIZ);
        let mut UNIT: i32 = 0;
        let mut XBDSET = ActualArray::<i32>::new(LBCELL..=MAXSIZ);
        let mut XN: i32 = 0;
        let mut XSFSET = ActualArray::<i32>::new(LBCELL..=MAXSIZ);
        let mut FOUND: bool = false;

        Self {
            FRNAME,
            BODSET,
            BODLST,
            BODYID,
            FRAMID,
            HANDLE,
            J,
            K,
            N,
            SRFSET,
            UNIT,
            XBDSET,
            XN,
            XSFSET,
            FOUND,
        }
    }
}

//$Procedure      F_DSKOBJ ( Test DSK body/surface coverage routines )
pub fn F_DSKOBJ(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DSKOBJ", ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create DSK files", ctx)?;

    //
    // Start fresh. Since we're going to create a DSK and
    // then append to it, we don't want to create an ever-growing
    // monster file.
    //
    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create a body list.
    //
    for I in 1..=(NBOD - 1) {
        save.BODLST[I] = ((100 * I) + 99);
    }

    save.BODLST[NBOD] = 10;

    //
    // Create a DSK file containing NDSK0 segments.
    //
    for I in 1..=NDSK0 {
        //
        // Generate a body ID for the Ith segment. Pick
        // a body from the body list. Recall we need to
        // restrict the body to the set for which there
        // are built-in body-fixed frame definitions.
        //
        save.J = (1 + intrinsics::MOD((I - 1), NBOD));
        save.BODYID = save.BODLST[save.J];

        spicelib::CIDFRM(
            save.BODYID,
            &mut save.FRAMID,
            &mut save.FRNAME,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        testutil::T_SMLDSK(save.BODYID, I, &save.FRNAME, DSK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create a DSK file containing NDSK1 segments.
    //
    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NDSK1 {
        //
        // Generate a body ID for the Ith segment. Pick
        // a body from the body list. Recall we need to
        // restrict the body to the set for which there
        // are built-in body-fixed frame definitions.
        //
        save.J = (1 + intrinsics::MOD((I - 1), NBOD));
        save.BODYID = save.BODLST[save.J];

        spicelib::CIDFRM(
            save.BODYID,
            &mut save.FRAMID,
            &mut save.FRNAME,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        testutil::T_SMLDSK(save.BODYID, -I, &save.FRNAME, DSK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check the body set for DSK0.", ctx)?;

    //
    // Create the expected body set.
    //
    spicelib::SSIZEI(MAXSIZ, save.XBDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NBOD {
        spicelib::INSRTI(save.BODLST[I], save.XBDSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Get the body set for DSK0.
    //
    spicelib::SSIZEI(MAXSIZ, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKOBJ(DSK0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check set cardinality.
    //
    save.N = spicelib::CARDI(save.BODSET.as_slice(), ctx)?;
    save.XN = NBOD;

    testutil::CHCKSI(b"CARDI(BODSET)", save.N, b"=", save.XN, 0, OK, ctx)?;

    if *OK {
        //
        // Check the body set.
        //
        testutil::CHCKAI(
            b"BODSET",
            save.BODSET.subarray(1),
            b"=",
            save.XBDSET.subarray(1),
            save.XN,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check the body set for DSK1.", ctx)?;

    //
    // Create the expected body set.
    //
    spicelib::SSIZEI(MAXSIZ, save.XBDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NBOD {
        spicelib::INSRTI(save.BODLST[I], save.XBDSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Get the body set for DSK1.
    //
    spicelib::SSIZEI(MAXSIZ, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKOBJ(DSK1, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check set cardinality.
    //
    save.N = spicelib::CARDI(save.BODSET.as_slice(), ctx)?;
    save.XN = NBOD;

    testutil::CHCKSI(b"CARDI(BODSET)", save.N, b"=", save.XN, 0, OK, ctx)?;

    if *OK {
        //
        // Check the body set.
        //
        testutil::CHCKAI(
            b"BODSET",
            save.BODSET.subarray(1),
            b"=",
            save.XBDSET.subarray(1),
            save.XN,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check the surface set for each body in DSK0.", ctx)?;

    spicelib::SSIZEI(MAXSIZ, save.SRFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI(MAXSIZ, save.XSFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDI(0, save.BODSET.as_slice_mut(), ctx)?;

    spicelib::DSKOBJ(DSK0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=spicelib::CARDI(save.BODSET.as_slice(), ctx)? {
        spicelib::SCARDI(0, save.SRFSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create the expected surface set.
        //
        spicelib::SCARDI(0, save.XSFSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.BODYID = save.BODSET[I];

        save.J = spicelib::ISRCHI(save.BODYID, NBOD, save.BODLST.as_slice());
        save.K = save.J;

        while (save.K <= NDSK0) {
            spicelib::INSRTI(save.K, save.XSFSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.K = (save.K + NBOD);
        }

        save.XN = spicelib::CARDI(save.XSFSET.as_slice(), ctx)?;

        spicelib::DSKSRF(DSK0, save.BODSET[I], save.SRFSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the cardinality of the surface set.
        //
        save.N = spicelib::CARDI(save.SRFSET.as_slice(), ctx)?;

        testutil::CHCKSI(b"CARDI(SRFSET)", save.N, b"=", save.XN, 0, OK, ctx)?;

        //
        // Check the surface set itself.
        //
        if *OK {
            testutil::CHCKAI(
                b"SRFSET",
                save.SRFSET.subarray(1),
                b"=",
                save.XSFSET.subarray(1),
                save.XN,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check the surface set for each body in DSK1.", ctx)?;

    spicelib::SSIZEI(MAXSIZ, save.SRFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI(MAXSIZ, save.XSFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDI(0, save.BODSET.as_slice_mut(), ctx)?;

    spicelib::DSKOBJ(DSK1, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=spicelib::CARDI(save.BODSET.as_slice(), ctx)? {
        spicelib::SCARDI(0, save.SRFSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create the expected surface set.
        //
        spicelib::SCARDI(0, save.XSFSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.BODYID = save.BODSET[I];

        save.J = spicelib::ISRCHI(save.BODYID, NBOD, save.BODLST.as_slice());
        save.K = save.J;

        while (save.K <= NDSK1) {
            spicelib::INSRTI(-save.K, save.XSFSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.K = (save.K + NBOD);
        }

        save.XN = spicelib::CARDI(save.XSFSET.as_slice(), ctx)?;

        spicelib::DSKSRF(DSK1, save.BODSET[I], save.SRFSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the cardinality of the surface set.
        //
        save.N = spicelib::CARDI(save.SRFSET.as_slice(), ctx)?;

        testutil::CHCKSI(b"CARDI(SRFSET)", save.N, b"=", save.XN, 0, OK, ctx)?;

        //
        // Check the surface set itself.
        //
        if *OK {
            testutil::CHCKAI(
                b"SRFSET",
                save.SRFSET.subarray(1),
                b"=",
                save.XSFSET.subarray(1),
                save.XN,
                OK,
                ctx,
            )?;
        }
    }

    //***********************************************************************
    //
    //
    //     DSKOBJ error cases
    //
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKOBJ: output cell is too small.", ctx)?;

    spicelib::SSIZEI(3, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKOBJ(DSK0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(CELLTOOSMALL)", OK, ctx)?;

    //
    // This call will leave the DSK closed.
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKOBJ: DSK doesn\'t exist.", ctx)?;

    spicelib::SCARDI(0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKOBJ(b"XXX", save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILENOTFOUND)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKOBJ: file is a non-DSK DAS.", ctx)?;

    testutil::TSTEK(EK0, 1, 10, false, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDI(0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKOBJ(EK0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKOBJ: file is a DAF.", ctx)?;

    testutil::TSTSPK(SPK0, false, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKOBJ(SPK0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARCHTYPE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKOBJ: file is an XFR file.", ctx)?;

    if spicelib::EXISTS(XFR0, ctx)? {
        spicelib::DELFIL(XFR0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::TXTOPN(XFR0, &mut save.UNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASBT(DSK0, save.UNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(save.UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::DSKOBJ(XFR0, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFORMAT)", OK, ctx)?;

    //***********************************************************************
    //
    //
    //     DSKSRF error cases
    //
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKSRF: DSK doesn\'t exist.", ctx)?;

    spicelib::SCARDI(0, save.SRFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKSRF(b"XXX", 1, save.SRFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILENOTFOUND)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKSRF: file is a non-DSK DAS.", ctx)?;

    spicelib::DSKSRF(EK0, 1, save.SRFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKSRF: file is a DAF.", ctx)?;

    spicelib::DSKSRF(SPK0, 1, save.SRFSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARCHTYPE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKSRF: file is an XFR file.", ctx)?;

    spicelib::DSKSRF(XFR0, 1, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFORMAT)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKSRF: output cell is too small.", ctx)?;

    spicelib::SSIZEI(3, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKSRF(DSK1, 199, save.BODSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(CELLTOOSMALL)", OK, ctx)?;

    //
    // This call will leave the DSK closed.
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Clean up: delete kernels.", ctx)?;

    spicelib::DELFIL(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(XFR0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(EK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
