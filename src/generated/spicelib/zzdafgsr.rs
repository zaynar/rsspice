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
const DPLEN: i32 = 8;
const INLEN: i32 = (DPLEN / 2);

struct SaveVars {
    STRBFF: ActualCharArray,
    NATBFF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
        let mut NATBFF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        NATBFF = 0;

        Self {
            STRBFF,
            NATBFF,
            FIRST,
        }
    }
}

//$Procedure ZZDAFGSR ( Private --- DAF Get Summary/Descriptor Record )
pub fn ZZDAFGSR(
    HANDLE: i32,
    RECNO: i32,
    ND: i32,
    NI: i32,
    DPREC: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DPREC = DummyArrayMut::new(DPREC, 1..);
    let mut CHRBUF = [b' '; CBFSIZ as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut DPBUF = StackArray::<f64, 128>::new(1..=128);
    let mut CINDEX: i32 = 0;
    let mut DINDEX: i32 = 0;
    let mut IAMH: i32 = 0;
    let mut IARCH: i32 = 0;
    let mut IBFF: i32 = 0;
    let mut INBUF = StackArray::<i32, 256>::new(1..=256);
    let mut IOSTAT: i32 = 0;
    let mut LEFT: i32 = 0;
    let mut LUN: i32 = 0;
    let mut NSUM: i32 = 0;
    let mut SUMSIZ: i32 = 0;
    let mut LOCFND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    // Length in bytes of double precision numbers and integers.
    //

    //
    // Local Variables
    //

    //
    // Equivalence DPBUF to INBUF.
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDAFGSR", ctx)?;
    }

    //
    // Perform some initialization tasks.
    //
    if save.FIRST {
        //
        // Populate STRBFF, the buffer that contains the labels
        // for each binary file format.
        //
        for I in 1..=NUMBFF {
            ZZDDHGSD(b"BFF", I, &mut save.STRBFF[I], ctx);
        }

        //
        // Fetch the native binary file format and determine its
        // integer code.
        //
        ZZPLATFM(b"FILE_FORMAT", &mut TMPSTR, ctx);
        UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);

        save.NATBFF = ISRCHC(&TMPSTR, NUMBFF, save.STRBFF.as_arg());

        if (save.NATBFF == 0) {
            SETMSG(b"The binary file format, \'#\', is not supported by this version of the toolkit. This is a serious problem, contact NAIF.", ctx);
            ERRCH(b"#", &TMPSTR, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZDAFGSR", ctx)?;
            return Ok(());
        }

        //
        // Do not perform initialization tasks again.
        //
        save.FIRST = false;
    }

    //
    // Assume the data record will not be found, until it has been read
    // from the file, and if necessary, successfully translated.
    //
    *FOUND = false;

    //
    // Retrieve information regarding the file from the handle manager.
    // The value of IARCH is not a concern, since this is a DAF routine
    // all values passed into handle manager entry points will have
    // 'DAF' as their architecture arguments.
    //
    ZZDDHNFO(
        HANDLE,
        &mut FNAME,
        &mut IARCH,
        &mut IBFF,
        &mut IAMH,
        &mut LOCFND,
        ctx,
    )?;

    if !LOCFND {
        SETMSG(b"Unable to locate file associated with HANDLE, #.  The most likely cause of this is the file that you are trying to read has been closed.", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(HANDLENOTFOUND)", ctx)?;
        CHKOUT(b"ZZDAFGSR", ctx)?;
        return Ok(());
    }

    //
    // Now get a logical unit for the handle.  Check FAILED()
    // in case an error occurs.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut LUN, ctx)?;

    if FAILED(ctx) {
        *FOUND = false;
        CHKOUT(b"ZZDAFGSR", ctx)?;
        return Ok(());
    }

    //
    // Branch based on whether the binary file format is native
    // or not.  Only supported formats can be opened by ZZDDHOPN,
    // so no check of IBFF is required.
    //
    if (IBFF == save.NATBFF) {
        //
        // In the native case, just read the array of double precision
        // numbers from the file.  The packed integers will be
        // processed properly by the READ.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(LUN)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                for I in intrinsics::range(1, 128, 1) {
                    DPBUF[I] = reader.read_f64()?;
                }
                reader.finish()?;
                Ok(())
            })?;
        }

        //
        // Since this routine does not signal any IOSTAT based
        // errors, return if a non-zero value is assigned to IOSTAT.
        //
        if (IOSTAT != 0) {
            CHKOUT(b"ZZDAFGSR", ctx)?;
            return Ok(());
        }

    //
    // Process the non-native binary file format case.
    //
    } else {
        //
        // Read the record as characters.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(LUN)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut CHRBUF)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        //
        // Again, since this routine does not signal any IOSTAT
        // based errors, return if one occurs.
        //
        if (IOSTAT != 0) {
            CHKOUT(b"ZZDAFGSR", ctx)?;
            return Ok(());
        }

        //
        // Translate the summary record.  First extract the leading
        // 3 double precision numbers from the summary record as these
        // respectively are NEXT, PREV, and NSUM.
        //
        ZZXLATED(
            IBFF,
            fstr::substr(&CHRBUF, 1..=(3 * DPLEN)),
            128,
            DPBUF.as_slice_mut(),
            ctx,
        )?;

        //
        // Check FAILED() in case the translation process fails for
        // any reason.
        //
        if FAILED(ctx) {
            CHKOUT(b"ZZDAFGSR", ctx)?;
            return Ok(());
        }

        //
        // Convert NSUM to an integer, and compute the number of
        // double precision numbers required to store each individual
        // summary in the record.
        //
        NSUM = (DPBUF[3] as i32);
        SUMSIZ = (ND + ((NI + 1) / 2));

        //
        // Convert each of the summaries one at a time.
        //
        for I in 1..=NSUM {
            //
            // Set the start index into the double precision array
            // to receive the components.  Also set the character
            // substring index to the start location for this summary.
            // In the diagram below, each box represents a double
            // precision number.  The figure assumes SUMSIZ is 5
            // double precision numbers:
            //
            //       |--- 1 ---|--- 2 ---|--- 3 ---|   |- (I-1) -|
            // -------------------------------------   -------------
            // | | | | | | | | | | | | | | | | | | |...| | | | | | |...
            // -------------------------------------   -------------
            // |-----|                                            ^
            //    ^                                               |
            //    |                                            Summary
            // NEXT, PREV, NSUM                                 Start
            //
            DINDEX = (((I - 1) * SUMSIZ) + 4);
            CINDEX = ((DPLEN * (DINDEX - 1)) + 1);

            //
            // First, check to see if there are any double precision
            // numbers to translate.  If so, translate, and then
            // increment DINDEX and CINDEX accordingly.
            //
            if (ND > 0) {
                //
                // DPBUF has room for 128 double precision numbers
                // total.  Compute the amount of space left in the
                // buffer.
                //
                LEFT = ((128 - ((I - 1) * SUMSIZ)) - 3);

                ZZXLATED(
                    IBFF,
                    fstr::substr(&CHRBUF, CINDEX..=((CINDEX + (ND * DPLEN)) - 1)),
                    LEFT,
                    DPBUF.subarray_mut(DINDEX),
                    ctx,
                )?;

                //
                // If the translation routine fails for any reason,
                // check out and return.
                //
                if FAILED(ctx) {
                    CHKOUT(b"ZZDAFGSR", ctx)?;
                    return Ok(());
                }

                DINDEX = (DINDEX + ND);
                CINDEX = (CINDEX + (DPLEN * ND));
            }

            //
            // At this point DINDEX and CINDEX are pointing at the
            // locations for the packed integers in the record.
            // Use DINDEX to compute the index into INBUF, the
            // equivalenced integer buffer and translate.
            //
            if (NI > 0) {
                //
                // INBUF has room for 256 integers total.  Compute
                // the amount of space left in the buffer.  Since
                // it is equivalenced to DPBUF, account for the
                // double precision numbers that were just added.
                //
                LEFT = (((256 - ((2 * (I - 1)) * SUMSIZ)) - (ND * 2)) - 6);

                ZZXLATEI(
                    IBFF,
                    fstr::substr(&CHRBUF, CINDEX..=((CINDEX + (NI * INLEN)) - 1)),
                    LEFT,
                    DummyArrayMut::<i32>::from_equiv(DPBUF.as_slice_mut(), 1..=256)
                        .subarray_mut(((2 * DINDEX) - 1)),
                    ctx,
                )?;

                //
                // If the translation routine fails for any reason,
                // check out and return.
                //
                if FAILED(ctx) {
                    CHKOUT(b"ZZDAFGSR", ctx)?;
                    return Ok(());
                }

                //
                // Now check to see if NI is odd.  If so, then zero
                // the last integer occupied by the newly translated
                // summary.  This is necessary to purge any garbage
                // present in memory.
                //
                if (intrinsics::MOD(NI, 2) == 1) {
                    DummyArrayMut::<i32>::from_equiv(DPBUF.as_slice_mut(), 1..=256)
                        [(((2 * DINDEX) - 1) + NI)] = 0;
                }
            }
        }

        //
        // Translating garbage is a bad idea in general, so set
        // the any remaining double precision numbers in the summary
        // record to 0.
        //
        DINDEX = ((NSUM * SUMSIZ) + 4);

        for I in DINDEX..=128 {
            DPBUF[I] = 0 as f64;
        }
    }

    //
    // Transfer the DPs to the output argument and return to the
    // caller.
    //
    *FOUND = true;
    MOVED(DPBUF.as_slice(), 128, DPREC.as_slice_mut());

    CHKOUT(b"ZZDAFGSR", ctx)?;
    Ok(())
}
