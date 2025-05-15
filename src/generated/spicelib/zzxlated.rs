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
const DPBLEN: i32 = 128;
const INBLEN: i32 = (2 * DPBLEN);
const SHFT8: i32 = intrinsics::pow(2, 8);
const SHFT16: i32 = intrinsics::pow(2, 16);
const SHFT24: i32 = intrinsics::pow(2, 24);

struct SaveVars {
    STRBFF: ActualCharArray,
    BIGINT: i32,
    NATBFF: i32,
    SMLINT: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
        let mut BIGINT: i32 = 0;
        let mut NATBFF: i32 = 0;
        let mut SMLINT: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        NATBFF = 0;

        Self {
            STRBFF,
            BIGINT,
            NATBFF,
            SMLINT,
            FIRST,
        }
    }
}

fn ZZICHR(CARG: &[u8]) -> i32 {
    let CARG = &CARG[..1 as usize];
    (intrinsics::ICHAR(CARG)
        - (intrinsics::MAX0(&[-1, intrinsics::MIN0(&[0, intrinsics::ICHAR(CARG)])]) * 256))
}

//$Procedure ZZXLATED ( Private --- Translate Double Precision Numbers )
pub fn ZZXLATED(
    INBFF: i32,
    INPUT: &[u8],
    SPACE: i32,
    OUTPUT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut OUTPUT = DummyArrayMut::new(OUTPUT, 1..);
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut DPBUFR = StackArray::<f64, 128>::new(1..=DPBLEN);
    let mut INBUFR = StackArray::<i32, 256>::new(1..=INBLEN);
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut LENIPT: i32 = 0;
    let mut NUMDP: i32 = 0;
    let mut OSIGN: i32 = 0;
    let mut OUTPOS: i32 = 0;
    let mut VALUE: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Length of the double precision and integer buffers that
    // are equivalenced.
    //

    //
    // These parameters are used for arithmetic shifting.
    //

    //
    // Local Variables
    //

    //
    // Equivalence DPBUFR to INBUFR.
    //

    //
    // Statement Functions
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Statement Function Definitions
    //
    // This function controls the conversion of characters to integers.
    // Some versions of the g77 implement ICHAR with a signed integer.
    // This function computes the value of ICHAR that this code requires
    // on any version of g77 for x86 Linux.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZXLATED", ctx)?;
    }

    //
    // Perform some initialization tasks.
    //
    if save.FIRST {
        //
        // Populate STRBFF.
        //
        for I in 1..=NUMBFF {
            ZZDDHGSD(b"BFF", I, &mut save.STRBFF[I], ctx);
        }

        //
        // Fetch the native binary file format.
        //
        ZZPLATFM(b"FILE_FORMAT", &mut TMPSTR, ctx);
        UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);

        save.NATBFF = ISRCHC(&TMPSTR, NUMBFF, save.STRBFF.as_arg());

        if (save.NATBFF == 0) {
            SETMSG(b"The binary file format, \'#\', is not supported by this version of the toolkit. This is a serious problem, contact NAIF.", ctx);
            ERRCH(b"#", &TMPSTR, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZXLATED", ctx)?;
            return Ok(());
        }

        //
        // Store the largest value a 32-bit integer can actually
        // hold.
        //
        save.BIGINT = 2147483647;

        //
        // Prepare the smallest value a 32-bit integer can actually
        // store, regardless of what INTMIN returns.
        //
        save.SMLINT = INTMIN();

        //
        // Set SMLINT to the appropriate value if INTMIN is too large.
        //
        if (save.SMLINT == -2147483647) {
            save.SMLINT = (save.SMLINT - 1);
        }

        //
        // Do not perform initialization tasks again.
        //
        save.FIRST = false;
    }

    //
    // Check to see if INBFF makes sense.
    //
    if ((INBFF < 1) || (INBFF > NUMBFF)) {
        SETMSG(b"The integer code used to indicate the binary file format of the input integers, #, is out of range.  This error should never occur.", ctx);
        ERRINT(b"#", INBFF, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZXLATED", ctx)?;
        return Ok(());
    }

    //
    // Retrieve the length of the input string, and set the position
    // into the output buffer to the beginning.
    //
    LENIPT = intrinsics::LEN(INPUT);
    OUTPOS = 1;

    //
    // Now branch based on NATBFF.
    //
    if (save.NATBFF == BIGI3E) {
        if (INBFF == LTLI3E) {
            //
            // Check to see that the length of the input string is
            // appropriate.  Since this is a string containing LTL-IEEE
            // d.p. values, and this is a BIG-IEEE machine characters
            // are 1-byte and d.p. values are 8-bytes.  So the length
            // of INPUT must be a multiple of 8.
            //
            NUMDP = (LENIPT / 8);

            if ((LENIPT - (NUMDP * 8)) != 0) {
                SETMSG(b"The input string that is to be translated from the binary format # to format # has a length that is not a multiple of 4 bytes.  This error should never occur.", ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATED", ctx)?;
                return Ok(());
            }

            //
            // Verify there is enough room to store the results of
            // the translation.
            //
            if (NUMDP > SPACE) {
                SETMSG(b"The caller specified that # double precision numbers are to be translated from binary format # to #.  However there is only room to hold # integers in the output array.  This error should never occur.", ctx);
                ERRINT(b"#", NUMDP, ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                ERRINT(b"#", SPACE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATED", ctx)?;
                return Ok(());
            }

            //
            // The remainder of this branch is devoted to translating
            // and copying blocks of DPBLEN double precision numbers
            // into OUTPUT.  Initialize K, the integer index into the
            // buffer equivalenced to DPBUFR.
            //
            K = 1;

            //
            // Start looping over each 8 character package in INPUT and
            // converting it to double precision numbers.
            //
            for I in 1..=NUMDP {
                //
                // Compute the substring index of the first character
                // in INPUT for this integer.
                //
                J = (-7 + (8 * I));

                //
                // Now arrange the bytes properly.  Since these characters
                // were read from a file utilizing LTL-IEEE:
                //
                //         .
                //         .
                //         .
                //      -------
                //     |   J   |  - Least Significant Byte of Mantissa
                //      -------
                //     |  J+1  |  - Sixth Most Significant Mantissa Byte
                //      -------
                //     |  J+2  |  - Fifth Most Significant Mantissa Byte
                //      -------
                //     |  J+3  |  - Fourth Most Significant Mantissa Byte
                //      -------
                //     |  J+4  |  - Third Most Significant Mantissa Byte
                //      -------
                //     |  J+5  |  - Second Most Significant Mantissa Byte
                //      -------
                //     |  J+6  |  - Tail of Exponent, Most Significant
                //      -------     Bits of the Mantissa
                //     |  J+7  |  - Sign Bit, Head of Exponent
                //      -------
                //         .
                //         .
                //         .
                //
                // Now rearrange the bytes to place them in the
                // proper order for d.p. values on BIG-IEEE machines.
                // This is accomplished in the following manner:
                //
                //        INPUT(J+4:J+4)
                //        INPUT(J+5:J+5)*SHFT8
                //        INPUT(J+6:J+6)*SHFT16
                //     +  INPUT(J+7:J+7)*SHFT24
                //     -------------------------
                //        INBUFR(K)
                //
                //        INPUT(J:J)
                //        INPUT(J+1:J+1)*SHFT8
                //        INPUT(J+2:J+2)*SHFT16
                //     +  INPUT(J+3:J+3)*SHFT24
                //     -------------------------
                //        INBUFR(K+1)
                //
                //
                // Perform the necessary computations.  What is outlined
                // above is implemented below using arithmetic operations.
                // The last "shifted 24 bits to the MSb" is handled
                // in a special way, since the sign bit can not be shifted
                // into place through simple multiplication.
                //
                VALUE = ZZICHR(fstr::substr(INPUT, (J + 4)..=(J + 4)));
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] = VALUE;

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 5)..=(J + 5)));
                VALUE = (VALUE * SHFT8);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 6)..=(J + 6)));
                VALUE = (VALUE * SHFT16);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 7)..=(J + 7)));

                //
                // In order to properly install the last byte,
                // the sign bit needs to be managed separately.
                //
                OSIGN = (VALUE / 128);

                //
                // Strip the sign bit if necessary.
                //
                if (OSIGN == 1) {
                    VALUE = (VALUE - 128);
                }

                //
                // Shift the non-sign bits out to their appropriate
                // positions and combine them with INBUFR(K).
                //
                VALUE = (VALUE * SHFT24);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + VALUE);

                //
                // Install the sign bit.  At the moment in INBUFR(K)
                // the bits are precisely as they need to be arranged.
                // Perform the following computations:
                //
                //    INBUFR(K) = (2**31-1) - INBUFR(K) + 1
                //
                // Break this up into steps since 2**31 is not
                // representable with 32 bit integers that utilize
                // 2's complement.
                //
                // First, negate the result:
                //
                //    INBUFR(K) = -INBUFR(K)
                //
                // But this negation is effectively:
                //
                //    INBUFR(K) = 2**32 - INBUFR(K)
                //
                // Which yields:
                //
                //    2**32 - (2**31) + INBUFR(K)
                //
                // or
                //
                //    2**31 + INBUFR(K)
                //
                // which is the desired quantity.  Note, 0 must be
                // treated as a special case.
                //
                if (OSIGN == 1) {
                    if (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] == 0) {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            save.SMLINT;
                    } else {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            (save.BIGINT
                                - DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K]);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + 1);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            -DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K];
                    }
                }

                VALUE = ZZICHR(fstr::substr(INPUT, J..=J));
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    VALUE;

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 1)..=(J + 1)));
                VALUE = (VALUE * SHFT8);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 2)..=(J + 2)));
                VALUE = (VALUE * SHFT16);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 3)..=(J + 3)));

                //
                // In order to properly install the last byte,
                // the sign bit needs to be managed separately.
                //
                OSIGN = (VALUE / 128);

                //
                // Strip the sign bit if necessary.
                //
                if (OSIGN == 1) {
                    VALUE = (VALUE - 128);
                }

                //
                // Shift the non-sign bits out to their appropriate
                // positions and combine them with INBUFR(K).
                //
                VALUE = (VALUE * SHFT24);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] + VALUE);

                //
                // Install the sign bit.  At the moment in INBUFR(K+1)
                // we have the bits precisely as they need to be
                // arranged.  Perform the computations as with the
                // previous integer.
                //
                if (OSIGN == 1) {
                    if (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] == 0)
                    {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] = save.SMLINT;
                    } else {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] = (save.BIGINT
                            - DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)
                                [(K + 1)]);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] =
                            (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)]
                                + 1);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] =
                            -DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)];
                    }
                }

                //
                // Check to see if the local buffer is full and the
                // double precision numbers need to be moved into the
                // next block of OUTPUT.
                //
                if (K == (INBLEN - 1)) {
                    MOVED(DPBUFR.as_slice(), DPBLEN, OUTPUT.subarray_mut(OUTPOS));
                    OUTPOS = (OUTPOS + DPBLEN);
                    K = 1;

                //
                // Otherwise, increment K.
                //
                } else {
                    K = (K + 2);
                }
            }

            //
            // Copy any remaining double precision numbers from DPBUFR
            // into OUTPUT.
            //
            if (K != 1) {
                MOVED(DPBUFR.as_slice(), (K / 2), OUTPUT.subarray_mut(OUTPOS));
            }
        } else {
            SETMSG(b"Unable to translate double precision values from binary file format # to #. This error should never occur and is indicative of a bug.  Contact NAIF.", ctx);
            ERRCH(b"#", &save.STRBFF[INBFF], ctx);
            ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZXLATED", ctx)?;
            return Ok(());
        }
    } else if (save.NATBFF == LTLI3E) {
        if (INBFF == BIGI3E) {
            //
            // Check to see that the length of the input string is
            // appropriate.  Since this is a string containing BIG-IEEE
            // d.p. values, and this is a LTL-IEEE machine characters
            // are 1-byte and d.p. values are 8-bytes.  So the length
            // of INPUT must be a multiple of 8.
            //
            NUMDP = (LENIPT / 8);

            if ((LENIPT - (NUMDP * 8)) != 0) {
                SETMSG(b"The input string that is to be translated from the binary format # to format # has a length that is not a multiple of 4 bytes.  This error should never occur.", ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATED", ctx)?;
                return Ok(());
            }

            //
            // Verify there is enough room to store the results of
            // the translation.
            //
            if (NUMDP > SPACE) {
                SETMSG(b"The caller specified that # double precision numbers are to be translated from binary format # to #.  However there is only room to hold # integers in the output array.  This error should never occur.", ctx);
                ERRINT(b"#", NUMDP, ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                ERRINT(b"#", SPACE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATED", ctx)?;
                return Ok(());
            }

            //
            // The remainder of this branch is devoted to translating
            // and copying blocks of DPBLEN double precision numbers
            // into OUTPUT.  Initialize K, the integer index into the
            // buffer equivalenced to DPBUFR.
            //
            K = 1;

            //
            // Start looping over each 8 character package in INPUT and
            // converting them to double precision numbers.
            //
            for I in 1..=NUMDP {
                //
                // Compute the substring index of the first character
                // in INPUT for this integer.
                //
                J = (-7 + (8 * I));

                //
                // Now arrange the bytes properly.  Since these characters
                // were read from a file utilizing BIG-IEEE:
                //
                //         .
                //         .
                //         .
                //      -------
                //     |   J   |  - Sign Bit, Head of Exponent
                //      -------
                //     |  J+1  |  - Tail of Exponent, Most Significant
                //      -------     Bits of the Mantissa
                //     |  J+2  |  - Second Most Significant Mantissa Byte
                //      -------
                //     |  J+3  |  - Third Most Significant Mantissa Byte
                //      -------
                //     |  J+4  |  - Fourth Most Significant Mantissa Byte
                //      -------
                //     |  J+5  |  - Fifth Most Significant Mantissa Byte
                //      -------
                //     |  J+6  |  - Sixth Most Significant Mantissa Byte
                //      -------
                //     |  J+7  |  - Least Significant Byte of Mantissa
                //      -------
                //         .
                //         .
                //         .
                //
                // Now rearrange the bytes to place them in the
                // proper order for d.p. values on LTL-IEEE machines.
                // This is accomplished in the following manner:
                //
                //        INPUT(J+7:J+7)
                //        INPUT(J+6:J+6)*SHFT8
                //        INPUT(J+5:J+5)*SHFT16
                //     +  INPUT(J+4:J+4)*SHFT24
                //     -------------------------
                //        INBUFR(K)
                //
                //        INPUT(J+3:J+3)
                //        INPUT(J+2:J+2)*SHFT8
                //        INPUT(J+1:J+1)*SHFT16
                //     +  INPUT(J:J)*SHFT24
                //     -------------------------
                //        INBUFR(K+1)
                //
                //
                // Perform the necessary computations. What is outlined
                // above is implemented below using arithmetic operations.
                // The last "shifted 24 bits to the MSb" is handled
                // in a special way, since the sign bit can not be shifted
                // into place through simple multiplication.
                //
                VALUE = ZZICHR(fstr::substr(INPUT, (J + 7)..=(J + 7)));
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] = VALUE;

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 6)..=(J + 6)));
                VALUE = (VALUE * SHFT8);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 5)..=(J + 5)));
                VALUE = (VALUE * SHFT16);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 4)..=(J + 4)));

                //
                // In order to properly install the last byte,
                // the sign bit needs to be managed separately.
                //
                OSIGN = (VALUE / 128);

                //
                // Strip the sign bit if necessary.
                //
                if (OSIGN == 1) {
                    VALUE = (VALUE - 128);
                }

                //
                // Shift the non-sign bits out to their appropriate
                // positions and combine them with INBUFR(K).
                //
                VALUE = (VALUE * SHFT24);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + VALUE);

                //
                // Install the sign bit.  At the moment in INBUFR(K)
                // the bits are precisely as they need to be arranged.
                // Perform the following computations:
                //
                //    INBUFR(K) = (2**31-1) - INBUFR(K) + 1
                //
                // Break this up into steps since 2**31 is not
                // representable with 32 bit integers that utilize
                // 2's complement.
                //
                // First, negate the result:
                //
                //    INBUFR(K) = -INBUFR(K)
                //
                // But this negation is effectively:
                //
                //    INBUFR(K) = 2**32 - INBUFR(K)
                //
                // Which yields:
                //
                //    2**32 - (2**31) + INBUFR(K)
                //
                // or
                //
                //    2**31 + INBUFR(K)
                //
                // which is the desired quantity.  Note, 0 must be
                // treated as a special case.
                //
                if (OSIGN == 1) {
                    if (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] == 0) {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            save.SMLINT;
                    } else {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            (save.BIGINT
                                - DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K]);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K] + 1);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[K] =
                            -DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[K];
                    }
                }

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 3)..=(J + 3)));
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    VALUE;

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 2)..=(J + 2)));
                VALUE = (VALUE * SHFT8);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 1)..=(J + 1)));
                VALUE = (VALUE * SHFT16);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, J..=J));

                //
                // In order to properly install the last byte,
                // the sign bit needs to be managed separately.
                //
                OSIGN = (VALUE / 128);

                //
                // Strip the sign bit if necessary.
                //
                if (OSIGN == 1) {
                    VALUE = (VALUE - 128);
                }

                //
                // Shift the non-sign bits out to their appropriate
                // positions and combine them with INBUFR(K).
                //
                VALUE = (VALUE * SHFT24);
                DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)[(K + 1)] =
                    (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] + VALUE);

                //
                // Install the sign bit.  At the moment in INBUFR(K+1)
                // we have the bits precisely as they need to be
                // arranged.  Perform the computations as with the
                // previous integer.
                //
                if (OSIGN == 1) {
                    if (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)] == 0)
                    {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] = save.SMLINT;
                    } else {
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] = (save.BIGINT
                            - DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)
                                [(K + 1)]);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] =
                            (DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)]
                                + 1);
                        DummyArrayMut::<i32>::from_equiv(DPBUFR.as_slice_mut(), 1..=INBLEN)
                            [(K + 1)] =
                            -DummyArray::<i32>::from_equiv(DPBUFR.as_slice(), 1..=INBLEN)[(K + 1)];
                    }
                }

                //
                // Check to see if the local buffer is full and the
                // double precision numbers need to be moved into the
                // next block of OUTPUT.
                //
                if (K == (INBLEN - 1)) {
                    MOVED(DPBUFR.as_slice(), DPBLEN, OUTPUT.subarray_mut(OUTPOS));
                    OUTPOS = (OUTPOS + DPBLEN);
                    K = 1;

                //
                // Otherwise, increment K.
                //
                } else {
                    K = (K + 2);
                }
            }

            //
            // Copy any remaining double precision numbers from DPBUFR
            // into OUTPUT.
            //
            if (K != 1) {
                MOVED(DPBUFR.as_slice(), (K / 2), OUTPUT.subarray_mut(OUTPOS));
            }
        } else {
            SETMSG(b"Unable to translate double precision values from binary file format # to #. This error should never occur and is indicative of a bug.  Contact NAIF.", ctx);
            ERRCH(b"#", &save.STRBFF[INBFF], ctx);
            ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZXLATED", ctx)?;
            return Ok(());
        }

    //
    // The native binary file format on this platform is not supported
    // for the conversion of integers.  This is a bug, as this branch
    // of code should never be reached in normal operation.
    //
    } else {
        SETMSG(b"The native binary file format of this toolkit build, #, is not currently supported for translation of double precision numbers from non-native formats.", ctx);
        ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZXLATED", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZXLATED", ctx)?;
    Ok(())
}
