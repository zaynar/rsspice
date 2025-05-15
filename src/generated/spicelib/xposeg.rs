//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Transpose a matrix, general
///
/// Transpose a matrix of arbitrary size (the matrix
/// need not be square).
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MATRIX     I   Matrix to be transposed.
///  NROW       I   Number of rows of input matrix MATRIX.
///  NCOL       I   Number of columns of input matrix MATRIX.
///  XPOSEM     O   Transposed matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MATRIX   is a matrix to be transposed.
///
///  NROW     is the number of rows of input matrix MATRIX.
///
///  NCOL     is the number of columns of input matrix MATRIX.
/// ```
///
/// # Detailed Output
///
/// ```text
///  XPOSEM   is the transposed matrix.
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
///  This routine transposes the input matrix MATRIX and writes the
///  result to the matrix XPOSEM. This algorithm is performed in
///  such a way that the transpose can be performed in place. That
///  is, MATRIX and XPOSEM can use the same storage area in memory.
///
///  NOTE:  The matrices MATRIX and XPOSEM are declared
///         one-dimensional for computational purposes only. The
///         calling program should declare them as MATRIX(NROW,NCOL)
///         and XPOSEM(NCOL,NROW).
///
///         This routine works on the assumption that the input and
///         output matrices are defined as described above. More
///         specifically it assures that the elements of the matrix
///         to be transformed are stored in contiguous memory locations
///         as shown here. On output these elements will be
///         rearranged in consecutive memory locations as shown.
///
///            MATRIX              XPOSEM
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
///  been stored as a string, is simply the action of a
///  permutation applied to that string. Since any permutation
///  can be decomposed as a product of disjoint cycles, it is
///  possible to transpose the matrix with only one additional
///  storage register. However, once a cycle has been computed
///  it is necessary to find the next entry in the string that
///  has not been moved by the permutation. For this reason the
///  algorithm is slower than would be necessary if the numbers
///  of rows and columns were known in advance.
/// ```
///
/// # Examples
///
/// ```text
///  This routine is primarily useful when attempting to transpose
///  large matrices, where inplace transposition is important. For
///  example suppose you have the following declarations
///
///      DOUBLE PRECISION      MATRIX ( 1003, 800  )
///
///  If the transpose of the matrix is needed, it may not be
///  possible to fit a second matrix requiring the same storage
///  into memory. Instead declare XPOSEM as below and use
///  an equivalence so that the same area of memory is allocated.
///
///      DOUBLE PRECISION      XPOSEM (  800, 1003 )
///      EQUIVALENCE         ( MATRIX (1,1), XPOSEM(1,1) )
///
///  To obtain the transpose simply execute
///
///      CALL XPOSEG ( MATRIX, 1003, 800, XPOSEM )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.2.3, 22-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.2.2, 04-MAY-1993 (HAN)
///
///         The example listed arguments in the call to XPOSEG incorrectly.
///         The number of rows was listed as the number of columns, and
///         the number of columns was listed as the number of rows.
///
/// -    SPICELIB Version 1.2.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.2.0, 06-AUG-1990 (WLT)
///
///         The original version of the routine had a bug. It worked
///         in place, but the fixed points (1,1) and (n,m) were not
///         moved so that the routine did not work if input and output
///         matrices were different.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (NJB)
/// ```
pub fn xposeg(matrix: &[f64], nrow: i32, ncol: i32, xposem: &mut [f64]) {
    XPOSEG(matrix, nrow, ncol, xposem);
}

//$Procedure XPOSEG ( Transpose a matrix, general )
pub fn XPOSEG(MATRIX: &[f64], NROW: i32, NCOL: i32, XPOSEM: &mut [f64]) {
    let MATRIX = DummyArray::new(MATRIX, 0..);
    let mut XPOSEM = DummyArrayMut::new(XPOSEM, 0..);
    let mut TEMP: f64 = 0.0;
    let mut SOURCE: f64 = 0.0;
    let mut DEST: i32 = 0;
    let mut MOVED: i32 = 0;
    let mut START: i32 = 0;
    let mut K: i32 = 0;
    let mut NMOVES: i32 = 0;
    let mut R: i32 = 0;
    let mut M: i32 = 0;
    let mut N: i32 = 0;

    //
    // Local Variables
    //

    //
    // Take care of dumb cases first.
    //
    if ((NROW <= 0) || (NCOL <= 0)) {
        return;
    }

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
            XPOSEM[DEST] = SOURCE;
            SOURCE = TEMP;

            MOVED = (MOVED + 1);
            K = (DEST / N);
            R = (DEST - (K * N));
            DEST = ((M * R) + K);
        }

        XPOSEM[DEST] = SOURCE;
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

    //
    // Just in case this isn't an in-place transpose, move the last
    // element of MATRIX to XPOSEM
    //
    XPOSEM[0] = MATRIX[0];
    XPOSEM[((N * M) - 1)] = MATRIX[((N * M) - 1)];
}
