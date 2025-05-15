//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;
const TXTSIZ: i32 = 11;
const LONGSZ: i32 = 3000;

//$Procedure      F_SEPOOL ( Family of tests for SEPOOL )
pub fn F_SEPOOL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TEXT = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
    let mut EXPECT = ActualCharArray::new(LONGSZ, 1..=5);
    let mut MYSTR = vec![b' '; LONGSZ as usize];
    let mut ESTRNG = vec![b' '; LONGSZ as usize];
    let mut TYPE = [b' '; 1 as usize];
    let mut ESIZE: i32 = 0;
    let mut N: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut GOTBCK: i32 = 0;
    let mut FIDX: i32 = 0;
    let mut LIDX: i32 = 0;
    let mut FOUND: bool = false;
    let mut GCFND: bool = false;
    let mut STFND: bool = false;

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SEPOOL", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);

    fstr::assign(
        TEXT.get_mut(2),
        b" MYSTRING = ( \'This is a long string that will be spread -\',",
    );
    fstr::assign(
        TEXT.get_mut(3),
        b"              \'across several lines of a spice text kernel -\',",
    );
    fstr::assign(
        TEXT.get_mut(4),
        b"              \'for the sake of testing SEPOOL.\',",
    );
    fstr::assign(
        TEXT.get_mut(5),
        b"              \'This is a second string.\',",
    );
    fstr::assign(
        TEXT.get_mut(6),
        b"              \'This is a third long string that will span -\',",
    );
    fstr::assign(
        TEXT.get_mut(7),
        b"              \'more than one line in a text kernel.\',",
    );
    fstr::assign(
        TEXT.get_mut(8),
        b"              \'Finally this is a last long string that will -\'",
    );
    fstr::assign(
        TEXT.get_mut(9),
        b"              \'be more than one line in the text kernel so -\'",
    );
    fstr::assign(
        TEXT.get_mut(10),
        b"              \'that we can test the fetching of long strings -\',",
    );
    fstr::assign(
        TEXT.get_mut(11),
        b"              \'from the kernel pool with SEPOOL.\' )",
    );

    fstr::assign(EXPECT.get_mut(1), b"This is a long string that will be spread across several lines of a spice text kernel for the sake of testing SEPOOL. ");

    fstr::assign(EXPECT.get_mut(2), b"This is a second string. ");

    fstr::assign(
        EXPECT.get_mut(3),
        b"This is a third long string that will span more than one line in a text kernel. ",
    );

    fstr::assign(EXPECT.get_mut(4), b"Finally this is a last long string that will be more than one line in the text kernel so that we can test the fetching of long strings from the kernel pool with SEPOOL. ");

    fstr::assign(
        EXPECT.get_mut(5),
        b"that we can test the fetching of long strings from the kernel pool with SEPOOL. ",
    );

    testutil::KILFIL(b"sample.txt", ctx)?;
    testutil::TSTTXT(b"sample.txt", TEXT.as_arg(), TXTSIZ, true, false, ctx)?;

    testutil::TCASE(b"Try to look up a string that isn\'t there.", ctx)?;

    fstr::assign(&mut MYSTR, b"xxx");
    spicelib::SEPOOL(
        b"MYITEM", 1, b"-", &mut MYSTR, &mut SIZE, &mut LIDX, &mut FOUND, ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::TCASE(b"See if we can get the loaded strings.", ctx)?;

    fstr::assign(&mut MYSTR, b"xxx");

    //
    // Fetch complete strings.
    //
    ESIZE = spicelib::RTRIM(&EXPECT[1]);
    FIDX = 1;

    spicelib::SEPOOL(
        b"MYSTRING",
        FIDX,
        b"-",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", ESIZE, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 3, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", &EXPECT[1], OK, ctx)?;

    ESIZE = spicelib::RTRIM(&EXPECT[2]);
    FIDX = (LIDX + 1);

    spicelib::SEPOOL(
        b"MYSTRING",
        FIDX,
        b"-",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", ESIZE, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", &EXPECT[2], OK, ctx)?;

    ESIZE = spicelib::RTRIM(&EXPECT[3]);
    FIDX = (LIDX + 1);

    spicelib::SEPOOL(
        b"MYSTRING",
        FIDX,
        b"-",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", ESIZE, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", &EXPECT[3], OK, ctx)?;

    ESIZE = spicelib::RTRIM(&EXPECT[4]);
    FIDX = (LIDX + 1);

    spicelib::SEPOOL(
        b"MYSTRING",
        FIDX,
        b"-",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", ESIZE, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", &EXPECT[4], OK, ctx)?;

    //
    // Fetch partial string for FIDX in the middle.
    //
    ESIZE = spicelib::RTRIM(&EXPECT[5]);
    FIDX = (LIDX - 1);

    spicelib::SEPOOL(
        b"MYSTRING",
        FIDX,
        b"-",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", ESIZE, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", &EXPECT[5], OK, ctx)?;

    //
    // Get .NOT. FOUNDs and blanks for too small and too big FIDXs.
    //
    FIDX = (LIDX + 1);

    spicelib::SEPOOL(
        b"MYSTRING",
        FIDX,
        b"-",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", b" ", OK, ctx)?;

    FIDX = 0;

    spicelib::SEPOOL(
        b"MYSTRING",
        FIDX,
        b"-",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"Check for compatibility with GCPOOL when there continuation symbol used is not the one specified in SEPOOL. ", ctx)?;

    spicelib::DTPOOL(b"MYSTRING", &mut FOUND, &mut N, &mut TYPE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    for I in 1..=N {
        spicelib::SEPOOL(
            b"MYSTRING",
            I,
            b"*",
            &mut MYSTR,
            &mut SIZE,
            &mut LIDX,
            &mut STFND,
            ctx,
        )?;
        spicelib::GCPOOL(
            b"MYSTRING",
            I,
            1,
            &mut GOTBCK,
            CharArrayMut::from_mut(&mut ESTRNG),
            &mut GCFND,
            ctx,
        )?;

        testutil::CHCKSL(b"GCFND", GCFND, true, OK, ctx)?;
        testutil::CHCKSL(b"STFND", STFND, GCFND, OK, ctx)?;
        testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", &ESTRNG, OK, ctx)?;

        ESIZE = spicelib::RTRIM(&ESTRNG);
        testutil::CHCKSI(b"SIZE", SIZE, b"=", ESIZE, 0, OK, ctx)?;

        testutil::CHCKSI(b"LIDX", LIDX, b"=", I, 0, OK, ctx)?;
    }

    spicelib::SEPOOL(
        b"MYSTRING",
        (N + 1),
        b"*",
        &mut MYSTR,
        &mut SIZE,
        &mut LIDX,
        &mut STFND,
        ctx,
    )?;
    testutil::CHCKSL(b"STFND", STFND, false, OK, ctx)?;
    testutil::CHCKSC(b"MYSTR", &MYSTR, b"=", b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", SIZE, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LIDX", LIDX, b"=", 0, 0, OK, ctx)?;

    //
    // Cleanup any debris we may have left around.
    //
    spicelib::CLPOOL(ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
