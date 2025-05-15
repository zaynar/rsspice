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
const SIZSTR: i32 = 16;
const SIZEXP: i32 = (3 * SIZSTR);
const SIZEND: i32 = 6;
const SIZFTP: i32 = (SIZSTR + (2 * SIZEND));
const SIZDLM: i32 = 1;
const NUMTST: i32 = 6;
const RECLEN: i32 = 1024;
const PREFTP: i32 = 603;
const FMTLEN: i32 = 8;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const INTLEN: i32 = 4;

struct SaveVars {
    STRBFF: ActualCharArray,
    FTPSTR: Vec<u8>,
    NATBFF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
        let mut FTPSTR = vec![b' '; SIZFTP as usize];
        let mut NATBFF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        NATBFF = 0;

        Self {
            STRBFF,
            FTPSTR,
            NATBFF,
            FIRST,
        }
    }
}

//$Procedure T_DAFWFR ( Write a DAF file record to a test DAF )
pub fn T_DAFWFR(
    UNIT: i32,
    OUTBFF: i32,
    IDWORD: &[u8],
    ND: i32,
    NI: i32,
    IFNAME: &[u8],
    FWARD: i32,
    BWARD: i32,
    FREE: i32,
    ADDFTP: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut LOCIDW = [b' '; IDWLEN as usize];
    let mut LOCIFN = [b' '; IFNLEN as usize];
    let mut LOCFMT = [b' '; FMTLEN as usize];
    let mut HOLDER = [b' '; INTLEN as usize];
    let mut PRESPC = [b' '; PREFTP as usize];
    let mut RECORD = [b' '; RECLEN as usize];
    let mut DELIM = [b' '; SIZDLM as usize];
    let mut TMPSTR = [b' '; STRSIZ as usize];
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
        spicelib::CHKIN(b"T_DAFWFR", ctx)?;
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
            spicelib::CHKOUT(b"T_DAFWFR", ctx)?;
            return Ok(());
        }

        //
        // Fetch the FTP string.
        //
        spicelib::ZZFTPSTR(&mut TSTSTR, &mut LFTBKT, &mut RGTBKT, &mut DELIM, ctx);

        fstr::assign(
            &mut save.FTPSTR,
            &fstr::concat(
                &fstr::concat(
                    fstr::substr(&LFTBKT, 1..=spicelib::RTRIM(&LFTBKT)),
                    fstr::substr(&TSTSTR, 1..=spicelib::RTRIM(&TSTSTR)),
                ),
                fstr::substr(&RGTBKT, 1..=spicelib::RTRIM(&RGTBKT)),
            ),
        );

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
        spicelib::CHKOUT(b"T_DAFWFR", ctx)?;
        return Ok(());
    }

    //
    // Prepare the local string buffers to hold the possible
    // string arguments.
    //
    fstr::assign(&mut LOCIDW, IDWORD);
    fstr::assign(&mut LOCIFN, IFNAME);
    fstr::assign(&mut LOCFMT, save.STRBFF.get(OUTBFF));
    fstr::assign(&mut PRESPC, b" ");

    //
    // First, determine if we are to write to the native file format.
    //
    if (OUTBFF == save.NATBFF) {
        fstr::assign(&mut RECORD, b" ");

        if ADDFTP {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(1))?;
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
                    writer.write_str(&PRESPC)?;
                    writer.write_str(&save.FTPSTR)?;
                    writer.write_str(fstr::substr(&RECORD, 1..=297))?;
                    writer.finish()?;
                    Ok(())
                })?;
            }
        } else {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(1))?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&LOCIDW)?;
                    writer.write_i32(ND)?;
                    writer.write_i32(NI)?;
                    writer.write_str(&LOCIFN)?;
                    writer.write_i32(FWARD)?;
                    writer.write_i32(BWARD)?;
                    writer.write_i32(FREE)?;
                    writer.write_str(fstr::substr(&RECORD, 1..=936))?;
                    writer.finish()?;
                    Ok(())
                })?;
            }
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Unable to write to #. IOSTAT was #", ctx);
            spicelib::ERRFNM(b"#", UNIT, ctx)?;
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            spicelib::CHKOUT(b"T_DAFWFR", ctx)?;
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
        fstr::assign(fstr::substr_mut(&mut RECORD, 1..=8), &LOCIDW);

        //
        // Convert and place ND, NI.
        //
        T_XLTFWI(&[ND], 1, OUTBFF, &mut HOLDER, ctx)?;
        fstr::assign(fstr::substr_mut(&mut RECORD, 9..=12), &HOLDER);

        T_XLTFWI(&[NI], 1, OUTBFF, &mut HOLDER, ctx)?;
        fstr::assign(fstr::substr_mut(&mut RECORD, 13..=16), &HOLDER);

        fstr::assign(fstr::substr_mut(&mut RECORD, 17..=76), &LOCIFN);

        //
        // Convert and place FWARD, BWARD, and FREE.
        //
        T_XLTFWI(&[FWARD], 1, OUTBFF, &mut HOLDER, ctx)?;
        fstr::assign(fstr::substr_mut(&mut RECORD, 77..=80), &HOLDER);

        T_XLTFWI(&[BWARD], 1, OUTBFF, &mut HOLDER, ctx)?;
        fstr::assign(fstr::substr_mut(&mut RECORD, 81..=84), &HOLDER);

        T_XLTFWI(&[FREE], 1, OUTBFF, &mut HOLDER, ctx)?;
        fstr::assign(fstr::substr_mut(&mut RECORD, 85..=88), &HOLDER);

        //
        // Add the FTP string if appropriate.
        //
        if ADDFTP {
            fstr::assign(fstr::substr_mut(&mut RECORD, 89..=96), &LOCFMT);
            fstr::assign(fstr::substr_mut(&mut RECORD, 700..), &save.FTPSTR);
        }

        //
        // Dump the record to the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(1))?;
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
            spicelib::CHKOUT(b"T_DAFWFR", ctx)?;
            return Ok(());
        }
    }

    spicelib::CHKOUT(b"T_DAFWFR", ctx)?;
    Ok(())
}
