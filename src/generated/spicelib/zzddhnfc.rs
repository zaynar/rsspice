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
    SAVBFF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
        let mut SAVBFF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            STRBFF,
            SAVBFF,
            FIRST,
        }
    }
}

//$Procedure ZZDDHNFC ( DDH, return native BFF format code )
pub fn ZZDDHNFC(NATBFF: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPSTR = [b' '; STRSIZ as usize];

    //
    // SPICELIB functions
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
    // This routine checks in on the first pass only.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if save.FIRST {
        CHKIN(b"ZZDDHNFC", ctx)?;

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

        save.SAVBFF = ISRCHC(&TMPSTR, NUMBFF, save.STRBFF.as_arg());

        if (save.SAVBFF == 0) {
            SETMSG(b"The binary file format, \'#\', is not supported by this version of the toolkit. This is a serious problem, contact NAIF.", ctx);
            ERRCH(b"#", &TMPSTR, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZDDHNFC", ctx)?;
            return Ok(());
        }

        //
        // Do not perform initialization tasks again.
        //
        save.FIRST = false;

        CHKOUT(b"ZZDDHNFC", ctx)?;
    }

    *NATBFF = save.SAVBFF;

    Ok(())
}
