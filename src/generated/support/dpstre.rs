//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MOSTDG: i32 = 33;
const FMTPIC: &[u8] = b"(1PE#.#)";

struct SaveVars {
    MAXSAV: i32,
    FMTSTR: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MAXSAV: i32 = 0;
        let mut FMTSTR = vec![b' '; 10 as usize];
        let mut FIRST: bool = false;

        FIRST = true;
        MAXSAV = 14;
        fstr::assign(&mut FMTSTR, b"(1PE20.13)");

        Self {
            MAXSAV,
            FMTSTR,
            FIRST,
        }
    }
}

//$Procedure      DPSTRE ( Double Precision Number to Character )
pub fn DPSTRE(X: f64, SIGDIG: i32, STRING: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IOSTAT: i32 = 0;
    let mut MAXSIG: i32 = 0;
    let mut NUMSTR = [b' '; (MOSTDG + 7) as usize];

    //
    // Local parameters.
    //
    // The maximum number of allowed significant digits is set to 33
    // (=16*2+1). This is an arbitrarily picked value because FORTRAN
    // doesn't seen to have a limit. But we do need a limit to make sure
    // that the formatted WRITE does not overflow the local buffer
    // string.
    //
    //

    //
    // Format template.
    //

    //
    // Local variables
    //

    //
    // Saved variables.
    //

    //
    // Initial values.
    //

    //
    // Reset the input number of significant digits if it is outside of
    // the allowed range (1 to 33).
    //
    MAXSIG = intrinsics::MIN0(&[MOSTDG, intrinsics::MAX0(&[1, SIGDIG])]);

    //
    // If the number of significant digits is less then or equal to 14,
    // outsource conversion to DPSTR.
    //
    if (MAXSIG <= 14) {
        spicelib::DPSTR(X, MAXSIG, STRING, ctx);
    } else {
        //
        // The number of significant digits is greater than 14. Make
        // output format. Do it only for the first call or if the
        // previous call had a different number of significant digits.
        // Otherwise, use the SAVEd format string from the previous
        // call.
        //
        if (save.FIRST || (MAXSIG != save.MAXSAV)) {
            fstr::assign(&mut save.FMTSTR, FMTPIC);
            spicelib::REPMI(
                &save.FMTSTR.to_vec(),
                b"#",
                (MAXSIG + 6),
                &mut save.FMTSTR,
                ctx,
            );
            spicelib::REPMI(
                &save.FMTSTR.to_vec(),
                b"#",
                (MAXSIG - 1),
                &mut save.FMTSTR,
                ctx,
            );

            save.MAXSAV = MAXSIG;
            save.FIRST = false;
        }

        //
        // Use WRITE to create a temporary output string. This string is
        // declared to have enough room for any allowed numbers of
        // significant digits. We should not get any errors.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let internal_file = io::InternalFile::open(&mut NUMSTR);
            let mut writer = io::FormattedWriter::new(internal_file, None, &save.FMTSTR)?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_f64(X)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        //
        // This is fail safe check. Since we made the format string
        // ourselves and declared the output string with enough room we
        // should never hit it. But we do this check anyway, just in
        // case.
        //
        if (IOSTAT != 0) {
            spicelib::CHKIN(b"DPSTRE", ctx)?;
            spicelib::SETMSG(
                b"Bug. FORTRAN WRITE failed; number = #; format = #; IOSTAT = #",
                ctx,
            );
            spicelib::ERRDP(b"#", X, ctx);
            spicelib::ERRCH(b"#", &save.FMTSTR, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(BUGWRITEFAILED)", ctx)?;
            spicelib::CHKOUT(b"DPSTRE", ctx)?;
            return Ok(());
        }

        //
        // NOTE 1: should we also check for '*'s?
        //
        // NOTE 2: should we check for 'E' in the string for cases of
        //         output FORTRAN WRITE for numbers greater than 1D100?
        //         (In this case GFORTRAN leaves E out and prints
        //         pi*1D101 like this -3.14+101.)
        //

        //
        // Assign output string.
        //
        fstr::assign(STRING, &NUMSTR);
    }

    Ok(())
}
