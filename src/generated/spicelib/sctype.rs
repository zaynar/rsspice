//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;

/// SCLK type
///
/// Return the spacecraft clock type for a specified spacecraft.
///
/// # Required Reading
///
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SC         I   NAIF spacecraft ID code.
///
///  The function returns the spacecraft clock type associated with
///  the spacecraft specified by SC.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is a NAIF ID code for a spacecraft, whose
///           spacecraft clock `type' is desired.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the spacecraft clock type associated with
///  the spacecraft specified by SC.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the kernel variable that assigns a SCLK type to the
///      spacecraft specified by SC is not found in the kernel pool, an
///      error is signaled by a routine in the call tree of this
///      routine. SCTYPE returns the value 0 in this case.
/// ```
///
/// # Particulars
///
/// ```text
///  The raison d'etre of this routine is that it consolidates the code
///  that maps spacecraft ID's to clock types. While any routine may
///  call SCTYPE, it is unlikely that there will be a need for
///  non-SPICELIB routines to call this routine directly.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Find the SCLK type for Galileo.
///
///         During program initialization, we load a SCLK kernel file
///         into the kernel pool. We will pretend that the name of
///         this file is GLLSCLK.KER. You must use the actual name of
///         an SCLK kernel that is accessible by your program to try
///         this example.
///
///             C
///             C     Load the SCLK kernel.
///             C
///                   CALL FURNSH ( 'GLLSCLK.KER' )
///                              .
///                              .
///                              .
///             C
///             C     Print out the clock type for Galileo.
///             C
///                   TYPE = SCTYPE ( -77 )
///
///                   PRINT *, 'Galileo clock type is ', TYPE
///
///
///  2)  Find the SCLK type for Mars Observer.
///
///
///             C
///             C     Load the SCLK kernel.
///             C
///                   CALL FURNSH ( 'MOSCLK.KER' )
///                              .
///                              .
///                              .
///             C
///             C     Print out the clock type for Mars Observer.
///             C
///                   TYPE = SCTYPE ( -94 )
///
///                   PRINT *, 'Mars Observer clock type is ', TYPE
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that an SCLK kernel appropriate to the
///      spacecraft specified by SC has been loaded into the kernel
///      pool.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.4.0, 12-AUG-2021 (NJB) (JDR)
///
///         Re-implemented using the type 1 SCLK database in order to
///         improve performance. The routine still works as before for
///         clock types other than 1.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.3.0, 09-SEP-2013 (BVS)
///
///         Updated to keep track of the POOL counter and call ZZCVPOOL.
///
/// -    SPICELIB Version 1.2.0, 05-MAR-2009 (NJB)
///
///         Bug fix: this routine now keeps track of whether its
///         kernel pool look-up succeeded. If not, a kernel pool
///         lookup is attempted on the next call to this routine.
///
/// -    SPICELIB Version 1.1.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.1.0, 22-MAR-1993 (JML)
///
///         1) The routine now uses the kernel pool watch capability.
///
///         2) The routine now returns a value of zero if RETURN is
///            .TRUE. on entry.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 04-SEP-1990 (NJB)
/// ```
pub fn sctype(ctx: &mut SpiceContext, sc: i32) -> crate::Result<i32> {
    let ret = SCTYPE(sc, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SCTYPE ( SCLK type )
pub fn SCTYPE(SC: i32, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let mut SCTYPE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        SCTYPE = 0;

        return Ok(SCTYPE);
    }

    CHKIN(b"SCTYPE", ctx)?;
    //
    // Get the type from the type 1 subsystem.
    //
    SCTY01(SC, &mut SCTYPE, ctx)?;

    CHKOUT(b"SCTYPE", ctx)?;
    Ok(SCTYPE)
}
