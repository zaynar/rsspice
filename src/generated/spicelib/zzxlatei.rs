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

//$Procedure ZZXLATEI ( Private --- Translate Integers )
pub fn ZZXLATEI(
    INBFF: i32,
    INPUT: &[u8],
    SPACE: i32,
    OUTPUT: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut OUTPUT = DummyArrayMut::new(OUTPUT, 1..);
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut J: i32 = 0;
    let mut LENIPT: i32 = 0;
    let mut NUMINT: i32 = 0;
    let mut OSIGN: i32 = 0;
    let mut VALUE: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    //
    // These parameters are used for arithmetic shifting.
    //

    //
    // Local Variables
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
        CHKIN(b"ZZXLATEI", ctx)?;
    }

    //
    // Perform some initialization tasks.
    //
    if save.FIRST {
        //
        // Populate STRBFF with the appropriate binary file
        // format labels.
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
            CHKOUT(b"ZZXLATEI", ctx)?;
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
    // Check to see if INBFF is valid.  This should never occur if this
    // routine is called properly.
    //
    if ((INBFF < 1) || (INBFF > NUMBFF)) {
        SETMSG(b"The integer code used to indicate the binary file format of the input integers, #, is out of range.  This error should never occur.", ctx);
        ERRINT(b"#", INBFF, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZXLATEI", ctx)?;
        return Ok(());
    }

    //
    // Retrieve the length of the input string.
    //
    LENIPT = intrinsics::LEN(INPUT);

    //
    // Now branch based on the value of NATBFF.
    //
    if (save.NATBFF == BIGI3E) {
        if (INBFF == LTLI3E) {
            //
            // Check to see that the length of the input string is
            // appropriate.  Since this is a string containing LTL-IEEE
            // integers and this is a BIG-IEEE machine, characters are
            // 1-byte and integers are 4-bytes.  So the length of INPUT
            // must be a multiple of 4.
            //
            NUMINT = (LENIPT / 4);

            if ((LENIPT - (NUMINT * 4)) != 0) {
                SETMSG(b"The input string that is to be translated from the binary format # to format # has a length that is not a multiple of 4 bytes.  This error should never occur.", ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATEI", ctx)?;
                return Ok(());
            }

            //
            // Verify there is enough room to store the results of
            // the translation.
            //
            if (NUMINT > SPACE) {
                SETMSG(b"The caller specified that # integers are to be translated from binary format # to #.  However there is only room to hold # integers in the output array.  This error should never occur.", ctx);
                ERRINT(b"#", NUMINT, ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                ERRINT(b"#", SPACE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATEI", ctx)?;
                return Ok(());
            }

            //
            // Start looping over each 4 character package in INPUT and
            // converting them to integers.
            //
            for I in 1..=NUMINT {
                //
                // Compute the substring index of the first character
                // in INPUT for this integer.
                //
                J = (-3 + (4 * I));

                //
                // Now arrange the bytes properly.  Since these characters
                // were read from a file utilizing LTL-IEEE, we know that
                // J is the least significant byte and that (J+3) is the
                // most significant.
                //
                // INPUT:
                //
                //         -------------------------------------
                //    . . .|     |  J  | J+1 | J+2 | J+3 |     |. . .
                //         -------------------------------------
                //
                // From this we construct OUTPUT(I) using the following
                // relation:
                //
                //         INPUT(J:J)
                //         INPUT(J+1:J+1) shifted 8 bits to the MSb
                //         INPUT(J+2:J+2) shifted 16 bits to the MSb
                //      +  INPUT(J+3:J+3) shifted 24 bits to the MSb
                //      -------------------------
                //         OUTPUT(I)
                //
                //
                // Perform the necessary computations.  What is outlined
                // above is implemented below using arithmetic operations.
                // The last "shifted 24 bits to the MSb" is handled
                // in a special way, since the sign bit can not be shifted
                // into place through simple multiplication.
                //
                VALUE = ZZICHR(fstr::substr(INPUT, J..=J));
                OUTPUT[I] = VALUE;

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 1)..=(J + 1)));
                VALUE = (VALUE * SHFT8);
                OUTPUT[I] = (OUTPUT[I] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 2)..=(J + 2)));
                VALUE = (VALUE * SHFT16);
                OUTPUT[I] = (OUTPUT[I] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 3)..=(J + 3)));

                //
                // In order to properly install the last byte,
                // the sign bit needs to be managed separately.
                OSIGN = (VALUE / 128);

                //
                // Strip the sign bit if necessary.
                //
                if (OSIGN == 1) {
                    VALUE = (VALUE - 128);
                }

                //
                // Shift the non-sign bits out to their appropriate
                // positions and combine them with OUTPUT(I).
                //
                VALUE = (VALUE * SHFT24);
                OUTPUT[I] = (OUTPUT[I] + VALUE);

                //
                // Install the sign bit.  At the moment in OUTPUT(I)
                // we have the bits precisely as they need to be
                // arranged.  Perform the following computations:
                //
                //    OUTPUT(I) = (2**31-1) - OUTPUT(I) + 1
                //
                // Break this up into steps since 2**31 is not
                // representable with 32 bit integers that utilize
                // 2's complement.
                //
                // First negate the result:
                //
                //    OUTPUT(I) = -OUTPUT(I)
                //
                // But this negation is effectively:
                //
                //    OUTPUT(I) = 2**32 - OUTPUT(I)
                //
                // Which yields:
                //
                //    2**32 - (2**31) + OUTPUT(I)
                //
                // or
                //
                //    2**31 + OUTPUT(I)
                //
                // which is the desired quantity.  Note, 0 must be
                // treated as a special case.
                //
                if (OSIGN == 1) {
                    if (OUTPUT[I] == 0) {
                        OUTPUT[I] = save.SMLINT;
                    } else {
                        OUTPUT[I] = (save.BIGINT - OUTPUT[I]);
                        OUTPUT[I] = (OUTPUT[I] + 1);
                        OUTPUT[I] = -OUTPUT[I];
                    }
                }
            }
        } else {
            SETMSG(b"Unable to translate integers from binary file format # to #.  This error should never occur and is indicative of a bug.  Contact NAIF.", ctx);
            ERRCH(b"#", &save.STRBFF[INBFF], ctx);
            ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZXLATEI", ctx)?;
            return Ok(());
        }
    } else if (save.NATBFF == LTLI3E) {
        if (INBFF == BIGI3E) {
            //
            // Check to see that the length of the input string is
            // appropriate.  Since this is a string containing BIG-IEEE
            // integers and this is a LTL-IEEE machine, characters are
            // 1-byte and integers are 4-bytes.  So the length of INPUT
            // must be a multiple of 4.
            //
            NUMINT = (LENIPT / 4);

            if ((LENIPT - (NUMINT * 4)) != 0) {
                SETMSG(b"The input string that is to be translated from the binary format # to format # has a length that is not a multiple of 4 bytes.  This error should never occur.", ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATEI", ctx)?;
                return Ok(());
            }

            //
            // Verify there is enough room to store the results of
            // the translation.
            //
            if (NUMINT > SPACE) {
                SETMSG(b"The caller specified that # integers are to be translated from binary format # to #.  However there is only room to hold # integers in the output array.  This error should never occur.", ctx);
                ERRINT(b"#", NUMINT, ctx);
                ERRCH(b"#", &save.STRBFF[INBFF], ctx);
                ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                ERRINT(b"#", SPACE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZXLATEI", ctx)?;
                return Ok(());
            }

            //
            // Start looping over each 4 character package in INPUT and
            // converting them to integers.
            //
            for I in 1..=NUMINT {
                //
                // Compute the substring index of the first character
                // in INPUT for this integer.
                //
                J = (-3 + (4 * I));

                //
                // Now arrange the bytes properly.  Since these characters
                // were read from a file utilizing BIG-IEEE, we know that
                // J is the most significant byte and that (J+3) is the
                // least significant.
                //
                // INPUT:
                //
                //         -------------------------------------
                //    . . .|     |  J  | J+1 | J+2 | J+3 |     |. . .
                //         -------------------------------------
                //
                // From this we construct OUTPUT(I) using the following
                // relation:
                //
                //         INPUT(J+3:J+3)
                //         INPUT(J+2:J+2)*SHFT8
                //         INPUT(J+1:J+1)*SHFT16
                //      +  INPUT(J:J)*SHFT24
                //      -------------------------
                //         OUTPUT(I)
                //
                //
                // Perform the necessary computations.  What is outlined
                // above is implemented below using arithmetic operations.
                // The last "shifted 24 bits to the MSb" is handled
                // in a special way, since the sign bit can not be shifted
                // into place through simple multiplication.
                //
                VALUE = ZZICHR(fstr::substr(INPUT, (J + 3)..=(J + 3)));
                OUTPUT[I] = VALUE;

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 2)..=(J + 2)));
                VALUE = (VALUE * SHFT8);
                OUTPUT[I] = (OUTPUT[I] + VALUE);

                VALUE = ZZICHR(fstr::substr(INPUT, (J + 1)..=(J + 1)));
                VALUE = (VALUE * SHFT16);
                OUTPUT[I] = (OUTPUT[I] + VALUE);

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
                // positions and combine them with OUTPUT(I).
                //
                VALUE = (VALUE * SHFT24);
                OUTPUT[I] = (OUTPUT[I] + VALUE);

                //
                // Install the sign bit.  At the moment in OUTPUT(I)
                // we have the bits precisely as they need to be
                // arranged.  Perform the following computations:
                //
                //    OUTPUT(I) = (2**31-1) - OUTPUT(I) + 1
                //
                // Break this up into steps since 2**31 is not
                // representable with 32 bit integers that utilize
                // 2's complement.
                //
                // First, negate the result:
                //
                //    OUTPUT(I) = -OUTPUT(I)
                //
                // But this negation is effectively:
                //
                //    OUTPUT(I) = 2**32 - OUTPUT(I)
                //
                // Which yields:
                //
                //    2**32 - (2**31) + OUTPUT(I)
                //
                // or
                //
                //    2**31 + OUTPUT(I)
                //
                // which is the desired quantity.  Note, 0 must be
                // treated as a special case.
                //
                if (OSIGN == 1) {
                    if (OUTPUT[I] == 0) {
                        OUTPUT[I] = save.SMLINT;
                    } else {
                        OUTPUT[I] = (save.BIGINT - OUTPUT[I]);
                        OUTPUT[I] = (OUTPUT[I] + 1);
                        OUTPUT[I] = -OUTPUT[I];
                    }
                }
            }
        } else {
            SETMSG(b"Unable to translate integers from binary file format # to #.  This error should never occur and is indicative of a bug.  Contact NAIF.", ctx);
            ERRCH(b"#", &save.STRBFF[INBFF], ctx);
            ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZXLATEI", ctx)?;
            return Ok(());
        }

    //
    // The native binary file format on this platform is not supported
    // for the conversion of integers.  This is a bug, as this branch
    // of code should never be reached in normal operation.
    //
    } else {
        SETMSG(b"The native binary file format of this toolkit build, #, is not currently supported for translation of integers from non-native formats.", ctx);
        ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZXLATEI", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZXLATEI", ctx)?;
    Ok(())
}
