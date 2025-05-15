//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EOLTSZ: i32 = 5;

//$Procedure ZZASCII ( determine/verify EOL terminators in a text file )
pub fn ZZASCII(
    FILE: &[u8],
    LINE: &mut [u8],
    CHECK: bool,
    TERMIN: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NATIVE = [b' '; EOLTSZ as usize];
    let mut DOSCNT: i32 = 0;
    let mut I: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut MACCNT: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut RECLEN: i32 = 0;
    let mut UNXCNT: i32 = 0;

    //
    // SPICELIB functions.
    //

    //
    // Local parameters.
    //

    //
    // Local variables.
    //

    //
    // Discovery check-in. Can't determine the terminator in RETURN
    // mode.
    //
    if RETURN(ctx) {
        fstr::assign(TERMIN, b"?");
        return Ok(());
    }

    //
    // Check-in to the error system.
    //
    CHKIN(b"ZZASCII", ctx)?;

    //
    // Retrieve the native line terminator.
    //
    ZZPLATFM(b"TEXT_FORMAT", &mut NATIVE, ctx);

    //
    // If it is VAX, return immediately with undefined terminator.
    //
    if EQSTR(&NATIVE, b"VAX") {
        fstr::assign(TERMIN, b"?");

        CHKOUT(b"ZZASCII", ctx)?;
        return Ok(());
    }

    //
    // Set the record length that will be used to read data from
    // the file.
    //
    RECLEN = intrinsics::LEN(LINE);

    //
    // Check the length of the work string is sufficient to perform the
    // operations. Less than 3 is a no-op.
    //
    if (intrinsics::LEN(LINE) < 3) {
        fstr::assign(TERMIN, b"?");

        SETMSG(
            b"Work string lacks sufficient length to perform operation.",
            ctx,
        );
        SIGERR(b"SPICE(STRINGTOOSHORT)", ctx)?;
        CHKOUT(b"ZZASCII", ctx)?;
        return Ok(());
    }

    //
    // Find a free logical unit for file access.
    //
    GETLUN(&mut NUMBER, ctx)?;

    //
    // Open the file for DIRECT access.
    //
    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(NUMBER),
            file: Some(fstr::substr(FILE, 1..=RTRIM(FILE))),
            access: Some(b"DIRECT"),
            status: Some(b"OLD"),
            recl: Some(RECLEN),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    if (IOSTAT != 0) {
        //
        // The open failed, can't determine the terminator if the routine
        // can't open the file.
        //
        fstr::assign(TERMIN, b"?");

        //
        // Execute a close, J.I.C.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(NUMBER),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        SETMSG(b"File open failed for file \'$1\'. IOSTAT  value $2.", ctx);
        ERRCH(b"$1", FILE, ctx);
        ERRINT(b"$2", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEOPENFAIL)", ctx)?;
        CHKOUT(b"ZZASCII", ctx)?;
        return Ok(());
    }

    //
    // Read a line into the LINE variable assigned by the user.
    //
    fstr::assign(LINE, b" ");

    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(NUMBER)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(LINE)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        //
        // If something went wrong during this read, a part or the whole
        // returned line may contain garbage. Instead of examining it and
        // making wrong determination based on it, set terminator to
        // undefined and return.
        //
        fstr::assign(TERMIN, b"?");

        //
        // Execute a close, J.I.C.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(NUMBER),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        CHKOUT(b"ZZASCII", ctx)?;
        return Ok(());
    }

    //
    // We have a line of text data. Use ICHAR to scan for carriage
    // returns and line feeds and count how may of various recognized
    // line termination sequences are in this line.
    //
    DOSCNT = 0;
    UNXCNT = 0;
    MACCNT = 0;

    I = 1;

    while (I < intrinsics::LEN(LINE)) {
        //
        // Check for ICHAR values of 10 (LF) and 13 (CR).
        //
        if (intrinsics::ICHAR(fstr::substr(LINE, I..=I)) == 10) {
            //
            // Found a UNIX line terminator LF.
            //
            UNXCNT = (UNXCNT + 1);
        } else if (intrinsics::ICHAR(fstr::substr(LINE, I..=I)) == 13) {
            //
            // Found CR, increment character counter and check
            // the next character.
            //
            I = (I + 1);

            if (intrinsics::ICHAR(fstr::substr(LINE, I..=I)) == 10) {
                //
                // Found a DOS line terminator CR+LF.
                //
                DOSCNT = (DOSCNT + 1);
            } else {
                //
                // Found a Classic Mac line terminator CR.
                //
                MACCNT = (MACCNT + 1);
            }
        }

        I = (I + 1);
    }

    //
    // Examine the counters.
    //
    if (((DOSCNT > 0) && (UNXCNT == 0)) && (MACCNT == 0)) {
        //
        // Only DOS terminator counter is non-zero. ID the file as DOS.
        //
        fstr::assign(TERMIN, b"CR-LF");
    } else if (((DOSCNT == 0) && (UNXCNT > 0)) && (MACCNT == 0)) {
        //
        // Only Unix terminator counter is non-zero. ID the file as UNIX.
        //
        fstr::assign(TERMIN, b"LF");
    } else if (((DOSCNT == 0) && (UNXCNT == 0)) && (MACCNT > 0)) {
        //
        // Only Mac terminator counter is non-zero. ID the file as Mac
        // Classic.
        //
        fstr::assign(TERMIN, b"CR");
    } else {
        //
        // We can get here in two cases. First if the line did not
        // contain any CRs or LFs. Second if the line contained more than
        // one kind of terminators. In either case the format of the file
        // is unclear.
        //
        fstr::assign(TERMIN, b"?");
    }

    //
    // Close the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(NUMBER),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // If we were told check the terminator against the native one, do
    // it.
    //
    if CHECK {
        //
        // If the terminator was identified and does not match the native
        // one, error out.
        //
        if (!EQSTR(TERMIN, &NATIVE) && !EQSTR(TERMIN, b"?")) {
            SETMSG(b"Text file \'$1\' contains lines terminated with \'$2\' while the expected terminator for this platform is \'$3\'. SPICE cannot process the file in the current form. This problem likely occurred because the file was copied in binary mode between operating systems where the operating systems use different text line terminators. Try converting the file to native text form using a utility such as dos2unix or unix2dos.", ctx);

            ERRCH(b"$1", FILE, ctx);
            ERRCH(b"$2", TERMIN, ctx);
            ERRCH(b"$3", &NATIVE, ctx);
            SIGERR(b"SPICE(INCOMPATIBLEEOL)", ctx)?;
            CHKOUT(b"ZZASCII", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"ZZASCII", ctx)?;

    Ok(())
}
