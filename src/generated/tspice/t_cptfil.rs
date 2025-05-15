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
const DASBIL: i32 = 85;
const DASFTP: i32 = 700;
const DAFFTP: i32 = 700;

//$Procedure T_CPTFIL ( Create Partial Test Files )
pub fn T_CPTFIL(
    NAME: &[u8],
    ARCH: i32,
    FDREC: i32,
    BFFID: &[u8],
    CNI: &[u8],
    CFDREC: &[u8],
    CNSUM: &[u8],
    ADDFTP: bool,
    BRKFTP: bool,
    NEWIDW: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RECORD = [b' '; RECLEN as usize];
    let mut DELIM = [b' '; SIZDLM as usize];
    let mut FTPSTR = [b' '; SIZFTP as usize];
    let mut LFTBKT = [b' '; SIZEND as usize];
    let mut RGTBKT = [b' '; SIZEND as usize];
    let mut TSTSTR = [b' '; SIZSTR as usize];
    let mut LUN: i32 = 0;
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
    // Standard SPICE error handling
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_CPTFIL", ctx)?;
    }

    //
    // Fetch the logical unit we're going to use to create the
    // file.
    //
    spicelib::GETLUN(&mut LUN, ctx)?;

    //
    // Open the file we are about to construct.
    //
    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(LUN),
            file: Some(NAME),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"NEW"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    //
    // Check IOSTAT for trouble.
    //
    //
    if (IOSTAT != 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        spicelib::SETMSG(b"Attempt to open file, \'#\', failed. IOSTAT was $.", ctx);
        spicelib::ERRCH(b"#", NAME, ctx);
        spicelib::ERRINT(b"$", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        spicelib::CHKOUT(b"T_CPTFIL", ctx)?;
        return Ok(());
    }

    //
    // Initialize RECORD to be a blank string.
    //
    fstr::assign(&mut RECORD, b" ");

    //
    // Do the absolute minimum to produce the necessary test file.
    // In this regard we will be placing only the patterns necessary
    // to exercise the logic of ZZDDHIFF and not focusing on the creation
    // of legitmate DAF and DAS files.
    //
    if (ARCH == DAF) {
        //
        // Assemble the file record.
        //
        fstr::assign(fstr::substr_mut(&mut RECORD, 1..=8), b"DAF/TEST");
        fstr::assign(fstr::substr_mut(&mut RECORD, 13..=16), CNI);
        fstr::assign(fstr::substr_mut(&mut RECORD, 77..=80), CFDREC);
        fstr::assign(fstr::substr_mut(&mut RECORD, 89..=96), BFFID);

        //
        // Before going any further, check to see if we are to replace
        // the ID word with NEWIDW.
        //
        if fstr::ne(NEWIDW, b" ") {
            fstr::assign(
                fstr::substr_mut(&mut RECORD, 1..=8),
                fstr::substr(NEWIDW, 1..=8),
            );
        }

        if ADDFTP {
            //
            // Get the FTP string.
            //
            spicelib::ZZFTPSTR(&mut TSTSTR, &mut LFTBKT, &mut RGTBKT, &mut DELIM, ctx);

            fstr::assign(
                &mut FTPSTR,
                &fstr::concat(
                    &fstr::concat(
                        fstr::substr(&LFTBKT, 1..=spicelib::RTRIM(&LFTBKT)),
                        fstr::substr(&TSTSTR, 1..=spicelib::RTRIM(&TSTSTR)),
                    ),
                    fstr::substr(&RGTBKT, 1..=spicelib::RTRIM(&RGTBKT)),
                ),
            );

            //
            // Check to see if we are to "break" the FTP string.
            //
            if BRKFTP {
                //
                // Replace the "<13>" in the first cluster with
                // "<10>" to generate the error.
                //
                fstr::assign(
                    fstr::substr_mut(
                        &mut FTPSTR,
                        (spicelib::RTRIM(&LFTBKT) + 2)..=(spicelib::RTRIM(&LFTBKT) + 2),
                    ),
                    &intrinsics::CHAR(10),
                );
            }

            //
            // Add the FTP string at position DASFTP.
            //
            fstr::assign(fstr::substr_mut(&mut RECORD, DAFFTP..), &FTPSTR);
        }

        //
        // Write the record.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(LUN)?, Some(1))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&RECORD)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        //
        // Now create the first descriptor record.
        //
        fstr::assign(fstr::substr_mut(&mut RECORD, 1..=16), b" ");
        fstr::assign(fstr::substr_mut(&mut RECORD, 17..=24), CNSUM);
        fstr::assign(fstr::substr_mut(&mut RECORD, 25..=42), b" ");

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(LUN)?, Some(FDREC))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&RECORD)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        //
        // Check IOSTAT for trouble.
        //
        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Attempt to write file \'#\' failed. Value of IOSTAT was #. The file has been deleted.", ctx);
            spicelib::ERRFNM(b"#", LUN, ctx)?;
            spicelib::ERRINT(b"#", IOSTAT, ctx);

            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUN),
                    status: Some(b"DELETE"),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }

            spicelib::SIGERR(b"SPICE(WRITEFAILURE)", ctx)?;
            spicelib::CHKOUT(b"T_CPTFIL", ctx)?;
            return Ok(());
        }

        //
        // We've created the test file, close the UNIT since we will be
        // re-opening it from the test code driver.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    } else if (ARCH == DAS) {
        //
        // We need only to place the BFF ID string in the appropriate
        // place in the file record along with the DAS indentifier.
        //
        fstr::assign(fstr::substr_mut(&mut RECORD, 1..=8), b"DAS/TEST");
        fstr::assign(fstr::substr_mut(&mut RECORD, DASBIL..=(DASBIL + 7)), BFFID);

        //
        // Before going any further, check to see if we are to replace
        // the ID word with NEWIDW.
        //
        if fstr::ne(NEWIDW, b" ") {
            fstr::assign(
                fstr::substr_mut(&mut RECORD, 1..=8),
                fstr::substr(NEWIDW, 1..=8),
            );
        }

        if ADDFTP {
            //
            // Get the FTP string.
            //
            spicelib::ZZFTPSTR(&mut TSTSTR, &mut LFTBKT, &mut RGTBKT, &mut DELIM, ctx);

            fstr::assign(
                &mut FTPSTR,
                &fstr::concat(
                    &fstr::concat(
                        fstr::substr(&LFTBKT, 1..=spicelib::RTRIM(&LFTBKT)),
                        fstr::substr(&TSTSTR, 1..=spicelib::RTRIM(&TSTSTR)),
                    ),
                    fstr::substr(&RGTBKT, 1..=spicelib::RTRIM(&RGTBKT)),
                ),
            );

            //
            // Check to see if we are to "break" the FTP string.
            //
            if BRKFTP {
                //
                // Replace the "<13>" in the first cluster with
                // "<10>" to generate the error.
                //
                fstr::assign(
                    fstr::substr_mut(
                        &mut FTPSTR,
                        (spicelib::RTRIM(&LFTBKT) + 2)..=(spicelib::RTRIM(&LFTBKT) + 2),
                    ),
                    &intrinsics::CHAR(10),
                );
            }

            //
            // Add the FTP string at position DASFTP.
            //
            fstr::assign(fstr::substr_mut(&mut RECORD, DASFTP..), &FTPSTR);
        }

        //
        // Write the record.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(LUN)?, Some(1))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&RECORD)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        //
        // Check IOSTAT for trouble.
        //
        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Attempt to write file \'#\' failed. Value of IOSTAT was #. The file has been deleted.", ctx);
            spicelib::ERRFNM(b"#", LUN, ctx)?;
            spicelib::ERRINT(b"#", IOSTAT, ctx);

            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUN),
                    status: Some(b"DELETE"),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }

            spicelib::SIGERR(b"SPICE(WRITEFAILURE)", ctx)?;
            spicelib::CHKOUT(b"T_CPTFIL", ctx)?;
            return Ok(());
        }

        //
        // We've created the test file, close the UNIT since we will be
        // re-opening it from the test code driver.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    } else {
        spicelib::SETMSG(b"Unknown architecture code.", ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_CPTFIL", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"T_CPTFIL", ctx)?;
    Ok(())
}
