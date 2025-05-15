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

//$Procedure T_DAFWDR ( Write a DAF summary/descriptor record to file )
pub fn T_DAFWDR(
    UNIT: i32,
    RECNO: i32,
    OUTBFF: i32,
    NUMDP: i32,
    ARRAY: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut RECORD = [b' '; RECLEN as usize];
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut DATREC = StackArray::<f64, 128>::new(1..=DPRLEN);
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
        spicelib::CHKIN(b"T_DAFWDR", ctx)?;
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
            spicelib::CHKOUT(b"T_DAFWDR", ctx)?;
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
        spicelib::CHKOUT(b"T_DAFWDR", ctx)?;
        return Ok(());
    }

    //
    // Perform some simple checks on NUMDP.
    //
    if ((NUMDP < 0) || (NUMDP > DPRLEN)) {
        spicelib::SETMSG(b"# double precision numbers were requested to be written to #.  Each record holds no more than # numbers.", ctx);
        spicelib::ERRINT(b"#", NUMDP, ctx);
        spicelib::ERRFNM(b"#", UNIT, ctx)?;
        spicelib::ERRINT(b"#", DPRLEN, ctx);
        spicelib::SIGERR(b"SPICE(NUMOUTOFBOUNDS)", ctx)?;
        spicelib::CHKOUT(b"T_DAFWDR", ctx)?;
        return Ok(());
    }

    //
    // First, determine if we are to write to the native file format.
    //
    if (OUTBFF == save.NATBFF) {
        //
        // Clear DATREC, and then move the appropriate values from
        // ARRAY into it.  This is necessary to write a full record
        // to the file.
        //
        spicelib::CLEARD(DPRLEN, DATREC.as_slice_mut());
        spicelib::MOVED(ARRAY.as_slice(), NUMDP, DATREC.as_slice_mut());

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                for I in intrinsics::range(1, DPRLEN, 1) {
                    writer.write_f64(DATREC[I])?;
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
            spicelib::CHKOUT(b"T_DAFWDR", ctx)?;
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
        T_XLTFWD(
            ARRAY.as_slice(),
            NUMDP,
            OUTBFF,
            fstr::substr_mut(&mut RECORD, 1..=((NUMDP * INTLEN) * 2)),
            ctx,
        )?;

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
            spicelib::CHKOUT(b"T_DAFWDR", ctx)?;
            return Ok(());
        }
    }

    spicelib::CHKOUT(b"T_DAFWDR", ctx)?;
    Ok(())
}
