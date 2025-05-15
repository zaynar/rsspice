//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const NWD: i32 = 128;
const NWI: i32 = 256;
const NWC: i32 = 1024;
const CHARDT: i32 = 1;
const DPDT: i32 = 2;
const INTDT: i32 = 3;
const LBCELL: i32 = -5;

struct SaveVars {
    FHSET: ActualArray<i32>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FHSET = ActualArray::<i32>::new(LBCELL..=FTSIZE);
        let mut PASS1: bool = false;

        PASS1 = true;

        Self { FHSET, PASS1 }
    }
}

/// DAS, close file
///
/// Close a DAS file.
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
///  HANDLE     I   Handle of an open DAS file.
///  FTSIZE     P   Maximum number of simultaneously open DAS files.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an open DAS file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  See $Particulars for a description of the effect of this routine.
/// ```
///
/// # Parameters
///
/// ```text
///  All parameters described here are declared in the SPICELIB
///  include file das.inc. See that file for parameter values.
///
///  FTSIZE   is the maximum number of DAS files that can be
///           open at any one time.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If HANDLE is not the handle of an open DAS file, no error
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of input argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the primary recommended method of closing an
///  open DAS file. It is also possible to close a DAS file without
///  segregating it by calling DASWBR and DASLLC. Closing a DAS file by
///  any other means may cause the DAS mechanism for keeping track of
///  which files are open to fail. Closing a DAS file that has been
///  opened for writing by any other means may result in the production
///  of something other than a DAS file.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Open a new DAS file called TEST.DAS, add 100 d.p. numbers
///     to it, and then close the file.
///
///     Example code begins here.
///
///
///           PROGRAM DASCLS_EX1
///           IMPLICIT NONE
///
///           INTEGER               NMAX
///           PARAMETER           ( NMAX   = 100 )
///
///           CHARACTER*(15)        FNAME
///           CHARACTER*(5)         FTYPE
///           CHARACTER*(15)        IFNAME
///
///           DOUBLE PRECISION      DDATA  ( NMAX )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               N
///           INTEGER               NCOMCH
///
///     C
///     C     We'll give the file the same internal file name
///     C     as the file's actual name.  We don't require any
///     C     comment records.
///     C
///           FNAME  = 'dascls_ex1.das'
///           FTYPE  = 'TEST'
///           IFNAME = FNAME
///           NCOMCH = 0
///
///           WRITE(*,*) 'Opening the DAS file for writing...'
///           CALL DASONW ( FNAME, FTYPE,  FNAME, NCOMCH, HANDLE )
///
///           DO I = 1, NMAX
///              DDATA(I)  =  DBLE(I)
///           END DO
///
///           N = NMAX
///
///           WRITE(*,*) 'Adding the double precision numbers...'
///           CALL DASADD ( HANDLE, N, DDATA )
///
///           WRITE(*,*) 'Closing the DAS file...'
///           CALL DASCLS ( HANDLE )
///
///           WRITE(*,*) 'All ok.'
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Opening the DAS file for writing...
///      Adding the double precision numbers...
///      Closing the DAS file...
///      All ok.
///
///
///     Note that after run completion, a new DAS file exists in
///     the output directory.
///
///
///  2) Dump several parameters from the first DLA segment of a DSK
///     file. Note that DSK files are based on DAS. The segment is
///     assumed to be of type 2.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASCLS_EX2
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///
///     C
///     C     Local parameters
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    DSK
///           CHARACTER*(LNSIZE)    OUTLIN
///
///           DOUBLE PRECISION      VOXORI ( 3 )
///           DOUBLE PRECISION      VOXSIZ
///           DOUBLE PRECISION      VTXBDS ( 2, 3 )
///
///           INTEGER               CGSCAL
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               NP
///           INTEGER               NV
///           INTEGER               NVXTOT
///           INTEGER               VGREXT ( 3 )
///           INTEGER               VOXNPL
///           INTEGER               VOXNPT
///           INTEGER               VTXNPL
///
///           LOGICAL               FOUND
///
///
///     C
///     C     Prompt for the name of the DSK to read.
///     C
///           CALL PROMPT ( 'Enter DSK name > ', DSK )
///     C
///     C     Open the DSK file for read access.
///     C     We use the DAS-level interface for
///     C     this function.
///     C
///           CALL DASOPR ( DSK, HANDLE )
///
///     C
///     C     Begin a forward search through the
///     C     kernel, treating the file as a DLA.
///     C     In this example, it's a very short
///     C     search.
///     C
///           CALL DLABFS ( HANDLE, DLADSC, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///     C
///     C        We arrive here only if the kernel
///     C        contains no segments.  This is
///     C        unexpected, but we're prepared for it.
///     C
///              CALL SETMSG ( 'No segments found '
///          .   //            'in DSK file #.'    )
///              CALL ERRCH  ( '#',  DSK           )
///              CALL SIGERR ( 'SPICE(NODATA)'     )
///
///           END IF
///
///     C
///     C     If we made it this far, DLADSC is the
///     C     DLA descriptor of the first segment.
///     C
///     C     Read and display type 2 bookkeeping data.
///     C
///           CALL DSKB02 ( HANDLE, DLADSC, NV,     NP,     NVXTOT,
///          .              VTXBDS, VOXSIZ, VOXORI, VGREXT, CGSCAL,
///          .              VTXNPL, VOXNPT, VOXNPL                 )
///
///     C
///     C     Show vertex and plate counts.
///     C
///           OUTLIN = 'Number of vertices:                 #'
///           CALL REPMI  ( OUTLIN, '#', NV, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Number of plates:                   #'
///           CALL REPMI  ( OUTLIN, '#', NP, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Voxel edge length (km):             #'
///           CALL REPMF  ( OUTLIN, '#', VOXSIZ, 6, 'E', OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Number of voxels:                   #'
///           CALL REPMI  ( OUTLIN, '#', NVXTOT, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///     C
///     C     Close the kernel.  This isn't necessary in a stand-
///     C     alone program, but it's good practice in subroutines
///     C     because it frees program and system resources.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos512.bds, the output
///     was:
///
///
///     Enter DSK name > phobos512.bds
///     Number of vertices:                 1579014
///     Number of plates:                   3145728
///     Voxel edge length (km):             1.04248E-01
///     Number of voxels:                   11914500
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 12-OCT-2021 (JDR) (NJB)
///
///         Bug fix: added call to FAILED following call to DASHAM.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Created
///         complete code example #1 from existing code fragment
///         and added code example #2 using example from DSKB02.
///
///         Updated entries in the $Revisions section.
///
/// -    SPICELIB Version 2.0.0, 10-FEB-2017 (NJB)
///
///         Updated to use DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 1.3.3, 05-OCT-2006 (NJB)
///
///         Corrected DASADD calling sequence error in code example.
///         Updated $Particulars header section to mention closing DAS
///         files without segregation via calls to DASWBR and DASLLC.
///
/// -    SPICELIB Version 1.3.2, 24-MAR-2003 (NJB)
///
///         DASWBR call has been reinstated for scratch DAS case.
///         This call has the side effect of freeing buffer records
///         owned by the file DASWBR writes to. Failing to free these
///         records can cause write errors on HP/Fortran systems.
///
/// -    SPICELIB Version 1.2.2, 27-FEB-2003 (NJB)
///
///         Tests whether file to be closed is a scratch DAS; if
///         so, buffer flushes and record segregation are omitted.
///
/// -    SPICELIB Version 1.1.1, 26-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
///         Modified the $Examples section to demonstrate the new ID word
///         format which includes a file type and to include a call to the
///         new routine DASONW, open new for write, which makes use of the
///         file type. Also,  a variable for the type of the file to be
///         created was added.
///
///         Changed the value of the parameter FTSIZE from 20 to 21. This
///         change makes the value of FTSIZE in DASCLS compatible with the
///         value in DASFM. See DASFM for a discussion of the reasons for
///         the increase in the value.
///
/// -    SPICELIB Version 1.1.0, 08-JUL-1993 (NJB)
///
///         FHSET is now saved.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.3.2, 24-MAR-2003 (NJB)
///
///         DASWBR call has been reinstated for scratch DAS case.
///         This call has the side effect of freeing buffer records
///         owned by the file DASWBR writes to. Failing to free these
///         records can cause write errors on HP/Fortran systems.
///
/// -    SPICELIB Version 1.2.2, 27-FEB-2003 (NJB)
///
///         Tests whether file to be closed is a scratch DAS; if
///         so, buffer flushes and record segregation are omitted.
///
/// -    SPICELIB Version 1.1.1, 26-OCT-1993 (KRG)
///
///         Changed the value of the parameter FTSIZE from 20 to 21. This
///         change makes the value of FTSIZE in DASCLS compatible with the
///         value in DASFM. See DASFM for a discussion of the reasons for
///         the increase in the value.
/// ```
pub fn dascls(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DASCLS(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASCLS ( DAS, close file )
pub fn DASCLS(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut METHOD = [b' '; 10 as usize];
    let mut IOSTAT: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut NOTSCR: bool = false;

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASCLS", ctx)?;

    if save.PASS1 {
        SSIZEI(FTSIZE, save.FHSET.as_slice_mut(), ctx)?;
        save.PASS1 = false;
    }

    //
    // There are only four items on our worklist:
    //
    //    1)  Determine whether the file open for reading or writing,
    //        and if it's open for writing, whether it's a scratch
    //        file.
    //
    //    2)  If the DAS file is open for writing, flush any updated
    //        records from the data buffers to the file.
    //
    //    3)  If the DAS file is open for writing, re-order the records
    //        in the file so that the data is segregated by data type.
    //
    //    4)  Close the file.
    //

    //
    // See whether the input handle designates an open DAS file.  If not,
    // return now.
    //
    DASHOF(save.FHSET.as_slice_mut(), ctx)?;

    if !ELEMI(HANDLE, save.FHSET.as_slice(), ctx)? {
        CHKOUT(b"DASCLS", ctx)?;
        return Ok(());
    }

    //
    // If the file is open for writing, flush any buffered
    // records that belong to it.
    //
    DASHAM(HANDLE, &mut METHOD, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASCLS", ctx)?;
        return Ok(());
    }

    if fstr::eq(&METHOD, b"WRITE ") {
        //
        // Make sure that all buffered records belonging to the
        // indicated file are written out.
        //
        DASWBR(HANDLE, ctx)?;

        //
        // We cannot directly test the status of the file, but if
        // the file is unnamed, it must be a scratch file.
        //
        ZZDDHHLU(HANDLE, b"DAS", false, &mut UNIT, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASCLS", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::io;

            let specs = io::InquireSpecs {
                unit: Some(UNIT),
                named: Some(&mut NOTSCR),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
        }

        if (IOSTAT != 0) {
            SETMSG(b"Error occurred while performing an  INQUIRE on a DAS file about to be closed.  IOSTAT = #. File handle was #.  Logical unit was #.", ctx);
            ERRINT(b"#", IOSTAT, ctx);
            ERRINT(b"#", HANDLE, ctx);
            ERRINT(b"#", UNIT, ctx);
            SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
            CHKOUT(b"DASCLS", ctx)?;
            return Ok(());
        }

        if NOTSCR {
            //
            // Segregate the data records in the file according to data
            // type.
            //
            DASSDR(HANDLE, ctx)?;
        }
    }

    //
    // Close the file.
    //
    DASLLC(HANDLE, ctx)?;

    CHKOUT(b"DASCLS", ctx)?;
    Ok(())
}
