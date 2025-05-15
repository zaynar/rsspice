//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const NWD: i32 = 128;
const NWI: i32 = 256;
const NWC: i32 = 1024;
const CHARDT: i32 = 1;
const DPDT: i32 = 2;
const INTDT: i32 = 3;
const FILSIZ: i32 = 255;

struct SaveVars {
    INTBFF: i32,
    NATBFF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut INTBFF: i32 = 0;
        let mut NATBFF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            INTBFF,
            NATBFF,
            FIRST,
        }
    }
}

//$Procedure ZZDASGRD ( DAS, get record, double precision )
pub fn ZZDASGRD(
    HANDLE: i32,
    RECNO: i32,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut RECORD = DummyArrayMut::new(RECORD, 1..=NWD);
    let mut CHRREC = [b' '; NWC as usize];
    let mut FNAME = [b' '; FILSIZ as usize];
    let mut INTAMH: i32 = 0;
    let mut INTARC: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDASGRD", ctx)?;

    if save.FIRST {
        //
        // Get the integer code for the host binary file format.
        //
        ZZDDHNFC(&mut save.NATBFF, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDASGRD", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // Get a logical unit for this file.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut UNIT, ctx)?;

    //
    // Get the binary file format of the file designated by HANDLE.
    //
    ZZDDHNFO(
        HANDLE,
        &mut FNAME,
        &mut INTARC,
        &mut save.INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDASGRD", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(b"Unable to locate file associated with HANDLE, #. The most likely cause of this is the file that you are trying to read has been closed.", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(HANDLENOTFOUND)", ctx)?;
        CHKOUT(b"ZZDASGRD", ctx)?;
        return Ok(());
    }

    if (save.INTBFF == save.NATBFF) {
        //
        // The file has native format.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                for n in RECORD.iter_mut() {
                    *n = reader.read_f64()?;
                }
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Could not read DAS d.p. record. File = # Record number = #. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", RECNO, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DASFILEREADFAILED)", ctx)?;
            CHKOUT(b"ZZDASGRD", ctx)?;
            return Ok(());
        }
    } else {
        //
        // Read the record as a character string, then translate it
        // to an array of d.p. numbers.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut CHRREC)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(b"Could not read non-native DAS d.p. record into character array. File = # Record number = #. IOSTAT = #.", ctx);
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", RECNO, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DASFILEREADFAILED)", ctx)?;
            CHKOUT(b"ZZDASGRD", ctx)?;
            return Ok(());
        }

        //
        // Translate the character record to double precision type.
        //
        ZZXLATED(save.INTBFF, &CHRREC, NWD, RECORD.as_slice_mut(), ctx)?;

        //
        // We don't test FAILED here because the routine
        // will return from this point.
        //
    }

    CHKOUT(b"ZZDASGRD", ctx)?;
    Ok(())
}
