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
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const NWC: i32 = 1024;
const BEGIDW: i32 = 1;
const ENDIDW: i32 = 8;
const BEGIFN: i32 = 9;
const ENDIFN: i32 = 68;
const BEGNRR: i32 = 69;
const ENDNRR: i32 = 72;
const BEGNRC: i32 = 73;
const ENDNRC: i32 = 76;
const BEGNCR: i32 = 77;
const ENDNCR: i32 = 80;
const BEGNCC: i32 = 81;
const ENDNCC: i32 = 84;

struct SaveVars {
    NATBFF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NATBFF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        NATBFF = -1;

        Self { NATBFF, FIRST }
    }
}

//$Procedure      ZZDASRFR ( DAS, read file record )
pub fn ZZDASRFR(
    HANDLE: i32,
    IDWORD: &mut [u8],
    IFNAME: &mut [u8],
    NRESVR: &mut i32,
    NRESVC: &mut i32,
    NCOMR: &mut i32,
    NCOMC: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CHRBUF = [b' '; NWC as usize];
    let mut TMPIDW = [b' '; IDWLEN as usize];
    let mut TMPIFN = [b' '; IFNLEN as usize];
    let mut IBFF: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Parameters for positions of file record elements:
    //

    //
    // ID word begin and end:
    //

    //
    // Internal file name begin and end:
    //

    //
    // Reserved record count begin and end:
    //

    //
    // Reserved record character count begin and end:
    //

    //
    // Comment area record count begin and end:
    //

    //
    // Comment area character count begin and end:
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

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDASRFR", ctx)?;

    //
    // On the first pass through this routine, get the integer code of
    // the host system's native binary file format.
    //
    if save.FIRST {
        ZZDDHNFC(&mut save.NATBFF, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDASRFR", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // Get a logical unit for this DAS file.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut UNIT, ctx)?;

    //
    // Get the integer code for the file's binary format.
    //
    ZZDDHPPF(UNIT, DAS, &mut IBFF, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDASRFR", ctx)?;
        return Ok(());
    }

    if (IBFF == save.NATBFF) {
        //
        // We're looking at a native file. Just read the file record.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut TMPIDW)?;
                reader.read_str(&mut TMPIFN)?;
                *NRESVR = reader.read_i32()?;
                *NRESVC = reader.read_i32()?;
                *NCOMR = reader.read_i32()?;
                *NCOMC = reader.read_i32()?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Could not DAS read file record. File was #.  IOSTAT was #.",
                ctx,
            );
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DASFILEREADFAILED)", ctx)?;
            CHKOUT(b"ZZDASRFR", ctx)?;
            return Ok(());
        }

        fstr::assign(IDWORD, &TMPIDW);
        fstr::assign(IFNAME, &TMPIFN);
    } else {
        //
        // The file is non-native.
        //
        // We don't check the access mode of the file, because we're
        // not going to reject files that are open for writing.
        //
        // We'll read the file record as a character string and then
        // pick it apart.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut CHRBUF)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(b"Could not read DAS file record. File is #. IOSTAT was #. File\'s BFF integer code is #.", ctx);
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            ERRINT(b"#", IBFF, ctx);
            SIGERR(b"SPICE(DASFILEREADFAILED)", ctx)?;
            CHKOUT(b"ZZDASRFR", ctx)?;
            return Ok(());
        }

        //
        // Set the string output arguments.
        //
        fstr::assign(IDWORD, fstr::substr(&CHRBUF, BEGIDW..=ENDIDW));
        fstr::assign(IFNAME, fstr::substr(&CHRBUF, BEGIFN..=ENDIFN));

        //
        // The integer output arguments require translation.
        //
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, BEGNRR..=ENDNRR),
            1,
            std::slice::from_mut(NRESVR),
            ctx,
        )?;
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, BEGNRC..=ENDNRC),
            1,
            std::slice::from_mut(NRESVC),
            ctx,
        )?;
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, BEGNCR..=ENDNCR),
            1,
            std::slice::from_mut(NCOMR),
            ctx,
        )?;
        ZZXLATEI(
            IBFF,
            fstr::substr(&CHRBUF, BEGNCC..=ENDNCC),
            1,
            std::slice::from_mut(NCOMC),
            ctx,
        )?;
    }

    CHKOUT(b"ZZDASRFR", ctx)?;
    Ok(())
}
