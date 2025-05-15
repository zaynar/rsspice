//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure KILFIL ( Kill a file  )
pub fn KILFIL(FILNAM: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;
    let mut LUNIT: i32 = 0;
    let mut OPENED: bool = false;

    //
    // Spicelib Routines
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }
    spicelib::CHKIN(b"KILFIL", ctx)?;

    //
    // Check to see if the filename we got is blank. If it is, signal an
    // error and return.
    //
    if fstr::eq(FILNAM, b" ") {
        spicelib::SETMSG(b"The file name is blank.", ctx);
        spicelib::SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        spicelib::CHKOUT(b"KILFIL", ctx)?;
        return Ok(());
    }

    //
    // If the file doesn't exist, there's nothing to do.
    //
    if !spicelib::EXISTS(FILNAM, ctx)? {
        spicelib::CHKOUT(b"KILFIL", ctx)?;
        return Ok(());
    }

    //
    // We inquire before we try opening anything to see if the file
    // exists or is currently open.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(FILNAM),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Not too likely, but if the INQUIRE statement fails signal an error
    // and return.
    //
    if (IOSTAT != 0) {
        spicelib::SETMSG(b"INQUIRE statement failed for file \'#\'. IOSTAT = #.", ctx);
        spicelib::ERRCH(b"#", FILNAM, ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        spicelib::CHKOUT(b"KILFIL", ctx)?;
        return Ok(());
    }

    //
    // The file is already open, we don't have to bother opening it
    // We can just delete it.
    //
    if OPENED {
        {
            use f2rust_std::io;

            let specs = io::InquireSpecs {
                file: Some(FILNAM),
                number: Some(&mut LUNIT),
                ..Default::default()
            };
            ctx.inquire(specs)?;
        }
    } else {
        //
        // Get an available logical unit and attempt to open the file.
        //
        spicelib::GETLUN(&mut LUNIT, ctx)?;

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(LUNIT),
                file: Some(FILNAM),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        //
        // If we had trouble opening the file, signal an appropriate error
        // and return.
        //
        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Attempt to open the file \'#\' failed.", ctx);
            spicelib::ERRCH(b"#", FILNAM, ctx);
            spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
            spicelib::CHKOUT(b"KILFIL", ctx)?;
            return Ok(());
        }
    }

    //
    // We opened the file successfully, so let's try to close it with
    // STATUS = 'DELETE'. If this fails, attempt to just close the file,
    // signal an error and return.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(LUNIT),
            status: Some(b"DELETE"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.close(specs))?;
    }

    if (IOSTAT != 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUNIT),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        spicelib::SETMSG(b"Attempt to delete the file \'#\' failed.", ctx);
        spicelib::ERRCH(b"#", FILNAM, ctx);
        spicelib::SIGERR(b"SPICE(FILEDELETEFAILED)", ctx)?;
        spicelib::CHKOUT(b"KILFIL", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"KILFIL", ctx)?;
    Ok(())
}
