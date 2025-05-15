//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const MXPART: i32 = 9999;
const PARTLN: i32 = 5;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const MXTSYS: i32 = 2;
const CTRSIZ: i32 = 2;

/// Spacecraft Clock Partition Information
///
/// Get spacecraft clock partition information from a spacecraft
/// clock kernel file.
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
///  SC         I   NAIF spacecraft identification code.
///  NPARTS     O   The number of spacecraft clock partitions.
///  PSTART     O   Array of partition start times.
///  PSTOP      O   Array of partition stop times.
///  MXPART     P   Maximum number of partitions.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is the NAIF ID for the spacecraft whose clock partition
///           information is being requested.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NPARTS   is the number of spacecraft clock time partitions
///           described in the kernel file for spacecraft SC.
///
///  PSTART   is an array containing NPARTS partition start times
///           represented as double precision, encoded SCLK
///           ("ticks"). The values contained in PSTART are whole
///           numbers.
///
///  PSTOP    is an array containing NPARTS partition end times
///           represented as double precision, encoded SCLK
///           ("ticks"). The values contained in PSTOP are whole
///           numbers.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file
///
///     sclk.inc
///
///  for sizes and limits used by the SCLK system.
///
///  MXPART   is the maximum number of partitions for any spacecraft
///           clock. SCLK kernels contain start and stop times for
///           each partition. See the include file sclk.inc for this
///           parameter's value.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the kernel variables containing the spacecraft clock
///      partition start and stop times have not been loaded in the
///      kernel pool, an error is signaled by a routine in the call
///      tree of this routine.
///
///  2)  If the number of start and stop times are different,
///      the error SPICE(NUMPARTSUNEQUAL) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  An SCLK kernel containing spacecraft clock partition start
///  and stop times for the spacecraft clock indicated by SC must
///  be loaded into the kernel pool.
/// ```
///
/// # Particulars
///
/// ```text
///  SCPART looks for two variables in the kernel pool for each
///  spacecraft's partition information. If SC = -nn, then the names of
///  the variables are
///
///     SCLK_PARTITION_START_nn
///     SCLK_PARTITION_END_nn
///
///  The start and stop times returned are in units of "ticks".
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) The following code example finds partition start and stop
///     times for the Stardust spacecraft from a spacecraft clock
///     kernel file. Since those times are always returned in units
///     of ticks, the program uses SCFMT to print the times in
///     Stardust clock format.
///
///     Use the SCLK kernel below to load the Stardust time
///     correlation data and spacecraft clock partition information.
///
///        sdu_sclkscet_00074.tsc
///
///
///     Example code begins here.
///
///
///           PROGRAM SCPART_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the parameters that define the sizes and limits
///     C     of the SCLK system.
///     C
///           INCLUDE 'sclk.inc'
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               CLKLEN
///           PARAMETER           ( CLKLEN = 30 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(CLKLEN)    START
///           CHARACTER*(CLKLEN)    STOP
///
///           DOUBLE PRECISION      PSTART (MXPART)
///           DOUBLE PRECISION      PSTOP  (MXPART)
///
///           INTEGER               I
///           INTEGER               NPARTS
///           INTEGER               SC
///
///     C
///     C     Assign the value for the Stardust spacecraft ID.
///     C
///           SC = -29
///
///     C
///     C     Load the SCLK file.
///     C
///           CALL FURNSH ( 'sdu_sclkscet_00074.tsc' )
///
///     C
///     C     Retrieve the arrays for PSTART and PSTOP and the
///     C     number of partitions within the SCLK.
///     C
///           CALL SCPART ( SC, NPARTS, PSTART, PSTOP )
///
///     C
///     C     Loop over each array value.
///     C
///           DO I= 1, NPARTS
///
///              CALL SCFMT ( SC, PSTART( I ), START )
///              CALL SCFMT ( SC, PSTOP ( I ), STOP )
///
///              WRITE(*,*)
///              WRITE(*,*) 'Partition: ', I
///              WRITE(*,*) '   Start : ', START
///              WRITE(*,*) '   Stop  : ', STOP
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Partition:            1
///         Start : 0000000000.000
///         Stop  : 0602741011.080
///
///      Partition:            2
///         Start : 0602741014.217
///         Stop  : 0605660648.173
///
///      Partition:            3
///         Start : 0605660649.000
///         Stop  : 0631375256.224
///
///      Partition:            4
///         Start : 0631375257.000
///         Stop  : 0633545577.218
///
///      Partition:            5
///         Start : 0633545578.000
///         Stop  : 0644853954.043
///
///      Partition:            6
///         Start : 0644853954.000
///         Stop  : 0655316480.089
///
///      Partition:            7
///         Start : 0655316480.000
///         Stop  : 0660405279.066
///
///      Partition:            8
///         Start : 0660405279.000
///         Stop  : 0670256568.229
///
///      Partition:            9
///         Start : 0670256569.000
///         Stop  : 0674564039.091
///
///      Partition:           10
///         Start : 0674564040.000
///         Stop  : 4294537252.255
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that an SCLK kernel appropriate to the
///      spacecraft identified by SC has been loaded into the kernel
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
///  R.E. Thurman       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.4.0, 06-JUL-2021 (JDR) (NJB)
///
///         Code was re-written to fetch partition data from the SC01
///         subsystem. This routine no longer sets watches on kernel
///         variables.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries from $Revisions section.
///
///         Added complete code example based on existing code fragment.
///
/// -    SPICELIB Version 2.3.1, 19-MAR-2014 (NJB)
///
///         Minor header comment updates were made.
///
/// -    SPICELIB Version 2.3.0, 09-SEP-2013 (BVS)
///
///         Updated to keep track of the POOL counter and call ZZCVPOOL.
///
/// -    SPICELIB Version 2.2.0, 05-MAR-2009 (NJB)
///
///         Bug fix: this routine now keeps track of whether its
///         kernel pool look-up succeeded. If not, a kernel pool
///         lookup is attempted on the next call to this routine.
///
/// -    SPICELIB Version 2.1.0, 05-FEB-2008 (NJB)
///
///         The values of the parameter MXPART is now
///         provided by the INCLUDE file sclk.inc.
///
/// -    SPICELIB Version 1.1.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.1.0, 22-MAR-1993 (JML)
///
///         The routine now uses the kernel pool watch capability.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 03-SEP-1990 (NJB) (JML) (RET)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.4.0, 04-NOV-2020 (JDR) (NJB)
///
///         Code was re-written to fetch partition data from the SC01
///         subsystem. This routine no longer sets watches on kernel
///         variables. Setting watches "touches" the kernel pool, which
///         thwarts optimizations of many SPICE subsystems.
/// ```
pub fn scpart(
    ctx: &mut SpiceContext,
    sc: i32,
    nparts: &mut i32,
    pstart: &mut [f64],
    pstop: &mut [f64],
) -> crate::Result<()> {
    SCPART(sc, nparts, pstart, pstop, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCPART ( Spacecraft Clock Partition Information )
pub fn SCPART(
    SC: i32,
    NPARTS: &mut i32,
    PSTART: &mut [f64],
    PSTOP: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PSTART = DummyArrayMut::new(PSTART, 1..);
    let mut PSTOP = DummyArrayMut::new(PSTOP, 1..);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SCPART", ctx)?;

    //
    // Get the partition values from the type 1 subsystem. If the
    // clock is not type 1, an error will be signaled.
    //
    SCPR01(SC, NPARTS, PSTART.as_slice_mut(), PSTOP.as_slice_mut(), ctx)?;

    CHKOUT(b"SCPART", ctx)?;
    Ok(())
}
