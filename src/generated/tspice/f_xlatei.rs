//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;
const NUMOUT: i32 = 256;

//$Procedure F_XLATEI ( ZZXLATEI Test Family )
pub fn F_XLATEI(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut INPUT = [b' '; 1024 as usize];
    let mut COMPAR = StackArray::<i32, 256>::new(1..=NUMOUT);
    let mut INBFF: i32 = 0;
    let mut OUTPUT = StackArray::<i32, 256>::new(1..=NUMOUT);
    let mut SPACE: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_XLATEI", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INBFF out of range error", ctx)?;

    //
    // Setup the inputs and outputs for checking the lower bound.
    //
    INBFF = 0;
    SPACE = 10;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0;
        COMPAR[I] = 0;
    }

    //
    // Invoke the module.
    //
    spicelib::ZZXLATEI(INBFF, &INPUT, SPACE, OUTPUT.as_slice_mut(), ctx)?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Check to see that OUTPUT was unmodified.
    //
    testutil::CHCKAI(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        OK,
        ctx,
    )?;

    //
    // Setup the inputs and outputs for checking the upper bound.
    //
    INBFF = (NUMBFF + 1);
    SPACE = 10;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0;
        COMPAR[I] = 0;
    }

    //
    // Invoke the module.
    //
    spicelib::ZZXLATEI(INBFF, &INPUT, SPACE, OUTPUT.as_slice_mut(), ctx)?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Check to see that OUTPUT was unmodified.
    //
    testutil::CHCKAI(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- Bad Byte Count BIG-IEEE INPUT", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = 10;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0;
        COMPAR[I] = 0;
    }

    //
    // Invoke the module. Restrict INPUT to 1:13 range, since
    // BIG-IEEE integers come in 4-byte packages.
    //
    spicelib::ZZXLATEI(
        INBFF,
        fstr::substr(&INPUT, 1..=13),
        SPACE,
        OUTPUT.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Check to see that OUTPUT was unmodified.
    //
    testutil::CHCKAI(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- Not enough SPACE to store OUTPUT", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = 10;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0;
        COMPAR[I] = 0;
    }

    //
    // Invoke the module. Restrict INPUT to 1:13 range, since
    // BIG-IEEE integers come in 4-byte packages.
    //
    spicelib::ZZXLATEI(
        INBFF,
        fstr::substr(&INPUT, 1..=80),
        SPACE,
        OUTPUT.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Check to see that OUTPUT was unmodified.
    //
    testutil::CHCKAI(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- Unsupported INBFF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = VAXDFL;
    SPACE = 10;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0;
        COMPAR[I] = 0;
    }

    //
    // Invoke the module. Restrict INPUT to 1:13 range, since
    // BIG-IEEE integers come in 4-byte packages.
    //
    spicelib::ZZXLATEI(
        INBFF,
        fstr::substr(&INPUT, 1..=40),
        SPACE,
        OUTPUT.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Check to see that OUTPUT was unmodified.
    //
    testutil::CHCKAI(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- BIG-IEEE Extreme Integer Values", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = 2;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0;
        COMPAR[I] = 0;
    }

    COMPAR[1] = spicelib::INTMAX();
    COMPAR[2] = spicelib::INTMIN();

    //
    // Get the smallest possible integer bit pattern in memory,
    // even if INTMIN does not cooperate.
    //
    if (COMPAR[2] == -2147483647) {
        COMPAR[2] = (COMPAR[2] - 1);
    }

    //
    // Prepare the INPUT buffer.
    //
    T_XLTFWI(
        COMPAR.as_slice(),
        2,
        INBFF,
        fstr::substr_mut(&mut INPUT, 1..=8),
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZXLATEI(
        INBFF,
        fstr::substr(&INPUT, 1..=8),
        SPACE,
        OUTPUT.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check to see that OUTPUT is reasonable.
    //
    testutil::CHCKAI(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- BIG-IEEE Integral Sequence", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = NUMOUT;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0;
        COMPAR[I] = 0;
    }

    for I in 1..=31 {
        COMPAR[I] = intrinsics::pow(2, (I - 1));
    }

    for I in 32..=62 {
        COMPAR[I] = -intrinsics::pow(2, (I - 32));
    }

    for I in 62..=NUMOUT {
        COMPAR[I] = (1000 * I);
    }

    //
    // Prepare the INPUT buffer.
    //
    T_XLTFWI(COMPAR.as_slice(), NUMOUT, INBFF, &mut INPUT, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZXLATEI(INBFF, &INPUT, SPACE, OUTPUT.as_slice_mut(), ctx)?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check to see that OUTPUT is as expected.
    //
    testutil::CHCKAI(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- BIG-IEEE Byte Value Cycle", ctx)?;

    //
    // Setup the inputs and outputs for the LSB byte case.
    //
    INBFF = BIGI3E;

    SPACE = 256;

    for J in 1..=4 {
        for I in 1..=SPACE {
            OUTPUT[I] = 0;
            COMPAR[I] = 0;
        }

        for I in 1..=SPACE {
            COMPAR[I] = intrinsics::ISHFT((I - 1), (8 * (J - 1)));
        }

        //
        // Prepare the INPUT buffer.
        //
        T_XLTFWI(
            COMPAR.as_slice(),
            SPACE,
            INBFF,
            fstr::substr_mut(&mut INPUT, 1..=(4 * SPACE)),
            ctx,
        )?;

        //
        // Invoke the module.
        //
        spicelib::ZZXLATEI(
            INBFF,
            fstr::substr(&INPUT, 1..=(4 * SPACE)),
            SPACE,
            OUTPUT.as_slice_mut(),
            ctx,
        )?;

        //
        // Check for an exception.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check to see that OUTPUT was unmodified.
        //
        testutil::CHCKAI(
            b"OUTPUT",
            OUTPUT.as_slice(),
            b"=",
            COMPAR.as_slice(),
            SPACE,
            OK,
            ctx,
        )?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
