//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;

/// DLA, same segment?
///
/// Return a logical value indicating whether a two DLA
/// segments, each identified by DAS handle and DLA descriptor,
/// are in fact the same segment.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
/// * [DLA](crate::required_reading::dla)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HAN1       I   Handle of an open DLA file.
///  HAN2       I   Handle of a second open DLA file.
///  DSC1       I   DLA descriptor of a segment in the first file.
///  DSC2       I   DLA descriptor of a segment in the second file.
///
///  The function returns .TRUE. if and only if the DLA segments
///  match.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HAN1     is the integer handle associated with a DLA file.
///           The file is open for read access.
///
///  HAN2     is the integer handle associated with a second DLA
///           file. The file is open for read access.
///
///  DSC1     is the DLA descriptor of a segment in the file
///           associated with HAN1.
///
///  DSC2     is the DLA descriptor of a segment in the file
///           associated with HAN2.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if and only if the DLA segments
///  match. The segments are considered to match if and only if the
///  input handles match and all elements of the DLA descriptors
///  match.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any of the inputs are invalid, this routine will
///      fail in an unspecified manner.
/// ```
///
/// # Files
///
/// ```text
///  See description of input arguments HAN1 and HAN2.
/// ```
///
/// # Particulars
///
/// ```text
///  DLA files are built using the DAS low-level format; DLA files are
///  a specialized type of DAS file in which data are organized as a
///  doubly linked list of segments. Each segment's data belong to
///  contiguous components of character, double precision, and integer
///  type.
///
///  This routine supports DLA and DSK routines by enabling
///  them to determine whether a given DLA segment matches one
///  they've previously examined. This may allow such routines
///  to avoid buffering information redundantly.
/// ```
///
/// # Examples
///
/// ```text
///  1)  A typical use of this routine is to enable a subroutine
///      to determine whether a DLA segment identified by a
///      handle and DLA descriptor matches one seen previously.
///      The logic of such a test can be implemented as follows:
///
///
///                SUBROUTINE SUBA ( HANDLE, DLADSC )
///                IMPLICIT NONE
///
///                INCLUDE 'dla.inc'
///
///                INTEGER               HANDLE
///                INTEGER               DLADSC ( * )
///
///          C
///          C     SPICELIB functions
///          C
///                LOGICAL               DLASSG
///                LOGICAL               FAILED
///          C
///          C     Local variables
///          C
///                INTEGER               PRVDSC ( DLADSZ )
///                INTEGER               PRVHAN
///
///          C
///          C     Saved variables
///          C
///                SAVE                  PRVDSC
///                SAVE                  PRVHAN
///
///          C
///          C     Initial values
///          C
///                DATA                  PRVHAN / 0 /
///
///                ...
///
///                IF ( .NOT. DLASSG( HANDLE, PRVHAN,
///               .                   DLADSC, PRVDSC ) ) THEN
///
///                   [Examine segment]
///
///                   IF ( .NOT. FAILED() ) THEN
///          C
///          C           Save values only if no error occurred.
///          C
///                      CALL MOVEI ( DLADSC, DLADSZ, PRVDSC )
///                      PRVHAN = HANDLE
///
///                   END IF
///
///                END IF
///
///                [Normal case]
///
///                ...
///
///                END
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine relies on uniqueness of DAS file handles.
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
/// -    SPICELIB Version 1.0.1, 22-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 19-MAY-2016 (NJB)
/// ```
pub fn dlassg(han1: i32, han2: i32, dsc1: &[i32; 8], dsc2: &[i32; 8]) -> bool {
    let ret = DLASSG(han1, han2, dsc1, dsc2);
    ret
}

//$Procedure DLASSG ( DLA, same segment? )
pub fn DLASSG(HAN1: i32, HAN2: i32, DSC1: &[i32], DSC2: &[i32]) -> bool {
    let DSC1 = DummyArray::new(DSC1, 1..=DLADSZ);
    let DSC2 = DummyArray::new(DSC2, 1..=DLADSZ);
    let mut DLASSG: bool = false;

    //
    // Local variables
    //

    //
    // Give the function an initial value.
    //
    DLASSG = false;

    //
    // If the handles don't match, we're done.
    //
    if (HAN1 != HAN2) {
        return DLASSG;
    }
    //
    // Compare the DLA descriptors. All elements, including pointers,
    // must match in order to have a matching result.
    //
    for I in 1..=DLADSZ {
        if (DSC1[I] != DSC2[I]) {
            return DLASSG;
        }
    }
    //
    // At this point, everything's a match.
    //
    DLASSG = true;

    DLASSG
}
