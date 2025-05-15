//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TXT: &[u8] = b"test.ker";
const LBCELL: i32 = -5;
const TABCDE: i32 = 9;
const LNSIZE: i32 = 60;
const MAXBUF: i32 = 21;

//$Procedure F_TABTXT ( Text kernel tab processing tests )
pub fn F_TABTXT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FILBUF = ActualCharArray::new(LNSIZE, LBCELL..=MAXBUF);
    let mut TABBUF = ActualCharArray::new(LNSIZE, LBCELL..=MAXBUF);
    let mut KV1 = ActualCharArray::new(LNSIZE, 1..=3);
    let mut XCVAR = ActualCharArray::new(LNSIZE, 1..=3);
    let mut KV2 = StackArray::<f64, 3>::new(1..=3);
    let mut KV3 = StackArray::<f64, 3>::new(1..=3);
    let mut XDPVAR = StackArray::<f64, 3>::new(1..=3);
    let mut I: i32 = 0;
    let mut N: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;
    let mut BEGDATC = [b' '; 20 as usize];
    let mut BEGTXTC = [b' '; 20 as usize];

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_TABTXT", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Init: create a normal text kernel file.", ctx)?;

    //
    // Initialize cell arrays, so Valgrind won't complain about
    // copying cells.
    //
    {
        let m1__: i32 = LBCELL;
        let m2__: i32 = MAXBUF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(FILBUF.get_mut(I), b" ");
            fstr::assign(TABBUF.get_mut(I), b" ");
            I += m3__;
        }
    }

    //
    // Rather than create a text kernel using an existing utility
    // routine, we create the file in-line so that we can ensure
    // the file has an instance of each structural feature.
    //
    spicelib::SSIZEC(MAXBUF, FILBUF.as_arg_mut(), ctx)?;

    testutil::BEGDAT(&mut BEGDATC);
    testutil::BEGTXT(&mut BEGTXTC);

    fstr::assign(FILBUF.get_mut(1), b" ");
    fstr::assign(FILBUF.get_mut(2), b" KPL/PCK");
    fstr::assign(FILBUF.get_mut(3), b" ");
    fstr::assign(FILBUF.get_mut(4), &fstr::concat(b" ", &BEGDATC));
    fstr::assign(FILBUF.get_mut(5), b" ");
    fstr::assign(FILBUF.get_mut(6), b" KV1 = ( \' A \', \' B \', \' C \' )");
    fstr::assign(FILBUF.get_mut(7), b" KV2 = ( 1.0, 2.0, 3.0 )");
    fstr::assign(FILBUF.get_mut(8), b" ");
    fstr::assign(FILBUF.get_mut(9), &fstr::concat(b" ", &BEGTXTC));
    fstr::assign(FILBUF.get_mut(10), b" ");
    fstr::assign(FILBUF.get_mut(11), b" Comment line 1");
    fstr::assign(FILBUF.get_mut(12), b" Comment line 2");
    fstr::assign(FILBUF.get_mut(13), b" Comment line 3");
    fstr::assign(FILBUF.get_mut(14), b" ");
    fstr::assign(FILBUF.get_mut(15), &fstr::concat(b" ", &BEGDATC));
    fstr::assign(FILBUF.get_mut(16), b" KV3 = ( -1.0, -2.0, ");
    fstr::assign(FILBUF.get_mut(17), b" ");
    fstr::assign(FILBUF.get_mut(18), b" -3.0 )");
    fstr::assign(FILBUF.get_mut(19), b" ");
    fstr::assign(FILBUF.get_mut(20), &fstr::concat(b" ", &BEGTXTC));
    fstr::assign(FILBUF.get_mut(21), b" ");

    spicelib::SCARDC(MAXBUF, FILBUF.as_arg_mut(), ctx)?;

    //
    // Open and write the file.
    //
    spicelib::TXTOPN(TXT, &mut UNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        I = 1;

        while (*OK && (I <= spicelib::CARDC(FILBUF.as_arg(), ctx)?)) {
            spicelib::WRITLN(&FILBUF[I], UNIT, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            I = (I + 1);
        }
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Control case: load normal file and check.", ctx)?;

    spicelib::LDPOOL(TXT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the string variable KV1.
    //
    spicelib::GCPOOL(b"KV1", 1, 3, &mut N, KV1.as_arg_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if FOUND {
        testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

        fstr::assign(XCVAR.get_mut(1), b" A");
        fstr::assign(XCVAR.get_mut(2), b" B");
        fstr::assign(XCVAR.get_mut(3), b" C");

        testutil::CHCKAC(b"KV1", KV1.as_arg(), b"=", XCVAR.as_arg(), 3, OK, ctx)?;
    }

    //
    // Check the d.p. variable KV2.
    //
    spicelib::GDPOOL(b"KV2", 1, 3, &mut N, KV2.as_slice_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if FOUND {
        testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

        XDPVAR[1] = 1.0;
        XDPVAR[2] = 2.0;
        XDPVAR[3] = 3.0;

        testutil::CHCKAD(
            b"KV2",
            KV2.as_slice(),
            b"=",
            XDPVAR.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Check the d.p. variable KV3.
    //
    spicelib::GDPOOL(b"KV3", 1, 3, &mut N, KV3.as_slice_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if FOUND {
        testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

        XDPVAR[1] = -1.0;
        XDPVAR[2] = -2.0;
        XDPVAR[3] = -3.0;

        testutil::CHCKAD(
            b"KV3",
            KV3.as_slice(),
            b"=",
            XDPVAR.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Money case: load tab-ridden file and check.", ctx)?;

    //
    // TABBUF will contain the same string data as FILBUF, except
    // that every blank in FILBUF is replaced by a tab character.
    //
    spicelib::SSIZEC(MAXBUF, TABBUF.as_arg_mut(), ctx)?;

    spicelib::COPYC(FILBUF.as_arg(), TABBUF.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(TABBUF.as_arg(), ctx)?;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPLCH(
                &TABBUF[I].to_vec(),
                b" ",
                &intrinsics::CHAR(TABCDE),
                &mut TABBUF[I],
            );

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The following lines of code, which are intentionally commented
            // out, can be used to verify that tab characters have been
            // inserted where expected. This can be useful because
            // applications one might use to view the file we're creating
            // don't necessarily display all tab characters as a fixed set of
            // consecutive blank spaces. For example, the programs cat, more,
            // vi, and emacs, at least as configured on the author's
            // computer, all show a single blank space between the words
            // "Comment" and "line", even though a tab character is actually
            // present between them, while other tabs are shown as multiple
            // consecutive blanks.
            //
            //  CALL REPLCH ( TABBUF(I), CHAR(TABCDE), '@', TABBUF(I) )
            //  WRITE (*,*) TABBUF(I)
            //  CALL REPLCH ( TABBUF(I), '@', CHAR(TABCDE), TABBUF(I) )
            //
            //
            // The output created by these lines is shown below.
            //
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @KPL/PCK@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @\begindata@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @KV1@=@(@'@A@',@'@B@',@'@C@'@)@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @KV2@=@(@1.0,@2.0,@3.0@)@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @\begintext@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @Comment@line@1@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @Comment@line@2@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @Comment@line@3@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @\begindata@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @KV3@=@(@-1.0,@-2.0,@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @-3.0@)@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @\begintext@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            //

            I += m3__;
        }
    }

    //
    // Empty the kernel pool. Delete the normal kernel.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TXT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open and write the new file containing tabs.
    //
    spicelib::TXTOPN(TXT, &mut UNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        I = 1;

        while (*OK && (I <= spicelib::CARDC(TABBUF.as_arg(), ctx)?)) {
            spicelib::WRITLN(&TABBUF[I], UNIT, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            I = (I + 1);
        }
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // Load the new kernel. We don't expect to hit any
    // SPICE errors, even though the file contains tabs.
    //
    spicelib::LDPOOL(TXT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the string variable KV1.
    //
    spicelib::GCPOOL(b"KV1", 1, 3, &mut N, KV1.as_arg_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if FOUND {
        testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

        fstr::assign(XCVAR.get_mut(1), b" A");
        fstr::assign(XCVAR.get_mut(2), b" B");
        fstr::assign(XCVAR.get_mut(3), b" C");

        //
        // On some systems, each tab in an input text line will be
        // replaced by one or more blanks on input, before RDKER has a
        // chance to process the line. This can cause the values
        // of the string variables to have extra leading blanks.
        // Compress out any such blanks.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::CMPRSS(b" ", 1, &KV1[I].to_vec(), &mut KV1[I]);
                I += m3__;
            }
        }

        testutil::CHCKAC(b"KV1", KV1.as_arg(), b"=", XCVAR.as_arg(), 3, OK, ctx)?;
    }

    //
    // Check the d.p. variable KV2.
    //
    spicelib::GDPOOL(b"KV2", 1, 3, &mut N, KV2.as_slice_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if FOUND {
        testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

        XDPVAR[1] = 1.0;
        XDPVAR[2] = 2.0;
        XDPVAR[3] = 3.0;

        testutil::CHCKAD(
            b"KV2",
            KV2.as_slice(),
            b"=",
            XDPVAR.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Check the d.p. variable KV3.
    //
    spicelib::GDPOOL(b"KV3", 1, 3, &mut N, KV3.as_slice_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if FOUND {
        testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

        XDPVAR[1] = -1.0;
        XDPVAR[2] = -2.0;
        XDPVAR[3] = -3.0;

        testutil::CHCKAD(
            b"KV3",
            KV3.as_slice(),
            b"=",
            XDPVAR.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up: delete text kernel file.", ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TXT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
