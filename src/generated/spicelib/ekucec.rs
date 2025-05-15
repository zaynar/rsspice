//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

/// EK, update character column entry
///
/// Update a character column entry in a specified EK record.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle attached to EK file.
///  SEGNO      I   Index of segment containing record.
///  RECNO      I   Record in which entry is to be updated.
///  COLUMN     I   Column name.
///  NVALS      I   Number of values in new column entry.
///  CVALS      I   Character string values to add to column.
///  ISNULL     I   Flag indicating whether column entry is null.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle attached to an EK open for
///           write access.
///
///  SEGNO    is the index of the segment containing the column
///           entry to be updated.
///
///  RECNO    is the index of the record containing the column
///           entry to be updated. This record number is
///           relative to the start of the segment indicated by
///           SEGNO; the first record in the segment has index 1.
///
///  COLUMN   is the name of the column containing the entry to
///           be updated.
///
///  NVALS,
///  CVALS    are, respectively, the number of values to add to
///           the specified column and the set of values
///           themselves. The data values are written in to the
///           specified column and record.
///
///           If the  column has fixed-size entries, then NVALS
///           must equal the entry size for the specified column.
///
///           For columns with variable-sized entries, the size
///           of the new entry need not match the size of the
///           entry it replaces. In particular, the new entry
///           may be larger.
///
///  ISNULL   is a logical flag indicating whether the entry is
///           null. If ISNULL is .FALSE., the column entry
///           defined by NVALS and CVALS is added to the
///           specified kernel file.
///
///           If ISNULL is .TRUE., NVALS and CVALS are ignored.
///           The contents of the column entry are undefined.
///           If the column has fixed-length, variable-size
///           entries, the number of entries is considered to
///           be 1.
///
///           The new entry may be null even though it replaces
///           a non-null value, and vice versa.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE is invalid, an error is signaled by a routine in the
///      call tree of this routine.
///
///  2)  If SEGNO is out of range, an error is signaled by a routine in
///      the call tree of this routine.
///
///  3)  If COLUMN is not the name of a declared column, an error
///      is signaled by a routine in the call tree of this routine.
///
///  4)  If COLUMN specifies a column of whose data type is not
///      CHARACTER, the error SPICE(WRONGDATATYPE) is signaled.
///
///  5)  If RECNO is out of range, an error is signaled by a routine in
///      the call tree of this routine.
///
///  6)  If the specified column has fixed-size entries and NVALS does
///      not match this size, an error is signaled by a routine in the
///      call tree of this routine.
///
///  7)  If the specified column has variable-size entries and NVALS is
///      non-positive, an error is signaled by a routine in the call
///      tree of this routine.
///
///  8)  If an attempt is made to add a null value to a column that
///      doesn't take null values, an error is signaled by a routine in
///      the call tree of this routine.
///
///  9)  If COLUMN specifies a column of whose class is not
///      a character class known to this routine, the error
///      SPICE(NOCLASS) is signaled.
///
///  10) If an I/O error occurs while reading or writing the indicated
///      file, the error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Files
///
/// ```text
///  See the EK Required Reading ek.req for a discussion of the EK file
///  format.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine operates by side effects: it modifies the named
///  EK file by adding data to the specified record in the specified
///  column. Data may be added to a segment in random order; it is not
///  necessary to fill in columns or rows sequentially. Data may only
///  be added one logical element at a time. Partial assignments of
///  logical elements are not supported.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) The following program demonstrates how to create a new EK and
///     add data to a character column in a given record within the
///     file, how to update the data in this record, and how to read
///     the data from it.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKUCEC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C
///           INCLUDE 'ekcnamsz.inc'
///
///     C
///     C     Local constants.
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'ekucec_ex1.bdb' )
///
///           CHARACTER*(*)         IFNAME
///           PARAMETER           ( IFNAME = 'Test EK'  )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE  = 'CHR_DATA' )
///
///           INTEGER               CVLEN
///           PARAMETER           ( CVLEN  = 9  )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               MAXVAL
///           PARAMETER           ( MAXVAL = 4  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 2  )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 6  )
///
///           INTEGER               NRESVC
///           PARAMETER           ( NRESVC = 0  )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(CVLEN)     CVALS  ( MAXVAL )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               NVALS
///           INTEGER               RECNO
///           INTEGER               SEGNO
///
///           LOGICAL               ISNULL
///
///
///     C
///     C     Open a new EK file.  For simplicity, we won't
///     C     reserve space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The constant IFNAME is the internal file name.
///     C
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the table and column names and declarations
///     C     for the CHR_DATA segment.  We'll index all of
///     C     the columns.
///     C
///           CNAMES(1) =  'CHR_COL_1'
///           CDECLS(1) =  'DATATYPE = CHARACTER*(*), '
///          .       //    'INDEXED = TRUE, NULLS_OK = TRUE'
///
///           CNAMES(2) =  'CHR_COL_2'
///           CDECLS(2) =  'DATATYPE = CHARACTER*(9), '
///          .       //    'SIZE = VARIABLE, NULLS_OK = TRUE'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///           DO I = 0, NROWS-1
///
///              CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///              ISNULL = ( I .EQ. 1 )
///
///              CALL INTSTR ( I, CVALS(1) )
///              CALL EKACEC ( HANDLE, SEGNO, RECNO, CNAMES(1),
///          .                 1,      CVALS, ISNULL           )
///
///     C
///     C        Array-valued columns follow.
///     C
///              CALL INTSTR ( 10*I,     CVALS(1) )
///              CALL INTSTR ( 10*I + 1, CVALS(2) )
///              CALL INTSTR ( 10*I + 2, CVALS(3) )
///              CALL INTSTR ( 10*I + 3, CVALS(4) )
///              CALL EKACEC ( HANDLE, SEGNO, RECNO, CNAMES(2),
///          .                 4,      CVALS, ISNULL           )
///
///           END DO
///
///     C
///     C     End the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the EK for write access.
///     C
///           CALL EKOPW ( EKNAME, HANDLE )
///
///     C
///     C     Negate the values in the odd-numbered records
///     C     using the update routines.
///     C
///           DO I = 1, NROWS, 2
///
///              RECNO  = I+1
///
///              ISNULL = ( I .EQ. 1 )
///
///              CALL INTSTR ( -I, CVALS(1) )
///              CALL EKUCEC ( HANDLE, SEGNO, RECNO, CNAMES(1),
///          .                 1,      CVALS, ISNULL           )
///
///     C
///     C        Array-valued columns follow.
///     C
///              CALL INTSTR ( -10*I,       CVALS(1) )
///              CALL INTSTR ( -(10*I + 1), CVALS(2) )
///              CALL INTSTR ( -(10*I + 2), CVALS(3) )
///              CALL INTSTR ( -(10*I + 3), CVALS(4) )
///              CALL EKUCEC ( HANDLE, SEGNO, RECNO, CNAMES(2),
///          .                 4,      CVALS, ISNULL           )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the created file. Show the values added.
///     C
///           CALL EKOPR ( EKNAME, HANDLE )
///
///           DO I = 1, NROWS
///
///              CALL EKRCEC ( HANDLE, SEGNO, I,     CNAMES(1),
///          .                 NVALS,  CVALS, ISNULL           )
///
///              IF ( .NOT. ISNULL ) THEN
///
///                 WRITE(*,*) 'Data from column: ', CNAMES(1)
///                 WRITE(*,*) '   record number: ', I
///                 WRITE(*,*) '   values       : ',
///          .                                ( CVALS(J), J=1,NVALS )
///                 WRITE(*,*) ' '
///
///              ELSE
///
///                 WRITE(*,*) 'Record ', I, 'flag is NULL.'
///                 WRITE(*,*) ' '
///
///              END IF
///
///     C
///     C        Array-valued columns follow.
///     C
///              CALL EKRCEC ( HANDLE, SEGNO, I,     CNAMES(2),
///          .                 NVALS,  CVALS, ISNULL           )
///
///              IF ( .NOT. ISNULL ) THEN
///
///                 WRITE(*,*) 'Data from column: ', CNAMES(2)
///                 WRITE(*,*) '   record number: ', I
///                 WRITE(*,*) '   values       : ',
///          .                                ( CVALS(J), J=1,NVALS )
///                 WRITE(*,*) ' '
///
///              END IF
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
///      Data from column: CHR_COL_1
///         record number:            1
///         values       : 0
///
///      Data from column: CHR_COL_2
///         record number:            1
///         values       : 0        1        2        3
///
///      Record            2 flag is NULL.
///
///      Data from column: CHR_COL_1
///         record number:            3
///         values       : 2
///
///      Data from column: CHR_COL_2
///         record number:            3
///         values       : 20       21       22       23
///
///      Data from column: CHR_COL_1
///         record number:            4
///         values       : -3
///
///      Data from column: CHR_COL_2
///         record number:            4
///         values       : -30      -31      -32      -33
///
///      Data from column: CHR_COL_1
///         record number:            5
///         values       : 4
///
///      Data from column: CHR_COL_2
///         record number:            5
///         values       : 40       41       42       43
///
///      Data from column: CHR_COL_1
///         record number:            6
///         values       : -5
///
///      Data from column: CHR_COL_2
///         record number:            6
///         values       : -50      -51      -52      -53
///
///
///     Note that the second record does not appear due to setting the
///     ISNULL flag to true for that record. The odd value record
///     numbers have negative values as a result of the update calls.
///
///     After run completion, a new EK exists in the output directory.
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
/// -    SPICELIB Version 1.2.1, 06-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 1.2.0, 06-FEB-2015 (NJB)
///
///         Now uses ERRHAN to insert DAS file name into
///         long error messages.
///
///         Corrected some header comment errors (cut-and-paste
///         errors referring to double precision or time data).
///
/// -    SPICELIB Version 1.1.0, 20-JUN-1999 (WLT)
///
///         Removed unbalanced call to CHKOUT.
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn ekucec(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: &mut i32,
    recno: i32,
    column: &str,
    nvals: i32,
    cvals: CharArray,
    isnull: bool,
) -> crate::Result<()> {
    EKUCEC(
        handle,
        segno,
        recno,
        column.as_bytes(),
        nvals,
        cvals,
        isnull,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKUCEC ( EK, update character column entry )
pub fn EKUCEC(
    HANDLE: i32,
    SEGNO: &mut i32,
    RECNO: i32,
    COLUMN: &[u8],
    NVALS: i32,
    CVALS: CharArray,
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CVALS = DummyCharArray::new(CVALS, None, 1..);
    let mut COLDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut CLASS: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut RECPTR: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut ISSHAD: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    // First step:  find the descriptor for the named segment.  Using
    // this descriptor, get the column descriptor.
    //
    ZZEKSDSC(HANDLE, *SEGNO, SEGDSC.as_slice_mut(), ctx)?;
    ZZEKCDSC(
        HANDLE,
        SEGDSC.as_slice(),
        COLUMN,
        COLDSC.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // This column had better be of character type.
    //
    DTYPE = COLDSC[TYPIDX];

    if (DTYPE != CHR) {
        CHKIN(b"EKUCEC", ctx)?;
        SETMSG(b"Column # is of type #; EKUCEC only works with character columns.  RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"EKUCEC", ctx)?;
        return Ok(());
    }

    //
    // Look up the record pointer for the target record.
    //
    ZZEKTRDP(HANDLE, SEGDSC[RTIDX], RECNO, &mut RECPTR, ctx)?;

    //
    // Determine whether the EK is shadowed.
    //
    EKSHDW(HANDLE, &mut ISSHAD);

    //
    // If the EK is shadowed, we must back up the current column entry
    // if the entry has not already been backed up.  ZZEKRBCK will
    // handle this task.
    //
    if ISSHAD {
        ZZEKRBCK(
            b"UPDATE",
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECNO,
        );
    }

    //
    // Now it's time to carry out the replacement.
    //
    CLASS = COLDSC[CLSIDX];

    if (CLASS == 3) {
        //
        // Class 3 columns contain scalar character data.
        //
        ZZEKUE03(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            CVALS.first(),
            ISNULL,
            ctx,
        )?;
    } else if (CLASS == 6) {
        //
        // Class 6 columns contain array-valued character data.
        //
        ZZEKUE06(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            NVALS,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
    } else {
        //
        // This is an unsupported character column class.
        //
        *SEGNO = SEGDSC[SNOIDX];

        CHKIN(b"EKUCEC", ctx)?;
        SETMSG(b"Class # from input column descriptor is not a supported character class.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"EKUCEC", ctx)?;
        return Ok(());
    }

    Ok(())
}
