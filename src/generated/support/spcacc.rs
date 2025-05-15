//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const MXCREC: i32 = 1000;
pub const LINLEN: i32 = 255;
const INTEOC: i32 = 4;
const INTEOL: i32 = 0;
const CASTRT: i32 = 2;

//$Procedure  SPCACC  ( SPK and CK add comments from a cell buffer  )
pub fn SPCACC(DAFHDL: i32, BUFFER: CharArray, ctx: &mut Context) -> f2rust_std::Result<()> {
    let BUFFER = DummyCharArray::new(BUFFER, None, LBCELL..);
    let mut LINE = [b' '; LINLEN as usize];
    let mut CRECRD = [b' '; MXCREC as usize];
    let mut EOCMRK = [b' '; 1 as usize];
    let mut EOLMRK = [b' '; 1 as usize];
    let mut IFNAME = [b' '; LINLEN as usize];
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut CURPOS: i32 = 0;
    let mut EOCPOS: i32 = 0;
    let mut NCRECS: i32 = 0;
    let mut NNRECS: i32 = 0;
    let mut NRRECS: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NLINES: i32 = 0;
    let mut SPACE: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut FREE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //
    //
    // This is needed for the call to DAFRFR to get some of the
    // information needed. It is not used anywhere else.
    //

    //
    // These are needed to call DAFRFR to get some of the information
    // needed. Only FIRST will be used, and this is to determine the
    // number of reserved records which exist.
    //
    //
    // Initial values
    //
    fstr::assign(&mut EOCMRK, &intrinsics::CHAR(INTEOC));
    fstr::assign(&mut EOLMRK, &intrinsics::CHAR(INTEOL));

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"SPCACC", ctx)?;
    }
    //
    // Give some of the variables a value so that they have one.
    //
    NCRECS = 0;
    NNRECS = 0;
    NRRECS = 0;
    //
    // First, extract the number of lines in the buffer
    //
    NLINES = spicelib::CARDC(BUFFER.as_arg(), ctx)?;
    //
    // Check for a non-positive number of lines.
    //
    if (NLINES <= 0) {
        spicelib::SETMSG(b"An invalid buffer length was found: #", ctx);
        spicelib::ERRINT(b"#", NLINES, ctx);
        spicelib::SIGERR(b"SPICE(NONPOSBUFLENGTH)", ctx)?;
        spicelib::CHKOUT(b"SPCACC", ctx)?;
        return Ok(());
    }
    //
    // Count the number of characters in the buffer, ignoring leading
    // and trailing blanks on non-blank lines. Blank lines will not count
    // here, their contribution to the size of the comment area will be
    // incorporated later. This is for determining the number of
    // character records to add to the file attached to handle DAFHDL.
    //
    NCHARS = 0;
    I = 0;
    while (I < NLINES) {
        I = (I + 1);
        fstr::assign(&mut LINE, BUFFER.get(I));
        LENGTH = spicelib::LASTNB(&LINE);
        NCHARS = (NCHARS + LENGTH);
    }
    //
    // Add NLINES + 1 to NCHARS to allow for the end of line markers
    // ( EOLMRK ) and the end of comments marker ( EOCMRK ).
    //
    NCHARS = ((NCHARS + NLINES) + 1);
    //
    // Get the number of reserved records from the file.
    //
    spicelib::DAFRFR(
        DAFHDL,
        &mut ND,
        &mut NI,
        &mut IFNAME,
        &mut FIRST,
        &mut LAST,
        &mut FREE,
        ctx,
    )?;

    //
    // Subtract 1 from FIRST to obtain the number of reserved records.
    //
    // Note that this should be one more than the number of comment
    // records in the comment area for the SPK or CK file comment area
    // to conform to the SPC comment area conventions. That is, the
    // number of reserved records = the number of comment records + 1.
    //
    NRRECS = (FIRST - 1);

    //
    // If the number of reserved records, NRRECS, is greater then 1,
    // determine the number of comment records in the comment area.
    // The comments begin on record CASTRT and should continue to record
    // NRRECS - 1. The comments are terminated by and end of comment
    // marker EOCMRK = CHAR(4).
    //
    EOCPOS = 0;
    I = 0;
    while ((I < (NRRECS - 1)) && (EOCPOS == 0)) {
        RECNO = (CASTRT + I);
        spicelib::DAFRCR(DAFHDL, RECNO, &mut CRECRD, ctx)?;
        EOCPOS = spicelib::CPOS(&CRECRD, &EOCMRK, 1);
        I = (I + 1);
    }

    if ((EOCPOS == 0) && (NRRECS > 1)) {
        spicelib::SETMSG(
            b"End-of-transmission character missing in comment area of binary file.",
            ctx,
        );
        spicelib::SIGERR(b"SPICE(MISSINGEOT)", ctx)?;
        spicelib::CHKOUT(b"SPCACC", ctx)?;
        return Ok(());
    }

    NCRECS = I;

    //
    // Check to see if the number of comment records is one less than
    // the number of reserved records. If not, signal an error.
    //
    //  IF ( NCRECS .NE. NRRECS - 1 ) THEN
    //     CALL SETMSG ( 'The number of comment records and the'//
    // .                 ' number of reserved records do not agree.'//
    // .                 ' The comment area could be bad.' )
    //     CALL SIGERR ( 'SPICE(BADCOMMENTAREA)' )
    //     CALL CHKOUT ( 'SPCACC' )
    //     RETURN
    //  END IF
    //
    // Determine the amount of free space in the comment area. This
    // will be the space remaining on the last comment record, i.e.,
    // the maximum length of a DAF character record - the position
    // of the end of comments marker - 1.
    //
    if (NCRECS > 0) {
        SPACE = (MXCREC - EOCPOS);
    } else {
        SPACE = 0;
    }
    //
    // Determine the number of extra reserved records which are
    // necessary to store the comments in the buffer.
    //
    if (NCHARS > SPACE) {
        NNRECS = (1 + ((NCHARS - SPACE) / MXCREC));
    } else {
        NNRECS = 0;
    }
    //
    // Now call the DAF routine to add reserved records to the file,
    // if we need to.
    //
    if (NNRECS > 0) {
        spicelib::DAFARR(DAFHDL, NNRECS, ctx)?;
    }
    //
    // At this point, we know that we have enough space to write the
    // comments in the buffer to the comment area. Either there was
    // enough space already there, or we figured out how many new
    // character records were needed, and we added them to the file.
    // So, now we begin 'packing' the comments into the character record.
    //
    // We begin by reading the last comment record if there is one,
    // otherwise we just initialize the appropriate variables.
    //
    if (NCRECS == 0) {
        RECNO = CASTRT;
        CURPOS = 0;
        fstr::assign(&mut CRECRD, b" ");
    } else {
        RECNO = ((CASTRT + NCRECS) - 1);
        spicelib::DAFRCR(DAFHDL, RECNO, &mut CRECRD, ctx)?;
        //
        // Find the end of comment marker again. This is really not
        // necessary, but it is here to localize all the info needed.
        //
        EOCPOS = spicelib::CPOS(&CRECRD, &EOCMRK, 1);
        //
        // Set the current record position
        //
        CURPOS = EOCPOS;
        //
        // Adjust the current record position to overwrite the
        // end of comment marker with the first character of the
        // new comments.
        //
        CURPOS = (CURPOS - 1);
    }

    I = 0;
    while (I < NLINES) {
        I = (I + 1);
        fstr::assign(&mut LINE, BUFFER.get(I));
        LENGTH = spicelib::LASTNB(&LINE);
        J = 0;
        while (J < LENGTH) {
            if (CURPOS < MXCREC) {
                J = (J + 1);
                CURPOS = (CURPOS + 1);
                fstr::assign(
                    fstr::substr_mut(&mut CRECRD, CURPOS..=CURPOS),
                    fstr::substr(&LINE, J..=J),
                );
            } else {
                spicelib::DAFWCR(DAFHDL, RECNO, &CRECRD, ctx)?;
                RECNO = (RECNO + 1);
                CURPOS = 0;
                fstr::assign(&mut CRECRD, b" ");
            }
        }
        //
        // Check to see if we happened to get exactly MXCREC characters
        // when we stopped moving characters from LINE. If we did, then
        // we need to write out the current record and appropriately
        // adjust the necessary variables.
        //
        if (CURPOS == MXCREC) {
            spicelib::DAFWCR(DAFHDL, RECNO, &CRECRD, ctx)?;
            RECNO = (RECNO + 1);
            CURPOS = 0;
            fstr::assign(&mut CRECRD, b" ");
        }
        CURPOS = (CURPOS + 1);
        fstr::assign(fstr::substr_mut(&mut CRECRD, CURPOS..=CURPOS), &EOLMRK);
    }
    //
    // We have now finished processing all of the lines, so we
    // need to append the end of comment marker to the current
    // record and write it to the file.
    //
    if (CURPOS == MXCREC) {
        spicelib::DAFWCR(DAFHDL, RECNO, &CRECRD, ctx)?;
        RECNO = (RECNO + 1);
        CURPOS = 0;
        fstr::assign(&mut CRECRD, b" ");
    }

    CURPOS = (CURPOS + 1);
    fstr::assign(fstr::substr_mut(&mut CRECRD, CURPOS..=CURPOS), &EOCMRK);
    spicelib::DAFWCR(DAFHDL, RECNO, &CRECRD, ctx)?;

    spicelib::CHKOUT(b"SPCACC", ctx)?;
    Ok(())
}
