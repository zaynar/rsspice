//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const NWI: i32 = 256;

/// DAS, Fortran I/O, integer
///
/// Perform Fortran reads and writes of DAS integer records.
/// This routine operates on DAS files having native binary
/// format.
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
///  ACTION     I   Action to take (read or write).
///  UNIT       I   Fortran unit connected to DAS file.
///  RECNO      I   Number of record to read or write.
///  RECORD    I-O  DAS integer record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ACTION   is a character string specifying whether to read
///           from or write to the specified DAS file. Possible
///           values are:
///
///              'READ'
///              'WRITE'
///
///           Case and leading or trailing blanks are not
///           significant.
///
///
///  UNIT     is the Fortran unit number connected to the DAS
///           file that is to be read or written. Given the
///           handle of the DAS file, the unit number can be
///           obtained using ZZDDHHLU.
///
///  RECNO    is the Fortran record number of the record to be
///           read or written.
///
///  RECORD   is an integer array whose contents are to be
///           written to record RECNO, if ACTION is WRITE.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is an integer array whose contents are to be
///           set equal to those of record RECNO, if ACTION
///           is READ.
/// ```
///
/// # Parameters
///
/// ```text
///  NWI      is the number of elements in a DAS integer record.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of ACTION is not recognized, the error
///      SPICE(UNRECOGNIZEDACTION) is signaled.
///
///  2)  If a Fortran read error occurs, the error
///      SPICE(DASFILEREADFAILED) is signaled.
///
///  3)  If a Fortran write error occurs, the error
///      SPICE(DASFILEWRITEFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument UNIT in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine may be used to write to and read from DAS files
///  having the native binary file format of the host system. The
///  routine ZZDASGDI should be used to read integer records from DAS
///  files that may have either native or non-native format.
///
///  Normally, routines outside of SPICELIB will not need to call this
///  routine directly. Writes to DAS files should be performed using
///  the DASADx and DASUDx routines; reads should be performed using
///  the DASRDx routines.
///
///  This routine centralizes I/O and the concomitant error handling
///  for DAS integer records.
///
///  Although most DAS routines use file handles to identify DAS
///  files, this routine uses Fortran logical units for this purpose.
///  Using unit numbers allows the DASIOx routines to be called from
///  any DAS routine, including entry points of DASFM.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Read and print to the screen integer records number 10
///      through 20 from the DAS file designated by HANDLE.
///
///         INTEGER               RECORD ( NWI )
///                        .
///                        .
///                        .
///
///         CALL ZZDDHHLU ( HANDLE, 'DAS', .FALSE., UNIT )
///         CALL DASHFN   ( HANDLE, NAME )
///
///         DO I = 1, 20
///
///            CALL DASIOI ( 'READ', UNIT, 10, RECORD )
///
///            LABEL = 'Contents of the # record in DAS file #: '
///
///            CALL REPMOT ( LABEL,  '#',  I,  'L',   LABEL )
///            CALL REPMC  ( LABEL,  '#',      NAME,  LABEL )
///
///            WRITE (*,*) LABEL
///            WRITE (*,*) ' '
///            WRITE (*,*) RECORD
///
///         END DO
///
///
///
///  2)  Write the contents of the array RECORD to record number
///      10 in the DAS file designated by HANDLE.
///
///
///         INTEGER               RECORD ( NWI )
///
///                        .
///                        .
///                        .
///
///         CALL ZZDDHHLU ( HANDLE,  'DAS', .FALSE., UNIT   )
///         CALL DASIOI   ( 'WRITE', UNIT,  10,      RECORD )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine may be used only on DAS files having
///      the native binary file format of the host system.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 05-FEB-2015 (NJB)
///
///         Header was updated to refer to ZZDDHHLU. $Restrictions section
///         was updated.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn dasioi(
    ctx: &mut SpiceContext,
    action: &str,
    unit: i32,
    recno: i32,
    record: &mut [i32; 256],
) -> crate::Result<()> {
    DASIOI(action.as_bytes(), unit, recno, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASIOI ( DAS, Fortran I/O, integer )
pub fn DASIOI(
    ACTION: &[u8],
    UNIT: i32,
    RECNO: i32,
    RECORD: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RECORD = DummyArrayMut::new(RECORD, 1..=NWI);
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if EQSTR(ACTION, b"READ") {
        //
        // We're supposed to read the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                for n in RECORD.iter_mut() {
                    *n = reader.read_i32()?;
                }
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            CHKIN(b"DASIOI", ctx)?;
            SETMSG(
                b"Could not read DAS integer record. File = # Record number = #. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", RECNO, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DASFILEREADFAILED)", ctx)?;
            CHKOUT(b"DASIOI", ctx)?;
            return Ok(());
        }
    } else if EQSTR(ACTION, b"WRITE") {
        //
        // We're supposed to write to the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                for n in RECORD.iter() {
                    writer.write_i32(*n)?;
                }
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            CHKIN(b"DASIOI", ctx)?;
            SETMSG(
                b"Could not write DAS integer record. File = # Record number = #. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", RECNO, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DASFILEWRITEFAILED)", ctx)?;
            CHKOUT(b"DASIOI", ctx)?;
            return Ok(());
        }
    } else {
        //
        // The requested action is a little too weird.
        //
        CHKIN(b"DASIOI", ctx)?;
        SETMSG(b"Action was #; should be READ or WRITE", ctx);
        ERRCH(b"#", ACTION, ctx);
        SIGERR(b"SPICE(UNRECOGNIZEDACTION)", ctx)?;
        CHKOUT(b"DASIOI", ctx)?;
        return Ok(());
    }

    Ok(())
}
