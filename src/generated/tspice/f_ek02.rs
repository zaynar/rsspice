//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ADSCSZ: i32 = 6;
const ATTCLS: i32 = 1;
const ATTTYP: i32 = (ATTCLS + 1);
const ATTLEN: i32 = (ATTTYP + 1);
const ATTSIZ: i32 = (ATTLEN + 1);
const ATTIDX: i32 = (ATTSIZ + 1);
const ATTNFL: i32 = (ATTIDX + 1);
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CNAMSZ: i32 = 32;
const MXCLSG: i32 = 100;
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const EK1: &[u8] = b"test1.ek";
const IFNAME: &[u8] = b"Test EK";
const TABLE: &[u8] = b"TEST_DATA";
const DECLEN: i32 = 200;
const IFNLEN: i32 = 60;
const LNSIZE: i32 = 80;
const MAXROW: i32 = 20;
const CVALSZ: i32 = MAXSTR;
const CVLEN: i32 = 80;
const NCOLS: i32 = 6;
const NROWS: i32 = 100;
const NRESVC: i32 = 0;
const MAXVAL: i32 = 10;
const MSGLEN: i32 = 240;

//$Procedure F_EK02 ( EK tests, subset 2 )
pub fn F_EK02(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CDECLS = ActualCharArray::new(DECLEN, 1..=MXCLSG);
    let mut CNAMES = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
    let mut CVALS = ActualCharArray::new(CVLEN, 1..=MAXVAL);
    let mut XCVALS = ActualCharArray::new(CVLEN, 1..=MAXVAL);
    let mut LABEL = [b' '; CVLEN as usize];
    let mut MSG = [b' '; MSGLEN as usize];
    let mut DVALS = StackArray::<f64, 10>::new(1..=MAXVAL);
    let mut XDVALS = StackArray::<f64, 10>::new(1..=MAXVAL);
    let mut HANDLE: i32 = 0;
    let mut IVALS = StackArray::<i32, 10>::new(1..=MAXVAL);
    let mut NVALS: i32 = 0;
    let mut XIVALS = StackArray::<i32, 10>::new(1..=MAXVAL);
    let mut RECNO: i32 = 0;
    let mut SEGNO: i32 = 0;
    let mut ISNULL: bool = false;
    let mut XNULL: bool = false;

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
    // Open the test family.
    //
    testutil::TOPEN(b"F_EK02", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a new EK using the record-oriented writing routines.",
        ctx,
    )?;

    //
    // Load a leapseconds kernel for UTC/ET conversion.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open a new EK file. For simplicity, we will not reserve any space
    // for the comment area, so the number of reserved comment
    // characters is zero. The constant IFNAME is the internal file
    // name.
    //
    if spicelib::EXISTS(EK1, ctx)? {
        spicelib::DELFIL(EK1, ctx)?;
    }

    //
    // Create an EK.
    //
    spicelib::EKOPN(EK1, IFNAME, NRESVC, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the table and column names and declarations for the first
    // segment.  We'll index all of the scalar columns.
    //
    fstr::assign(CNAMES.get_mut(1), b"INT_COL_1");
    fstr::assign(
        CDECLS.get_mut(1),
        b"DATATYPE = INTEGER, INDEXED  = TRUE, NULLS_OK = TRUE",
    );

    fstr::assign(CNAMES.get_mut(2), b"DP_COL_1");
    fstr::assign(
        CDECLS.get_mut(2),
        b"DATATYPE = DOUBLE PRECISION, INDEXED  = TRUE, NULLS_OK = TRUE",
    );

    fstr::assign(CNAMES.get_mut(3), b"CHR_COL_1");
    fstr::assign(
        CDECLS.get_mut(3),
        b"DATATYPE = CHARACTER*(*), INDEXED  = TRUE, NULLS_OK = TRUE",
    );

    fstr::assign(CNAMES.get_mut(4), b"INT_COL_2");
    fstr::assign(
        CDECLS.get_mut(4),
        b"DATATYPE = INTEGER, SIZE     = VARIABLE, NULLS_OK = TRUE",
    );

    fstr::assign(CNAMES.get_mut(5), b"DP_COL_2");
    fstr::assign(
        CDECLS.get_mut(5),
        b"DATATYPE = DOUBLE PRECISION, SIZE     = VARIABLE, NULLS_OK = TRUE",
    );

    fstr::assign(CNAMES.get_mut(6), b"CHR_COL_2");
    fstr::assign(
        CDECLS.get_mut(6),
        b"DATATYPE = CHARACTER*(80), SIZE     = VARIABLE, NULLS_OK = TRUE",
    );

    //
    // Start the segment.
    //
    spicelib::EKBSEG(
        HANDLE,
        TABLE,
        NCOLS,
        CNAMES.as_arg(),
        CDECLS.as_arg(),
        &mut SEGNO,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load values into the columns.
    //
    for I in 0..=(NROWS - 1) {
        //
        // Append an empty record to the segment.
        //
        spicelib::EKAPPR(HANDLE, SEGNO, &mut RECNO, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Add an integer column entry.
        //
        IVALS[1] = I;
        ISNULL = (I == 1);

        spicelib::EKACEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[1],
            1,
            IVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Add a d.p. column entry.
        //
        DVALS[1] = I as f64;
        ISNULL = (I == 1);

        spicelib::EKACED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[2],
            1,
            DVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Add a character column entry.
        //
        spicelib::INTSTR(I, &mut CVALS[1], ctx);
        ISNULL = (I == 1);

        spicelib::EKACEC(
            HANDLE,
            SEGNO,
            RECNO,
            &CNAMES[3],
            1,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Array-valued columns follow.
        //
        IVALS[1] = (10 * I);
        IVALS[2] = ((10 * I) + 1);
        ISNULL = (I == 1);

        spicelib::EKACEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[4],
            2,
            IVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        DVALS[1] = (10 * I) as f64;
        DVALS[2] = ((10 * I) + 1) as f64;
        DVALS[3] = ((10 * I) + 1) as f64;
        ISNULL = (I == 1);

        spicelib::EKACED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[5],
            3,
            DVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INTSTR((10 * I), &mut CVALS[1], ctx);
        spicelib::INTSTR(((10 * I) + 1), &mut CVALS[2], ctx);
        spicelib::INTSTR(((10 * I) + 2), &mut CVALS[3], ctx);
        spicelib::INTSTR(((10 * I) + 3), &mut CVALS[4], ctx);
        ISNULL = (I == 1);

        spicelib::EKACEC(
            HANDLE,
            SEGNO,
            RECNO,
            &CNAMES[6],
            4,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // End the file.
    //
    spicelib::EKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Update the EK: knock out the odd-numbered records.", ctx)?;

    //
    // Open the file for write access.
    //
    spicelib::EKOPW(EK1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Knock out all of the records containing even numbers.
    //
    SEGNO = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (NROWS / 2);
        let m3__: i32 = 1;
        RECNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::EKDELR(HANDLE, SEGNO, &mut RECNO, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            RECNO += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Replace the missing records with records containing the negatives of the original values.", ctx)?;

    for I in intrinsics::range(0, (NROWS - 1), 2) {
        RECNO = (I + 1);
        //
        // Insert a record at index RECNO.
        //
        spicelib::EKINSR(HANDLE, SEGNO, RECNO, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Add a scalar integer column entry.
        //
        IVALS[1] = -I;
        ISNULL = (I == 1);

        spicelib::EKACEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[1],
            1,
            IVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Add a d.p. column entry.
        //
        DVALS[1] = -I as f64;
        ISNULL = (I == 1);

        spicelib::EKACED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[2],
            1,
            DVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Add a character column entry.
        //
        spicelib::INTSTR(-I, &mut CVALS[1], ctx);
        ISNULL = (I == 1);

        spicelib::EKACEC(
            HANDLE,
            SEGNO,
            RECNO,
            &CNAMES[3],
            1,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Array-valued columns follow.
        //
        IVALS[1] = -(10 * I);
        IVALS[2] = -((10 * I) + 1);
        ISNULL = (I == 1);

        spicelib::EKACEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[4],
            2,
            IVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        DVALS[1] = -(10 * I) as f64;
        DVALS[2] = -((10 * I) + 1) as f64;
        DVALS[3] = -((10 * I) + 1) as f64;
        ISNULL = (I == 1);

        spicelib::EKACED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[5],
            3,
            DVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INTSTR(-(10 * I), &mut CVALS[1], ctx);
        spicelib::INTSTR(-((10 * I) + 1), &mut CVALS[2], ctx);
        spicelib::INTSTR(-((10 * I) + 2), &mut CVALS[3], ctx);
        spicelib::INTSTR(-((10 * I) + 3), &mut CVALS[4], ctx);
        ISNULL = (I == 1);

        spicelib::EKACEC(
            HANDLE,
            SEGNO,
            RECNO,
            &CNAMES[6],
            4,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Negate the values in the even-numbered records using the update routines.",
        ctx,
    )?;

    for I in intrinsics::range(1, (NROWS - 1), 2) {
        RECNO = (I + 1);

        //
        // Update the scalar integer column entry.
        //
        IVALS[1] = -I;
        ISNULL = (I == 1);

        spicelib::EKUCEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[1],
            1,
            IVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Update the d.p. column entry.
        //
        DVALS[1] = -I as f64;
        ISNULL = (I == 1);

        spicelib::EKUCED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[2],
            1,
            DVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Update the character column entry.
        //
        spicelib::INTSTR(-I, &mut CVALS[1], ctx);
        ISNULL = (I == 1);

        spicelib::EKUCEC(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[3],
            1,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Array-valued columns follow.
        //
        IVALS[1] = -(10 * I);
        IVALS[2] = -((10 * I) + 1);
        ISNULL = (I == 1);

        spicelib::EKUCEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[4],
            2,
            IVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        DVALS[1] = -(10 * I) as f64;
        DVALS[2] = -((10 * I) + 1) as f64;
        DVALS[3] = -((10 * I) + 1) as f64;
        ISNULL = (I == 1);

        spicelib::EKUCED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[5],
            3,
            DVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INTSTR(-(10 * I), &mut CVALS[1], ctx);
        spicelib::INTSTR(-((10 * I) + 1), &mut CVALS[2], ctx);
        spicelib::INTSTR(-((10 * I) + 2), &mut CVALS[3], ctx);
        spicelib::INTSTR(-((10 * I) + 3), &mut CVALS[4], ctx);
        ISNULL = (I == 1);

        spicelib::EKUCEC(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[6],
            4,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // End the file.
    //
    spicelib::EKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Open the file for read access; check the values we've written.
    //

    testutil::TCASE(
        b"Opening the file for read access; checking values using the EK low-level readers.",
        ctx,
    )?;

    spicelib::EKOPR(EK1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 0..=(NROWS - 1) {
        //
        // --- Case: ------------------------------------------------------
        //
        RECNO = (I + 1);

        fstr::assign(&mut MSG, b"Checking row number #.");
        spicelib::REPMI(&MSG.clone(), b"#", RECNO, &mut MSG, ctx);

        testutil::TCASE(&MSG, ctx)?;

        XIVALS[1] = -I;
        XNULL = (I == 1);

        //
        // Check the scalar integer column entry.
        //
        spicelib::EKRCEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[1],
            &mut NVALS,
            IVALS.as_slice_mut(),
            &mut ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"Null flag", ISNULL, XNULL, OK, ctx)?;
        testutil::CHCKSI(b"NVALS", NVALS, b"=", 1, 0, OK, ctx)?;

        if !ISNULL {
            fstr::assign(&mut LABEL, b"Column");
            spicelib::SUFFIX(&CNAMES[1], 1, &mut LABEL);

            testutil::CHCKSI(&LABEL, IVALS[1], b"=", XIVALS[1], 0, OK, ctx)?;
        }

        //
        // Check the d.p. column entry.
        //
        XDVALS[1] = -I as f64;
        XNULL = (I == 1);

        spicelib::EKRCED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[2],
            &mut NVALS,
            DVALS.as_slice_mut(),
            &mut ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"Null flag", ISNULL, XNULL, OK, ctx)?;
        testutil::CHCKSI(b"NVALS", NVALS, b"=", 1, 0, OK, ctx)?;

        if !ISNULL {
            fstr::assign(&mut LABEL, b"Column");
            spicelib::SUFFIX(&CNAMES[2], 1, &mut LABEL);

            testutil::CHCKSD(&LABEL, DVALS[1], b"=", XDVALS[1], 0.0, OK, ctx)?;
        }

        //
        // Check the character column entry.
        //
        spicelib::INTSTR(-I, &mut XCVALS[1], ctx);
        XNULL = (I == 1);

        spicelib::EKRCEC(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[3],
            &mut NVALS,
            CVALS.as_arg_mut(),
            &mut ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"Null flag", ISNULL, XNULL, OK, ctx)?;
        testutil::CHCKSI(b"NVALS", NVALS, b"=", 1, 0, OK, ctx)?;

        if !ISNULL {
            fstr::assign(&mut LABEL, b"Column");
            spicelib::SUFFIX(&CNAMES[3], 1, &mut LABEL);

            testutil::CHCKSC(&LABEL, &CVALS[1], b"=", &XCVALS[1], OK, ctx)?;
        }

        //
        // Array-valued columns follow.
        //
        XIVALS[1] = -(10 * I);
        XIVALS[2] = -((10 * I) + 1);
        XNULL = (I == 1);

        spicelib::EKRCEI(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[4],
            &mut NVALS,
            IVALS.as_slice_mut(),
            &mut ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if ISNULL {
            testutil::CHCKSI(b"NVALS", NVALS, b"=", 1, 0, OK, ctx)?;
        } else {
            testutil::CHCKSI(b"NVALS", NVALS, b"=", 2, 0, OK, ctx)?;

            fstr::assign(&mut LABEL, b"Column");
            spicelib::SUFFIX(&CNAMES[4], 1, &mut LABEL);

            testutil::CHCKAI(
                &LABEL,
                IVALS.as_slice(),
                b"=",
                XIVALS.as_slice(),
                2,
                OK,
                ctx,
            )?;
        }

        XDVALS[1] = -(10 * I) as f64;
        XDVALS[2] = -((10 * I) + 1) as f64;
        XDVALS[3] = -((10 * I) + 1) as f64;
        XNULL = (I == 1);

        spicelib::EKRCED(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[5],
            &mut NVALS,
            DVALS.as_slice_mut(),
            &mut ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if ISNULL {
            testutil::CHCKSI(b"NVALS", NVALS, b"=", 1, 0, OK, ctx)?;
        } else {
            testutil::CHCKSI(b"NVALS", NVALS, b"=", 3, 0, OK, ctx)?;

            fstr::assign(&mut LABEL, b"Column");
            spicelib::SUFFIX(&CNAMES[5], 1, &mut LABEL);

            testutil::CHCKAD(
                &LABEL,
                DVALS.as_slice(),
                b"=",
                XDVALS.as_slice(),
                3,
                0.0,
                OK,
                ctx,
            )?;
        }

        spicelib::INTSTR(-(10 * I), &mut XCVALS[1], ctx);
        spicelib::INTSTR(-((10 * I) + 1), &mut XCVALS[2], ctx);
        spicelib::INTSTR(-((10 * I) + 2), &mut XCVALS[3], ctx);
        spicelib::INTSTR(-((10 * I) + 3), &mut XCVALS[4], ctx);
        XNULL = (I == 1);

        spicelib::EKRCEC(
            HANDLE,
            &mut SEGNO,
            RECNO,
            &CNAMES[6],
            &mut NVALS,
            CVALS.as_arg_mut(),
            &mut ISNULL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if ISNULL {
            testutil::CHCKSI(b"NVALS", NVALS, b"=", 1, 0, OK, ctx)?;
        } else {
            testutil::CHCKSI(b"NVALS", NVALS, b"=", 4, 0, OK, ctx)?;

            fstr::assign(&mut LABEL, b"Column");
            spicelib::SUFFIX(&CNAMES[6], 1, &mut LABEL);

            for J in 1..=NVALS {
                testutil::CHCKSC(&LABEL, &CVALS[J], b"=", &XCVALS[J], OK, ctx)?;
            }
        }
    }

    //
    // End the file.
    //
    spicelib::EKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Unload the EK.  The TSPICE system will delete the file.
    //
    testutil::TCASE(b"Unload EK from query system.", ctx)?;

    spicelib::UNLOAD(EK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
