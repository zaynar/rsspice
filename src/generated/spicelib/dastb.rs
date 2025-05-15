//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BEGCOM: &[u8] = b"BEGIN_COMMENT_BLOCK";
const ENDCOM: &[u8] = b"END_COMMENT_BLOCK";
const BCBLK: &[u8] = b"BEGIN_CHARACTER_BLOCK";
const ECBLK: &[u8] = b"END_CHARACTER_BLOCK";
const BDBLK: &[u8] = b"BEGIN_DP_BLOCK";
const EDBLK: &[u8] = b"END_DP_BLOCK";
const BIBLK: &[u8] = b"BEGIN_INTEGER_BLOCK";
const EIBLK: &[u8] = b"END_INTEGER_BLOCK";
const TCMBLK: &[u8] = b"TOTAL_COMMENT_BLOCKS";
const TCBLKS: &[u8] = b"TOTAL_CHARACTER_BLOCKS";
const TDBLKS: &[u8] = b"TOTAL_DP_BLOCKS";
const TIBLKS: &[u8] = b"TOTAL_INTEGER_BLOCKS";
const CBFLEN: i32 = 4;
const CRLEN: i32 = 1024;
const LINLEN: i32 = 255;
const ERRLEN: i32 = 320;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const BUFSIZ: i32 = 1024;
const BCBPOS: i32 = 1;
const ECBPOS: i32 = CBFLEN;

/// DAS, convert transfer file to binary file
///
/// Convert the contents of a DAS transfer file into an equivalent
/// binary DAS file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  XFRLUN     I   Logical unit of an open DAS transfer file.
///  BINFIL     I   Name of the binary DAS file to be created.
/// ```
///
/// # Detailed Input
///
/// ```text
///  XFRLUN   is the Fortran logical unit number of a previously opened
///           DAS transfer file.
///
///           The file pointer should be positioned ready to read
///           the DAS file ID word.
///
///  BINFIL   is the name of the binary DAS file to be created.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the DAS transfer file cannot be read, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  2)  If the specified file is not a DAS file, as indicated by the
///      file's ID word, the error SPICE(NOTADASFILE) is signaled.
///
///  3)  If an error occurs while attempting to decode data in the DAS
///      transfer file, the error SPICE(BADDASTRANSFERFILE) is
///      signaled.
///
///  4)  If the DAS file cannot be written, an error is signaled by a
///      routine in the call tree of this routine.
///
///  5)  The binary DAS file opened by this routine, BINFIL, is only
///      GUARANTEED to be closed upon successful completion of the
///      text to binary conversion process. In the event of an error,
///      the caller of this routine is required to close the binary
///      DAS file BINFIL.
/// ```
///
/// # Files
///
/// ```text
///  See arguments XFRLUN, BINFIL.
/// ```
///
/// # Particulars
///
/// ```text
///  Any binary DAS file may be transferred between heterogeneous
///  Fortran environments by converting it to an equivalent file
///  containing only ASCII characters called a DAS transfer file.
///  Such a file can be transferred almost universally using any number
///  of established protocols. Once transferred, the DAS transfer file
///  can be converted to a binary file using the representations native
///  to the new host environment.
///
///  This routine provides a mechanism for converting a DAS
///  transfer file created by DASBT, or an equivalent procedure,
///  into an equivalent binary DAS file which may be used with the
///  SPICE system. It is one of a pair of routines for performing
///  conversions between the binary format of a DAS file and the DAS
///  transfer file. The inverse of this routine is the routine DASTB.
///
///  Upon successful completion, the binary DAS file specified by
///  BINFIL will have been created. The binary DAS file that was
///  created will be closed when this routine exits. The DAS transfer
///  file will remain open, as it was on entry, and it will be
///  positioned to read the first line after the encoded DAS file data.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///     XFRLUN   be the Fortran logical unit attached to a DAS transfer
///              file which is to be converted into its binary DAS
///              equivalent.
///
///     BINFIL   be the name of the binary DAS file which will be
///              created.
///
///  Then, the following subroutine call would read the DAS transfer
///  file attached to the Fortran logical unit XFRLUN, convert its data
///  into binary format, and write that data to the binary DAS file
///  which is being created:
///
///     CALL DASTB( XFRLUN, BINFIL )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that it is positioned ready to read the
///      DAS file ID word from the encoded text DAS file.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.3.0, 02-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.2.0, 05-FEB-2015 (NJB)
///
///         Updated to support integration with the handle
///         manager subsystem.
///
///         Corrected typo in a long error message and several
///         typos in comments. Re-ordered header sections.
///
/// -    SPICELIB Version 3.1.0, 06-DEC-1995 (KRG)
///
///         Updated the call to DASONW; a new argument was added to the
///         call for reserving comment records.
///
/// -    SPICELIB Version 3.0.0, 13-AUG-1994 (KRG)
///
///         Updated the header and in line comments to reflect the change
///         from calling files text files to calling them transfer files.
///
///         Changed the variable name XFRLUN to XFRLUN to make it
///         compatible with the change in terminology.
///
///         Changed the short error message "BADDASTEXTFILE" to the
///         message "BADDASTRANSFERFILE".
///
/// -    SPICELIB Version 2.0.0, 27-OCT-1993 (KRG)
///
///        Updated the routine to use the new format ID words which
///        contain type as well as architecture information.
///
///        Fixed a typo in the description of the DAS encoded text file:
///        ncomc appeared where nresvc should have been.
///
/// -    SPICELIB Version 1.0.0, 02-NOV-1992 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.1.0, 06-DEC-1995 (KRG)
///
///         Updated the call to DASONW; a new argument was added to the
///         call for reserving comment records. The value used here is
///         zero (0).
///
/// -    SPICELIB Version 3.0.0, 13-AUG-1994 (KRG)
///
///         Updated the header and in line comments to reflect the change
///         from calling files text files to calling them transfer files.
///
///         Changed the variable name XFRLUN to XFRLUN to make it
///         compatible with the change in terminology.
///
///         Changed the short error message "BADDASTEXTFILE" to the
///         message "BADDASTRANSFERFILE".
///
/// -    SPICELIB Version 2.0.0, 27-OCT-1993 (KRG)
///
///         Updated the routine to use the new format ID words which
///         contain type as well as architecture information.
///
///         Changed the wording of exception '2)' so that it would make
///         sense with the ID word format change that was made.
///
///         Changed the error
///
///            SPICE(DASIDWORDNOTKNOWN)
///
///         to
///
///            SPICE(NOTADASFILE)
///
///         Added variables to support the file architecture and type
///         stored in the ID word. These are used in order to verify that
///         the text file that is to be converted is indeed a DAS file.
///         This test is performed instead of testing whether the ID word
///         is equal to 'NAIF/DAS'.
///
///         Modified the long error message that was set to conform to the
///         ID word change.
///
///         Changed the DASOPN call to DASONW to support the addition of
///         type information to the ID word.
///
///         Fixed a typo in the description of the DAS encoded text file:
///         ncomc appeared where nresvc should have been.
///
/// -    SPICELIB Version 1.0.0, 02-NOV-1992 (KRG)
/// ```
pub fn dastb(ctx: &mut SpiceContext, xfrlun: i32, binfil: &str) -> crate::Result<()> {
    DASTB(xfrlun, binfil.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASTB ( DAS, convert transfer file to binary file )
pub fn DASTB(XFRLUN: i32, BINFIL: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CBUFFR = ActualCharArray::new(CBFLEN, 1..=BUFSIZ);
    let mut CRECRD = [b' '; CRLEN as usize];
    let mut ERRMSG = [b' '; ERRLEN as usize];
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut LINE = [b' '; LINLEN as usize];
    let mut REST = [b' '; LINLEN as usize];
    let mut TARCH = [b' '; IDWLEN as usize];
    let mut TTYPE = [b' '; IDWLEN as usize];
    let mut WORD = [b' '; LINLEN as usize];
    let mut DBUFFR = ActualArray::<f64>::new(1..=BUFSIZ);
    let mut BCOUNT: i32 = 0;
    let mut BINDEX: i32 = 0;
    let mut BLKCNT: i32 = 0;
    let mut DASLUN: i32 = 0;
    let mut DTACNT: i32 = 0;
    let mut ECOUNT: i32 = 0;
    let mut EINDEX: i32 = 0;
    let mut ERRPTR: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut IBUFFR = ActualArray::<i32>::new(1..=BUFSIZ);
    let mut IOSTAT: i32 = 0;
    let mut NCDATA: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NDDATA: i32 = 0;
    let mut NIDATA: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NUMBLK: i32 = 0;
    let mut NUMDTA: i32 = 0;
    let mut NUMLFT: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut TCOUNT: i32 = 0;
    let mut INBLK: bool = false;
    let mut MORE: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local Parameters
    //
    //  CHARACTER*(*)         BEGRES
    //  PARAMETER           ( BEGRES = 'BEGIN_RESERVED_BLOCK'   )
    //
    //  CHARACTER*(*)         ENDRES
    //  PARAMETER           ( ENDRES = 'END_RESERVED_BLOCK'     )
    //
    //  CHARACTER*(*)         TRRBLK
    //  PARAMETER           ( TRRBLK = 'TOTAL_RESERVED_BLOCKS'  )
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DASTB", ctx)?;
    }
    //
    // A DAS transfer file contains in an encoded form all of the data
    // from the original binary DAS file. This includes the reserved
    // record area, the comment area, and the character, double
    // precision, and integer data arrays as well.
    //
    // Currently, the reserved record area has not been implemented, as
    // there is no need for it at this time. If, or when, the reserved
    // record area is implemented, this routine will need to be modified
    // in order to support it. See the code for details.
    //
    // The data in the DAS transfer file are available as sequences of
    // small blocks of data. This is to provide a means for performing
    // some error detection when converting a DAS transfer file into its
    // binary equivalent. Each block of data is enclosed within begin and
    // end block markers which hold the count of data items in a data
    // block. When all of the data blocks for a data area have been
    // written, a total blocks line is read to verify that all of the
    // data has been converted.
    //
    // The data in the DAS transfer file MUST appear in the following
    // order for this routine to work properly.
    //
    //       1) Reserved records (when/if implemented)
    //       2) Comment area
    //       3) Character data array
    //       4) Double precision data array
    //       5) Integer data array
    //
    // If the data count for any of these DAS data areas is zero,
    // conversion proceeds with the next DAS data area in the list.
    //
    // For example, suppose that we have a binary DAS file where there
    // are 0 reserved characters in the reserved record area, 5000
    // comment characters in the comment area, and that the character,
    // double precision, and integer array counts are 0, 2300, and
    // 6900, respectively. Then, the DAS encoded text file will contain
    // no reserved record data blocks, 2 comment data blocks, no
    // character data blocks, 3 double precision data blocks, and 7
    // integer data blocks, in that order.
    //
    // DAS encoded text file description.
    // ----------------------------------
    //
    //
    // A brief description of the DAS encoded file format and its
    // intended use follows. This description is intended to provide a
    // simple ``picture'' of the DAS transfer file format to aid in the
    // understanding of this routine. This description is NOT intended to
    // be a detailed specification of the file format.
    //
    // A DAS transfer file contains all of the data from a binary
    // DAS file in an encoded ASCII format. It also contains some
    // bookkeeping information for maintaining the integrity of the
    // data. The DAS transfer file format allows the full precision of
    // character, integer, and floating point numeric data to be
    // maintained in a portable fashion. The DAS transfer file format is
    // intended to provide a reliable and accurate means for porting data
    // among multiple computer systems and for the archival storage of
    // data.
    //
    // A DAS transfer file is not intended to be used directly to provide
    // data to a program. The equivalent binary DAS file is to be used
    // for this purpose. In no way should any program, other than a DAS
    // binary <-> transfer conversion program, rely on the DAS transfer
    // file format.
    //
    // To correctly understand the DAS transfer file description  the
    // reader should be familiar with the DAS file architecture. Items
    // enclosed in angle brackets, '<' and '>', are used to represent the
    // data which are to be placed at that position in the file. The
    // bookkeeping information which appears is represented exactly as it
    // would appear in a DAS transfer file.
    //
    // Let
    //
    //    <BOF>  denote the beginning of the file
    //    <EOF>  denote the end of the file
    //
    // and
    //
    //    nresvb  denote the number of encoded reserved record data
    //            blocks generated
    //    nresvc  denote the total number of reserved record characters
    //            in the  reserved record area of a DAS file
    //    ncomb   denote the number of encoded comment data blocks
    //            generated
    //    ncomc   denote the total number of comment characters in the
    //            comment area of a DAS file
    //    nchrb   denote the number of encoded character data blocks
    //            generated
    //    nchrs   denote the count of characters in the DAS character
    //            data array
    //    ndpb    denote the number of encoded double precision data
    //            blocks generated
    //    ndps    denote the count of double precision numbers in the DAS
    //            double precision data array
    //    nintb   denote the number of encoded integer data blocks
    //            generated
    //    nints   denote the count of integers in the DAS integer data
    //            array
    //
    // A DAS encoded transfer file has the following format:
    //
    //    <BOF>
    //    < Information line >
    //    < DAS file ID word >
    //    < Internal filename >
    //    < Encoded count of reserved records >
    //    < Encoded count of reserved characters >
    //    < Encoded count of comment records >
    //    < Encoded count of comment characters >
    //    < Blocks of encoded reserved record data, if nresvc > 0 >
    //    TOTAL_RESERVED_BLOCKS nresvb nresvc
    //    < Blocks of encoded comment data, if ncomc > 0 >
    //    TOTAL_COMMENT_BLOCKS ncomb ncomc
    //    < Encoded count of character data >
    //    < Encoded count of double precision data >
    //    < Encoded count of integer data >
    //    < Blocks of encoded character data, if nchrs > 0 >
    //    TOTAL_CHARACTER_BLOCKS nchrb nchrs
    //    < Blocks of encoded double precision data, if ndps > 0 >
    //    TOTAL_DP_BLOCKS ndpb ndps
    //    < Blocks of encoded integer data, if nints > 0 >
    //    TOTAL_INTEGER_BLOCKS nintb nints
    //    <EOF>
    //
    // This routine will check the SPICELIB function FAILED() after
    // each call, or consecutive sequence of calls, to data encoding
    // routines, and if an error was signaled it will simply check out
    // and return to the caller.
    //
    // This routine will check the SPICELIB function FAILED() after
    // each DAS file access call, and if an error was signaled it will
    // simply check out and return to the caller.
    //
    // We begin by reading the DAS file ID word from the DAS transfer
    // file. We should have been positioned ready to read this. If an
    // error occurs, set an appropriate error message and signal the
    // error.
    //
    fstr::assign(&mut IDWORD, b" ");
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::ListDirectedReader::new(ctx.io_unit(XFRLUN)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error reading the file ID word from the DAS transfer file: #. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Check the DAS ID word. When checking the ID word all we care about
    // is that we are attempting to convert a DAS file. So, split the
    // ID word into its architecture and type and check the architecture.
    //
    IDW2AT(&IDWORD, &mut TARCH, &mut TTYPE, ctx)?;

    if fstr::ne(&TARCH, b"DAS") {
        SETMSG(b"File architecture was not \'DAS\' for file #.", ctx);
        ERRFNM(b"#", XFRLUN, ctx)?;
        SIGERR(b"SPICE(NOTADASFILE)", ctx)?;
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Read the internal filename for the DAS file.
    //
    fstr::assign(&mut IFNAME, b" ");
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::ListDirectedReader::new(ctx.io_unit(XFRLUN)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IFNAME)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error reading the internal filename from the DAS transfer file: #. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Open a new binary DAS file and write its file record.
    //
    DASONW(BINFIL, &TTYPE, &IFNAME, 0, &mut HANDLE, ctx)?;

    if FAILED(ctx) {
        //
        // If an error occurred while opening the new DAS file,
        // then check out and return.
        //
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Write the initial file record to the newly opened DAS file. This
    // call will overwrite the ID word set when we opened the file with
    // the ID word from the DAS transfer file. We got to this point, so
    // we know that the ID word was a good one.
    //
    NCOMR = 0;
    NCOMC = 0;
    NRESVR = 0;
    NRESVC = 0;
    DASWFR(HANDLE, &IDWORD, &IFNAME, NRESVR, NRESVC, NCOMR, NCOMC, ctx)?;

    if FAILED(ctx) {
        //
        // If an error occurred while writing the DAS file record,
        // attempt to close the binary file, then check out and return.
        //
        DASCLS(HANDLE, ctx)?;
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Read and decode the number of reserved records and reserved
    // characters.
    //
    RDENCI(XFRLUN, 1, std::slice::from_mut(&mut NRESVR), ctx)?;
    RDENCI(XFRLUN, 1, std::slice::from_mut(&mut NRESVC), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Check to be sure that the number of reserved records and the
    // number of reserved characters are not being used. The DAS
    // reserved record area is not currently implemented, so nobody
    // should be using it.
    //
    if (NRESVC != 0) {
        //
        // Close the file, signal the error, and exit.
        //
        DASCLS(HANDLE, ctx)?;
        SETMSG(b"The number of reserved characters was nonzero (#) in file: #, but the DAS reserved record area has NOT been implemented yet!", ctx);
        ERRINT(b"#", NRESVC, ctx);
        ERRFNM(b"#", XFRLUN, ctx)?;
        SIGERR(b"SPICE(BADDASFILE)", ctx)?;
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }

    if (NRESVR != 0) {
        //
        // Close the file, signal the error, and exit.
        //
        DASCLS(HANDLE, ctx)?;
        SETMSG(b"The number of reserved records was nonzero (#) in file: #, but the DAS reserved record area has NOT been implemented yet!", ctx);
        ERRINT(b"#", NRESVR, ctx);
        ERRFNM(b"#", XFRLUN, ctx)?;
        SIGERR(b"SPICE(BADDASFILE)", ctx)?;
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Read and decode the number of comment records and comment
    // characters.
    //
    RDENCI(XFRLUN, 1, std::slice::from_mut(&mut NCOMR), ctx)?;
    RDENCI(XFRLUN, 1, std::slice::from_mut(&mut NCOMC), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASTB", ctx)?;
        return Ok(());
    }
    //
    // Begin converting the DAS transfer file into an equivalent
    // binary DAS file here.
    //
    // The reserved records, if there are any.
    //
    // **************************************************************
    // When/if the reserved record area is implemented, the code to
    // read it from the DAS transfer file and convert it to binary
    // should go here. It should be possible to simply copy the code
    // for the comment area, making all of the necessary variable
    // name changes, etc., since the reserved record area is going
    // to contain ONLY character data.
    // **************************************************************
    //

    //
    // The comments, if there are any.
    //
    if (NCOMC > 0) {
        //
        // We assume that the condition NCOMC > 0 and NCOMR <= 0
        // cannot occur.
        //
        // The binary DAS file that we are creating is already open,
        // so just add the comments. But first, convert the DAS file
        // handle into its equivalent logical unit.
        //
        ZZDDHHLU(HANDLE, b"DAS", false, &mut DASLUN, ctx)?;

        if FAILED(ctx) {
            //
            // If an error occurred, attempt to close the binary file,
            // then check out and return.
            //
            DASCLS(HANDLE, ctx)?;
            CHKOUT(b"DASTB", ctx)?;
            return Ok(());
        }
        //
        // Allocate the necessary comment records.
        //
        DASACR(HANDLE, NCOMR, ctx)?;

        if FAILED(ctx) {
            //
            // If an error occurred, attempt to close the binary file,
            // then checkout and return.
            //
            DASCLS(HANDLE, ctx)?;
            CHKOUT(b"DASTB", ctx)?;
            return Ok(());
        }
        //
        // Initialize a few things: the block counter, the data
        // counter, and the starting record position. The starting
        // record position is one short of the actual first comment
        // record. We will increment the record number before we
        // write anything.
        //
        BLKCNT = 0;
        DTACNT = 0;
        RECNO = (1 + NRESVR);
        //
        // We currently have more to process.
        //
        MORE = true;
        //
        // We are currently not processing a comment block.
        //
        INBLK = false;

        while MORE {
            fstr::assign(&mut CRECRD, b" ");

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                //
                // If an error occurred while reading from the DAS transfer
                // file close the binary file, set an appropriate error
                // message, then check out and return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error reading from the DAS transfer file #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // At this point, we should be beginning a comment block,
            // ending a comment block, or scanning for the total number
            // of comment blocks. So look for the appropriate keyword.
            //
            NEXTWD(&LINE, &mut WORD, &mut REST);

            if fstr::eq(&WORD, BEGCOM) {
                //
                // Get the comment block index.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the begin block
                    // index, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin comment block error, could not parse block number. Error: # File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of characters in the block.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the beginning
                    // data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin comment block error, could not parse the data count for block: #. Error: # File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we are inside a comment block, so set
                // the in block flag, INBLK, to .TRUE. and increment the
                // block counter.
                //
                INBLK = true;
                BLKCNT = (BLKCNT + 1);
            } else if fstr::eq(&WORD, ENDCOM) {
                //
                // Get the data block index.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut EINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the end comment
                    // block index, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"End comment block error, could not parse block number. Error: # File: #",
                        ctx,
                    );
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of characters in the DAS array.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut ECOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the ending data
                    // count, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"End comment block error, could not parse the data count for  block: #. Error: # File: #", ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending array indices
                // match.
                //
                if (EINDEX != BINDEX) {
                    //
                    // If the begin and end data block indices do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Comment block index mismatch: Beginning index: #; Ending index: #. File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending comment data
                // counts match.
                //
                if (ECOUNT != BCOUNT) {
                    //
                    // If the begin and end data block counts do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Comment block count mismatch: Beginning count: #; Ending count: #. File: #", ctx);
                    ERRINT(b"#", BCOUNT, ctx);
                    ERRINT(b"#", ECOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully ended the
                // processing of a comment block, so set the in block
                // flag INBLK, to .FALSE..
                //
                INBLK = false;
            } else if fstr::eq(&WORD, TCMBLK) {
                //
                // We have the total comment blocks keyword to parse, so
                // get the total number of comment blocks processed.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut NUMBLK, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the total number of
                    // data blocks, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Comment block count error, could not parse the total number of character blocks: #. File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the total count of comment characters.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut TCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the comment
                    // data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"Comment count error, could not parse the total count. Error: # File: #",
                        ctx,
                    );
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Compare the computed block count with the block count
                // from the file.
                //
                if (BLKCNT != NUMBLK) {
                    //
                    // If the computed number of comment blocks and the
                    // number of comment blocks from the text file do
                    // not match, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"The number of comment data blocks processed (#) was not equal to the number of comment data blocks placed in the DAS text file (#). File: #", ctx);
                    ERRINT(b"#", BLKCNT, ctx);
                    ERRINT(b"#", NUMBLK, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the total count and the computed count
                // match.
                //
                if (TCOUNT != DTACNT) {
                    //
                    // If the total count and computed count do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"Comment count mismatch: computed count: #; expected count: #. File: #",
                        ctx,
                    );
                    ERRINT(b"#", DTACNT, ctx);
                    ERRINT(b"#", TCOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully processed the
                // entire DAS comment area in the text file, so there is
                // no more comment data.
                //
                MORE = false;
            } else {
                //
                // We got an unknown keyword of some sort, so set an
                // appropriate error message, close the DAS file, and
                // return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Unknown keyword \'#\' encountered while processing the DAS transfer file #.",
                    ctx,
                );
                ERRCH(b"#", &WORD, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // If we have begun a block, then process it. Otherwise, we
            // have ended a block.
            //
            if INBLK {
                //
                // Increment the record number by one for each comment
                // data block we process, because each block contains a
                // comment record.
                //
                RECNO = (RECNO + 1);
                //
                // Set the count of comment characters yet to be decoded and
                // placed in the binary DAS file.
                //
                NUMLFT = BCOUNT;

                while (NUMLFT > 0) {
                    //
                    // Now read and decode the data in the current
                    // comment data block, placing the data in the
                    // comment area of the binary DAS file.
                    //
                    if (NUMLFT >= CRLEN) {
                        NUMDTA = CRLEN;
                    } else {
                        NUMDTA = NUMLFT;
                    }
                    //
                    // Read and decode a record of encoded comment data
                    // from the text file.
                    //
                    RDENCC(XFRLUN, NUMDTA, CharArrayMut::from_mut(&mut CRECRD), ctx)?;
                    //
                    // Write the comment data to the comment area in the
                    // binary DAS file.
                    //
                    DASIOC(b"WRITE", DASLUN, RECNO, &mut CRECRD, ctx)?;

                    if FAILED(ctx) {
                        //
                        // If an error occurred, attempt to close the
                        // binary file, then checkout and return.
                        //
                        DASCLS(HANDLE, ctx)?;
                        CHKOUT(b"DASTB", ctx)?;
                        return Ok(());
                    }
                    //
                    // Decrement the counter for the amount of data
                    // remaining to be moved from the current comment
                    // block, NUMLFT.
                    //
                    NUMLFT = (NUMLFT - NUMDTA);
                    //
                    // Increment the counter for the amount of data that
                    // has been successfully moved into the comment area
                    // of the binary DAS file.
                    //
                    DTACNT = (DTACNT + NUMDTA);
                }
                //
                // At this point, we have finished reading in an entire
                // comment block.
                //
            }
            //
            // If we got to here, we have successfully written a comment
            // block to the binary file.
            //
        }
        //
        // At this point, we will have successfully written the entire
        // comment area to the binary DAS file, if there was a comment
        // area.
        //
        // Write the file record to the DAS file, to update the number
        // of comment characters.
        //
        DASWFR(HANDLE, &IDWORD, &IFNAME, NRESVR, NRESVC, NCOMR, NCOMC, ctx)?;
    }
    //
    // Read the data counts from the DAS transfer file. These will be
    // useful in determining which data types to expect in the text file
    // when converting back to binary.
    //
    RDENCI(XFRLUN, 1, std::slice::from_mut(&mut NCDATA), ctx)?;
    RDENCI(XFRLUN, 1, std::slice::from_mut(&mut NDDATA), ctx)?;
    RDENCI(XFRLUN, 1, std::slice::from_mut(&mut NIDATA), ctx)?;
    //
    // Process the character data array, if there is some character data.
    //
    if (NCDATA > 0) {
        //
        // Initialize a few things: the block counter, and the data
        // counter.
        //
        BLKCNT = 0;
        DTACNT = 0;
        //
        // We currently have more to process.
        //
        MORE = true;
        //
        // We are currently not processing a data block.
        //
        INBLK = false;

        while MORE {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                //
                // If an error occurred while reading from the encoded text
                // DAS file close the binary file, set an appropriate error
                // message, then check out and return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error reading from the DAS transfer file #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // At this point, we should be beginning a data block, ending a
            // data block, or scanning for the total number of data blocks.
            // So look for the appropriate keyword.
            //
            NEXTWD(&LINE, &mut WORD, &mut REST);

            if fstr::eq(&WORD, BCBLK) {
                //
                // Get the block number.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the begin block
                    // index, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin character block error, could not parse block number. Error: # File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of characters in the block.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the beginning
                    // data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin character block error, could not parse the data count for block: #. Error: # File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we are inside a data block, so set
                // the in block flag, INBLK, to .TRUE. and increment the
                // data block counter.
                //
                INBLK = true;
                BLKCNT = (BLKCNT + 1);
            } else if fstr::eq(&WORD, ECBLK) {
                //
                // Get the data block index.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut EINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the end block
                    // index, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"End character block error, could not parse block number. Error: # File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of characters in the DAS array.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut ECOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the ending data
                    // count, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"End character block error, could not parse the data count for block: #. Error: # File: #", ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending array indices
                // match.
                //
                if (EINDEX != BINDEX) {
                    //
                    // If the begin and end data block indices do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Character block index mismatch: Beginning index: #; Ending index: #. File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending array data
                // counts match.
                //
                if (ECOUNT != BCOUNT) {
                    //
                    // If the begin and end data block counts do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Character block count mismatch: Beginning count: #; Ending count: #. File: #", ctx);
                    ERRINT(b"#", BCOUNT, ctx);
                    ERRINT(b"#", ECOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully ended the
                // processing of a data block, so set the in block flag,
                // INBLK, to  .FALSE..
                //
                INBLK = false;
            } else if fstr::eq(&WORD, TCBLKS) {
                //
                // We have the total data blocks keyword to parse, so get
                // the total number of character data blocks processed.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut NUMBLK, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the total number of
                    // data blocks, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Block count error, could not parse the total number of character blocks: #. File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the total count of characters.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut TCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the character
                    // data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"Character count error, could not parse the total count. Error: # File: #",
                        ctx,
                    );
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Compare the computed block count with the block count
                // from the file.
                //
                if (BLKCNT != NUMBLK) {
                    //
                    // If the  calculated data block count and the data
                    // block count from the text file do not match, close
                    // the binary file, set an appropriate error message,
                    // then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"The number of character data blocks processed (#) was not equal to the number of character data blocks placed in the DAS transfer file (#). File: #", ctx);
                    ERRINT(b"#", BLKCNT, ctx);
                    ERRINT(b"#", NUMBLK, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the total count and the computed count
                // match.
                //
                if (TCOUNT != DTACNT) {
                    //
                    // If the total count and computed count do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"Character count mismatch: computed count: #; expected count: #. File: #",
                        ctx,
                    );
                    ERRINT(b"#", DTACNT, ctx);
                    ERRINT(b"#", TCOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully processed the
                // entire character data portion of the DAS transfer file,
                // so there is no more character data.
                //
                MORE = false;
            } else {
                //
                // We got an unknown keyword of some sort, so set an
                // appropriate error message, close the DAS file, and
                // return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Unknown keyword \'#\' encountered while processing the DAS transfer file #.",
                    ctx,
                );
                ERRCH(b"#", &WORD, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // If we have begun a block, then process it. Otherwise, we
            // have ended a block.
            //
            if INBLK {
                //
                // Read and decode the data in the current DAS character
                // array data block.
                //
                // Set the count of characters yet to be decoded and placed
                // in the binary DAS file.
                //
                NUMLFT = BCOUNT;

                while (NUMLFT > 0) {
                    //
                    // Now read and decode the data in the current
                    // character data block, placing the data in the
                    // character array in the binary DAS file.
                    //
                    if (NUMLFT >= (BUFSIZ * CBFLEN)) {
                        NUMDTA = (BUFSIZ * CBFLEN);
                    } else {
                        NUMDTA = NUMLFT;
                    }
                    //
                    // Read and decode a buffer of encoded character data
                    // from the text file.
                    //
                    RDENCC(XFRLUN, NUMDTA, CBUFFR.as_arg_mut(), ctx)?;
                    //
                    // Write the character data to the DAS character
                    // array in the binary DAS file.
                    //
                    DASADC(HANDLE, NUMDTA, BCBPOS, ECBPOS, CBUFFR.as_arg(), ctx)?;

                    if FAILED(ctx) {
                        //
                        // If an error occurred, attempt to close the
                        // binary file, then checkout and return.
                        //
                        DASCLS(HANDLE, ctx)?;
                        CHKOUT(b"DASTB", ctx)?;
                        return Ok(());
                    }
                    //
                    // Decrement the counter for the amount of data
                    // remaining to be moved from the current data block,
                    // NUMLFT.
                    //
                    NUMLFT = (NUMLFT - NUMDTA);
                    //
                    // Increment the counter for the amount of data that
                    // has been successfully moved into the current array
                    // in the binary DAS file.
                    //
                    DTACNT = (DTACNT + NUMDTA);
                    //
                    // At this point, we have either finished reading in an
                    // entire data block, or we have more data to read in
                    // the current data block.
                    //
                }
            }
            //
            // If we got to here, we have successfully written a data
            // block to the binary file.
            //
        }
        //
        // At this point, we will have successfully written the entire
        // character data array to the binary DAS file, if there was
        // any character data to be written.
    }
    //
    // Process the double precision data array, if there is some
    // double precision data.
    //
    if (NDDATA > 0) {
        //
        // Initialize a few things: the block counter, and the data
        // counter.
        //
        BLKCNT = 0;
        DTACNT = 0;
        //
        // We currently have more to process.
        //
        MORE = true;
        //
        // We are currently not processing a data block.
        //
        INBLK = false;

        while MORE {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                //
                // If an error occurred while reading from the encoded text
                // DAS file close the binary file, set an appropriate error
                // message, then check out and return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error reading from the DAS transfer file #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // At this point, we should be beginning a data block, ending a
            // data block, or scanning for the total number of data blocks.
            // So look for the appropriate keyword.
            //
            NEXTWD(&LINE, &mut WORD, &mut REST);

            if fstr::eq(&WORD, BDBLK) {
                //
                // Get the block number.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the begin block
                    // index, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin double precision block error, could not parse block number. Error: # File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of double precision numbers in the block.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the beginning
                    // data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin double precision block error, could not parse the data count for block: #. Error: # File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we are inside a data block, so set
                // the in block flag, INBLK, to .TRUE. and increment the
                // data block counter.
                //
                INBLK = true;
                BLKCNT = (BLKCNT + 1);
            } else if fstr::eq(&WORD, EDBLK) {
                //
                // Get the data block index.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut EINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the end block
                    // index, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"End double precision block error, could not parse block number. Error: # File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of double precision numbers in the block.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut ECOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the ending data
                    // count, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"End double precision block error, could not parse the data count for block: #. Error: # File: #", ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending array indices
                // match.
                //
                if (EINDEX != BINDEX) {
                    //
                    // If the begin and end data block indices do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Double precision block index mismatch: Beginning index: #; Ending index: #. File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending array data
                // counts match.
                //
                if (ECOUNT != BCOUNT) {
                    //
                    // If the begin and end data block counts do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Double precision block count mismatch: Beginning count: #; Ending count: #. File: #", ctx);
                    ERRINT(b"#", BCOUNT, ctx);
                    ERRINT(b"#", ECOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully ended the
                // processing of a data block, so set the in block flag,
                // INBLK, to  .FALSE..
                //
                INBLK = false;
            } else if fstr::eq(&WORD, TDBLKS) {
                //
                // We have the total data blocks keyword to parse, so get
                // the total number of character data blocks processed.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut NUMBLK, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the total number of
                    // data blocks, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Block count error, could not parse the total number of double precision data blocks: #. File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the total count of double precision numbers.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut TCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the double
                    // precision data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Double precision count error, could not parse the total count. Error: # File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Compare the computed block count with the block count
                // from the file.
                //
                if (BLKCNT != NUMBLK) {
                    //
                    // If the  calculated data block count and the data
                    // block count from the text file do not match, close
                    // the binary file, set an appropriate error message,
                    // then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"The number of double precision data blocks processed (#) was not equal to the number of double precision data blocks placed in the DAS transfer file (#). File: #", ctx);
                    ERRINT(b"#", BLKCNT, ctx);
                    ERRINT(b"#", NUMBLK, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the total count and the computed count
                // match.
                //
                if (TCOUNT != DTACNT) {
                    //
                    // If the total count and computed count do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Double precision count mismatch: computed count: #; expected count: #. File: #", ctx);
                    ERRINT(b"#", DTACNT, ctx);
                    ERRINT(b"#", TCOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully processed the
                // entire DAS double precision data portion of the text
                // file, so there is no more double precision data.
                //
                MORE = false;
            } else {
                //
                // We got an unknown keyword of some sort, so set an
                // appropriate error message, close the DAS file, and
                // return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Unknown keyword \'#\' encountered while processing the DAS transfer  file #.",
                    ctx,
                );
                ERRCH(b"#", &WORD, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // If we have begun a block, then process it. Otherwise, we
            // have ended a block.

            if INBLK {
                //
                // Read and decode the data in the current DAS double
                // precision array data block.
                //
                // Set the count of double precision numbers yet to be
                // decoded and placed in the binary DAS file.
                //
                NUMLFT = BCOUNT;

                while (NUMLFT > 0) {
                    //
                    // Now read and decode the data in the current double
                    // precision data block, placing the data in the double
                    // precision array in the binary DAS file.
                    //
                    if (NUMLFT >= BUFSIZ) {
                        NUMDTA = BUFSIZ;
                    } else {
                        NUMDTA = NUMLFT;
                    }
                    //
                    // Read and decode a buffer of encoded double precision
                    // data from the text file.
                    //
                    RDENCD(XFRLUN, NUMDTA, DBUFFR.as_slice_mut(), ctx)?;
                    //
                    // Write the double precision data to the DAS double
                    // precision array in the binary DAS file.
                    //
                    DASADD(HANDLE, NUMDTA, DBUFFR.as_slice(), ctx)?;

                    if FAILED(ctx) {
                        //
                        // If an error occurred, attempt to close the
                        // binary file, then checkout and return.
                        //
                        DASCLS(HANDLE, ctx)?;
                        CHKOUT(b"DASTB", ctx)?;
                        return Ok(());
                    }
                    //
                    // Decrement the counter for the amount of data
                    // remaining to be moved from the current data block,
                    // NUMLFT.
                    //
                    NUMLFT = (NUMLFT - NUMDTA);
                    //
                    // Increment the counter for the amount of data that
                    // has been successfully moved into the current array
                    // in the binary DAS file.
                    //
                    DTACNT = (DTACNT + NUMDTA);
                    //
                    // At this point, we have either finished reading in an
                    // entire data block, or there is still some data
                    // remaining to be read.
                    //
                }
            }
            //
            // If we got to here, we have successfully written a data
            // block to the binary file.
            //
        }
        //
        // At this point, we will have successfully written the entire
        // double precision data array to the binary DAS file, if there
        // was any double precision data to be written.
    }
    //
    // Process the integer data array, if there is some integer data.
    //
    if (NIDATA > 0) {
        //
        // Initialize a few things: the block counter, and the data
        // counter.
        //
        BLKCNT = 0;
        DTACNT = 0;
        //
        // We currently have more to process.
        //
        MORE = true;
        //
        // We are currently not processing a data block.
        //
        INBLK = false;

        while MORE {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                //
                // If an error occurred while reading from the encoded text
                // DAS file close the binary file, set an appropriate error
                // message, then check out and return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error reading from the DAS transfer file #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // At this point, we should be beginning a data block, ending a
            // data block, or scanning for the total number of data blocks.
            // So look for the appropriate keyword.
            //
            NEXTWD(&LINE, &mut WORD, &mut REST);

            if fstr::eq(&WORD, BIBLK) {
                //
                // Get the block number.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the begin block
                    // index, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin integer block error, could not parse block number. Error: # File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of integers in the block.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut BCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the beginning
                    // data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Begin integer block error, could not parse the data count for  block: #. Error: # File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we are inside a data block, so set
                // the in block flag, INBLK, to .TRUE. and increment the
                // data block counter.
                //
                INBLK = true;
                BLKCNT = (BLKCNT + 1);
            } else if fstr::eq(&WORD, EIBLK) {
                //
                // Get the data block index.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut EINDEX, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the end block
                    // index, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"End integer block error, could not parse block number. Error: # File: #",
                        ctx,
                    );
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the count of integers in the block.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut ECOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the ending data
                    // count, close the binary file, set an appropriate
                    // error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"End integer block error, could not parse the data count for block: #.Error: # File: #", ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending array indices
                // match.
                //
                if (EINDEX != BINDEX) {
                    //
                    // If the begin and end data block indices do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Integer block index mismatch: Beginning index: #; Ending index: #. File: #", ctx);
                    ERRINT(b"#", BINDEX, ctx);
                    ERRINT(b"#", EINDEX, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the beginning and ending array data
                // counts match.
                //
                if (ECOUNT != BCOUNT) {
                    //
                    // If the begin and end data block counts do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Integer block count mismatch: Beginning count: #; Ending count: #. File: #", ctx);
                    ERRINT(b"#", BCOUNT, ctx);
                    ERRINT(b"#", ECOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully ended the
                // processing of a data block, so set the in block flag,
                // INBLK, to .FALSE..
                //
                INBLK = false;
            } else if fstr::eq(&WORD, TIBLKS) {
                //
                // We have the total data blocks keyword to parse, so get
                // the total number of character data blocks processed.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut NUMBLK, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the total number of
                    // data blocks, close the binary file, set an appropriate
                    // error  message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"Block count error, could not parse the total number of integer data blocks: #. File: #", ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Parse the total count of integers.
                //
                NEXTWD(&REST.clone(), &mut WORD, &mut REST);
                NPARSI(&WORD, &mut TCOUNT, &mut ERRMSG, &mut ERRPTR, ctx);

                if fstr::ne(&ERRMSG, b" ") {
                    //
                    // If an error occurred while parsing the integer
                    // data count, close the binary file, set an
                    // appropriate error message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"Integer count error, could not parse the total count. Error: # File: #",
                        ctx,
                    );
                    ERRCH(b"#", &ERRMSG, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Compare the computed block count with the block count
                // from the file.
                //
                if (BLKCNT != NUMBLK) {
                    //
                    // If the  calculated data block count and the data
                    // block count from the text file do not match, close
                    // the binary file, set an appropriate error message,
                    // then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(b"The number of integer data blocks processed (#) was not equal to the number of integer data blocks placed in the DAS transfer file (#). File: #", ctx);
                    ERRINT(b"#", BLKCNT, ctx);
                    ERRINT(b"#", NUMBLK, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // Check to see if the total count and the computed count
                // match.
                //
                if (TCOUNT != DTACNT) {
                    //
                    // If the total count and computed count do not match,
                    // close the binary file, set an appropriate error
                    // message, then check out and return.
                    //
                    DASCLS(HANDLE, ctx)?;
                    SETMSG(
                        b"Integer count mismatch: computed count: #; expected count: #. File: #",
                        ctx,
                    );
                    ERRINT(b"#", DTACNT, ctx);
                    ERRINT(b"#", TCOUNT, ctx);
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                    CHKOUT(b"DASTB", ctx)?;
                    return Ok(());
                }
                //
                // If we got to here, we have successfully processed the
                // entire DAS integer data portion of the text file, so
                // there is no more integer data.
                //
                MORE = false;
            } else {
                //
                // We got an unknown keyword of some sort, so set an
                // appropriate error message, close the DAS file, and
                // return.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Unknown keyword \'#\' encountered while processing the DAS transfer  file #.",
                    ctx,
                );
                ERRCH(b"#", &WORD, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDASTRANSFERFILE)", ctx)?;
                CHKOUT(b"DASTB", ctx)?;
                return Ok(());
            }
            //
            // If we have begun a block, then process it. Otherwise, we
            // have ended a block.

            if INBLK {
                //
                // Read and decode the data in the current DAS integer
                // array data block.
                //
                // Set the count of integers yet to be decoded and placed
                // in the binary DAS file.
                //
                NUMLFT = BCOUNT;

                while (NUMLFT > 0) {
                    //
                    // Now read and decode the data in the current
                    // integer data block, placing the data in the
                    // integer precision array in the binary DAS file.
                    //
                    if (NUMLFT >= BUFSIZ) {
                        NUMDTA = BUFSIZ;
                    } else {
                        NUMDTA = NUMLFT;
                    }
                    //
                    // Read and decode a buffer of encoded integer data
                    // from the text file.
                    //
                    RDENCI(XFRLUN, NUMDTA, IBUFFR.as_slice_mut(), ctx)?;
                    //
                    // Write the integer data to the DAS integer array in
                    // the binary DAS file.
                    //
                    DASADI(HANDLE, NUMDTA, IBUFFR.as_slice(), ctx)?;

                    if FAILED(ctx) {
                        //
                        // If an error occurred, attempt to close the
                        // binary file, then checkout and return.
                        //
                        DASCLS(HANDLE, ctx)?;
                        CHKOUT(b"DASTB", ctx)?;
                        return Ok(());
                    }
                    //
                    // Decrement the counter for the amount of data
                    // remaining to be moved from the current data block,
                    // NUMLFT.
                    //
                    NUMLFT = (NUMLFT - NUMDTA);
                    //
                    // Increment the counter for the amount of data that
                    // has been successfully moved into the current array
                    // in the binary DAS file.
                    //
                    DTACNT = (DTACNT + NUMDTA);
                    //
                    // At this point, we have either finished reading in an
                    // entire data block, or there is still data remaining
                    // to be read.
                    //
                }
            }
            //
            // If we got to here, we have successfully written a data
            // block to the binary file.
            //
        }
        //
        // At this point, we will have successfully written the entire
        // integer data array to the binary DAS file, if there was any
        // integer data to be written.
    }
    //
    // Close only the binary file.
    //
    DASCLS(HANDLE, ctx)?;

    CHKOUT(b"DASTB", ctx)?;
    Ok(())
}
