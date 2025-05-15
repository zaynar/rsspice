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

//$Procedure ZZDDHINI ( Private --- DDH Initialize Structures )
pub fn ZZDDHINI(
    NATBFF: &mut i32,
    SUPBFF: &mut [i32],
    NUMSUP: &mut i32,
    STRAMH: CharArrayMut,
    STRARC: CharArrayMut,
    STRBFF: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SUPBFF = DummyArrayMut::new(SUPBFF, 1..);
    let mut STRAMH = DummyCharArrayMut::new(STRAMH, None, 1..);
    let mut STRARC = DummyCharArrayMut::new(STRARC, None, 1..);
    let mut STRBFF = DummyCharArrayMut::new(STRBFF, None, 1..);
    let mut LINSTR = [b' '; STRLEN as usize];
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut I: i32 = 0;
    let mut DONE: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables.
    //

    //
    // Standard SPICE error handling with discovery check in/out.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Populate the STR### arrays.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NUMAMH;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZDDHGSD(b"METHOD", I, &mut STRAMH[I], ctx);
            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = NUMARC;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZDDHGSD(b"ARCH", I, &mut STRARC[I], ctx);
            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = NUMBFF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZDDHGSD(b"BFF", I, &mut STRBFF[I], ctx);
            I += m3__;
        }
    }

    //
    // Get the native binary file format.
    //
    ZZPLATFM(b"FILE_FORMAT", &mut TMPSTR, ctx);
    UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);

    *NATBFF = ISRCHC(&TMPSTR, NUMBFF, STRBFF.as_arg());

    if (*NATBFF == 0) {
        CHKIN(b"ZZDDHINI", ctx)?;
        SETMSG(b"The binary file format, \'#\', is not supported by this version of the toolkit. This is a serious problem, contact NAIF.", ctx);
        ERRCH(b"#", &TMPSTR, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZDDHINI", ctx)?;
        return Ok(());
    }

    //
    // Now fetch the list of supported binary file formats.
    //
    ZZPLATFM(b"READS_BFF", &mut LINSTR, ctx);

    //
    // Parse the wordlist that is sitting in LINSTR.
    //
    I = 0;
    DONE = false;

    while !DONE {
        //
        // Increment the counter and pop the next word
        // off.
        //
        I = (I + 1);
        NEXTWD(&LINSTR.clone(), &mut TMPSTR, &mut LINSTR);

        //
        // See if we're done.
        //
        DONE = ((I > NUMBFF) || fstr::eq(&TMPSTR, b" "));

        //
        // If we're not done, then convert this string to the
        // appropriate integer code.
        //
        if !DONE {
            SUPBFF[I] = ISRCHC(&TMPSTR, NUMBFF, STRBFF.as_arg());

            //
            // Check to see if the binary file format listed
            // is properly supported.
            //
            if (SUPBFF[I] == 0) {
                CHKIN(b"ZZDDHINI", ctx)?;
                SETMSG(b"The binary file format, \'#\', is not supported by this version of the toolkit. This is a serious problem, contact NAIF.            ", ctx);
                ERRCH(b"#", &TMPSTR, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZDDHINI", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Now setup NUMSUP.  Given the way the WHILE loop above executes,
    // we need to subtract one from I to get the number of entries added
    // to SUPBFF.  This smacks of kludge... but it works.
    //
    *NUMSUP = (I - 1);

    Ok(())
}
