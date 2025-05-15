//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const WDSIZE: i32 = 32;
const LNSIZE: i32 = 140;

//$Procedure F_POOL ( Family of tests for POOL.)
pub fn F_POOL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TYPE = [b' '; 1 as usize];
    let mut NAME = [b' '; 64 as usize];
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut TEXT = ActualCharArray::new(LNSIZE, 1..=100);
    let mut CVALS = ActualCharArray::new(LNSIZE, 1..=20);
    let mut CVALUE = ActualCharArray::new(LNSIZE, 1..=20);
    let mut ITEM = ActualCharArray::new(WDSIZE, 1..=3);
    let mut NAMES = ActualCharArray::new(WDSIZE, 1..=10);
    let mut VARNAM = [b' '; WDSIZE as usize];
    let mut MYSTRS = ActualCharArray::new(WDSIZE, 1..=20);
    let mut DVALS = StackArray::<f64, 1>::new(1..=1);
    let mut EVALS = StackArray::<f64, 20>::new(1..=20);
    let mut VALUES = StackArray::<f64, 20>::new(1..=20);
    let mut MYDATA = StackArray::<f64, 20>::new(1..=20);
    let mut ESIZE = StackArray::<i32, 20>::new(1..=20);
    let mut INTVAL = StackArray::<i32, 20>::new(1..=20);
    let mut IVALS = StackArray::<i32, 20>::new(1..=20);
    let mut MYINTS = StackArray::<i32, 20>::new(1..=20);
    let mut N: i32 = 0;
    let mut PTR: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;
    let mut UPDATE: bool = false;
    let mut BNAMES = ActualCharArray::new(WDSIZE, 1..=2);
    let mut NNAMES = ActualCharArray::new(WDSIZE, 1..=2);
    let mut INAMES = ActualCharArray::new(WDSIZE, 1..=2);
    let mut NBILL: i32 = 0;
    let mut NNAT: i32 = 0;
    let mut NIAN: i32 = 0;
    let mut BUPDAT: bool = false;
    let mut NUPDAT: bool = false;
    let mut IUPDAT: bool = false;
    let mut USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut USRCT1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut USRCT2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut USRCT3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut USRCT4 = StackArray::<i32, 2>::new(1..=CTRSIZ);

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // POOL state counter.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_POOL", ctx)?;

    //
    // Case 1
    //
    testutil::TCASE(b"Load and check types for numeric values.", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = PI");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11  = ( 5, 4, 3, 2 )");
    fstr::assign(TEXT.get_mut(21), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(22), b" ");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");
    fstr::assign(TEXT.get_mut(25), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 25, false, true, ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ESIZE[1] = 1;
    ESIZE[2] = 1;
    ESIZE[3] = 1;
    ESIZE[4] = 1;
    ESIZE[5] = 1;
    ESIZE[6] = 1;
    ESIZE[7] = 1;
    ESIZE[8] = 1;
    ESIZE[9] = 1;
    ESIZE[10] = 10;
    ESIZE[11] = 6;

    for I in 1..=11 {
        fstr::assign(&mut VARNAM, b"VALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);

        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"N", N, b"=", ESIZE[I], 0, OK, ctx)?;
        testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    }

    //
    // Case 2
    //
    testutil::TCASE(b"Check that loaded values are as expected.", ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 1", ctx);
    spicelib::GDPOOL(
        b"VALUE1",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::NPARSD(b"1", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE1",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 2", ctx);
    spicelib::GDPOOL(
        b"VALUE2",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"2", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE2",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 3", ctx);
    spicelib::GDPOOL(
        b"VALUE3",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"PI", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE3",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 4", ctx);
    spicelib::GDPOOL(
        b"VALUE4",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"3", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE4",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 5", ctx);
    spicelib::GDPOOL(
        b"VALUE5",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"4", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE5",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 6", ctx);
    spicelib::GDPOOL(
        b"VALUE6",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"5", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE6",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 7", ctx);
    spicelib::NPARSD(b"1.276828E+11", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE7",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE7",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 8", ctx);
    spicelib::NPARSD(
        b"-28.19729871E+12",
        &mut EVALS[1],
        &mut ERROR,
        &mut PTR,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE8",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE8",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 9", ctx);
    spicelib::TPARSE(b"1-JAN-1994", &mut EVALS[1], &mut ERROR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE9",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE9",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    EVALS[1] = 1 as f64;
    EVALS[2] = 2 as f64;
    EVALS[3] = 3 as f64;
    EVALS[4] = 4 as f64;
    EVALS[5] = 5 as f64;
    EVALS[6] = 6 as f64;
    EVALS[7] = 7 as f64;
    EVALS[8] = 8 as f64;
    EVALS[9] = 9 as f64;
    EVALS[10] = 10 as f64;

    testutil::TSTMSG(b"#", b"Checking Value 10", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        3,
        5,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE10",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(3),
        5,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 10 part 2.", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        1,
        10,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE10",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(1),
        10,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 10, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 10 part 3", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        11,
        5,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    EVALS[1] = 5 as f64;
    EVALS[2] = 4 as f64;
    EVALS[3] = 3 as f64;
    EVALS[4] = 2 as f64;
    EVALS[5] = 1 as f64;
    EVALS[6] = 0 as f64;

    testutil::TSTMSG(b"#", b"Checking Value 11", ctx);
    spicelib::GDPOOL(
        b"VALUE11",
        1,
        10,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE11",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(1),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::TSTMSG(b"#", b" ", ctx);

    //
    // Case 3
    //
    testutil::TCASE(b"Write the kernel pool out and clear the kernel pool.  Then read in the output from the preceding write and make sure everything matches. ", ctx)?;

    testutil::KILFIL(b"writdata.ker", ctx)?;
    spicelib::TXTOPN(b"writdata.ker", &mut UNIT, ctx)?;
    spicelib::WRPOOL(UNIT, ctx)?;
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }
    spicelib::CLPOOL(ctx)?;
    spicelib::LDPOOL(b"writdata.ker", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ESIZE[1] = 1;
    ESIZE[2] = 1;
    ESIZE[3] = 1;
    ESIZE[4] = 1;
    ESIZE[5] = 1;
    ESIZE[6] = 1;
    ESIZE[7] = 1;
    ESIZE[8] = 1;
    ESIZE[9] = 1;
    ESIZE[10] = 10;
    ESIZE[11] = 6;

    testutil::TSTMSG(b"#", b"Checking type and size of values.", ctx);
    for I in 1..=11 {
        testutil::TSTMSG(b"#", b"I is #", ctx);
        testutil::TSTMSI(I, ctx);

        fstr::assign(&mut VARNAM, b"VALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);

        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"N", N, b"=", ESIZE[I], 0, OK, ctx)?;
        testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    }

    testutil::TSTMSG(b"#", b"Checking Value 1", ctx);
    spicelib::GDPOOL(
        b"VALUE1",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::NPARSD(b"1", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE1",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 2", ctx);
    spicelib::GDPOOL(
        b"VALUE2",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"2", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE2",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 3", ctx);
    spicelib::GDPOOL(
        b"VALUE3",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"PI", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE3",
        VALUES.as_slice(),
        b"~/",
        EVALS.as_slice(),
        1,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 4", ctx);
    spicelib::GDPOOL(
        b"VALUE4",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"3", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE4",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 5", ctx);
    spicelib::GDPOOL(
        b"VALUE5",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"4", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE5",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 6", ctx);
    spicelib::GDPOOL(
        b"VALUE6",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"5", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE6",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 7", ctx);
    spicelib::NPARSD(b"1.276828E+11", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE7",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE7",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 8", ctx);
    spicelib::NPARSD(
        b"-28.19729871E+12",
        &mut EVALS[1],
        &mut ERROR,
        &mut PTR,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE8",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE8",
        VALUES.as_slice(),
        b"~/",
        EVALS.as_slice(),
        1,
        0.0000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 9", ctx);
    spicelib::TPARSE(b"1-JAN-1994", &mut EVALS[1], &mut ERROR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE9",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE9",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    EVALS[1] = 1 as f64;
    EVALS[2] = 2 as f64;
    EVALS[3] = 3 as f64;
    EVALS[4] = 4 as f64;
    EVALS[5] = 5 as f64;
    EVALS[6] = 6 as f64;
    EVALS[7] = 7 as f64;
    EVALS[8] = 8 as f64;
    EVALS[9] = 9 as f64;
    EVALS[10] = 10 as f64;

    testutil::TSTMSG(b"#", b"Checking Value 10", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        3,
        5,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE10",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(3),
        5,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 10 part 2", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        1,
        10,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE10",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(1),
        10,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 10, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 10 part 3", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        11,
        5,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    EVALS[1] = 5 as f64;
    EVALS[2] = 4 as f64;
    EVALS[3] = 3 as f64;
    EVALS[4] = 2 as f64;
    EVALS[5] = 1 as f64;
    EVALS[6] = 0 as f64;

    testutil::TSTMSG(b"#", b"Checking Value 11 ", ctx);
    spicelib::GDPOOL(
        b"VALUE11",
        1,
        10,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE11",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(1),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Case 4
    //
    testutil::TCASE(
        b"Make sure that CLPOOL really empties the kernel pool. ",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;

    for I in 1..=11 {
        fstr::assign(&mut VARNAM, b"VALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);

        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
        testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;
        testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    }

    //
    // Case 5
    //
    testutil::TCASE(
        b"Check out the routine RTPOOL to make sure it still works as before. ",
        ctx,
    )?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = PI");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11  = ( 5, 4, 3, 2 )");
    fstr::assign(TEXT.get_mut(21), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(22), b" ");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");
    fstr::assign(TEXT.get_mut(25), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 25, false, true, ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    spicelib::RTPOOL(b"VALUE4", &mut N, VALUES.as_slice_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N      for VALUE4", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSD(
        b"Values for VALUE4",
        *VALUES.first(),
        b"=",
        3.0,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND  for VALUE4", FOUND, true, OK, ctx)?;

    spicelib::RTPOOL(b"VALUE11", &mut N, VALUES.as_slice_mut(), &mut FOUND, ctx)?;

    EVALS[1] = 5.0;
    EVALS[2] = 4.0;
    EVALS[3] = 3.0;
    EVALS[4] = 2.0;
    EVALS[5] = 1.0;
    EVALS[6] = 0.0;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N      for VALUE11", N, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"Values for VALUE11",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND  for VALUE4", FOUND, true, OK, ctx)?;

    //
    // Case 6
    //
    testutil::TCASE(
        b"Make sure we can get integer values out of the kernel pool. ",
        ctx,
    )?;

    spicelib::GIPOOL(
        b"VALUE11",
        1,
        20,
        &mut N,
        INTVAL.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    IVALS[1] = 5;
    IVALS[2] = 4;
    IVALS[3] = 3;
    IVALS[4] = 2;
    IVALS[5] = 1;
    IVALS[6] = 0;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N for VALUE11", N, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKAI(
        b"VALUE11",
        INTVAL.as_slice(),
        b"=",
        IVALS.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // Case 7
    //
    testutil::TCASE(b"Make sure we can load and retrieve text values. ", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b" ");
    fstr::assign(TEXT.get_mut(3), b"   SVALUE1  = \'String 1\'");
    fstr::assign(TEXT.get_mut(4), b"   SVALUE2  = \'String 2\'");
    fstr::assign(TEXT.get_mut(5), b"   SVALUE3  = \'String 3\'");
    fstr::assign(TEXT.get_mut(6), b"   SVALUE4  = \'String 4\'");
    fstr::assign(TEXT.get_mut(7), b"   SVALUE5  = \'String 5\'");
    fstr::assign(
        TEXT.get_mut(8),
        b"   SVALUE6  = \'String 6.1\', \'String 6.2\', \'String 6.3\'",
    );
    fstr::assign(TEXT.get_mut(9), b"   SVALUE6  = \'String 6\'");
    fstr::assign(TEXT.get_mut(10), b" ");
    fstr::assign(TEXT.get_mut(11), b"   SVALUE7 += \'String 7.0\'");
    fstr::assign(TEXT.get_mut(12), b"   SVALUE7 += \'String 7.1\'");
    fstr::assign(TEXT.get_mut(13), b"   SVALUE7 += \'String 7.2\'");
    fstr::assign(TEXT.get_mut(14), b"   SVALUE7 += \'String 7.3\'");
    fstr::assign(TEXT.get_mut(15), b"   SVALUE7 += \'String 7.4\'");
    fstr::assign(TEXT.get_mut(16), b" ");
    fstr::assign(
        TEXT.get_mut(17),
        b"   SVALUE8 = ( \'String 8.0\', \'String 8.1\',",
    );
    fstr::assign(
        TEXT.get_mut(18),
        b"               \'String 8.2\', \'String 8.3\' )",
    );
    fstr::assign(TEXT.get_mut(19), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 19, false, true, ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ESIZE[1] = 1;
    ESIZE[2] = 1;
    ESIZE[3] = 1;
    ESIZE[4] = 1;
    ESIZE[5] = 1;
    ESIZE[6] = 1;
    ESIZE[7] = 5;
    ESIZE[8] = 4;

    for I in 1..=8 {
        fstr::assign(&mut VARNAM, b"SVALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);
        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"FOUND for ", &VARNAM));
        fstr::assign(ITEM.get_mut(2), &fstr::concat(b"N     for ", &VARNAM));
        fstr::assign(ITEM.get_mut(3), &fstr::concat(b"TYPE  for ", &VARNAM));

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(&ITEM[1], FOUND, true, OK, ctx)?;
        testutil::CHCKSI(&ITEM[2], N, b"=", ESIZE[I], 0, OK, ctx)?;
        testutil::CHCKSC(&ITEM[3], &TYPE, b"=", b"C", OK, ctx)?;
    }

    //
    // Case 8
    //
    testutil::TCASE(
        b"Check that the text items loaded in the previous test have the expected values. ",
        ctx,
    )?;

    fstr::assign(CVALS.get_mut(1), b"String 7.0");
    fstr::assign(CVALS.get_mut(2), b"String 7.1");
    fstr::assign(CVALS.get_mut(3), b"String 7.2");
    fstr::assign(CVALS.get_mut(4), b"String 7.3");
    fstr::assign(CVALS.get_mut(5), b"String 7.4");

    spicelib::GCPOOL(
        b"SVALUE7",
        1,
        20,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for SVALUE7", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for SVALUE7", N, b"=", 5, 0, OK, ctx)?;

    testutil::CHCKSC(b"Value 1 of SVALUE7", &CVALUE[1], b"=", &CVALS[1], OK, ctx)?;
    testutil::CHCKSC(b"Value 2 of SVALUE7", &CVALUE[2], b"=", &CVALS[2], OK, ctx)?;
    testutil::CHCKSC(b"Value 3 of SVALUE7", &CVALUE[3], b"=", &CVALS[3], OK, ctx)?;
    testutil::CHCKSC(b"Value 4 of SVALUE7", &CVALUE[4], b"=", &CVALS[4], OK, ctx)?;
    testutil::CHCKSC(b"Value 5 of SVALUE7", &CVALUE[5], b"=", &CVALS[5], OK, ctx)?;

    fstr::assign(CVALS.get_mut(1), b"String 8.0");
    fstr::assign(CVALS.get_mut(2), b"String 8.1");
    fstr::assign(CVALS.get_mut(3), b"String 8.2");
    fstr::assign(CVALS.get_mut(4), b"String 8.3");

    spicelib::GCPOOL(
        b"SVALUE8",
        3,
        20,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for SVALUE7", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for SVALUE7", N, b"=", 2, 0, OK, ctx)?;

    testutil::CHCKSC(b"Value 3 of SVALUE8", &CVALUE[1], b"=", &CVALS[3], OK, ctx)?;
    testutil::CHCKSC(b"Value 4 of SVALUE8", &CVALUE[2], b"=", &CVALS[4], OK, ctx)?;

    //
    // Case 9
    //
    testutil::TCASE(
        b"Make sure we don\'t get string and numeric values confused. ",
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SVALUE8",
        1,
        20,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N for d.p.", N, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"D.P. FOUND for SVALUE8", FOUND, false, OK, ctx)?;

    spicelib::GIPOOL(
        b"SVALUE8",
        1,
        20,
        &mut N,
        IVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N for Int", N, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"Integer FOUND for SVALUE8", FOUND, false, OK, ctx)?;

    spicelib::GCPOOL(
        b"VALUE1",
        1,
        20,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N for Char", N, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"Char FOUND for VALUE1", FOUND, false, OK, ctx)?;

    //
    // Case 10
    //
    testutil::TCASE(
        b"Make sure that we can write out string data via WRPOOL ",
        ctx,
    )?;

    testutil::KILFIL(b"write.dat", ctx)?;
    spicelib::TXTOPN(b"write.dat", &mut UNIT, ctx)?;
    spicelib::WRPOOL(UNIT, ctx)?;
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CLPOOL(ctx)?;

    spicelib::DTPOOL(b"SVALUE1", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(b"SVALUE2", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(b"SVALUE3", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE4", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE5", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE6", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE7", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE8", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::LDPOOL(b"write.dat", ctx)?;

    spicelib::DTPOOL(b"SVALUE1", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"C", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    spicelib::DTPOOL(b"SVALUE2", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"C", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    spicelib::DTPOOL(b"SVALUE7", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"C", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE4", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE5", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE6", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE10", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 10, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE11", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    fstr::assign(CVALS.get_mut(1), b"String 7.0");
    fstr::assign(CVALS.get_mut(2), b"String 7.1");
    fstr::assign(CVALS.get_mut(3), b"String 7.2");
    fstr::assign(CVALS.get_mut(4), b"String 7.3");
    fstr::assign(CVALS.get_mut(5), b"String 7.4");

    spicelib::GCPOOL(
        b"SVALUE7",
        1,
        20,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for SVALUE7", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for SVALUE7", N, b"=", 5, 0, OK, ctx)?;

    spicelib::RTPOOL(b"VALUE11", &mut N, VALUES.as_slice_mut(), &mut FOUND, ctx)?;

    EVALS[1] = 5.0;
    EVALS[2] = 4.0;
    EVALS[3] = 3.0;
    EVALS[4] = 2.0;
    EVALS[5] = 1.0;
    EVALS[6] = 0.0;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N      for VALUE11", N, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"Values for VALUE11",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Case 11
    //
    testutil::TCASE(
        b"Make sure we can set watchers and that they get triggered at the appropriate times. ",
        ctx,
    )?;

    //
    // Create a list of variables to be watched.
    //
    fstr::assign(NAMES.get_mut(1), b"SVALUE8");
    fstr::assign(NAMES.get_mut(2), b"VALUE10");
    fstr::assign(NAMES.get_mut(3), b"SPUD");

    //
    // Establish a watcher on the string variable and on the
    // numeric variable.
    //
    spicelib::SWPOOL(b"S_POOL", 1, NAMES.as_arg(), ctx)?;
    spicelib::SWPOOL(b"D_POOL", 1, NAMES.subarray(2), ctx)?;
    spicelib::SWPOOL(b"DUMMY", 1, NAMES.subarray(3), ctx)?;
    //
    // Check to see that S_POOL, D_POOL and DUMMY have notifications
    // pending.
    //
    spicelib::CVPOOL(b"S_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, true, OK, ctx)?;

    spicelib::CVPOOL(b"D_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for D_POOL", UPDATE, true, OK, ctx)?;

    spicelib::CVPOOL(b"DUMMY", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, true, OK, ctx)?;
    //
    // Now check that S_POOL, D_POOL and DUMMY do not have
    // notifications pending.
    //
    spicelib::CVPOOL(b"S_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"D_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"DUMMY", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"UNDECLAGENT", &mut UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UNDECLAGENT", UPDATE, false, OK, ctx)?;

    //
    // Clear the kernel pool and see if everything behaves as
    // expected.
    //
    spicelib::CLPOOL(ctx)?;

    testutil::TSTMSG(b"#", b"Same tests after clearing the pool.", ctx);
    //
    // Check to see that S_POOL, D_POOL and DUMMY have notifications
    // pending.
    //
    spicelib::CVPOOL(b"S_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, true, OK, ctx)?;

    spicelib::CVPOOL(b"D_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for D_POOL", UPDATE, true, OK, ctx)?;

    spicelib::CVPOOL(b"DUMMY", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, true, OK, ctx)?;
    //
    // Now check that S_POOL, D_POOL and DUMMY do not have
    // notifications pending.
    //
    spicelib::CVPOOL(b"S_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"D_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"DUMMY", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"UNDECLAGENT", &mut UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UNDECLAGENT", UPDATE, false, OK, ctx)?;

    //
    // Now load a kernel and see if these items get updated
    // as expected
    //
    spicelib::LDPOOL(b"write.dat", ctx)?;
    testutil::TSTMSG(b"#", b"Same watcher tests after loading a file.", ctx);

    spicelib::CVPOOL(b"S_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, true, OK, ctx)?;

    spicelib::CVPOOL(b"D_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for D_POOL", UPDATE, true, OK, ctx)?;

    spicelib::CVPOOL(b"DUMMY", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    //
    // Now check that S_POOL, D_POOL and DUMMY do not have
    // notifications pending.
    //
    spicelib::CVPOOL(b"S_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"D_POOL", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"DUMMY", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    spicelib::CVPOOL(b"UNDECLAGENT", &mut UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UNDECLAGENT", UPDATE, false, OK, ctx)?;
    testutil::TSTMSG(b"#", b" ", ctx);

    //
    // Delete watchers used in this test.
    //
    spicelib::DWPOOL(b"S_POOL", ctx)?;
    spicelib::DWPOOL(b"D_POOL", ctx)?;
    spicelib::DWPOOL(b"DUMMY", ctx)?;

    //
    // Case 12
    //
    testutil::TCASE(
        b"Make sure that RTPOOL does not work on string valued variables. ",
        ctx,
    )?;

    N = 123;
    spicelib::RTPOOL(b"SVALUE4", &mut N, VALUES.as_slice_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"N    ", N, b"=", 123, 0, OK, ctx)?;

    //
    // Case 13
    //
    testutil::TCASE(
        b"Make sure that the entry point EXPOOL works only on numeric data types. ",
        ctx,
    )?;

    //
    // EXPOOL should not find any of the string variables.
    //
    for I in 1..=8 {
        fstr::assign(&mut VARNAM, b"SVALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);

        spicelib::EXPOOL(&VARNAM, &mut FOUND, ctx)?;

        spicelib::PREFIX(b"FOUND for ", 1, &mut VARNAM);
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(&VARNAM, FOUND, false, OK, ctx)?;
    }
    //
    // EXPOOL should find values for all of the numeric variables
    //
    for I in 1..=11 {
        fstr::assign(&mut VARNAM, b"VALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);

        spicelib::EXPOOL(&VARNAM, &mut FOUND, ctx)?;

        spicelib::PREFIX(b"FOUND for ", 1, &mut VARNAM);
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(&VARNAM, FOUND, true, OK, ctx)?;
    }

    //
    // Case 14
    //
    testutil::TCASE(
        b"Make sure that GIPOOL doesn\'t try to swallow integers that are too big. ",
        ctx,
    )?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b" ");
    fstr::assign(TEXT.get_mut(3), b"      VALUE12 = 1.0D15");
    fstr::assign(TEXT.get_mut(4), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 4, true, false, ctx)?;

    spicelib::GIPOOL(
        b"VALUE12",
        1,
        10,
        &mut N,
        IVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INTOUTOFRANGE)", OK, ctx)?;

    //
    // Case 15
    //
    testutil::TCASE(
        b"Make sure that an error is signaled if we give a non-positive room size to GCPOOL ",
        ctx,
    )?;

    spicelib::GCPOOL(
        b"SVALUE1",
        1,
        0,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADARRAYSIZE)", OK, ctx)?;

    //
    // Case 16
    //
    testutil::TCASE(
        b"Make sure that an error is signaled if we give a non-positive room size to GIPOOL ",
        ctx,
    )?;

    spicelib::GIPOOL(
        b"VALUE1",
        1,
        0,
        &mut N,
        IVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADARRAYSIZE)", OK, ctx)?;

    //
    // Case 17
    //
    testutil::TCASE(
        b"Make sure that an error is signaled if we give a non-positive room size to GDPOOL ",
        ctx,
    )?;

    spicelib::GDPOOL(
        b"VALUE10",
        1,
        0,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADARRAYSIZE)", OK, ctx)?;

    //
    // Case 18
    //
    testutil::TCASE(b"Make sure an error is triggered if we try to load a string and numeric value into a variable. ", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"      VALUE13 = 123, \'123\'");
    fstr::assign(TEXT.get_mut(3), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 4, false, true, ctx)?;

    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    // CALL GETMSG ( 'LONG',  LONG )
    testutil::CHCKXC(true, b"SPICE(TYPEMISMATCH)", OK, ctx)?;
    // CALL TSTLOG (  LONG, .FALSE. )

    //
    // Case 19
    //
    testutil::TCASE(b"Make sure an error is triggered if we try to load a numeric value and a string value (in that order) into a variable. ", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"   SVALUE9 = \'123\', 123");
    fstr::assign(TEXT.get_mut(3), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 4, false, true, ctx)?;

    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    // CALL GETMSG ( 'LONG',  LONG )
    testutil::CHCKXC(true, b"SPICE(TYPEMISMATCH)", OK, ctx)?;
    // CALL TSTLOG (  LONG, .FALSE. )

    //
    // Case 20
    //
    testutil::TCASE(
        b"Add d.p. data to the kernel pool through the PDPOOL interface. ",
        ctx,
    )?;

    MYDATA[1] = 1.0;
    MYDATA[2] = 2.0;
    MYDATA[3] = 3.0;
    MYDATA[4] = 4.0;
    MYDATA[5] = 5.0;
    MYDATA[6] = 6.0;

    spicelib::PDPOOL(b"MYDPS", 6, MYDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DTPOOL(b"MYDPS", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;

    spicelib::GDPOOL(
        b"MYDPS",
        1,
        7,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"VALUES",
        VALUES.as_slice(),
        b"=",
        MYDATA.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Case 21
    //
    testutil::TCASE(b"Add string data through the PCPOOL interface.", ctx)?;

    fstr::assign(MYSTRS.get_mut(1), b"String 1");
    fstr::assign(MYSTRS.get_mut(2), b"String 2");
    fstr::assign(MYSTRS.get_mut(3), b"String 3");
    fstr::assign(MYSTRS.get_mut(4), b"String 4");
    fstr::assign(MYSTRS.get_mut(5), b"String 5");

    spicelib::PCPOOL(b"MYSTRS", 5, MYSTRS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DTPOOL(b"MYSTRS", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"C", OK, ctx)?;

    spicelib::GCPOOL(
        b"MYSTRS",
        1,
        7,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;

    testutil::CHCKSC(b"CVALUE(1)", &CVALUE[1], b"=", &MYSTRS[1], OK, ctx)?;
    testutil::CHCKSC(b"CVALUE(2)", &CVALUE[2], b"=", &MYSTRS[2], OK, ctx)?;
    testutil::CHCKSC(b"CVALUE(3)", &CVALUE[3], b"=", &MYSTRS[3], OK, ctx)?;
    testutil::CHCKSC(b"CVALUE(4)", &CVALUE[4], b"=", &MYSTRS[4], OK, ctx)?;
    testutil::CHCKSC(b"CVALUE(5)", &CVALUE[5], b"=", &MYSTRS[5], OK, ctx)?;

    //
    // Case 22
    //
    testutil::TCASE(b"Add integer data through the PIPOOL interface", ctx)?;

    MYINTS[1] = 5;
    MYINTS[2] = 6;
    MYINTS[3] = 7;
    MYINTS[4] = 8;
    MYINTS[5] = 9;
    MYINTS[6] = 10;

    spicelib::PIPOOL(b"MYINTS", 4, MYINTS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DTPOOL(b"MYINTS", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;

    spicelib::GIPOOL(
        b"MYINTS",
        1,
        7,
        &mut N,
        IVALS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"IVALS",
        IVALS.as_slice(),
        b"=",
        MYINTS.as_slice(),
        4,
        OK,
        ctx,
    )?;

    //
    // Case 23
    //
    testutil::TCASE(b"See if we can still clear the kernel pool. ", ctx)?;

    spicelib::CLPOOL(ctx)?;
    spicelib::DTPOOL(b"MYDPS", &mut FOUND, &mut N, &mut TYPE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;

    spicelib::DTPOOL(b"MYSTRS", &mut FOUND, &mut N, &mut TYPE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"X", OK, ctx)?;

    //
    // Case 24
    //
    testutil::TCASE(b"Check that loading from a text buffer works.Note that these tests are the same as test case 1 and 2 but for variables loaded from an internal text buffer. ", ctx)?;

    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = PI");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11  = ( 5, 4, 3, 2 )");
    fstr::assign(TEXT.get_mut(21), b" ");
    fstr::assign(TEXT.get_mut(22), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");

    spicelib::LMPOOL(TEXT.subarray(2), 23, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ESIZE[1] = 1;
    ESIZE[2] = 1;
    ESIZE[3] = 1;
    ESIZE[4] = 1;
    ESIZE[5] = 1;
    ESIZE[6] = 1;
    ESIZE[7] = 1;
    ESIZE[8] = 1;
    ESIZE[9] = 1;
    ESIZE[10] = 10;
    ESIZE[11] = 6;

    for I in 1..=11 {
        fstr::assign(&mut VARNAM, b"VALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);

        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;
        testutil::TSTMSG(b"#", &VARNAM, ctx);
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"N", N, b"=", ESIZE[I], 0, OK, ctx)?;
        testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"N", OK, ctx)?;
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TSTMSG(b"#", b"Checking Value 1", ctx);
    spicelib::GDPOOL(
        b"VALUE1",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::NPARSD(b"1", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE1",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 2", ctx);
    spicelib::GDPOOL(
        b"VALUE2",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"2", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE2",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 3", ctx);
    spicelib::GDPOOL(
        b"VALUE3",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"PI", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE3",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 4", ctx);
    spicelib::GDPOOL(
        b"VALUE4",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"3", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE4",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 5", ctx);
    spicelib::GDPOOL(
        b"VALUE5",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"4", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE5",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 6", ctx);
    spicelib::GDPOOL(
        b"VALUE6",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::NPARSD(b"5", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKAD(
        b"VALUE6",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 7", ctx);
    spicelib::NPARSD(b"1.276828E+11", &mut EVALS[1], &mut ERROR, &mut PTR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE7",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE7",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 8", ctx);
    spicelib::NPARSD(
        b"-28.19729871E+12",
        &mut EVALS[1],
        &mut ERROR,
        &mut PTR,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE8",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE8",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 9", ctx);
    spicelib::TPARSE(b"1-JAN-1994", &mut EVALS[1], &mut ERROR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::GDPOOL(
        b"VALUE9",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKAD(
        b"VALUE9",
        VALUES.as_slice(),
        b"=",
        EVALS.as_slice(),
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    EVALS[1] = 1 as f64;
    EVALS[2] = 2 as f64;
    EVALS[3] = 3 as f64;
    EVALS[4] = 4 as f64;
    EVALS[5] = 5 as f64;
    EVALS[6] = 6 as f64;
    EVALS[7] = 7 as f64;
    EVALS[8] = 8 as f64;
    EVALS[9] = 9 as f64;
    EVALS[10] = 10 as f64;

    testutil::TSTMSG(b"#", b"Checking Value 10", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        3,
        5,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE10",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(3),
        5,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 10 part 2.", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        1,
        10,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE10",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(1),
        10,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 10, 0, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Checking Value 10 part 3", ctx);
    spicelib::GDPOOL(
        b"VALUE10",
        11,
        5,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    EVALS[1] = 5 as f64;
    EVALS[2] = 4 as f64;
    EVALS[3] = 3 as f64;
    EVALS[4] = 2 as f64;
    EVALS[5] = 1 as f64;
    EVALS[6] = 0 as f64;

    testutil::TSTMSG(b"#", b"Checking Value 11", ctx);
    spicelib::GDPOOL(
        b"VALUE11",
        1,
        10,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"VALUE11",
        VALUES.as_slice(),
        b"=",
        EVALS.subarray(1),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::TSTMSG(b"#", b" ", ctx);

    //
    // Case 25
    //
    testutil::TCASE(b"Make sure we can read strings in from a text buffer. This is the same set of checks as used when we loaded string valued variables from a text file.  This time we load directly from a text buffer. ", ctx)?;

    fstr::assign(TEXT.get_mut(2), b" ");
    fstr::assign(TEXT.get_mut(3), b"   SVALUE1  = \'String 1\'");
    fstr::assign(TEXT.get_mut(4), b"   SVALUE2  = \'String 2\'");
    fstr::assign(TEXT.get_mut(5), b"   SVALUE3  = \'String 3\'");
    fstr::assign(TEXT.get_mut(6), b"   SVALUE4  = \'String 4\'");
    fstr::assign(TEXT.get_mut(7), b"   SVALUE5  = \'String 5\'");
    fstr::assign(
        TEXT.get_mut(8),
        b"   SVALUE6  = \'String 6.1\', \'String 6.2\', \'String 6.3\'",
    );
    fstr::assign(TEXT.get_mut(9), b"   SVALUE6  = \'String 6\'");
    fstr::assign(TEXT.get_mut(10), b" ");
    fstr::assign(TEXT.get_mut(11), b"   SVALUE7 += \'String 7.0\'");
    fstr::assign(TEXT.get_mut(12), b"   SVALUE7 += \'String 7.1\'");
    fstr::assign(TEXT.get_mut(13), b"   SVALUE7 += \'String 7.2\'");
    fstr::assign(TEXT.get_mut(14), b"   SVALUE7 += \'String 7.3\'");
    fstr::assign(TEXT.get_mut(15), b"   SVALUE7 += \'String 7.4\'");
    fstr::assign(TEXT.get_mut(16), b" ");
    fstr::assign(
        TEXT.get_mut(17),
        b"   SVALUE8 = ( \'String 8.0\', \'String 8.1\',",
    );
    fstr::assign(
        TEXT.get_mut(18),
        b"               \'String 8.2\', \'String 8.3\' )",
    );

    spicelib::LMPOOL(TEXT.subarray(2), 17, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ESIZE[1] = 1;
    ESIZE[2] = 1;
    ESIZE[3] = 1;
    ESIZE[4] = 1;
    ESIZE[5] = 1;
    ESIZE[6] = 1;
    ESIZE[7] = 5;
    ESIZE[8] = 4;

    for I in 1..=8 {
        fstr::assign(&mut VARNAM, b"SVALUE#");
        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);
        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"FOUND for ", &VARNAM));
        fstr::assign(ITEM.get_mut(2), &fstr::concat(b"N     for ", &VARNAM));
        fstr::assign(ITEM.get_mut(3), &fstr::concat(b"TYPE  for ", &VARNAM));

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(&ITEM[1], FOUND, true, OK, ctx)?;
        testutil::CHCKSI(&ITEM[2], N, b"=", ESIZE[I], 0, OK, ctx)?;
        testutil::CHCKSC(&ITEM[3], &TYPE, b"=", b"C", OK, ctx)?;
    }

    fstr::assign(CVALS.get_mut(1), b"String 7.0");
    fstr::assign(CVALS.get_mut(2), b"String 7.1");
    fstr::assign(CVALS.get_mut(3), b"String 7.2");
    fstr::assign(CVALS.get_mut(4), b"String 7.3");
    fstr::assign(CVALS.get_mut(5), b"String 7.4");

    spicelib::GCPOOL(
        b"SVALUE7",
        1,
        20,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for SVALUE7", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for SVALUE7", N, b"=", 5, 0, OK, ctx)?;

    testutil::CHCKSC(b"Value 1 of SVALUE7", &CVALUE[1], b"=", &CVALS[1], OK, ctx)?;
    testutil::CHCKSC(b"Value 2 of SVALUE7", &CVALUE[2], b"=", &CVALS[2], OK, ctx)?;
    testutil::CHCKSC(b"Value 3 of SVALUE7", &CVALUE[3], b"=", &CVALS[3], OK, ctx)?;
    testutil::CHCKSC(b"Value 4 of SVALUE7", &CVALUE[4], b"=", &CVALS[4], OK, ctx)?;
    testutil::CHCKSC(b"Value 5 of SVALUE7", &CVALUE[5], b"=", &CVALS[5], OK, ctx)?;

    fstr::assign(CVALS.get_mut(1), b"String 8.0");
    fstr::assign(CVALS.get_mut(2), b"String 8.1");
    fstr::assign(CVALS.get_mut(3), b"String 8.2");
    fstr::assign(CVALS.get_mut(4), b"String 8.3");

    spicelib::GCPOOL(
        b"SVALUE8",
        3,
        20,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for SVALUE7", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for SVALUE7", N, b"=", 2, 0, OK, ctx)?;

    testutil::CHCKSC(b"Value 3 of SVALUE8", &CVALUE[1], b"=", &CVALS[3], OK, ctx)?;
    testutil::CHCKSC(b"Value 4 of SVALUE8", &CVALUE[2], b"=", &CVALS[4], OK, ctx)?;

    //
    // Case 26
    //
    testutil::TCASE(
        b"Make sure that watchers work when a direct write to the kernel pool takes place. ",
        ctx,
    )?;

    fstr::assign(BNAMES.get_mut(1), b"SVALUE1");
    fstr::assign(BNAMES.get_mut(2), b"MYDPS");

    fstr::assign(NNAMES.get_mut(1), b"MYSTRS");
    fstr::assign(NNAMES.get_mut(2), b"MYDPS");

    fstr::assign(INAMES.get_mut(1), b"SVALUE1");
    fstr::assign(INAMES.get_mut(2), b"MYINTS");

    NBILL = 2;
    NNAT = 2;
    NIAN = 2;

    spicelib::SWPOOL(b"BILL", NBILL, BNAMES.as_arg(), ctx)?;
    spicelib::SWPOOL(b"NAT", NNAT, NNAMES.as_arg(), ctx)?;
    spicelib::SWPOOL(b"IAN", NIAN, INAMES.as_arg(), ctx)?;

    testutil::TSTMSG(b"#", b"Initial check of watchers.", ctx);

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;
    spicelib::CVPOOL(b"IAN", &mut IUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"IUPDAT", IUPDAT, true, OK, ctx)?;

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;
    spicelib::CVPOOL(b"IAN", &mut IUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"IUPDAT", IUPDAT, false, OK, ctx)?;

    MYDATA[1] = 1.0;
    MYDATA[2] = 2.0;
    MYDATA[3] = 3.0;
    MYDATA[4] = 4.0;
    MYDATA[5] = 5.0;
    MYDATA[6] = 6.0;

    spicelib::PDPOOL(b"MYDPS", 6, MYDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTMSG(b"#", b"Check of watchers after call to PDPOOL.", ctx);

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;
    spicelib::CVPOOL(b"IAN", &mut IUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"IUPDAT", IUPDAT, false, OK, ctx)?;

    fstr::assign(TEXT.get_mut(2), b" ");
    fstr::assign(TEXT.get_mut(3), b"   SVALUE1  = \'String 1\'");
    fstr::assign(TEXT.get_mut(4), b"   SVALUE2  = \'String 2\'");
    fstr::assign(TEXT.get_mut(5), b"   SVALUE3  = \'String 3\'");
    fstr::assign(TEXT.get_mut(6), b"   SVALUE4  = \'String 4\'");
    fstr::assign(TEXT.get_mut(7), b"   SVALUE5  = \'String 5\'");
    fstr::assign(
        TEXT.get_mut(8),
        b"   SVALUE6  = \'String 6.1\', \'String 6.2\', \'String 6.3\'",
    );
    fstr::assign(TEXT.get_mut(9), b"   SVALUE6  = \'String 6\'");
    fstr::assign(TEXT.get_mut(10), b" ");
    fstr::assign(TEXT.get_mut(11), b"   SVALUE7 += \'String 7.0\'");
    fstr::assign(TEXT.get_mut(12), b"   SVALUE7 += \'String 7.1\'");
    fstr::assign(TEXT.get_mut(13), b"   SVALUE7 += \'String 7.2\'");
    fstr::assign(TEXT.get_mut(14), b"   SVALUE7 += \'String 7.3\'");
    fstr::assign(TEXT.get_mut(15), b"   SVALUE7 += \'String 7.4\'");
    fstr::assign(TEXT.get_mut(16), b" ");
    fstr::assign(
        TEXT.get_mut(17),
        b"   SVALUE8 = ( \'String 8.0\', \'String 8.1\',",
    );
    fstr::assign(
        TEXT.get_mut(18),
        b"               \'String 8.2\', \'String 8.3\' )",
    );

    spicelib::LMPOOL(TEXT.subarray(2), 17, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTMSG(b"#", b"Check of watchers after call to LMPOOL.", ctx);

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;
    spicelib::CVPOOL(b"IAN", &mut IUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"IUPDAT", IUPDAT, true, OK, ctx)?;

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;
    spicelib::CVPOOL(b"IAN", &mut IUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"IUPDAT", IUPDAT, false, OK, ctx)?;

    MYINTS[1] = 5;
    MYINTS[2] = 6;
    MYINTS[3] = 7;
    MYINTS[4] = 8;
    MYINTS[5] = 9;
    MYINTS[6] = 10;

    testutil::TSTMSG(b"#", b"Check of watchers after call to PIPOOL.", ctx);

    spicelib::PIPOOL(b"MYINTS", 4, MYINTS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;
    spicelib::CVPOOL(b"IAN", &mut IUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"IUPDAT", IUPDAT, true, OK, ctx)?;

    testutil::TSTMSG(b"#", b"Check of watchers after call to PCPOOL.", ctx);

    fstr::assign(MYSTRS.get_mut(1), b"String 1");
    fstr::assign(MYSTRS.get_mut(2), b"String 2");
    fstr::assign(MYSTRS.get_mut(3), b"String 3");
    fstr::assign(MYSTRS.get_mut(4), b"String 4");
    fstr::assign(MYSTRS.get_mut(5), b"String 5");

    spicelib::PCPOOL(b"MYSTRS", 5, MYSTRS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;
    spicelib::CVPOOL(b"IAN", &mut IUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"IUPDAT", IUPDAT, false, OK, ctx)?;

    //
    // Case 27
    //
    testutil::TCASE(b"Make a cursory examination of SZPOOL", ctx)?;

    spicelib::SZPOOL(b"MAXVAR", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"maxval", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"Maxlin", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"MAXCHR", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"MXNOTE", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"maxlen", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"MAXAGT", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"STELLA", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::SZPOOL(b"stella", &mut N, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    //
    // Case 28
    //
    testutil::TCASE(b"Check 1. for the empty vector assignment exception. ", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = PI");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11  = ( )");
    fstr::assign(TEXT.get_mut(21), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(22), b" ");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");
    fstr::assign(TEXT.get_mut(25), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 25, false, true, ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARASSIGN)", OK, ctx)?;

    //
    // Case 29
    //
    testutil::TCASE(b"Check 2. for the empty vector assignment exception. ", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = \'PI\' ");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11  = ( 1 2 )");
    fstr::assign(TEXT.get_mut(21), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(22), b" ");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");
    fstr::assign(TEXT.get_mut(25), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 25, false, true, ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(TEXT.get_mut(20), b"         VALUE11  += ( )");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 25, false, true, ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARASSIGN)", OK, ctx)?;

    //
    // Case 30
    //
    testutil::TCASE(b"Check 3 for the empty vector assignment exception. ", ctx)?;

    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = PI");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11  = ( )");
    fstr::assign(TEXT.get_mut(21), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(22), b" ");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");

    spicelib::LMPOOL(TEXT.subarray(2), 22, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARASSIGN)", OK, ctx)?;

    //
    // Case 31
    //
    testutil::TCASE(b"Check 4 for the empty vector assignment exception. ", ctx)?;

    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = PI");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11 += ( )");
    fstr::assign(TEXT.get_mut(21), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(22), b" ");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");

    spicelib::LMPOOL(TEXT.subarray(2), 22, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARASSIGN)", OK, ctx)?;

    //
    // Case 32
    //
    testutil::TCASE(
        b"See if we can fetch variable names from the kernel pool via the interface GNPOOL ",
        ctx,
    )?;

    fstr::assign(TEXT.get_mut(20), b"         VALUE11 += ( 10 )");

    spicelib::CLPOOL(ctx)?;

    fstr::assign(CVALS.get_mut(1), b"VALUE1");
    fstr::assign(CVALS.get_mut(2), b"VALUE2");
    fstr::assign(CVALS.get_mut(3), b"VALUE3");
    fstr::assign(CVALS.get_mut(4), b"VALUE4");
    fstr::assign(CVALS.get_mut(5), b"VALUE5");
    fstr::assign(CVALS.get_mut(6), b"VALUE6");
    fstr::assign(CVALS.get_mut(7), b"VALUE7");
    fstr::assign(CVALS.get_mut(8), b"VALUE8");
    fstr::assign(CVALS.get_mut(9), b"VALUE9");
    fstr::assign(CVALS.get_mut(10), b"VALUE10");
    fstr::assign(CVALS.get_mut(11), b"VALUE11");

    spicelib::LMPOOL(TEXT.subarray(2), 22, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GNPOOL(b"*", 1, 22, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 11, 0, OK, ctx)?;

    spicelib::SHELLC(11, CVALS.as_arg_mut());
    spicelib::SHELLC(11, TEXT.as_arg_mut());

    for I in 1..=11 {
        testutil::CHCKSC(b"TEXT", &TEXT[I], b"=", &CVALS[I], OK, ctx)?;
    }

    spicelib::GNPOOL(b"*", 1, 5, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;

    spicelib::GNPOOL(b"*X*", 1, 22, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    spicelib::GNPOOL(
        b"%%%%%%%",
        1,
        22,
        &mut N,
        TEXT.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;

    fstr::assign(CVALS.get_mut(1), b"VALUE10");
    fstr::assign(CVALS.get_mut(2), b"VALUE11");

    spicelib::SHELLC(2, CVALS.as_arg_mut());
    spicelib::SHELLC(2, TEXT.as_arg_mut());

    for I in 1..=2 {
        testutil::CHCKSC(b"TEXT", &TEXT[I], b"=", &CVALS[I], OK, ctx)?;
    }

    spicelib::GNPOOL(
        b"%%%%%%",
        -1,
        22,
        &mut N,
        TEXT.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 9, 0, OK, ctx)?;

    fstr::assign(CVALS.get_mut(1), b"VALUE1");
    fstr::assign(CVALS.get_mut(2), b"VALUE2");
    fstr::assign(CVALS.get_mut(3), b"VALUE3");
    fstr::assign(CVALS.get_mut(4), b"VALUE4");
    fstr::assign(CVALS.get_mut(5), b"VALUE5");
    fstr::assign(CVALS.get_mut(6), b"VALUE6");
    fstr::assign(CVALS.get_mut(7), b"VALUE7");
    fstr::assign(CVALS.get_mut(8), b"VALUE8");
    fstr::assign(CVALS.get_mut(9), b"VALUE9");

    spicelib::SHELLC(9, CVALS.as_arg_mut());
    spicelib::SHELLC(9, TEXT.as_arg_mut());

    for I in 1..=9 {
        testutil::CHCKSC(b"TEXT", &TEXT[I], b"=", &CVALS[I], OK, ctx)?;
    }

    spicelib::GNPOOL(b"%%%%%%", 3, 22, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 7, 0, OK, ctx)?;

    spicelib::GNPOOL(b"VALUE", 1, -1, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADARRAYSIZE)", OK, ctx)?;

    //
    // Case 33
    //
    testutil::TCASE(
        b"Test to see if we can remove kernel pool variables using DVPOOL. ",
        ctx,
    )?;

    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = PI");
    fstr::assign(TEXT.get_mut(5), b"         VALUE4  = 3");
    fstr::assign(TEXT.get_mut(6), b"         VALUE5  = 4");
    fstr::assign(TEXT.get_mut(7), b"         VALUE6  = 5");
    fstr::assign(TEXT.get_mut(8), b"         VALUE7  = 1.276828E+11");
    fstr::assign(TEXT.get_mut(9), b"         VALUE8  = -28.19729871E+12");
    fstr::assign(TEXT.get_mut(10), b"         VALUE9  = @1-JAN-1994");
    fstr::assign(TEXT.get_mut(11), b"         VALUE10 = (");
    fstr::assign(TEXT.get_mut(12), b"                     1,");
    fstr::assign(TEXT.get_mut(13), b"                     2,");
    fstr::assign(TEXT.get_mut(14), b"                     3,");
    fstr::assign(TEXT.get_mut(15), b"                     4,");
    fstr::assign(TEXT.get_mut(16), b"                     5,");
    fstr::assign(TEXT.get_mut(17), b"                     6,");
    fstr::assign(TEXT.get_mut(18), b"                     7,");
    fstr::assign(TEXT.get_mut(19), b"                     8 )");
    fstr::assign(TEXT.get_mut(20), b"         VALUE11 += ( 5 )");
    fstr::assign(TEXT.get_mut(21), b"         VALUE10 +=  9");
    fstr::assign(TEXT.get_mut(22), b" ");
    fstr::assign(TEXT.get_mut(23), b"         VALUE11 += ( 1, 0 )");
    fstr::assign(TEXT.get_mut(24), b"         VALUE10 += 10");

    spicelib::CLPOOL(ctx)?;
    spicelib::LMPOOL(TEXT.subarray(2), 22, ctx)?;

    NBILL = 1;
    fstr::assign(BNAMES.get_mut(1), b"VALUE1");

    NNAT = 1;
    fstr::assign(NNAMES.get_mut(1), b"VALUE2");
    fstr::assign(NNAMES.get_mut(2), b"VALUE3");

    spicelib::SWPOOL(b"BILL", NBILL, BNAMES.as_arg(), ctx)?;
    spicelib::SWPOOL(b"NAT", NNAT, NNAMES.as_arg(), ctx)?;

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, true, OK, ctx)?;

    spicelib::GNPOOL(b"*", 1, 22, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 11, 0, OK, ctx)?;

    spicelib::DTPOOL(b"VALUE1", &mut FOUND, &mut N, &mut TYPE, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::DVPOOL(b"VALUE1", ctx)?;

    spicelib::DTPOOL(b"VALUE1", &mut FOUND, &mut N, &mut TYPE, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::GNPOOL(b"*", 1, 22, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 10, 0, OK, ctx)?;

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, true, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, false, OK, ctx)?;

    fstr::assign(CVALS.get_mut(1), b"VALUE2");
    fstr::assign(CVALS.get_mut(2), b"VALUE3");
    fstr::assign(CVALS.get_mut(3), b"VALUE4");
    fstr::assign(CVALS.get_mut(4), b"VALUE5");
    fstr::assign(CVALS.get_mut(5), b"VALUE6");
    fstr::assign(CVALS.get_mut(6), b"VALUE7");
    fstr::assign(CVALS.get_mut(7), b"VALUE8");
    fstr::assign(CVALS.get_mut(8), b"VALUE9");
    fstr::assign(CVALS.get_mut(9), b"VALUE10");
    fstr::assign(CVALS.get_mut(10), b"VALUE11");

    spicelib::SHELLC(10, CVALS.as_arg_mut());
    spicelib::SHELLC(10, TEXT.as_arg_mut());

    for I in 1..=10 {
        testutil::CHCKSC(b"TEXT(I)", &TEXT[I], b"=", &CVALS[I], OK, ctx)?;
    }

    spicelib::DTPOOL(b"VALUE2", &mut FOUND, &mut N, &mut TYPE, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::DVPOOL(b"VALUE2", ctx)?;

    spicelib::DTPOOL(b"VALUE2", &mut FOUND, &mut N, &mut TYPE, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::GNPOOL(b"*", 1, 22, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 9, 0, OK, ctx)?;

    spicelib::CVPOOL(b"BILL", &mut BUPDAT, ctx)?;
    spicelib::CVPOOL(b"NAT", &mut NUPDAT, ctx)?;

    testutil::CHCKSL(b"BUPDAT", BUPDAT, false, OK, ctx)?;
    testutil::CHCKSL(b"NUPDAT", NUPDAT, true, OK, ctx)?;

    fstr::assign(CVALS.get_mut(1), b"VALUE3");
    fstr::assign(CVALS.get_mut(2), b"VALUE4");
    fstr::assign(CVALS.get_mut(3), b"VALUE5");
    fstr::assign(CVALS.get_mut(4), b"VALUE6");
    fstr::assign(CVALS.get_mut(5), b"VALUE7");
    fstr::assign(CVALS.get_mut(6), b"VALUE8");
    fstr::assign(CVALS.get_mut(7), b"VALUE9");
    fstr::assign(CVALS.get_mut(8), b"VALUE10");
    fstr::assign(CVALS.get_mut(9), b"VALUE11");

    spicelib::SHELLC(9, CVALS.as_arg_mut());
    spicelib::SHELLC(9, TEXT.as_arg_mut());

    for I in 1..=9 {
        testutil::CHCKSC(b"TEXT(I)", &TEXT[I], b"=", &CVALS[I], OK, ctx)?;
    }

    //
    // Case 34
    //
    testutil::TCASE(
        b"Test to see if long string values are processed correctly. ",
        ctx,
    )?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"SVALUE1  = \'Seventy-nine character long string ------------------------------------------89\'");
    fstr::assign(TEXT.get_mut(3), b"SVALUE2  = \'Eighty character long string ------------------------------------------------890\'");
    fstr::assign(TEXT.get_mut(4), b"SVALUE3  = \'Eighty-one character long string --------------------------------------------8901\'");
    fstr::assign(TEXT.get_mut(5), b"SVALUE4  = \'Seventy-nine character long string with middle quote \'\'-----------------------89\'");
    fstr::assign(TEXT.get_mut(6), b"SVALUE5  = \'Seventy-nine character long string with closing quote -----------------------8\'\'\'");
    fstr::assign(TEXT.get_mut(7), b"SVALUE6  = \'Eighty character long string with middle quote \'\'-----------------------------890\'");
    fstr::assign(TEXT.get_mut(8), b"SVALUE7  = \'Eighty character long string with closing quote -----------------------------89\'\'\'");
    fstr::assign(TEXT.get_mut(9), b"SVALUE8  = \'Eighty-one character long string with middle quote \'\'-------------------------8901\'");
    fstr::assign(TEXT.get_mut(10), b"SVALUE9  = \'Eighty-one character long string with closing quote -------------------------890\'\'\'");
    fstr::assign(TEXT.get_mut(11), b"SVALUE10 = \'Seventy-nine character long string with two closing quotes -----------------7\'\'\'\'\'");
    fstr::assign(TEXT.get_mut(12), b"SVALUE11 = \'Eighty character long string with two closing quotes -----------------------78\'\'\'\'\'");
    fstr::assign(TEXT.get_mut(13), b"SVALUE12 = \'Eighty-one character long string with two closing quotes -------------------789\'\'\'\'\'");
    fstr::assign(TEXT.get_mut(14), b"SVALUE13 = \'Ten space-separated quotes string \'\' \'\' \'\' \'\' \'\' \'\' \'\' \'\' \'\' \'\'\'");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 14, false, true, ctx)?;
    spicelib::CLPOOL(ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    fstr::assign(
        CVALS.get_mut(1),
        b"Seventy-nine character long string ------------------------------------------89",
    );
    fstr::assign(
        CVALS.get_mut(2),
        b"Eighty character long string ------------------------------------------------890",
    );
    fstr::assign(
        CVALS.get_mut(3),
        b"Eighty-one character long string --------------------------------------------890",
    );
    fstr::assign(
        CVALS.get_mut(4),
        b"Seventy-nine character long string with middle quote \'-----------------------89",
    );
    fstr::assign(
        CVALS.get_mut(5),
        b"Seventy-nine character long string with closing quote -----------------------8\'",
    );
    fstr::assign(
        CVALS.get_mut(6),
        b"Eighty character long string with middle quote \'-----------------------------890",
    );
    fstr::assign(
        CVALS.get_mut(7),
        b"Eighty character long string with closing quote -----------------------------89\'",
    );
    fstr::assign(
        CVALS.get_mut(8),
        b"Eighty-one character long string with middle quote \'-------------------------890",
    );
    fstr::assign(
        CVALS.get_mut(9),
        b"Eighty-one character long string with closing quote -------------------------890",
    );
    fstr::assign(
        CVALS.get_mut(10),
        b"Seventy-nine character long string with two closing quotes -----------------7\'\'",
    );
    fstr::assign(
        CVALS.get_mut(11),
        b"Eighty character long string with two closing quotes -----------------------78\'\'",
    );
    fstr::assign(
        CVALS.get_mut(12),
        b"Eighty-one character long string with two closing quotes -------------------789\'",
    );
    fstr::assign(
        CVALS.get_mut(13),
        b"Ten space-separated quotes string \' \' \' \' \' \' \' \' \' \'",
    );

    for I in 1..=13 {
        fstr::assign(&mut VARNAM, b"SVALUE#");

        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);
        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"FOUND for ", &VARNAM));
        fstr::assign(ITEM.get_mut(2), &fstr::concat(b"N     for ", &VARNAM));
        fstr::assign(ITEM.get_mut(3), &fstr::concat(b"TYPE  for ", &VARNAM));

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(&ITEM[1], FOUND, true, OK, ctx)?;
        testutil::CHCKSI(&ITEM[2], N, b"=", 1, 0, OK, ctx)?;
        testutil::CHCKSC(&ITEM[3], &TYPE, b"=", b"C", OK, ctx)?;

        spicelib::GCPOOL(
            &VARNAM,
            1,
            1,
            &mut N,
            CVALUE.subarray_mut(1),
            &mut FOUND,
            ctx,
        )?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"VALUE for ", &VARNAM));

        testutil::CHCKSC(&ITEM[1], &CVALUE[1], b"=", &CVALS[I], OK, ctx)?;
    }

    //
    // Case 35
    //
    testutil::TCASE(b"Test to see if long/truncated kernel file lines containing string array assignments are processed correctly. ", ctx)?;

    fstr::assign(TEXT.get_mut(2), b"SVALUE1 = (                           ");
    fstr::assign(TEXT.get_mut(3), b"           \'short value\'            ");
    fstr::assign(TEXT.get_mut(4), b"                                                 \'Eighty character long string under 132-character line limit -----------------890\'");
    fstr::assign(TEXT.get_mut(5), b"          )                           ");
    fstr::assign(TEXT.get_mut(6), b"SVALUE2 = (                           ");
    fstr::assign(TEXT.get_mut(7), b"           \'short value\'            ");
    fstr::assign(TEXT.get_mut(8), b"                                                  \'Eighty character long string at 132-character line limit (a)-----------------890\'");
    fstr::assign(TEXT.get_mut(9), b"          )                           ");
    fstr::assign(TEXT.get_mut(10), b"SVALUE3 = (                           ");
    fstr::assign(TEXT.get_mut(11), b"           \'short value\'            ");
    fstr::assign(TEXT.get_mut(12), b"                                                   \'Eighty character long string at 132-character line limit (b)-----------------890\'");
    fstr::assign(TEXT.get_mut(13), b"          )                           ");
    fstr::assign(TEXT.get_mut(14), b"SVALUE4 = (                           ");
    fstr::assign(TEXT.get_mut(15), b"           \'short value\'            ");
    fstr::assign(TEXT.get_mut(16), b"                                                    \'Eighty character long string over 132-character line limit (a)---------------890\'");
    fstr::assign(TEXT.get_mut(17), b"          )                           ");
    fstr::assign(TEXT.get_mut(18), b"SVALUE5 = (                           ");
    fstr::assign(TEXT.get_mut(19), b"           \'short value\'            ");
    fstr::assign(TEXT.get_mut(20), b"                                                     \'Eighty character long string over 132-character line limit (b)---------------890\'");
    fstr::assign(TEXT.get_mut(21), b"          )                           ");
    fstr::assign(TEXT.get_mut(22), b"SVALUE6 = (                           ");
    fstr::assign(TEXT.get_mut(23), b"           \'short value\'            ");
    fstr::assign(TEXT.get_mut(24), b"                                                  \'Eighty character long string at 132-character line limit (a)-----------------8\'\'\'");
    fstr::assign(TEXT.get_mut(25), b"          )                           ");
    fstr::assign(TEXT.get_mut(26), b"SVALUE7 = (                           ");
    fstr::assign(TEXT.get_mut(27), b"           \'short value\'            ");
    fstr::assign(TEXT.get_mut(28), b"                                                  \'Eighty character long string at 132-character line limit (a)-----------------89\'\'\'");
    fstr::assign(TEXT.get_mut(29), b"          )                           ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 29, false, true, ctx)?;
    spicelib::CLPOOL(ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    fstr::assign(
        CVALS.get_mut(1),
        b"Eighty character long string under 132-character line limit -----------------890",
    );
    fstr::assign(
        CVALS.get_mut(2),
        b"Eighty character long string at 132-character line limit (a)-----------------890",
    );
    fstr::assign(
        CVALS.get_mut(3),
        b"Eighty character long string at 132-character line limit (b)-----------------890",
    );
    fstr::assign(
        CVALS.get_mut(4),
        b"Eighty character long string over 132-character line limit (a)---------------89",
    );
    fstr::assign(
        CVALS.get_mut(5),
        b"Eighty character long string over 132-character line limit (b)---------------8",
    );
    fstr::assign(
        CVALS.get_mut(6),
        b"Eighty character long string at 132-character line limit (a)-----------------8\'",
    );
    fstr::assign(
        CVALS.get_mut(7),
        b"Eighty character long string at 132-character line limit (a)-----------------89\'",
    );

    for I in 1..=7 {
        fstr::assign(&mut VARNAM, b"SVALUE#");

        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);
        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"FOUND for ", &VARNAM));
        fstr::assign(ITEM.get_mut(2), &fstr::concat(b"N     for ", &VARNAM));
        fstr::assign(ITEM.get_mut(3), &fstr::concat(b"TYPE  for ", &VARNAM));

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(&ITEM[1], FOUND, true, OK, ctx)?;
        testutil::CHCKSI(&ITEM[2], N, b"=", 2, 0, OK, ctx)?;
        testutil::CHCKSC(&ITEM[3], &TYPE, b"=", b"C", OK, ctx)?;

        spicelib::GCPOOL(
            &VARNAM,
            2,
            1,
            &mut N,
            CVALUE.subarray_mut(1),
            &mut FOUND,
            ctx,
        )?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"VALUE for ", &VARNAM));

        testutil::CHCKSC(&ITEM[1], &CVALUE[1], b"=", &CVALS[I], OK, ctx)?;
    }

    //
    // Case 36
    //
    testutil::TCASE(b"Test to see if long/truncated kernel file lines containing string value assignments are processed correctly. ", ctx)?;

    fstr::assign(TEXT.get_mut(2), b"SVALUE1 =                                        \'Eighty character long string under 132-character line limit -----------------890\'");
    fstr::assign(TEXT.get_mut(3), b"SVALUE2 =                                         \'Eighty character long string at 132-character line limit (a)-----------------890\'");
    fstr::assign(TEXT.get_mut(4), b"SVALUE3 =                                          \'Eighty character long string at 132-character line limit (b)-----------------890\'");
    fstr::assign(TEXT.get_mut(5), b"SVALUE4 =                                           \'Eighty character long string over 132-character line limit (a)---------------890\'");
    fstr::assign(TEXT.get_mut(6), b"SVALUE5 =                                            \'Eighty character long string over 132-character line limit (b)---------------890\'");
    fstr::assign(TEXT.get_mut(7), b"SVALUE6 =                                         \'Eighty character long string at 132-character line limit (a)-----------------8\'\'\'");
    fstr::assign(TEXT.get_mut(8), b"SVALUE7 =                                         \'Eighty character long string at 132-character line limit (a)-----------------89\'\'\'");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 8, false, true, ctx)?;
    spicelib::CLPOOL(ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    fstr::assign(
        CVALS.get_mut(1),
        b"Eighty character long string under 132-character line limit -----------------890",
    );
    fstr::assign(
        CVALS.get_mut(2),
        b"Eighty character long string at 132-character line limit (a)-----------------890",
    );
    fstr::assign(
        CVALS.get_mut(3),
        b"Eighty character long string at 132-character line limit (b)-----------------890",
    );
    fstr::assign(
        CVALS.get_mut(4),
        b"Eighty character long string over 132-character line limit (a)---------------89",
    );
    fstr::assign(
        CVALS.get_mut(5),
        b"Eighty character long string over 132-character line limit (b)---------------8",
    );
    fstr::assign(
        CVALS.get_mut(6),
        b"Eighty character long string at 132-character line limit (a)-----------------8\'",
    );
    fstr::assign(
        CVALS.get_mut(7),
        b"Eighty character long string at 132-character line limit (a)-----------------89\'",
    );

    for I in 1..=7 {
        fstr::assign(&mut VARNAM, b"SVALUE#");

        spicelib::REPMI(&VARNAM.clone(), b"#", I, &mut VARNAM, ctx);
        spicelib::DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut TYPE, ctx)?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"FOUND for ", &VARNAM));
        fstr::assign(ITEM.get_mut(2), &fstr::concat(b"N     for ", &VARNAM));
        fstr::assign(ITEM.get_mut(3), &fstr::concat(b"TYPE  for ", &VARNAM));

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(&ITEM[1], FOUND, true, OK, ctx)?;
        testutil::CHCKSI(&ITEM[2], N, b"=", 1, 0, OK, ctx)?;
        testutil::CHCKSC(&ITEM[3], &TYPE, b"=", b"C", OK, ctx)?;

        spicelib::GCPOOL(
            &VARNAM,
            1,
            1,
            &mut N,
            CVALUE.subarray_mut(1),
            &mut FOUND,
            ctx,
        )?;

        fstr::assign(ITEM.get_mut(1), &fstr::concat(b"VALUE for ", &VARNAM));

        testutil::CHCKSC(&ITEM[1], &CVALUE[1], b"=", &CVALS[I], OK, ctx)?;
    }

    //
    // Case 37
    //
    testutil::TCASE(b"Confirm an error signal for kernel pool variable names longer than the MAXLEN value defined in pool.f:  PCPOOL, PIPOOL, PDPOOL ...", ctx)?;

    fstr::assign(&mut NAME, b"ABDEF123456789012345678901234567890");
    N = 1;
    IVALS[1] = 1;
    DVALS[1] = 1.0;
    fstr::assign(CVALS.get_mut(1), b"A");
    fstr::assign(TEXT.get_mut(1), b"ABDEF123456789012345678901234567890 = 1");

    //
    // Call the PXPOOL routines using NAME as the variable name.
    //
    spicelib::KCLEAR(ctx)?;

    spicelib::PCPOOL(&NAME, N, CVALS.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARNAME)", OK, ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::PDPOOL(&NAME, N, DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARNAME)", OK, ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::PIPOOL(&NAME, N, IVALS.as_slice(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARNAME)", OK, ctx)?;

    //
    // Case 38: Call LMPOOL with a string of the form NAME = value.
    //

    testutil::TCASE(b"... LMPOOL ...", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::LMPOOL(TEXT.subarray(1), 1, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARNAME)", OK, ctx)?;

    //
    // Case 39: same using FURNSH.
    //
    testutil::TCASE(b"... and FURNSH.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    testutil::KILFIL(b"pool_t.ker", ctx)?;
    spicelib::TXTOPN(b"pool_t.ker", &mut UNIT, ctx)?;

    fstr::assign(TEXT.get_mut(1), b"KPL/FK");
    testutil::BEGDAT(&mut TEXT[2]);
    fstr::assign(
        TEXT.get_mut(3),
        b"      ABCD123456789012345678901234567890 = 1",
    );
    fstr::assign(TEXT.get_mut(4), b" ");

    for I in 1..=4 {
        spicelib::WRITLN(&TEXT[I], UNIT, ctx)?;
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::FURNSH(b"pool_t.ker", ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVARNAME)", OK, ctx)?;

    //
    // Case 40: test POOL state counter.
    //
    testutil::TCASE(b"Test POOL state counter update", ctx)?;

    //
    // Initialize the local POOL state counter.
    //
    spicelib::ZZCTRUIN(USRCTR.as_slice_mut(), ctx);

    //
    // Test initial counter update and no update on the second
    // immediate call.
    //
    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"initial counter UPDATE", UPDATE, true, OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no initial counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after CLPOOL.
    //
    spicelib::CLPOOL(ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"CLPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after LDPOOL.
    //
    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"         VALUE1  = 1");
    fstr::assign(TEXT.get_mut(3), b"         VALUE2  = 2.E0");
    fstr::assign(TEXT.get_mut(4), b"         VALUE3  = \'BOO\'");
    fstr::assign(TEXT.get_mut(5), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 5, false, true, ctx)?;

    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"LDPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after RTPOOL.
    //
    spicelib::RTPOOL(b"VALUE2", &mut N, VALUES.as_slice_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"N      for VALUE2", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSD(
        b"Values for VALUE2",
        *VALUES.first(),
        b"=",
        2.0,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND  for VALUE2", FOUND, true, OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no RTPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after EXPOOL.
    //
    spicelib::EXPOOL(b"VALUE1", &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"VALUE1", FOUND, true, OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no EXPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after WRPOOL.
    //
    testutil::KILFIL(b"writdata.ker", ctx)?;
    spicelib::TXTOPN(b"writdata.ker", &mut UNIT, ctx)?;
    spicelib::WRPOOL(UNIT, ctx)?;
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no WRPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after SWPOOL.
    //
    fstr::assign(NAMES.get_mut(1), b"VALUE1");
    spicelib::SWPOOL(b"COUNTER", 1, NAMES.as_arg(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"SWPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after CVPOOL.
    //
    spicelib::CVPOOL(b"COUNTER", &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for COUNTER watcher", FOUND, true, OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no CVPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after GCPOOL.
    //
    spicelib::GCPOOL(
        b"VALUE3",
        1,
        1,
        &mut N,
        CVALUE.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for VALUE3", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for VALUE3", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSC(b"Value for VALUE3", &CVALUE[1], b"=", b"BOO", OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no GCPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after GDPOOL.
    //
    spicelib::GDPOOL(
        b"VALUE2",
        1,
        1,
        &mut N,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for VALUE2", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for VALUE2", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSD(
        b"Value for VALUE2",
        *VALUES.first(),
        b"=",
        2.0,
        0.0,
        OK,
        ctx,
    )?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no GDPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after GIPOOL.
    //
    spicelib::GIPOOL(
        b"VALUE1",
        1,
        1,
        &mut N,
        INTVAL.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for VALUE1", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for VALUE1", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSI(b"Value for VALUE1", *INTVAL.first(), b"=", 1, 0, OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no GIPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after DTPOOL.
    //
    spicelib::DTPOOL(b"VALUE1", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND for VALUE1", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N     for VALUE1", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE  for VALUE1", &TYPE, b"=", b"N", OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no DTPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after PCPOOL.
    //
    fstr::assign(MYSTRS.get_mut(1), b"String 1");

    spicelib::PCPOOL(b"MYSTRS", 1, MYSTRS.as_arg(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"PCPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after PDPOOL.
    //
    MYDATA[1] = 1.0;

    spicelib::PDPOOL(b"MYDPS", 1, MYDATA.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"PDPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after PIPOOL.
    //
    MYINTS[1] = 5;

    spicelib::PIPOOL(b"MYINTS", 1, MYINTS.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"PIPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after LMPOOL.
    //
    fstr::assign(TEXT.get_mut(1), b"         VALUE4  = 11");
    fstr::assign(TEXT.get_mut(2), b"         VALUE5  = 22.E0");
    fstr::assign(TEXT.get_mut(3), b"         VALUE6  = \'BOOBOO\'");

    spicelib::LMPOOL(TEXT.as_arg(), 3, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"LMPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after SZPOOL
    //
    spicelib::SZPOOL(b"MAXVAR", &mut N, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b">", 0, 0, OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no SZPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after DVPOOL
    //
    spicelib::DVPOOL(b"VALUE6", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"DVPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Test counter after GNPOOL
    //
    spicelib::GNPOOL(b"*", 1, 5, &mut N, TEXT.as_arg_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 5, 0, OK, ctx)?;

    UPDATE = true;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"no GNPOOL counter UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Test counter after DWPOOL
    //
    spicelib::DWPOOL(b"COUNTER", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UPDATE = false;
    spicelib::ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);
    testutil::CHCKSL(b"DWPOOL counter UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Case 41: test POOL state counter.
    //
    testutil::TCASE(
        b"Make sure we can set watchers and check them using ZZCVPOOL instead of CVPOOL. ",
        ctx,
    )?;

    //
    // Initialize the local POOL state counter.
    //
    spicelib::ZZCTRUIN(USRCT1.as_slice_mut(), ctx);
    spicelib::ZZCTRUIN(USRCT2.as_slice_mut(), ctx);
    spicelib::ZZCTRUIN(USRCT3.as_slice_mut(), ctx);
    spicelib::ZZCTRUIN(USRCT4.as_slice_mut(), ctx);

    //
    // Create a list of variables to be watched.
    //
    fstr::assign(NAMES.get_mut(1), b"SVALUE8");
    fstr::assign(NAMES.get_mut(2), b"VALUE10");
    fstr::assign(NAMES.get_mut(3), b"SPUD");

    //
    // Establish a watcher on the string variable and on the
    // numeric variable.
    //
    spicelib::SWPOOL(b"S_POOL", 1, NAMES.as_arg(), ctx)?;
    spicelib::SWPOOL(b"D_POOL", 1, NAMES.subarray(2), ctx)?;
    spicelib::SWPOOL(b"DUMMY", 1, NAMES.subarray(3), ctx)?;

    //
    // Check to see that S_POOL, D_POOL and DUMMY have notifications
    // pending.
    //
    spicelib::ZZCVPOOL(b"S_POOL", USRCT1.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, true, OK, ctx)?;

    spicelib::ZZCVPOOL(b"D_POOL", USRCT2.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for D_POOL", UPDATE, true, OK, ctx)?;

    spicelib::ZZCVPOOL(b"DUMMY", USRCT3.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, true, OK, ctx)?;

    //
    // Now check that S_POOL, D_POOL and DUMMY do not have
    // notifications pending.
    //
    spicelib::ZZCVPOOL(b"S_POOL", USRCT1.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"D_POOL", USRCT2.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"DUMMY", USRCT3.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"UNDECLAGENT", USRCT4.as_slice_mut(), &mut UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UNDECLAGENT", UPDATE, false, OK, ctx)?;

    //
    // Clear the kernel pool and see if everything behaves as
    // expected.
    //
    spicelib::CLPOOL(ctx)?;

    testutil::TSTMSG(b"#", b"Same tests after clearing the pool.", ctx);

    //
    // Check to see that S_POOL, D_POOL and DUMMY have notifications
    // pending.
    //
    spicelib::ZZCVPOOL(b"S_POOL", USRCT1.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, true, OK, ctx)?;

    spicelib::ZZCVPOOL(b"D_POOL", USRCT2.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for D_POOL", UPDATE, true, OK, ctx)?;

    spicelib::ZZCVPOOL(b"DUMMY", USRCT3.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, true, OK, ctx)?;

    //
    // Now check that S_POOL, D_POOL and DUMMY do not have
    // notifications pending.
    //
    spicelib::ZZCVPOOL(b"S_POOL", USRCT1.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"D_POOL", USRCT2.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"DUMMY", USRCT3.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"UNDECLAGENT", USRCT4.as_slice_mut(), &mut UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UNDECLAGENT", UPDATE, false, OK, ctx)?;

    //
    // Now load a kernel and see if these items get updated
    // as expected
    //
    spicelib::LDPOOL(b"write.dat", ctx)?;
    testutil::TSTMSG(b"#", b"Same watcher tests after loading a file.", ctx);

    spicelib::ZZCVPOOL(b"S_POOL", USRCT1.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, true, OK, ctx)?;

    spicelib::ZZCVPOOL(b"D_POOL", USRCT2.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for D_POOL", UPDATE, true, OK, ctx)?;

    spicelib::ZZCVPOOL(b"DUMMY", USRCT3.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    //
    // Now check that S_POOL, D_POOL and DUMMY do not have
    // notifications pending.
    //
    spicelib::ZZCVPOOL(b"S_POOL", USRCT1.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"D_POOL", USRCT2.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for S_POOL", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"DUMMY", USRCT3.as_slice_mut(), &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE for DUMMY", UPDATE, false, OK, ctx)?;

    spicelib::ZZCVPOOL(b"UNDECLAGENT", USRCT4.as_slice_mut(), &mut UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UNDECLAGENT", UPDATE, false, OK, ctx)?;
    testutil::TSTMSG(b"#", b" ", ctx);

    //
    // Delete watchers used in this test.
    //
    spicelib::DWPOOL(b"S_POOL", ctx)?;
    spicelib::DWPOOL(b"D_POOL", ctx)?;
    spicelib::DWPOOL(b"DUMMY", ctx)?;

    //
    // Unload everything, clean the kernel pool.
    //
    spicelib::KCLEAR(ctx)?;

    testutil::KILFIL(b"pool_t.ker", ctx)?;
    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::KILFIL(b"writdata.ker", ctx)?;
    testutil::KILFIL(b"write.dat", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
