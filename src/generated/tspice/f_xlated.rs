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

//$Procedure F_XLATED ( ZZXLATED Test Family )
pub fn F_XLATED(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut INPUT = [b' '; 2048 as usize];
    let mut COMPAR = StackArray::<f64, 256>::new(1..=NUMOUT);
    let mut OUTPUT = StackArray::<f64, 256>::new(1..=NUMOUT);
    let mut ICOMP = ActualArray::<i32>::new(1..=(2 * NUMOUT));
    let mut INBFF: i32 = 0;
    let IOUTP = ActualArray::<i32>::new(1..=(2 * NUMOUT));
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
    testutil::TOPEN(b"F_XLATED", ctx)?;

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
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    //
    // Invoke the module.
    //
    spicelib::ZZXLATED(INBFF, &INPUT, SPACE, OUTPUT.as_slice_mut(), ctx)?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Check to see that OUTPUT was unmodified.
    //
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Setup the inputs and outputs for checking the upper bound.
    //
    INBFF = (NUMBFF + 1);
    SPACE = 10;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    //
    // Invoke the module.
    //
    spicelib::ZZXLATED(INBFF, &INPUT, SPACE, OUTPUT.as_slice_mut(), ctx)?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Check to see that OUTPUT was unmodified.
    //
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        0.0,
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
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    //
    // Invoke the module. Restrict INPUT to 1:13 range, since
    // BIG/LTL-IEEE integers come in 4-byte packages.
    //
    spicelib::ZZXLATED(
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
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        0.0,
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
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    //
    // Invoke the module. Restrict INPUT to 1:13 range, since
    // BIG-IEEE integers come in 4-byte packages.
    //
    spicelib::ZZXLATED(
        INBFF,
        fstr::substr(&INPUT, 1..=160),
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
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        0.0,
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
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    //
    // Invoke the module. Restrict INPUT to 1:13 range, since
    // BIG-IEEE integers come in 4-byte packages.
    //
    spicelib::ZZXLATED(
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
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- BIG-IEEE Extreme D.P. Values", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = 2;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    COMPAR[1] = spicelib::DPMAX();
    COMPAR[2] = spicelib::DPMIN();

    //
    // Prepare the INPUT buffer.
    //
    T_XLTFWD(
        COMPAR.as_slice(),
        2,
        INBFF,
        fstr::substr_mut(&mut INPUT, 1..=16),
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZXLATED(
        INBFF,
        fstr::substr(&INPUT, 1..=16),
        SPACE,
        OUTPUT.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for an exception, we expect none.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check to see that OUTPUT is correct.
    //
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        NUMOUT,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- BIG-IEEE Bit-Cycle Sequence", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = NUMOUT;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    for I in 1..=31 {
        DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))[((2 * I) - 1)] =
            intrinsics::pow(2, (I - 1));
    }

    for I in 32..=62 {
        DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))[((2 * I) - 1)] =
            -intrinsics::pow(2, (I - 32));
    }

    for I in 63..=93 {
        DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))[(2 * I)] =
            intrinsics::pow(2, (I - 63));
    }

    for I in 94..=124 {
        DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))[(2 * I)] =
            -intrinsics::pow(2, (I - 94));
    }

    //
    // Prepare the INPUT buffer.
    //
    T_XLTFWD(COMPAR.as_slice(), NUMOUT, INBFF, &mut INPUT, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZXLATED(INBFF, &INPUT, SPACE, OUTPUT.as_slice_mut(), ctx)?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check to see that OUTPUT is as expected.  We will perform
    // the comparison in the equivalenced integers.  The patterns
    // we inserted into the double precision arrays may not represent
    // real double precision numbers.
    //
    testutil::CHCKAI(
        b"IOUTP",
        DummyArray::<i32>::from_equiv(OUTPUT.as_slice(), 1..=(2 * NUMOUT)).as_slice(),
        b"=",
        DummyArray::<i32>::from_equiv(COMPAR.as_slice(), 1..=(2 * NUMOUT)).as_slice(),
        (2 * NUMOUT),
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- BIG-IEEE Byte-Cycle Sequence", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = 256;

    for J in 1..=8 {
        for I in 1..=SPACE {
            OUTPUT[I] = 0.0;
            COMPAR[I] = 0.0;
        }

        if (J <= 4) {
            for I in 1..=SPACE {
                DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))
                    [((2 * I) - 1)] = intrinsics::ISHFT((I - 1), (8 * (J - 1)));
            }
        } else {
            for I in 1..=SPACE {
                DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))
                    [(2 * I)] = intrinsics::ISHFT((I - 1), (8 * (J - 5)));
            }
        }

        //
        // Prepare the INPUT buffer.
        //
        T_XLTFWD(
            COMPAR.as_slice(),
            SPACE,
            INBFF,
            fstr::substr_mut(&mut INPUT, 1..=(8 * SPACE)),
            ctx,
        )?;

        //
        // Invoke the module.
        //
        spicelib::ZZXLATED(
            INBFF,
            fstr::substr(&INPUT, 1..=(8 * SPACE)),
            SPACE,
            OUTPUT.as_slice_mut(),
            ctx,
        )?;

        //
        // Check for the absence of an exception.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Verify that OUTPUT was converted properly.
        //
        testutil::CHCKAI(
            b"IOUTP",
            DummyArray::<i32>::from_equiv(OUTPUT.as_slice(), 1..=(2 * NUMOUT)).as_slice(),
            b"=",
            DummyArray::<i32>::from_equiv(COMPAR.as_slice(), 1..=(2 * NUMOUT)).as_slice(),
            (2 * NUMOUT),
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LTL-IEEE -- BIG-IEEE Miscellaneous Patterns", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    INBFF = BIGI3E;

    SPACE = 16;

    for I in 1..=NUMOUT {
        OUTPUT[I] = 0.0;
        COMPAR[I] = 0.0;
    }

    COMPAR[1] = 1.0;
    COMPAR[2] = -1.0;
    COMPAR[3] = 0.0;
    COMPAR[4] = 10000000000.0;
    COMPAR[5] = 0.0000000001;
    COMPAR[6] = -10000000000.0;
    COMPAR[7] = -0.0000000001;
    COMPAR[8] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    COMPAR[9] = 0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
    COMPAR[10] = -10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    COMPAR[11] = -0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
    COMPAR[12] = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    COMPAR[13] = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
    COMPAR[14] = -1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    COMPAR[15] = -0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;

    //
    // And the MOVED problematic pattern...
    //
    DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))[31] = -771000;
    DummyArrayMut::<i32>::from_equiv(COMPAR.as_slice_mut(), 1..=(2 * NUMOUT))[32] = -771900;

    //
    // Prepare the INPUT buffer.
    //
    T_XLTFWD(
        COMPAR.as_slice(),
        SPACE,
        INBFF,
        fstr::substr_mut(&mut INPUT, 1..=(8 * SPACE)),
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZXLATED(
        INBFF,
        fstr::substr(&INPUT, 1..=(8 * SPACE)),
        SPACE,
        OUTPUT.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare OUTPUT with the expected results.
    //
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        COMPAR.as_slice(),
        15,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"IOUTP",
        DummyArray::<i32>::from_equiv(OUTPUT.as_slice(), 1..=(2 * NUMOUT)).subarray(31),
        b"=",
        DummyArray::<i32>::from_equiv(COMPAR.as_slice(), 1..=(2 * NUMOUT)).subarray(31),
        ((2 * NUMOUT) - 30),
        OK,
        ctx,
    )?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
