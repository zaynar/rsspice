//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SIZSTR: i32 = 16;
const SIZEXP: i32 = (3 * SIZSTR);
const SIZEND: i32 = 6;
const SIZFTP: i32 = (SIZSTR + (2 * SIZEND));
const SIZDLM: i32 = 1;
const NUMTST: i32 = 6;
const PREFTP: i32 = 603;
const PSTFTP: i32 = 297;
const FMTLEN: i32 = 8;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;

struct SaveVars {
    PRENUL: Vec<u8>,
    PSTNUL: Vec<u8>,
    FTPSTR: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRENUL = vec![b' '; PREFTP as usize];
        let mut PSTNUL = vec![b' '; PSTFTP as usize];
        let mut FTPSTR = vec![b' '; SIZFTP as usize];
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            PRENUL,
            PSTNUL,
            FTPSTR,
            FIRST,
        }
    }
}

//$Procedure ZZDAFNFR ( Private --- DAF write New File Record )
pub fn ZZDAFNFR(
    LUN: i32,
    IDWORD: &[u8],
    ND: i32,
    NI: i32,
    IFNAME: &[u8],
    FWARD: i32,
    BWARD: i32,
    FREE: i32,
    FORMAT: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut NULLCH = [b' '; 1 as usize];
    let mut LOCFMT = [b' '; FMTLEN as usize];
    let mut LOCIDW = [b' '; IDWLEN as usize];
    let mut LOCIFN = [b' '; IFNLEN as usize];
    let mut DELIM = [b' '; SIZDLM as usize];
    let mut LFTBKT = [b' '; SIZEND as usize];
    let mut RGTBKT = [b' '; SIZEND as usize];
    let mut TSTSTR = [b' '; SIZSTR as usize];
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    // Amount of space measured in characters necessary to
    // null pad between the last character of FORMAT and the
    // first character of FTPSTR to keep FTPSTR at character
    // 700 in a 1024 byte record.
    //

    //
    // Amount of space measured in characters necessary to
    // null pad from the last character of FTPSTR to the
    // end of the file record. Note: This value assumes the
    // length of the file record is 1024 bytes.  The DAF
    // specification only requires the presence of 1000
    // characters, so this may require modification for
    // non-standard platforms.
    //

    //
    // Lengths of internal file name, ID word, and format word.
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
        CHKIN(b"ZZDAFNFR", ctx)?;
    }

    //
    // On the first pass, format the PRENUL and PSTNUL strings,
    // and build FTPSTR from its components.
    //
    if save.FIRST {
        //
        // Store NULL into NULLCH.
        //
        fstr::assign(&mut NULLCH, &intrinsics::CHAR(0));

        //
        // Set all of the characters of PRENUL to nulls.
        //
        for I in 1..=PREFTP {
            fstr::assign(fstr::substr_mut(&mut save.PRENUL, I..=I), &NULLCH);
        }

        //
        // Set all of the characters of PSTNUL to nulls.
        //
        for I in 1..=PSTFTP {
            fstr::assign(fstr::substr_mut(&mut save.PSTNUL, I..=I), &NULLCH);
        }

        //
        //        Build FTPSTR from its components that come back from
        //        ZZFTPSTR.  This private SPICE routine returns the
        //        following components:
        //7
        //           TSTSTR - The test component of the FTP string
        //           LFTBKT - The left bracketing, printable, component of
        //                    the FTP string.
        //           RGTBKT - The right bracketing, printable, component of
        //                    the FTP string.
        //           DELIM  - The printable delimiter that separates the
        //                    individual test character blocks in TSTSTR.
        //
        //        which are assembled into the FTP string as it appears in
        //        the DAF file record.
        //
        ZZFTPSTR(&mut TSTSTR, &mut LFTBKT, &mut RGTBKT, &mut DELIM, ctx);

        fstr::assign(
            &mut save.FTPSTR,
            &fstr::concat(
                &fstr::concat(
                    fstr::substr(&LFTBKT, 1..=RTRIM(&LFTBKT)),
                    fstr::substr(&TSTSTR, 1..=RTRIM(&TSTSTR)),
                ),
                fstr::substr(&RGTBKT, 1..=RTRIM(&RGTBKT)),
            ),
        );

        //
        // Stop this block from executing except on the first pass.
        //
        save.FIRST = false;
    }

    //
    // Make local copies of each of the string arguments.  This way we
    // maintain the proper sizes for each of the string objects, in
    // the event larger or smaller strings are passed in.
    //
    fstr::assign(&mut LOCIDW, IDWORD);
    fstr::assign(&mut LOCIFN, IFNAME);
    fstr::assign(&mut LOCFMT, FORMAT);

    //
    // Write the file record components out to the first record of the
    // file.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(LUN)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&LOCIDW)?;
            writer.write_i32(ND)?;
            writer.write_i32(NI)?;
            writer.write_str(&LOCIFN)?;
            writer.write_i32(FWARD)?;
            writer.write_i32(BWARD)?;
            writer.write_i32(FREE)?;
            writer.write_str(&LOCFMT)?;
            writer.write_str(&save.PRENUL)?;
            writer.write_str(&save.FTPSTR)?;
            writer.write_str(&save.PSTNUL)?;
            writer.finish()?;
            Ok(())
        })?;
    }
    //
    // Check IOSTAT for errors.
    //
    if (IOSTAT != 0) {
        //
        // Since we are unable to write to the file record, make
        // certain the output file is destroyed.
        //
        SETMSG(b"Attempt to write file \'#\' failed. Value of IOSTAT was #. The file has been deleted.", ctx);
        ERRFNM(b"#", LUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);

        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUN),
                status: Some(b"DELETE"),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"ZZDAFNFR", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZDAFNFR", ctx)?;

    Ok(())
}
