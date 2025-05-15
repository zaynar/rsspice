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
    FIRST: bool,
    SECS0: f64,
    SECSP: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIRST: bool = false;
        let mut SECS0: f64 = 0.0;
        let mut SECSP: f64 = 0.0;

        FIRST = true;

        Self {
            FIRST,
            SECS0,
            SECSP,
        }
    }
}

//$Procedure T_ELAPSD ( Time program execution )
pub fn T_ELAPSD(
    REPORT: bool,
    TITLE: &[u8],
    KIND: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TVEC = StackArray::<f64, 6>::new(1..=6);
    let mut SECS: f64 = 0.0;
    let mut HLINE = [b' '; 50 as usize];
    let mut HWORD = [b' '; 32 as usize];

    //
    // SPICELIB functions.
    //

    //
    // Local variables
    //

    //
    // Save some.
    //

    //
    // Initial values.
    //

    //
    // Get time. Compute seconds from the start of the day.
    //
    support::CPUTIM(TVEC.as_slice_mut(), ctx)?;
    SECS = (((TVEC[4] * 3600.0) + (TVEC[5] * 60.0)) + TVEC[6]);

    //
    // Set initial time.
    //
    if save.FIRST {
        save.SECS0 = SECS;
        save.SECSP = SECS;
        save.FIRST = false;
    }

    //
    // Report total elapsed time or running elapsed time, if requeated.
    //
    if REPORT {
        fstr::assign(&mut HLINE, TITLE);
        spicelib::RJUST(&HLINE.clone(), &mut HLINE);

        if spicelib::EQSTR(KIND, b"TOTAL") {
            spicelib::DPFMT((SECS - save.SECS0), b"xxxxxx.xxx", &mut HWORD, ctx)?;
            spicelib::TOSTDO(
                &fstr::concat(&fstr::concat(&HLINE, b": elapsed "), &HWORD),
                ctx,
            )?;
        } else {
            spicelib::DPFMT((SECS - save.SECSP), b"xxxxxx.xxx", &mut HWORD, ctx)?;
            spicelib::TOSTDO(
                &fstr::concat(&fstr::concat(&HLINE, b": elapsed "), &HWORD),
                ctx,
            )?;
        }
    }

    save.SECSP = SECS;

    Ok(())
}
