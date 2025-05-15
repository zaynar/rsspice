//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_ZZPLAT ( Test Fortran Intrinsics )
pub fn F_ZZPLAT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BFFID = [b' '; 8 as usize];
    let mut DEQUIV: f64 = 0.0;
    let mut IEQUIV = StackArray::<i32, 2>::new(1..=2);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // EQUIVALENCE statements
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_ZZPLAT", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Binary File Format Consistency Check", ctx)?;

    //
    // Fetch the binary file format ID string.
    //
    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut BFFID, ctx);

    //
    // "Touch" IEQUIV to make sure it does not get optimized out,
    // as is the case with IFORT on Linux.
    //
    DummyArrayMut::<i32>::from_equiv(std::slice::from_mut(&mut DEQUIV), 1..=2)[1] =
        spicelib::TOUCHI(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1]);
    DummyArrayMut::<i32>::from_equiv(std::slice::from_mut(&mut DEQUIV), 1..=2)[2] =
        spicelib::TOUCHI(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2]);

    //
    // Set the test value. "Touch" it as well.
    //
    DEQUIV = 1.0;

    DEQUIV = spicelib::TOUCHD(DEQUIV);

    //
    // Branch based on the value returned; check equivalenced
    // integers for the appropriate values.
    //
    if spicelib::EQSTR(&BFFID, b"BIG-IEEE") {
        testutil::CHCKSI(
            b"IEQUIV(1)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1],
            b"=",
            1072693248,
            0,
            OK,
            ctx,
        )?;
        testutil::CHCKSI(
            b"IEQUIV(2)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2],
            b"=",
            0,
            0,
            OK,
            ctx,
        )?;
    } else if spicelib::EQSTR(&BFFID, b"LTL-IEEE") {
        testutil::CHCKSI(
            b"IEQUIV(1)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1],
            b"=",
            0,
            0,
            OK,
            ctx,
        )?;
        testutil::CHCKSI(
            b"IEQUIV(2)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2],
            b"=",
            1072693248,
            0,
            OK,
            ctx,
        )?;
    } else if spicelib::EQSTR(&BFFID, b"VAX-GFLT") {
        testutil::CHCKSI(
            b"IEQUIV(1)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1],
            b"=",
            16400,
            0,
            OK,
            ctx,
        )?;
        testutil::CHCKSI(
            b"IEQUIV(2)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2],
            b"=",
            0,
            0,
            OK,
            ctx,
        )?;
    } else if spicelib::EQSTR(&BFFID, b"VAX-DFLT") {
        testutil::CHCKSI(
            b"IEQUIV(1)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1],
            b"=",
            16512,
            0,
            OK,
            ctx,
        )?;
        testutil::CHCKSI(
            b"IEQUIV(2)",
            DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2],
            b"=",
            0,
            0,
            OK,
            ctx,
        )?;
    } else {
        testutil::TSTMSG(b"#", b"This test does not support this format.", ctx);
        testutil::CHCKSI(b"ERROR", 0, b"=", 1, 0, OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
