//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Transpose a matrix, general dimension, in place
///
/// Transpose a matrix of arbitrary size and shape in place.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NROW       I   Number of rows of input matrix.
///  NCOL       I   Number of columns of input matrix.
///  MATRIX    I-O  Matrix to be transposed/transposed matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NROW     is the number of rows of input matrix MATRIX.
///
///  NCOL     is the number of columns of input matrix MATRIX.
///
///  MATRIX   is a matrix to be transposed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MATRIX   is the transposed matrix: element (i,j) of the input
///           matrix is element (j,i) of the output matrix.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If either NROW or NCOL is less than or equal to zero, no
///      action is taken. The routine simply returns.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine replaces the input matrix MATRIX with its transpose.
///
///  NOTE:  The matrix MATRIX is declared one-dimensional for
///         computational purposes only. The calling program may
///         declare it as MATRIX(NROW,NCOL) or MATRIX(NCOL,NROW).
///
///         This routine assumes that the elements of the matrix to be
///         transformed are stored in contiguous memory locations as
///         shown here. On output these elements will be rearranged
///         in consecutive memory locations as shown.
///
///            MATRIX on input      MATRIX on output
///
///             m_11                m_11
///             m_21                m_12
///             m_31                m_13
///              .                   .
///              .                   .
///              .                  m_1ncol
///             m_nrow1             m_21
///             m_12                m_22
///             m_22                m_23
///             m_32                 .
///              .                   .
///              .                  m_2ncol
///              .                   .
///             m_nrow2
///              .                   .
///
///              .                   .
///
///              .                   .
///             m_1ncol
///             m_2ncol             m_nrow1
///             m_3ncol             m_nrow2
///              .                  m_nrow3
///              .                   .
///              .                   .
///             m_nrowncol          m_nrowncol
///
///
///  For those familiar with permutations, this algorithm relies
///  upon the fact that the transposition of a matrix, which has
///  been stored as a 1-dimensional array, is simply the action of a
///  permutation applied to that array. Since any permutation
///  can be decomposed as a product of disjoint cycles, it is
///  possible to transpose the matrix with only one additional
///  storage register. However, once a cycle has been computed
///  it is necessary to find the next entry in the array that
///  has not been moved by the permutation. For this reason the
///  algorithm is slower than would be necessary if the numbers
///  of rows and columns were known in advance.
/// ```
///
/// # Examples
///
/// ```text
///  This routine is provided for situation where it is convenient to
///  transpose a general two-dimensional matrix
///  in place rather than store the result in a
///  separate array. Note that the call
///
///     CALL XPOSEG ( MATRIX, NROW, NCOL, MATRIX )
///
///  is not permitted by the ANSI Fortran 77 standard; this routine
///  can be called instead to achieve the same result.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
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
/// -    SPICELIB Version 1.0.0, 19-SEP-2006 (EDW)
///
///         Initial version date unknown. Version data entry
///         added this date.
/// ```
pub fn xpsgip(nrow: i32, ncol: i32, matrix: &mut [f64]) {
    XPSGIP(nrow, ncol, matrix);
}

//$Procedure XPSGIP ( Transpose a matrix, general dimension, in place )
pub fn XPSGIP(NROW: i32, NCOL: i32, MATRIX: &mut [f64]) {
    let mut MATRIX = DummyArrayMut::new(MATRIX, 0..);
    let mut SOURCE: f64 = 0.0;
    let mut TEMP: f64 = 0.0;
    let mut DEST: i32 = 0;
    let mut K: i32 = 0;
    let mut M: i32 = 0;
    let mut MOVED: i32 = 0;
    let mut N: i32 = 0;
    let mut NMOVES: i32 = 0;
    let mut R: i32 = 0;
    let mut START: i32 = 0;

    //
    // Local Variables
    //

    //
    // Take care of dumb cases first.
    //
    if ((NROW <= 0) || (NCOL <= 0)) {
        return;
    }

    //
    // Use the abbreviations N and M for NROW and NCOL.
    //
    N = NROW;
    M = NCOL;

    //
    // Set up the upper bound for the number of objects to be moved and
    // initialize the counters.
    //
    NMOVES = ((N * M) - 2);
    MOVED = 0;
    START = 1;

    //
    // Until MOVED is equal to NMOVES, there is some matrix element that
    // has not been moved to its proper location in the transpose matrix.
    //
    while (MOVED < NMOVES) {
        SOURCE = MATRIX[START];
        K = (START / N);
        R = (START - (N * K));
        DEST = ((R * M) + K);

        //
        // Perform this cycle of the permutation.  We will be done when
        // the destination of the next element is equal to the starting
        // position of the first element to be moved in this cycle.
        //
        while (DEST != START) {
            TEMP = MATRIX[DEST];
            MATRIX[DEST] = SOURCE;
            SOURCE = TEMP;

            MOVED = (MOVED + 1);
            K = (DEST / N);
            R = (DEST - (K * N));
            DEST = ((M * R) + K);
        }

        MATRIX[DEST] = SOURCE;
        DEST = 0;
        MOVED = (MOVED + 1);

        //
        // Find the next element of the matrix that has not already been
        // moved by the transposition operation.
        //
        if (MOVED < NMOVES) {
            while (DEST != START) {
                START = (START + 1);
                K = (START / N);
                R = (START - (K * N));
                DEST = ((R * M) + K);

                while (DEST > START) {
                    K = (DEST / N);
                    R = (DEST - (K * N));
                    DEST = ((M * R) + K);
                }
            }
        }
    }
}
