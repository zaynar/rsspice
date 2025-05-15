//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NCK: i32 = 30;
const NIDS: i32 = ((NCK * 2) + 13);
const WDSIZE: i32 = 32;

struct SaveVars {
    SCLKWS: ActualCharArray,
    SCLKWB: ActualCharArray,
    SPKKWS: ActualCharArray,
    SPKKWB: ActualCharArray,
    KEYWRD: Vec<u8>,
    CKSM: StackArray<i32, 73>,
    CKBG: StackArray<i32, 73>,
    SCLKSM: StackArray<i32, 73>,
    SCLKBG: StackArray<i32, 73>,
    SCLKPS: StackArray<i32, 73>,
    SCLKPB: StackArray<i32, 73>,
    SPKSM: StackArray<i32, 73>,
    SPKBG: StackArray<i32, 73>,
    SPKPS: StackArray<i32, 73>,
    SPKPB: StackArray<i32, 73>,
    IDCODE: i32,
    SCLKID: i32,
    SPKID: i32,
    SUNIT: i32,
    J: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SCLKWS = ActualCharArray::new(WDSIZE, 1..=NIDS);
        let mut SCLKWB = ActualCharArray::new(WDSIZE, 1..=NIDS);
        let mut SPKKWS = ActualCharArray::new(WDSIZE, 1..=NIDS);
        let mut SPKKWB = ActualCharArray::new(WDSIZE, 1..=NIDS);
        let mut KEYWRD = vec![b' '; (WDSIZE * 2) as usize];
        let mut CKSM = StackArray::<i32, 73>::new(1..=NIDS);
        let mut CKBG = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SCLKSM = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SCLKBG = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SCLKPS = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SCLKPB = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SPKSM = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SPKBG = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SPKPS = StackArray::<i32, 73>::new(1..=NIDS);
        let mut SPKPB = StackArray::<i32, 73>::new(1..=NIDS);
        let mut IDCODE: i32 = 0;
        let mut SCLKID: i32 = 0;
        let mut SPKID: i32 = 0;
        let mut SUNIT: i32 = 0;
        let mut J: i32 = 0;

        Self {
            SCLKWS,
            SCLKWB,
            SPKKWS,
            SPKKWB,
            KEYWRD,
            CKSM,
            CKBG,
            SCLKSM,
            SCLKBG,
            SCLKPS,
            SCLKPB,
            SPKSM,
            SPKBG,
            SPKPS,
            SPKPB,
            IDCODE,
            SCLKID,
            SPKID,
            SUNIT,
            J,
        }
    }
}

//$Procedure F_CKMETA ( Family of CKMETA tests )
pub fn F_CKMETA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters.
    //
    // NCK is the size of the buffer in CKMETA
    //

    //
    // NIDS is the number of IDs we will test.
    //

    //
    // Local Variables
    //

    //
    // Save everything.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CKMETA", ctx)?;

    //
    // Populate test arrays and make test text kernel.
    //
    testutil::KILFIL(b"myconns.ker", ctx)?;
    spicelib::TXTOPN(b"myconns.ker", &mut save.SUNIT, ctx)?;
    let mut arg0 = vec![b' '; 16];
    testutil::BEGDAT(&mut arg0);
    spicelib::WRITLN(&arg0, save.SUNIT, ctx)?;

    for I in 1..=NIDS {
        //
        // CK IDs greater than -1000 and CKs IDs equal to or less than
        // -1000.
        //
        save.CKSM[I] = (-1000 + I);
        save.CKBG[I] = (-(1000 * I) - intrinsics::MOD(I, 1000));

        //
        // Default SCLK and SPK IDs for CK IDs greater than -1000.
        //
        save.SCLKSM[I] = 0;
        save.SPKSM[I] = 0;

        //
        // Default SCLK and SPK IDs for CKs IDs equal to or less than
        // -1000.
        //
        save.SCLKBG[I] = -I;
        save.SPKBG[I] = -I;

        //
        // POOL IDs for SCLK and SPK IDs for CK IDs greater than -1000.
        //
        save.SCLKPS[I] = save.CKBG[I];
        save.SPKPS[I] = (save.CKBG[I] - 2);

        //
        // POOL IDs SCLK and SPK IDs for CKs IDs equal to or less than
        // -1000.
        //
        save.SCLKPB[I] = (save.CKBG[I] - 1);
        save.SPKPB[I] = (save.CKBG[I] - 3);

        //
        // POOL variables and agents.
        //
        fstr::assign(&mut save.KEYWRD, b"CK_#_SCLK = #");
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.CKSM[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.SCLKPS[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::WRITLN(&save.KEYWRD, save.SUNIT, ctx)?;
        spicelib::NTHWD(&save.KEYWRD, 1, &mut save.SCLKWS[I], &mut save.J);

        fstr::assign(&mut save.KEYWRD, b"CK_#_SPK = #");
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.CKSM[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.SPKPS[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::WRITLN(&save.KEYWRD, save.SUNIT, ctx)?;
        spicelib::NTHWD(&save.KEYWRD, 1, &mut save.SPKKWS[I], &mut save.J);

        fstr::assign(&mut save.KEYWRD, b"CK_#_SCLK = #");
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.CKBG[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.SCLKPB[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::WRITLN(&save.KEYWRD, save.SUNIT, ctx)?;
        spicelib::NTHWD(&save.KEYWRD, 1, &mut save.SCLKWB[I], &mut save.J);

        fstr::assign(&mut save.KEYWRD, b"CK_#_SPK = #");
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.CKBG[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::REPMI(
            &save.KEYWRD.to_vec(),
            b"#",
            save.SPKPB[I],
            &mut save.KEYWRD,
            ctx,
        );
        spicelib::WRITLN(&save.KEYWRD, save.SUNIT, ctx)?;
        spicelib::NTHWD(&save.KEYWRD, 1, &mut save.SPKKWB[I], &mut save.J);
    }

    let mut arg0 = vec![b' '; 16];
    testutil::BEGTXT(&mut arg0);
    spicelib::WRITLN(&arg0, save.SUNIT, ctx)?;
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(save.SUNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CLPOOL(ctx)?;

    //
    // *****************************************************************
    //
    //    Error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Test exception for unrecognized meta item.
    //
    testutil::TCASE(b"Unrecognized CKMETA item", ctx)?;

    save.IDCODE = 1;

    spicelib::CKMETA(-10000, b"IK", &mut save.IDCODE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNCKMETA)", OK, ctx)?;
    testutil::CHCKSI(b"IDCODE", save.IDCODE, b"=", 0, 0, OK, ctx)?;

    //
    // *****************************************************************
    //
    //    Normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Test default IDs.
    //
    testutil::TCASE(b"Default ID codes both SCLK and SPK.", ctx)?;

    for I in 1..=NIDS {
        spicelib::CKMETA(save.CKSM[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKSM[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKSM[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKSM[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKBG[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKBG[I], 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Test POOL IDs.
    //
    spicelib::LDPOOL(b"myconns.ker", ctx)?;

    testutil::TCASE(b"POOL ID codes for both SCLK and SPK. ", ctx)?;

    for I in 1..=NIDS {
        spicelib::CKMETA(save.CKSM[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKPS[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKSM[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKPS[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKPB[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKPB[I], 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Test default SCLK IDs and POOL SPK IDs
    //
    for I in 1..=NIDS {
        spicelib::DVPOOL(&save.SCLKWS[I], ctx)?;
        spicelib::DVPOOL(&save.SCLKWB[I], ctx)?;
    }

    testutil::TCASE(b"Default ID codes for SCLK, POOL ID codes for SPK.", ctx)?;

    for I in 1..=NIDS {
        spicelib::CKMETA(save.CKSM[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKSM[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKSM[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKPS[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKBG[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKPB[I], 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Test POOL IDs again to make sure that all watchers are set.
    //
    spicelib::CLPOOL(ctx)?;

    spicelib::LDPOOL(b"myconns.ker", ctx)?;

    testutil::TCASE(b"POOL ID codes for both SCLK and SPK, pass 2. ", ctx)?;

    for I in 1..=NIDS {
        spicelib::CKMETA(save.CKSM[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKPS[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKSM[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKPS[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKPB[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKPB[I], 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Test default SPK IDs and POOL SCLK IDs
    //
    for I in 1..=NIDS {
        spicelib::DVPOOL(&save.SPKKWS[I], ctx)?;
        spicelib::DVPOOL(&save.SPKKWB[I], ctx)?;
    }

    testutil::TCASE(b"Default ID codes for SPK, POOL ID codes for SCLK.", ctx)?;

    for I in 1..=NIDS {
        spicelib::CKMETA(save.CKSM[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKPS[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKSM[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKSM[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKPB[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKBG[I], 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Clear POOL and test default IDs again.
    //
    spicelib::CLPOOL(ctx)?;

    testutil::TCASE(b"Default ID codes both SCLK and SPK, pass 2.", ctx)?;

    for I in 1..=NIDS {
        spicelib::CKMETA(save.CKSM[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKSM[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKSM[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKSM[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SCLK", &mut save.SCLKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SCLKID", save.SCLKID, b"=", save.SCLKBG[I], 0, OK, ctx)?;

        spicelib::CKMETA(save.CKBG[I], b"SPK", &mut save.SPKID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"SPKID", save.SPKID, b"=", save.SPKBG[I], 0, OK, ctx)?;
    }

    //
    // Clean up.
    //
    testutil::KILFIL(b"myconns.ker", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
