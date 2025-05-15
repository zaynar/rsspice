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
const RECLEN: i32 = 1024;
const DPRLEN: i32 = 128;
const INRLEN: i32 = (2 * DPRLEN);
const INTLEN: i32 = 4;

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

//$Procedure T_DAFWSR ( Write a DAF summary/descriptor record to file )
pub fn T_DAFWSR(
    UNIT: i32,
    RECNO: i32,
    OUTBFF: i32,
    ND: i32,
    NI: i32,
    NEXT: i32,
    PREV: i32,
    NSUM: i32,
    ARRAY: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut RECORD = [b' '; RECLEN as usize];
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut DPREC = StackArray::<f64, 128>::new(1..=DPRLEN);
    let mut DPBUF = StackArray::<f64, 128>::new(1..=DPRLEN);
    let INBUF = StackArray::<i32, 256>::new(1..=INRLEN);
    let mut NUMINT: i32 = 0;
    let mut CINDEX: i32 = 0;
    let mut DINDEX: i32 = 0;
    let mut IOSTAT: i32 = 0;

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
    // Equivalence DPBUF to INBUF to handle unpacking.
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
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_DAFWSR", ctx)?;
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
            spicelib::ZZDDHGSD(b"BFF", I, &mut save.STRBFF[I], ctx);
        }

        //
        // Fetch the native binary file format.
        //
        spicelib::ZZPLATFM(b"FILE_FORMAT", &mut TMPSTR, ctx);
        spicelib::UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);

        save.NATBFF = spicelib::ISRCHC(&TMPSTR, NUMBFF, save.STRBFF.as_arg());

        if (save.NATBFF == 0) {
            spicelib::SETMSG(b"The binary file format, \'#\', is not supported by this version of the toolkit. This is a serious problem, contact NAIF.", ctx);
            spicelib::ERRCH(b"#", &TMPSTR, ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"T_DAFWSR", ctx)?;
            return Ok(());
        }

        //
        // Do not perform initialization tasks again.
        //
        save.FIRST = false;
    }

    //
    // Check to see if OUTBFF is valid.  This should never occur if this
    // routine is called properly.
    //
    if ((OUTBFF < 1) || (OUTBFF > NUMBFF)) {
        spicelib::SETMSG(b"The integer code used to indicate the binary file format of the input integers, #, is out of range.  This error should never occur.", ctx);
        spicelib::ERRINT(b"#", OUTBFF, ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_DAFWSR", ctx)?;
        return Ok(());
    }

    //
    // First, determine if we are to write to the native file format.
    //
    if (OUTBFF == save.NATBFF) {
        //
        // Clear DPREC.
        //
        spicelib::CLEARD(DPRLEN, DPREC.as_slice_mut());

        DPREC[1] = (NEXT as f64);
        DPREC[2] = (PREV as f64);
        DPREC[3] = (NSUM as f64);

        spicelib::MOVED(
            ARRAY.as_slice(),
            (NSUM * (ND + ((NI + 1) / 2))),
            DPREC.subarray_mut(4),
        );

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                for I in intrinsics::range(1, DPRLEN, 1) {
                    writer.write_f64(DPREC[I])?;
                }
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Unable to write to #. IOSTAT was #", ctx);
            spicelib::ERRFNM(b"#", UNIT, ctx)?;
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            spicelib::CHKOUT(b"T_DAFWSR", ctx)?;
            return Ok(());
        }

    //
    // Handle the non-native case.
    //
    } else {
        //
        // Clear RECORD.
        //
        fstr::assign(&mut RECORD, b" ");

        //
        // Populate RECORD.
        //
        DPREC[1] = (NEXT as f64);
        DPREC[2] = (PREV as f64);
        DPREC[3] = (NSUM as f64);

        T_XLTFWD(
            DPREC.as_slice(),
            3,
            OUTBFF,
            fstr::substr_mut(&mut RECORD, 1..=((2 * INTLEN) * 3)),
            ctx,
        )?;

        //
        // Set the index into RECORD to start placing summaries.
        //
        CINDEX = (((2 * INTLEN) * 3) + 1);

        //
        // Now process the summaries.
        //
        for I in 1..=NSUM {
            //
            // Set the starting index into ARRAY for the next summary.
            //
            DINDEX = (((I - 1) * (ND + ((NI + 1) / 2))) + 1);

            //
            // Convert the DPs.
            //
            T_XLTFWD(
                ARRAY.subarray(DINDEX),
                ND,
                OUTBFF,
                fstr::substr_mut(&mut RECORD, CINDEX..=((CINDEX + (ND * (2 * INTLEN))) - 1)),
                ctx,
            )?;

            //
            // Increment CINDEX and DINDEX to the position where the
            // integers will be placed and located.
            //
            DINDEX = (DINDEX + ND);
            CINDEX = (CINDEX + (ND * (2 * INTLEN)));

            //
            // Unpack the integer components.
            //
            spicelib::MOVED(ARRAY.subarray(DINDEX), ((NI + 1) / 2), DPBUF.as_slice_mut());

            //
            // Translate the integers. We are translating 2*((NI+1)/2)
            // integers, which is not always NI integers.  Rather than
            // introduce special exception for the two cases (NI odd and
            // NI even, we will just translate the garbage in the last
            // integer in the odd case.
            //
            NUMINT = (2 * ((NI + 1) / 2));
            T_XLTFWI(
                DummyArray::<i32>::from_equiv(DPBUF.as_slice(), 1..=INRLEN).as_slice(),
                NUMINT,
                OUTBFF,
                fstr::substr_mut(&mut RECORD, CINDEX..=((CINDEX + (NUMINT * INTLEN)) - 1)),
                ctx,
            )?;

            //
            // Increment CINDEX.  We ignore DINDEX because it will be
            // set properly at the start of the loop.
            //
            CINDEX = (CINDEX + (NUMINT * INTLEN));
        }

        //
        // Dump the record to the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&RECORD)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Unable to write to #. IOSTAT was #", ctx);
            spicelib::ERRFNM(b"#", UNIT, ctx)?;
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            spicelib::CHKOUT(b"T_DAFWSR", ctx)?;
            return Ok(());
        }
    }

    spicelib::CHKOUT(b"T_DAFWSR", ctx)?;
    Ok(())
}
