//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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
const MXCLSG: i32 = 100;
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
const NTYPES: i32 = 4;
const SHORT: i32 = 4;

struct SaveVars {
    TYPSTR: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TYPSTR = ActualCharArray::new(SHORT, 1..=NTYPES);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CHR"),
                Val::C(b"DP"),
                Val::C(b"INT"),
                Val::C(b"TIME"),
            ]
            .into_iter();
            TYPSTR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { TYPSTR }
    }
}

/// EK, return segment summary
///
/// Return summary information for a specified segment in a
/// specified EK.
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
///  HANDLE     I   Handle of EK.
///  SEGNO      I   Number of segment to be summarized.
///  TABNAM     O   Name of table containing segment.
///  NROWS      O   Number of rows in segment.
///  NCOLS      O   Number of columns in segment.
///  CNAMES     O   Names of columns in segment.
///  DTYPES     O   Data types of columns in segment.
///  SIZES      O   Entry sizes of columns in segment.
///  STRLNS     O   String lengths of columns in segment.
///  INDEXD     O   Flags indicating whether columns are indexed.
///  NULLOK     O   Flags indicating whether columns allow nulls.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is an EK file handle specifying the EK containing
///           the segment to be summarized.
///
///  SEGNO    is the number of the segment whose summary is
///           desired. Segments are numbered from 1 to NSEG,
///           where NSEG is the count of segments in the file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABNAM   is the name of the table to which the segment
///           belongs.
///
///  NROWS    is the number of rows in the segment.
///
///  NCOLS    is the number of columns in the segment. The
///           maximum number of columns in a segment is given
///           by the parameter MXCLSG, which is defined in the
///           include file
///
///              ekglimit.inc.
///
///           Currently, this limit is set at 100 columns.
///
///  CNAMES   is an array of names of columns in the segment.
///
///  DTYPES   is an array of data types of columns in the
///           segment. Each data type is indicated by a short
///           character string. The strings and their meanings
///           are:
///
///              'CHR'       Character type.
///              'DP'        Double precision type.
///              'INT'       Integer type.
///              'TIME'      Time type.
///
///           The Ith element of DTYPES corresponds to the
///           column whose name is the Ith element of CNAMES.
///
///  SIZES    is an array of declared sizes of column entries.
///           The Ith element of SIZES is the declared size of
///           the column whose name is the Ith element of CNAMES.
///           Scalar-valued columns have size 1; fixed-size,
///           array-valued columns have size greater than 1.
///           Array valued columns of variable size have a size
///           value of -1.
///
///  STRLNS   is an array of declared string lengths of
///           character column entries. These lengths are
///           defined only for columns of character type.
///           The Ith element of SIZES is the declared size of
///           the column whose name is the Ith element of CNAMES,
///           if that column has character type; otherwise, the
///           Ith element of STRLNS is undefined. For
///           character columns having variable string length,
///           the returned value of STRLNS is -1.
///
///  INDEXD   is an array of logical flags indicating whether the
///           corresponding columns are indexed. The Ith element
///           of INDEXD applies to the column whose name is the
///           Ith element of CNAMES.
///
///  NULLOK   is an array of logical flags indicating whether the
///           corresponding columns allow null values. The Ith
///           element of NULLOK applies to the column whose name
///           is the Ith element of CNAMES.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE is invalid, an error is signaled by a routine in the
///      call tree of this routine. The output arguments will not be
///      modified.
///
///  2)  If SEGNO is not the index of an existing segment in the
///      specified file, the error SPICE(INDEXOUTOFRANGE) is signaled.
///      The output arguments will not be modified.
///
///  3)  If an I/O error occurs while attempting to obtain summary
///      information for the specified segment, the error is signaled
///      by a routine in the call tree of this routine. The output
///      arguments may be modified in this case.
/// ```
///
/// # Files
///
/// ```text
///  See the description of HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine supports the function of summarizing a binary
///  EK file, allowing NAIF Toolkit users to determine whether it
///  contains data of interest. The routine also also provides
///  address information necessary to retrieve information from the
///  segment.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Dump the table and column names of the segments in an EK.
///
///         C
///         C     Open the EK for read access and get the number of
///         C     segments it contains.
///         C
///               CALL EKOPR ( EKNAME, HANDLE )
///
///               NSEG = EKNSEG ( HANDLE )
///
///         C
///         C     Loop through the segments, dumping the desired
///         C     summary information for each one.
///         C
///               WRITE (*,*) ' '
///               WRITE (*,*) ' '
///               WRITE (*,*) 'Segment summary for file ', EKNAME
///               WRITE (*,*) ' '
///               WRITE (*,*) ' '
///
///               DO I = 1, NSEG
///
///                  CALL EKSSUM (  HANDLE,  SEGNO,   TABNAM,  NROWS,
///              .                  NCOLS,   CNAMES,  DTYPES,  SIZES,
///              .                  STRLNS,  INDEXD,  NULLOK         )
///
///                  WRITE (*,*)
///              .   '========================================'      //
///              .   '========================================'
///
///
///                  WRITE (*,*) 'Table containing segment: ', TABNAM
///
///                  WRITE (*,*) ' '
///                  WRITE (*,*) 'Number of rows:     ', NROWS
///                  WRITE (*,*) 'Number of columns:  ', NCOLS
///                  WRITE (*,*) ' '
///                  WRITE (*,*) 'Column names and attributes: '
///                  WRITE (*,*) ' '
///
///                  DO J = 1, NCOLS
///
///                     WRITE (*,*) 'Column:   '//CNAMES(J)
///                     WRITE (*,*) ' '
///                     WRITE (*,*) 'Data type: ', DTYPES(J)
///                     WRITE (*,*) 'Dimension: ', SIZES(J)
///
///                     IF ( DTYPES(J) .EQ. 'CHR' ) THEN
///                        WRITE (*,*) 'String length: ', STRLNS(J)
///                     END IF
///
///                     IF ( INDEXD(J) ) THEN
///                        WRITE (*,*) 'Indexed'
///                     END IF
///
///                     IF ( NULLOK(J) ) THEN
///                        WRITE (*,*) 'Nulls allowed'
///                     ELSE
///                        WRITE (*,*) 'Nulls not allowed'
///                     END IF
///
///                     WRITE (*,*) ' '
///                  END DO
///
///                  WRITE (*,*)
///              .   '========================================'      //
///              .   '========================================'
///
///               END DO
///
///               END
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 02-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed previous
///         version number.
///
/// -    SPICELIB Version 1.1.0, 07-JUL-1996 (NJB)
///
///         Bug fix: correct parameter is now used to set dimension
///         of local variable SEGDSC.
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 07-JUL-1996 (NJB)
///
///         Bug fix: correct parameter SDSCSZ is now used to set dimension
///         of local variable SEGDSC. Previously, the parameter
///         CDSCSZ had been used.
/// ```
pub fn ekssum(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: i32,
    tabnam: &mut str,
    nrows: &mut i32,
    ncols: &mut i32,
    cnames: CharArrayMut,
    dtypes: CharArrayMut,
    sizes: &mut [i32],
    strlns: &mut [i32],
    indexd: &mut [bool],
    nullok: &mut [bool],
) -> crate::Result<()> {
    EKSSUM(
        handle,
        segno,
        fstr::StrBytes::new(tabnam).as_mut(),
        nrows,
        ncols,
        cnames,
        dtypes,
        sizes,
        strlns,
        indexd,
        nullok,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKSSUM ( EK, return segment summary )
pub fn EKSSUM(
    HANDLE: i32,
    SEGNO: i32,
    TABNAM: &mut [u8],
    NROWS: &mut i32,
    NCOLS: &mut i32,
    CNAMES: CharArrayMut,
    DTYPES: CharArrayMut,
    SIZES: &mut [i32],
    STRLNS: &mut [i32],
    INDEXD: &mut [bool],
    NULLOK: &mut [bool],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CNAMES = DummyCharArrayMut::new(CNAMES, None, 1..);
    let mut DTYPES = DummyCharArrayMut::new(DTYPES, None, 1..);
    let mut SIZES = DummyArrayMut::new(SIZES, 1..);
    let mut STRLNS = DummyArrayMut::new(STRLNS, 1..);
    let mut INDEXD = DummyArrayMut::new(INDEXD, 1..);
    let mut NULLOK = DummyArrayMut::new(NULLOK, 1..);
    let mut CDSCRS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=MXCLSG);
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);

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
    } else {
        CHKIN(b"EKSSUM", ctx)?;
    }

    //
    // Get the info from a knowledgeable source.
    //
    ZZEKSINF(
        HANDLE,
        SEGNO,
        TABNAM,
        SEGDSC.as_slice_mut(),
        CNAMES.as_arg_mut(),
        CDSCRS.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"EKSSUM", ctx)?;
        return Ok(());
    }

    *NROWS = SEGDSC[NRIDX];
    *NCOLS = SEGDSC[NCIDX];

    for I in 1..=*NCOLS {
        fstr::assign(DTYPES.get_mut(I), save.TYPSTR.get(CDSCRS[[TYPIDX, I]]));

        SIZES[I] = CDSCRS[[SIZIDX, I]];

        if (CDSCRS[[TYPIDX, I]] == CHR) {
            STRLNS[I] = CDSCRS[[LENIDX, I]];
        } else {
            STRLNS[I] = 0;
        }

        INDEXD[I] = (CDSCRS[[IXTIDX, I]] != IFALSE);
        NULLOK[I] = (CDSCRS[[NFLIDX, I]] != IFALSE);
    }

    CHKOUT(b"EKSSUM", ctx)?;
    Ok(())
}
