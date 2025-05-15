//! Constants
//!
//! ```text
//! C
//! C     Include file zzsclk01.inc
//! C
//! C     SPICE private file intended solely for the support of SPICE
//! C     routines. Users should not include this file directly due
//! C     to the volatile nature of this file.
//! C
//! C     Include file for SPICE-private declarations of SC01.
//! C
//! C-    Version 1.0.0, 11-NOV-2020
//! C
//!
//! C
//! C    Parameters copied from sclk.inc:
//! C
//! C        NOTE: these declarations are duplicated in the include file
//! C        sclk.inc. Those declarations must be kept in sync with
//! C        these ones.
//! C
//! C
//! C     Number of supported SCLK field delimiters:
//! C
//!       INTEGER               NDELIM
//!       PARAMETER           ( NDELIM = 5 )
//!
//! C
//! C     Supported SCLK string field delimiters:
//! C
//!       CHARACTER*(NDELIM)    DELIMS
//!       PARAMETER           ( DELIMS = '.:-, ' )
//!
//! C
//! C     Time system codes
//! C
//!       INTEGER               TDB
//!       PARAMETER           ( TDB    =      1 )
//!
//!       INTEGER               TDT
//!       PARAMETER           ( TDT    =      2 )
//!
//! C
//! C     Maximum number of partitions:
//! C
//!       INTEGER               MXPART
//!       PARAMETER           ( MXPART = 9999 )
//!
//! C
//! C     Maximum number of coefficient records:
//! C
//!       INTEGER               MXCOEF
//!       PARAMETER           ( MXCOEF = 100000 )
//!
//! C
//! C     Maximum number of fields in an SCLK string:
//! C
//!       INTEGER               MXNFLD
//!       PARAMETER           ( MXNFLD = 10 )
//!
//! C
//! C     Length of strings used to represent D.P. numbers:
//! C
//!       INTEGER               DPLEN
//!       PARAMETER           ( DPLEN = 30 )
//!
//! C
//! C     End of duplicated declarations.
//! C
//!
//! C
//! C     Indices of integer data items, relative to base index into
//! C     the integer data buffer:
//! C
//! C
//! C     Index of number of fields
//! C
//!       INTEGER               IXNFLD
//!       PARAMETER           ( IXNFLD = 1 )
//!
//! C
//! C     Index of delimiter code
//! C
//!       INTEGER               IXDLIM
//!       PARAMETER           ( IXDLIM = IXNFLD + 1 )
//! C
//! C     Index of time system code
//! C
//!       INTEGER               IXTSYS
//!       PARAMETER           ( IXTSYS = IXDLIM + 1 )
//!
//! C
//! C     Index of coefficient count. The count is 3 x the number of
//! C     coefficient records.
//! C
//!       INTEGER               IXNCOF
//!       PARAMETER           ( IXNCOF = IXTSYS + 1 )
//!
//! C
//! C     Index of the number of partitions
//! C
//!       INTEGER               IXNPRT
//!       PARAMETER           ( IXNPRT = IXNCOF + 1 )
//!
//! C
//! C     Index of the base index in the double precision buffer of the
//! C     coefficient set
//! C
//!       INTEGER               IXBCOF
//!       PARAMETER           ( IXBCOF = IXNPRT + 1 )
//!
//! C
//! C     Index of the base index in the double precision buffer of the
//! C     partition start times
//! C
//!       INTEGER               IXBSTR
//!       PARAMETER           ( IXBSTR = IXBCOF + 1 )
//!
//! C
//! C     Index of the base index in the double precision buffer of the
//! C     partition end times
//! C
//!       INTEGER               IXBEND
//!       PARAMETER           ( IXBEND = IXBSTR + 1 )
//!
//! C
//! C     Index of the base index in the double precision buffer of the
//! C     SCLK field moduli
//! C
//!       INTEGER               IXBMOD
//!       PARAMETER           ( IXBMOD  = IXBEND + 1 )
//!
//! C
//! C     Index of the base index in the double precision buffer of the
//! C     SCLK field offsets
//! C
//!       INTEGER               IXBOFF
//!       PARAMETER           ( IXBOFF   = IXBMOD + 1 )
//!
//! C
//! C     Number of integer values per clock
//! C
//!       INTEGER               NIVALS
//!       PARAMETER           ( NIVALS = IXBOFF )
//!
//!
//! C
//! C     Data structure parameters
//! C
//!
//! C
//! C     Maximum number of clocks
//! C
//!       INTEGER               MXNCLK
//!       PARAMETER           ( MXNCLK = 100 )
//!
//! C
//! C     DP buffer size
//! C
//! C     The buffer is large enough to hold data for one clock having the
//! C     maximum amount of data.
//! C
//!       INTEGER               DBUFSZ
//!       PARAMETER           ( DBUFSZ =   ( 3 * MXCOEF )
//!      .                               + ( 2 * MXPART )
//!      .                               + ( 2 * MXNFLD ) )
//!
//! C
//! C     Integer buffer size
//! C
//!       INTEGER               IBUFSZ
//!       PARAMETER           ( IBUFSZ = NIVALS * MXNCLK )
//!
//! C
//! C     Lower bound of control area of singly linked list:
//! C
//!       INTEGER               LBSNGL
//!       PARAMETER           ( LBSNGL = -5 )
//!
//! C
//! C     The add-only hash pool for frame IDs is singly linked.
//! C
//!       INTEGER               CPLSIZ
//!       PARAMETER           ( CPLSIZ = MXNCLK - LBSNGL + 1 )
//!
//! C
//! C     End include file zzsc01.inc
//! C
//! ```

pub const NDELIM: i32 = 5;
pub const DELIMS: &str = ".:-, ";
pub const TDB: i32 = 1;
pub const TDT: i32 = 2;
pub const MXPART: i32 = 9999;
pub const MXCOEF: i32 = 100000;
pub const MXNFLD: i32 = 10;
pub const DPLEN: i32 = 30;
pub const IXNFLD: i32 = 1;
pub const IXDLIM: i32 = (IXNFLD + 1);
pub const IXTSYS: i32 = (IXDLIM + 1);
pub const IXNCOF: i32 = (IXTSYS + 1);
pub const IXNPRT: i32 = (IXNCOF + 1);
pub const IXBCOF: i32 = (IXNPRT + 1);
pub const IXBSTR: i32 = (IXBCOF + 1);
pub const IXBEND: i32 = (IXBSTR + 1);
pub const IXBMOD: i32 = (IXBEND + 1);
pub const IXBOFF: i32 = (IXBMOD + 1);
pub const NIVALS: i32 = IXBOFF;
pub const MXNCLK: i32 = 100;
pub const DBUFSZ: i32 = (((3 * MXCOEF) + (2 * MXPART)) + (2 * MXNFLD));
pub const IBUFSZ: i32 = (NIVALS * MXNCLK);
pub const LBSNGL: i32 = -5;
pub const CPLSIZ: i32 = ((MXNCLK - LBSNGL) + 1);
