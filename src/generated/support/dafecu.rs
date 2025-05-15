//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LINLEN: i32 = 1000;
const BUFSIZ: i32 = 22;

//$Procedure      DAFECU( DAF extract comments to a logical unit )
pub fn DAFECU(
    HANDLE: i32,
    COMLUN: i32,
    COMNTS: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COMBUF = ActualCharArray::new(LINLEN, 1..=BUFSIZ);
    let mut IOSTAT: i32 = 0;
    let mut NUMCOM: i32 = 0;
    let mut EOC: bool = false;
    let mut GOTSOM: bool = false;
    let mut OPENED: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local parameters
    //
    // Set the value for the maximum length of a text line.
    //
    //
    // Set the size of the comment buffer.
    //
    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"DAFECU", ctx)?;
    }
    //
    // Verify that the DAF file attached to HANDLE is opened for reading.
    //
    spicelib::DAFSIH(HANDLE, b"READ", ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"DAFECU", ctx)?;
        return Ok(());
    }
    //
    // Logical units must be positive. If it is not, signal an error.
    //
    if (COMLUN <= 0) {
        spicelib::SETMSG(
            b"# is not a valid logical unit. Logical units must be positive.",
            ctx,
        );
        spicelib::ERRINT(b"#", COMLUN, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"DAFECU", ctx)?;
        return Ok(());
    }
    //
    // Verify that there is an open ASCII text file attached to COMLUN.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            unit: Some(COMLUN),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(
            b"The INQUIRE on logical unit # failed. The value of IOSTAT was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", COMLUN, ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        spicelib::CHKOUT(b"DAFECU", ctx)?;
        return Ok(());
    }

    if !OPENED {
        spicelib::SETMSG(
            b"There is no open file attached to logical unit #, so no comments could be written.",
            ctx,
        );
        spicelib::ERRINT(b"#", COMLUN, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"DAFECU", ctx)?;
        return Ok(());
    }
    //
    // Initialize some things before the loop.
    //
    NUMCOM = 0;
    EOC = false;
    GOTSOM = false;

    while !EOC {
        //
        // While we have not reached the end of the comments, get some
        // more.
        //
        spicelib::DAFEC(
            HANDLE,
            BUFSIZ,
            &mut NUMCOM,
            COMBUF.as_arg_mut(),
            &mut EOC,
            ctx,
        )?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"DAFECU", ctx)?;
            return Ok(());
        }

        if (NUMCOM > 0) {
            //
            // If NUMCOM .GT. 0 then we did get some comments, and we need
            // to write them out, but first, set the flag indicating that
            // we got some comments.
            //
            if !GOTSOM {
                GOTSOM = true;
            }

            spicelib::WRITLA(NUMCOM, COMBUF.as_arg(), COMLUN, ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"DAFECU", ctx)?;
                return Ok(());
            }
        }
    }
    //
    // Set the output flag indicating whether or not we got any comments.
    //
    *COMNTS = GOTSOM;

    spicelib::CHKOUT(b"DAFECU", ctx)?;
    Ok(())
}
