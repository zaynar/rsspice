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

//$Procedure ZZDAFGDR ( Private --- DAF Get Data Record )
pub fn ZZDAFGDR(
    HANDLE: i32,
    RECNO: i32,
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
    let mut IAMH: i32 = 0;
    let mut IARCH: i32 = 0;
    let mut IBFF: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LUN: i32 = 0;
    let mut LOCFND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
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
        CHKIN(b"ZZDAFGDR", ctx)?;
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
            CHKOUT(b"ZZDAFGDR", ctx)?;
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
        CHKOUT(b"ZZDAFGDR", ctx)?;
        return Ok(());
    }

    //
    // Now get a logical unit for the handle.  Check FAILED() in
    // case an error occurs.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut LUN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDAFGDR", ctx)?;
        return Ok(());
    }

    //
    // Branch based on whether the binary file format is native
    // or not.  Only supported formats can be opened by ZZDDHOPN,
    // so no check of IBFF is required.
    //
    if (IBFF == save.NATBFF) {
        //
        // In the native case, just read the double precision
        // numbers from the file.
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
            CHKOUT(b"ZZDAFGDR", ctx)?;
            return Ok(());
        }

    //
    // Process the non-native binary file format case.
    //
    } else {
        //
        // Read the data record as characters.
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
            CHKOUT(b"ZZDAFGDR", ctx)?;
            return Ok(());
        }

        //
        // Translate the data record.  Assume (improperly in the
        // general case) that ZZXLATED will translate the contents
        // of the entire record without signaling an error.  This
        // is appropriate at this stage since ZZXLATED simply swaps
        // bytes between BIG-IEEE and LTL-IEEE environments.  In
        // the future, updates may be necessary to prevent
        // processing of garbage data.
        //
        ZZXLATED(IBFF, &CHRBUF, 128, DPBUF.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDAFGDR", ctx)?;
            return Ok(());
        }
    }

    //
    // Transfer the DPs to the output argument and return
    // to the caller.
    //
    *FOUND = true;
    MOVED(DPBUF.as_slice(), 128, DPREC.as_slice_mut());

    CHKOUT(b"ZZDAFGDR", ctx)?;
    Ok(())
}
