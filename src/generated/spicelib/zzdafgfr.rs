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
const FRLOCR: i32 = 1;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const IFNSTR: i32 = 17;
const INLEN: i32 = 4;

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

//$Procedure ZZDAFGFR ( Private --- DAF Get File Record )
pub fn ZZDAFGFR(
    HANDLE: i32,
    IDWORD: &mut [u8],
    ND: &mut i32,
    NI: &mut i32,
    IFNAME: &mut [u8],
    FWARD: &mut i32,
    BWARD: &mut i32,
    FREE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CHRBUF = [b' '; CBFSIZ as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut LOCIDW = [b' '; IDWLEN as usize];
    let mut LOCIFN = [b' '; IFNLEN as usize];
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut CINDEX: i32 = 0;
    let mut IAMH: i32 = 0;
    let mut IARCH: i32 = 0;
    let mut IBFF: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LOCBWD: i32 = 0;
    let mut LOCFRE: i32 = 0;
    let mut LOCFWD: i32 = 0;
    let mut LOCND: i32 = 0;
    let mut LOCNI: i32 = 0;
    let mut LUN: i32 = 0;
    let mut LOCFND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    // Record Number of the file record in a DAF.
    //

    //
    // Length of the IDWORD string.
    //

    //
    // Length of the internal filename string.
    //

    //
    // Starting location in bytes of the internal filename in the
    // file record.
    //

    //
    // Size of an integer in bytes.
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
        CHKIN(b"ZZDAFGFR", ctx)?;
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
            CHKOUT(b"ZZDAFGFR", ctx)?;
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
        CHKOUT(b"ZZDAFGFR", ctx)?;
        return Ok(());
    }

    //
    // Now get a logical unit for the handle.  Check FAILED() in
    // case an error occurs.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut LUN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDAFGFR", ctx)?;
        return Ok(());
    }

    //
    // Branch based on whether the binary file format is native
    // or not.  Only supported formats can be opened by ZZDDHOPN,
    // so no check of IBFF is required.
    //
    if (IBFF == save.NATBFF) {
        //
        // In the native case, just read the components of the file
        // record from the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(LUN)?, Some(FRLOCR))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut LOCIDW)?;
                LOCND = reader.read_i32()?;
                LOCNI = reader.read_i32()?;
                reader.read_str(&mut LOCIFN)?;
                LOCFWD = reader.read_i32()?;
                LOCBWD = reader.read_i32()?;
                LOCFRE = reader.read_i32()?;
                reader.finish()?;
                Ok(())
            })?;
        }

        //
        // Since this routine does not signal any IOSTAT based
        // errors, return if a non-zero value is assigned to IOSTAT.
        //
        if (IOSTAT != 0) {
            CHKOUT(b"ZZDAFGFR", ctx)?;
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

            let mut reader = io::UnformattedReader::new(ctx.io_unit(LUN)?, Some(FRLOCR))?;
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
            CHKOUT(b"ZZDAFGFR", ctx)?;
            return Ok(());
        }

        //
        // Assign the character components of the file record.
        //
        fstr::assign(&mut LOCIDW, fstr::substr(&CHRBUF, 1..=IDWLEN));
        fstr::assign(
            &mut LOCIFN,
            fstr::substr(&CHRBUF, IFNSTR..=((IFNSTR + IFNLEN) - 1)),
        );

        //
        // Convert the integer components.
        //
        CINDEX = (IDWLEN + 1);
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, CINDEX..=((CINDEX + INLEN) - 1)),
            1,
            std::slice::from_mut(&mut LOCND),
            ctx,
        )?;

        CINDEX = (CINDEX + INLEN);
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, CINDEX..=((CINDEX + INLEN) - 1)),
            1,
            std::slice::from_mut(&mut LOCNI),
            ctx,
        )?;

        CINDEX = (IFNSTR + IFNLEN);
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, CINDEX..=((CINDEX + INLEN) - 1)),
            1,
            std::slice::from_mut(&mut LOCFWD),
            ctx,
        )?;

        CINDEX = (CINDEX + INLEN);
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, CINDEX..=((CINDEX + INLEN) - 1)),
            1,
            std::slice::from_mut(&mut LOCBWD),
            ctx,
        )?;

        CINDEX = (CINDEX + INLEN);
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, CINDEX..=((CINDEX + INLEN) - 1)),
            1,
            std::slice::from_mut(&mut LOCFRE),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDAFGFR", ctx)?;
            return Ok(());
        }
    }

    //
    // Transfer the contents of the record to the output arguments
    // and return to the caller.
    //
    *FOUND = true;

    fstr::assign(IDWORD, &LOCIDW);
    *ND = LOCND;
    *NI = LOCNI;
    fstr::assign(IFNAME, &LOCIFN);
    *FWARD = LOCFWD;
    *BWARD = LOCBWD;
    *FREE = LOCFRE;

    CHKOUT(b"ZZDAFGFR", ctx)?;
    Ok(())
}
