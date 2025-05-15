//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure   ZZGFDSPS  ( GF, display string  )
pub fn ZZGFDSPS(
    NLEAD: i32,
    STRING: &[u8],
    FMT: &[u8],
    NTRAIL: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TMPSTR = [b' '; LNSIZE as usize];
    let mut IOSTAT: i32 = 0;
    let mut L: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Use discovery check-in.
    //

    //
    // Get a local copy of the input string.
    //
    fstr::assign(&mut TMPSTR, STRING);
    L = RTRIM(&TMPSTR);
    //
    // Write leading blank lines, if any. We use cursor control
    // here so as to avoid overwriting the last status line.
    //
    for I in 1..=NLEAD {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::FormattedWriter::new(ctx.default_write_unit()?, None, b"(A)")?;
            writer.start()?;
            writer.write_str(&fstr::concat(&intrinsics::CHAR(27), b"[B"))?;
            writer.finish()?;
        }
    }

    //
    // Write the status line, then move the cursor up one line.
    // This places the cursor at the start of the line that
    // was just written.
    //
    SUFFIX(&fstr::concat(&intrinsics::CHAR(27), b"[A"), 0, &mut TMPSTR);

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.default_write_unit()?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(fstr::substr(&TMPSTR, 1..=RTRIM(&TMPSTR)))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        CHKIN(b"ZZGFDSPS", ctx)?;
        SETMSG(b"Error writing to standard output: IOSTAT = #.", ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(WRITEFAILED)", ctx)?;
        CHKOUT(b"ZZGFDSPS", ctx)?;
        return Ok(());
    }

    //
    // Write trailing blank lines, if any. We use cursor control
    // here so as to avoid overwriting the last status line.
    //
    for I in 1..=NTRAIL {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::FormattedWriter::new(ctx.default_write_unit()?, None, b"(A)")?;
            writer.start()?;
            writer.write_str(&fstr::concat(&intrinsics::CHAR(27), b"[B"))?;
            writer.finish()?;
        }
    }

    Ok(())
}
