//! #  Common Problems
//!
//!  
//!
//! ```text
//!    Failure is not an option.
//!  
//!     -- "Apollo 13"
//! ```
//!
//!  Last revised on 2017 MAR 14 by E. D. Wright
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  Descriptions of the more commonly encountered SPICE problems, broken
//!    down into functional areas with suggestions on how to avoid problems.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  While NAIF strives to make correct use of SPICE an effortless
//!    experience, more remains to be done. NAIF's decade of experience with
//!    SPICE customers shows certain problems seem to recur fairly regularly.
//!    This document aims to assist you in preventing these problems, or if
//!    necessary, troubleshooting them.
//!
//!  Most of this document is concerned with matching symptoms to possible
//!    causes and solutions. However, before starting to write a SPICE-based
//!    application, we strongly encourage you to consider the steps necessary
//!    to avoid problems.
//!
//!  
//!
//!
//!  
//! ##  Getting it Right
//!
//!  It's generally much easier and quicker to make sure you're doing things
//!    right in the first place than it is to explain why your program isn't
//!    behaving as expected. For best results, carefully ascertain you have the
//!    proper input data and problem definition before proceeding.
//!
//!  One of the most frequently occurring user-support questions fielded by
//!    NAIF is basically "I compared my results from SPICE with \[an alternate
//!    source], and they disagree. Why?" The answer usually is the two
//!    computations are for some unintended reason solving different problems.
//!
//!  Here's a checklist of things to get right before embarking on solving a
//!    problem with SPICE, or comparing SPICE results with those obtained from
//!    alternate sources.
//!
//!  
//!
//! *  1. Read the directions: read the relevant SPICE documentation, both subroutine
//! headers and Required Reading files. SPICE software interfaces aspire to be
//! intuitively clear, but may fall short.
//!
//!  *  2. Understand the definitions: many geometric quantities have a variety of
//! definitions. For example, there are a variety of ways of computing a
//! sub-observer point, involving both different applications of light time
//! corrections and different definitions of "sub-point" (closest point to
//! the observer vs. surface intercept of observer-target center vector).
//!
//!  *  3. Understand the expected accuracy and precision: for example, the
//! Astronomical Almanac frequently presents results having claimed accuracy of
//! 0.01 degree. These cannot be expected to agree with SPICE results at the
//! arcsecond level. On the other hand, if your application requires
//! extraordinarily high accuracy, you may need to check whether SPICE meets
//! your requirements.
//!
//!  *  4. Use the right files: this is probably the single most frequent cause of
//! disagreements. You MUST use the same ephemeris, rotational elements, shape
//! models, pointing, instrument models, frame definitions, leapseconds,
//! spacecraft clock coefficients, and so forth if you're attempting to match
//! results from alternative computations.
//!
//!  *  5. Use the right time: another major cause of disagreements. Is the input time
//! supposed to be UTC, "ET" or TDB (barycentric dynamical time), TDT
//! (terrestrial dynamical time), TAI, etc.? Frequently calendar dates and
//! Julian dates are written down without specification of whether they're UTC
//! or TDB, even though the distinction is critical for high-accuracy work.
//!
//!  *  In order for SPICE-based time conversions to be correct, current
//! leapseconds and/or SCLK (spacecraft clock) kernels are essential.
//!
//!  *  6. Use the right reference frame: state vectors will not compare well if
//! they're specified relative to different reference frames. "EME50" frames
//! are problematic because there are a variety of similar but non-identical
//! frames all designated by this name. Examples are: B1950, FK4, DE-125.
//! Implementations of body-fixed frames may differ as well. Also, some
//! extended bodies have a variety of frames associated with them: Jupiter
//! system I vs. system III for example.
//!
//!  *  7. Use the right coordinate system: sometimes disagreements result from
//! mismatched coordinate systems, for example planetocentric vs. planetodetic
//! coordinates.
//!
//!  *  8. Use the right aberration corrections: there can be a large difference
//! between geometric (uncorrected) states and states corrected for light-time
//! or light time and stellar aberration. When computing quantities involving
//! surface locations on extended bodies, whether or not the rotation is
//! retarded by light time is an issue.
//!
//!  *  9. Use the current software: out-of-date Toolkits may contain bugs corrected
//! in the current release.
//!
//!  *  10. When diagnosing a problem, make sure a problem exists. Correct answers are
//! sometimes counterintuitive. Not infrequently, a disagreement between a
//! SPICE result and a "cross-check" result occurs because the latter is not
//! mathematically equivalent to the former.
//!
//!  In the discussion below, it's implicit that any of the problem areas
//!    listed above should be examined whenever you diagnose a failure.
//!
//!  
//!
//!
//!  
//! #  Problems by Functional Area
//!
//!  The functional areas are listed alphabetically.
//!
//!  
//!
//!
//!  
//! ##  Accuracy
//!
//!  While for general applications, SPICE is usually capable of much higher
//!    accuracy than required, for some specialized applications such as radio
//!    science, certain SPICE-based computations may not be sufficiently
//!    accurate.
//!
//!  
//!
//!
//!  
//! ###  Problem: Arithmetic on time values yields incorrect results
//!
//!  Within the SPICE system TDB times are represented as double precision
//!    numbers, and these are not generally accurate to better than 1.E-7
//!    second.
//!
//!  
//!
//!
//!  
//! ###  Problem: States from the SPK and NAVIO systems are not identical
//!
//!  This problem to date has proved illusory.
//!
//!  State vectors obtained from SPK files have been tested and shown to
//!    agree with those obtained from the parent NAVIO files to levels orders
//!    of magnitude below the knowledge error in the data. The differences in
//!    state vectors returned by the two systems tend to reflect round-off
//!    level differences in the handling of time.
//!
//!  
//!
//!
//!  
//! ###  Problem: UTC-TDB conversion in SPICE does not appear accurate
//!
//!  This is not truly a common problem; it has arisen only in the context of
//!    radio science applications.
//!
//!  The UTC-TDB conversion in SPICE is accurate to about 4.E-5 seconds.
//!
//!  
//!
//!
//!  
//! ###  Problem: Light time corrections in SPICE seem to be inaccurate
//!
//!  Versions of the SPICE Toolkit released prior to May, 1995 used an
//!    unnecessarily inaccurate light time computation: they returned the
//!    distance between the geometric positions of observer and target at the
//!    request time, divided by the speed of light. Later versions of SPICE use
//!    the position of the target evaluated at the light-time corrected epoch.
//!
//!  Be aware the SPICE aberration corrections do not account for
//!    relativistic effects.
//!
//!  
//!
//!
//!  
//! ##  Body-Fixed Frames
//!
//!  
//!
//!
//!  
//! ###  Problem: Inertial/Bodyfixed position conversion gives SPICE error
//!
//!  Make sure you're using the correct ID code for the body.
//!
//!  Check your kernel file lists data for the body in question at the epoch
//!    of interest. If you're using a text PCK, this problem occurs because
//!    data is simply absent for the body in question.
//!
//!  Binary PCK files have coverage for limited time spans. Make sure the
//!    request time falls within the coverage interval for the body if a binary
//!    PCK is used. The available coverage may be ascertained by summarizing
//!    the PCK file with SPACIT.
//!
//!  If the rotation of the body is retarded by one-way light time, remember
//!    PCK data for the body must be available at the light-time corrected
//!    epoch.
//!
//!  
//!
//!
//!  
//! ###  Problem: Inertial/Bodyfixed position conversion is incorrect
//!
//!  Rotational models for extended bodies vary. For any given body, the
//!    model and model parameter values may evolve over time, so verify the
//!    version you're using is correct.
//!
//!  Some bodies have rotational models based on different physical
//!    attributes, for example rotation of the magnetic field or rotation of
//!    the atmosphere. Confirm the model you expect is provided by the PCK file
//!    you're using.
//!
//!  The epoch at which the target body's orientation should be evaluated
//!    depends on whether the actual or apparent orientation of the body is to
//!    be computed. Check whether the request epoch should be adjusted for
//!    light time.
//!
//!  
//!
//!
//!  
//! ###  Problem: Inertial/Body-fixed state conversion is incorrect
//!
//!  All of the considerations listed above apply.
//!
//!  Additionally, note that velocity transformations involve the time
//!    derivative of the inertial-to-bodyfixed transformation. If P and V are
//!    inertially-referenced position and velocity vectors, M is the
//!    inertial-to-bodyfixed transformation matrix, and P_b and V_b are the
//!    body-fixed position and velocity, then we have (by the Chain Rule for
//!    derivatives):
//!
//!  
//!
//! ```text
//!    P_b = M*P
//!    V_b = M*V +  (dM/dt)*P
//! ```
//!
//!  Some applications external to SPICE erroneously ignore the second term
//!    in the second equation.
//!
//!  
//!
//!
//!  
//! ##  CK/C-Kernel/C-Matrix/Pointing
//!
//!  
//!
//!
//!  
//! ###  Problem: How does one determine the attributes of a C-kernel?
//!
//!  Use CKBRIEF or SPACIT to summarize the kernel. Depending on your
//!    computer system, you may need to log the output to a file to view it
//!    conveniently.
//!
//!  C-kernel data are contained in a series of one or more chunks called
//!    "segments." SPACIT will output a series of summaries, one for each
//!    segment. SPACIT will tell you what instrument the pointing data is for,
//!    which base frame the pointing is referenced to, whether angular velocity
//!    data are also present in the segment, and the data type (internal
//!    representation) of the segment. The time bounds of each segment are also
//!    shown.
//!
//!  CKBRIEF is a more flexible and robust summary program than SPACIT; you
//!    normally will find CKBRIEF more convenient to use. However, the current
//!    version of CKBRIEF does not output the data types of segments.
//!
//!  
//!
//!
//!  
//! ###  Problem: no pointing found at desired epoch
//!
//!  Check the ID code you're supplying to the CK reader matches that in the
//!    C-kernel. If you're interested in pointing for a spacecraft instrument,
//!    you may need to get pointing for another entity, usually the spacecraft
//!    bus or a scan platform, then apply pointing offsets from a Frame kernel
//!    or Instrument kernel to obtain instrument pointing.
//!
//!  If the ID code is correct, it may be the tolerance value supplied to the
//!    C-kernel reader routine, either [CKGP](crate::raw::ckgp) or [CKGPAV](crate::raw::ckgpav), should be increased. By
//!    increasing the tolerance value you supply to the CK reader called in
//!    your application, you may be able to pick up pointing at a nearby time
//!    sufficiently close to your time of interest.
//!
//!  There may be no nearby pointing due to coverage gaps in the C-kernel,
//!    either between segments or in the interior of some segment. C-kernel
//!    segments, unlike SPK segments, do not necessarily have continuous
//!    coverage. In fact, type 1 C-kernels contain discrete pointing and do not
//!    yield interpolated pointing. C-kernel data types 2-4 have coverage over
//!    a series of time intervals, but there may be gaps between the intervals.
//!
//!  Finally, you may be using the CK wrong reader. The reader [CKGPAV](crate::raw::ckgpav) returns
//!    pointing data only if pointing data AND angular velocity data exists at
//!    the request time. You should use [CKGP](crate::raw::ckgp) if your C-kernel lacks angular
//!    velocity data as [CKGP](crate::raw::ckgp) doesn't require the presence such data.
//!
//!  
//!
//!
//!  
//! ###  Problem: unclear what lookup tolerance to use
//!
//!  The choice of lookup tolerance can be a complex issue involving
//!    trade-offs between accuracy and completing as much data processing as
//!    possible. Choosing a non-zero tolerance means accepting pointing for a
//!    time other than your time of interest. What magnitude of inaccuracy is
//!    introduced by this choice? It depends on how much the structure of
//!    interest can or did move during the tolerance interval. Knowledge of the
//!    spacecraft or structure dynamics may be required to select the maximum
//!    acceptable tolerance. This may vary depending on mission phase, ACS
//!    (attitude control system) state, the specific structure for which
//!    pointing is desired, or other time-dependent factors.
//!
//!  There are a few cases that do admit simple guidelines:
//!
//!  If you're using a type 1 C-kernel (discrete pointing), the tolerance
//!    should normally be at least half the nominal spacing between the
//!    pointing instances. Otherwise, no pointing will be found at request
//!    times near the midpoints between times where pointing is available.
//!
//!  If you're using a C-kernel of type 2, 3, or 4, and you know the data
//!    does not suffer from gaps, you may use a tolerance of zero. This choice
//!    guarantees you'll get pointing interpolated to your request time. A
//!    tolerance of zero is frequently acceptable when using predict pointing,
//!    which normally should not have any coverage anomalies.
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE quaternions appear invalid
//!
//!  See "Quaternions" below.
//!
//!  
//!
//!
//!  
//! ##  Coordinates
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE does not produce expected lat/lon values
//!
//!  See "Body-Fixed Frames" above.
//!
//!  If the Cartesian body-fixed coordinates of a point are as expected,
//!    latitude and longitude may differ because of
//!
//!  
//!
//! * The difference between planetocentric and planetodetic latitude.
//!
//!  * The difference between planetocentric and planetographic latitude or
//! longitude.
//!
//!  * Differences in the range of allowed longitude values.
//!
//!  * Differences in angular units. SPICE uses radians in all coordinate
//! transformation routines.
//!
//!  * For planetodetic or planetographic coordinates, latitude is a function of
//! the equatorial and polar radii of the defining body. Check these values are
//! as expected.
//!
//!     
//! ##  Documentation
//!
//!  
//!
//!
//!  
//! ###  Problem: "I can't find the routine I need"
//!
//!  Use the SPICE permuted index in the /doc subdirectory of your SPICE
//!    Toolkit installation. The permuted index associates short functional
//!    descriptions with names of SPICELIB functions. You can browse this
//!    document or search/grep it for keywords.
//!
//!  Only a small fraction of the routines in SPICELIB tend to be called
//!    directly from users' applications. To familiarize yourself with this
//!    subset of routines, look at the document "Most Used Subroutines" in
//!    the /doc directory of your SPICE Toolkit installation.
//!
//!  To quickly get up to speed in using the elementary features of SPICE,
//!    examine the cookbook programs in the
//!
//!  
//!
//! ```text
//!    /src/cookbook
//! ```
//!
//!  subdirectory of your SPICE installation.
//!
//!  For help in understanding how SPICE routines are combined to solve more
//!    involved problems, examine the code examples in the Required Reading
//!    files in the /doc subdirectory of your Toolkit.
//!
//!  
//!
//!
//!  
//! ##  E-kernel
//!
//!  
//!
//!
//!  
//! ###  Problem: Query takes forever to complete
//!
//!  For queries not involving the ordering of output, this sometimes happens
//!    because inadequate constraints were supplied. Queries on large databases
//!    should employ a WHERE clause to restrict to a manageable size the set of
//!    matching rows. A typical example of a query generating a huge number of
//!    matching rows would be an attempted equi-join with the equi-join
//!    constraint accidentally omitted.
//!
//!  For queries involving ordering on a single column, generating more than
//!    50000 matching rows will guarantee a long wait, because scratch files
//!    will be used to store temporary results.
//!
//!  Queries involving multiple order-by columns are typically slow because
//!    scratch files are used when comparisons are required on columns other
//!    than the primary order-by column.
//!
//!  
//!
//!
//!  
//! ###  Problem: Writing EK file takes forever
//!
//!  Use the "fast loader" routines, which are roughly two orders of
//!    magnitude faster than the record-oriented writers. See the header of
//!    [EKIFLD](crate::raw::ekifld) (EK, initiate fast load) for an example.
//!
//!  
//!
//!
//!  
//! ###  Problem: EK routines signal very mysterious errors
//!
//!  Because of the complexity of the EK system, errors in application calls
//!    to EK routines are sometimes diagnosed at locations very remote from the
//!    original error. Mismatched arguments in subroutine calls are the typical
//!    root cause of these error messages.
//!
//!  
//!
//!
//!  
//! ##  Error Handling
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE errors abort my application program
//!
//!  The default error handling response is to abort the application. This
//!    response can be changed so SPICE routines return on entry. See the Error
//!    Required Reading, [error.req](crate::required_reading::error), or the routine [ERRACT](crate::raw::erract).
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE error messages are written to standard output
//!
//!  The target file for SPICE error messages can be reset. See the Error
//!    Required Reading, [error.req](crate::required_reading::error), or the routine ERRDEV.
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE(NAMESDONOTMATCH) error is displayed
//!
//!  If your application directly calls CHKIN and CHKOUT, unpaired calls to
//!    these routines may result in this error message. Due to recursion
//!    restrictions in FORTRAN, this message does not pass through the normal
//!    SPICE error handling mechanism.
//!
//!  
//!
//!
//!  
//! ###  Problem: "Oh, by the way..." message is annoying
//!
//!  The set of SPICE error messages can be re-configured. Any type of
//!    message (long, short, traceback, default) can be suppressed. See the
//!    Error Required Reading document, [error.req](crate::required_reading::error), or the routine [ERRPRT](crate::raw::errprt).
//!
//!  
//!
//!
//!  
//! ##  Euler Angles
//!
//!  
//!
//!
//!  
//! ###  Problem: [M2EUL](crate::raw::m2eul) or [XF2EUL](crate::raw::xf2eul) don't produce the expected angles
//!
//!  Generally, Euler angles are unique only when their ranges are
//!    appropriately restricted. Otherwise, there are usually multiple
//!    combinations of angles that map to the same rotation matrix.
//!
//!  Review the headers of [M2EUL](crate::raw::m2eul) or [XF2EUL](crate::raw::xf2eul) carefully to determine what output
//!    ranges are used.
//!
//!  
//!
//!
//!  
//! ##  File I/O
//!
//!  
//!
//!
//!  
//! ###  Problem: File open error is signaled from SPICE-based utility
//!
//!  Attempts to open a file for read access will fail if that file does not
//!    exist.
//!
//!  Attempts to open a new file will normally fail if that file exists.
//!
//!  Any open attempt will fail if the application attempting the operation
//!    does not have permission to access the file.
//!
//!  SPICE utilities and some SPICELIB routines use scratch files. Scratch
//!    files are sometimes kept in locations other than the user's current or
//!    home directory. If permission to write to the scratch directory is not
//!    granted, a write error will occur.
//!
//!  SPICE kernel loader routines attempt to diagnose and report errors when
//!    used to open inappropriate kernel types.
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE kernel reads fail within user's application
//!
//!  Read errors can occur when a file is corrupted. They also can occur when
//!    a file is deleted while a SPICE application is attempting a read.
//!
//!  Logical unit conflicts can cause nonsense data to be returned when
//!    reading SPICE kernels. The symptoms can be obscure. See below.
//!
//!  
//!
//!
//!  
//! ###  Problem: Logical unit conflict
//!
//!  Logical unit conflicts are peculiar to FORTRAN applications. However,
//!    they may occur in C applications that link to both CSPICE and other C
//!    code generated by running f2c on FORTRAN source code.
//!
//!  The SPICE kernel loaders allocate logical units at run time. SPICE does
//!    not allocate logical units currently in use.
//!
//!  If a user application loads kernels or otherwise performs actions that
//!    cause SPICE to allocate logical units, and then uses those same logical
//!    units for its own I/O operations, a logical unit conflict exists. SPICE
//!    will then attempt to read from whatever file the application has
//!    connected to the logical units SPICE had allocated. This will typically
//!    result in SPICE reading something other than valid kernel data.
//!
//!  There are a couple of simple solutions. First, if an application is
//!    known to use a particular set of units, you can tell SPICE not to touch
//!    those units by calling RESLUN.
//!
//!  If you are writing a new application, it may be convenient to use [GETLUN](crate::raw::getlun)
//!    to allocate logical units at run time. This avoids hard-coded logical
//!    units, which may cause portability and integration problems.
//!
//!  
//!
//!
//!  
//! ###  Problem: Error occurs when trying to close EK or DAS file
//!
//!  The SPICE DAS system, which underlies the EK system, requires a scratch
//!    file for sorting when a newly written file is closed. Attempting to open
//!    this file could cause a system limit on open files or logical units to
//!    be encountered, resulting in a file open error.
//!
//!  If the application does not have permission to open the scratch file, an
//!    error will be signaled. See also "File I/O."
//!
//!  If the scratch file is opened successfully, it is possible the disk
//!    space will be exhausted while writing to the file. This will result in a
//!    write error.
//!
//!  
//!
//!
//!  
//! ##  File Transfer
//!
//!  
//!
//!
//!  
//! ###  Problem: A text kernel causes a SPICE(INCOMPATIBLEEOL) error
//!
//!  These problems usually occur after accidentally performing a binary ftp
//!    transfer of the text file in question. SPICE use of such files can be
//!    expected to work only if the transfer is between systems having
//!    identical text file formats.
//!
//!  It is also possible the file was corrupted in transfer due to running
//!    out of disk space on the target system during the transfer.
//!
//!  SPICE Toolkits since version N57 include an error check to text file
//!    readers to ensure the files had the correct line terminators for the
//!    platform. NAIF added this check as loading non-native text kernels
//!    proved a continual problem for SPICE users - trying to load a non-native
//!    kernel usually caused no change to the kernel pool state resulting in
//!    unexpected results and confused users.
//!
//!  Most (if not all) modern Microsoft compilers perform an internal
//!    conversion from DOS terminators to Unix terminators, explaining why one
//!    seldom saw this problem on Windows.
//!
//!  As of Toolkit version N59, CSPICE/Icy text kernel loaders perform the
//!    conversion between text line terminators on Windows and Unix platforms;
//!    SPICELIB (the FORTRAN toolkit library) lacks this conversion capability
//!    and so signals INCOMPATIBLEEOL.
//!
//!  If the file being transferred is a NAIF "transfer format" version of a
//!    binary kernel, TOBIN can often diagnose file corruption when attempting
//!    the conversion to binary format.
//!
//!  
//!
//!
//!  
//! ###  Problem: binary kernel imported from a second system does not work
//!
//!  If the file was transferred between two systems with compatible binary
//!    file formats, for example, between HP and Sun workstations, the problem
//!    may be due to accidentally having transferred the file in ASCII rather
//!    than binary ftp mode.
//!
//!  Also, it is possible the disk space was exhausted on the target system
//!    during the transfer.
//!
//!  If the file was transferred between two systems with incompatible binary
//!    file formats, for example an HP workstation and a PC, the problem is
//!    that binary kernels on one system are not designed to work on the other.
//!    The kernel must be converted to transfer format on the source system,
//!    the transfer format file must be transferred in ASCII mode, and the
//!    received transfer file must be converted back to binary format on the
//!    target system. Use TOXFR and TOBIN, respectively, to perform conversions
//!    from binary to transfer format and from transfer format to binary
//!    format. The utility SPACIT is also capable of performing these
//!    conversions. See the User's Guides for any of these utilities --
//!    [tobin.ug](crate::raw::tobin.ug), toxfr.ug, [spacit.ug](crate::raw::spacit.ug) -- for further information.
//!
//!  
//!
//!
//!  
//! ##  Installing the SPICE Toolkit
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE routines don't compile, link, or run
//!
//!  First, the FORTRAN compiler must be capable of being invoked from the
//!    command line in the directory where the installation is done.
//!
//!  On some systems, the compiler may not be installed, or libraries
//!    required by the compiler may not be available.
//!
//!  If your application cannot link against SPICELIB, or it links but does
//!    not run, there may be a compiler or system library version
//!    incompatibility. This problem can occur if you have installed the
//!    SPICELIB library simply by unpacking a delivery tar file, and there is a
//!    discrepancy between the version of your compiler and that used to create
//!    the object modules in the delivery tar file. This problem may be solved
//!    by doing a complete build of the SPICE Toolkit on your system. See the
//!    installation instructions for details.
//!
//!  In general, if you have difficulty building the SPICE Toolkit, it may be
//!    a useful test to see whether you can build a simple "hello world"
//!    program in the same environment. If that test fails, it's time to
//!    consult with your system administrator.
//!
//!  
//!
//!
//!  
//! ##  Linear Algebra
//!
//!  
//!
//!
//!  
//! ###  Problem: Bogus results returned by general-dimension routines
//!
//!  The general-dimension matrix and vector routines, unlike their
//!    3-dimensional counterparts, usually do not permit overwriting input
//!    arguments with output values. Check the subroutine headers for details.
//!
//!  
//!
//!
//!  
//! ##  PCK/Pc-Kernel/Planetary constants
//!
//!  
//!
//!
//!  
//! ###  Problem: PCK file does not contain desired contents
//!
//!  PCK kernels supplied by NAIF normally contain data intended for use by a
//!    general class of users. PCK kernels are text files and can be edited, so
//!    you can easily customize an existing one, deleting unnecessary data,
//!    adding new data, or changing existing values.
//!
//!  If you modify a kernel supplied by NAIF, it's a good idea to comment the
//!    file so as to make clear what changes were made. All NAIF text kernels
//!    allow comments to be inserted. See Kernel Required Reading, [kernel.req](crate::required_reading::kernel),
//!    for further information on the NAIF text kernel format.
//!
//!  
//!
//!
//!  
//! ###  Problem: Earth orientation given by a text PCK is too inaccurate
//!
//!  The SPICELIB PCK system supports binary PCK files that are capable of
//!    supporting high-accuracy rotation models. Currently NAIF has the
//!    capability of producing high-accuracy PCK files for the earth; these
//!    take into account precession, nutation, TAI-UT1, polar motion, and
//!    nutation corrections.
//!
//!  NAIF is also developing the capability of producing high-accuracy PCK
//!    files for the moon.
//!
//!  
//!
//!
//!  
//! ##  Performance
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE-based application is too slow
//!
//!  Loading kernel files in a loop causes slow execution. Normally, kernel
//!    files should be loaded once per program run, usually during
//!    initialization.
//!
//!  Two possible "global" improvements are using compiler optimization and
//!    disabling SPICELIB call tracing.
//!
//!  The FORTRAN library SPICELIB is normally built without using compiler
//!    optimization. On some systems, in particular Sun Sparc machines running
//!    Sun FORTRAN, using optimization has resulted in some code generation
//!    errors. NAIF is working to resolve this problem. CSPICE on the other
//!    hand is always built using compiler optimization. If you have the
//!    opportunity to use either library, using CSPICE may result in a
//!    considerable speed-up.
//!
//!  SPICELIB applications can be somewhat sped up by disabling the routine
//!    call tracing done internally by SPICELIB. This is done by calling TRCOFF
//!    once during program initialization:
//!
//!  
//!
//! ```text
//!    CALL TRCOFF
//! ```
//!
//!  Normally it is desirable to retain SPICELIB's call tracing while an
//!    application is still being debugged. See Error Required Reading, [error.req](crate::required_reading::error), or the header of TRCOFF (located in the umbrella routine
//!    trcpkg) for further information.
//!
//!  It's also possible to achieve speed gains via local code modifications.
//!    Before trying this, it's a good idea to profile your application to
//!    locate bottlenecks.
//!
//!  Often it's possible to re-organize your SPICE calls so as to minimize
//!    the number of expensive operations. There is usually a speed/complexity
//!    trade-off to consider when making such changes. For example, if you're
//!    computing a large number of geometric quantities that all require the
//!    same spacecraft-to-target state vector, and each quantity is computed by
//!    a separate routine, you may want to compute the state vector once and
//!    pass it into the geometry routines.
//!
//!  Sometimes speed gains can be achieved by calling lower-level SPICELIB
//!    routines. For example, if you're computing the apparent states of many
//!    targets as seen from a single observer at a given epoch, rather than
//!    using the high-level reader [SPKEZR](crate::raw::spkezr), you can look up the observer state
//!    relative to the solar system barycenter via [SPKSSB](crate::raw::spkssb), then separately look
//!    up each apparent target state relative to the solar system barycenter
//!    via [SPKAPP](crate::raw::spkapp). This eliminates redundant computations of the observer's
//!    state.
//!
//!  
//!
//!
//!  
//! ##  Quaternions
//!
//!  
//!
//!
//!  
//! ###  Problem: NAIF quaternions appear incorrect
//!
//!  There are two styles of quaternions in common use. The NAIF style is in
//!    common use by mathematicians and physicists; the alternate style is in
//!    common use throughout JPL and elsewhere in the aerospace engineering
//!    community.
//!
//!  NAIF style quaternions are related to rotations as follows: if a
//!    rotation transformation rotates a vector V in the right-handed sense
//!    about an axis
//!
//!  
//!
//! ```text
//!    A = (a1, a2, a3)
//! ```
//!
//!  by an angle of theta radians, then the NAIF quaternion representing this
//!    rotation is
//!
//!  
//!
//! ```text
//!    ( cos(theta/2), sin(theta/2)a1, sin(theta/2)a2, sin(theta/2)a3 )
//! ```
//!
//!  A NAIF quaternion
//!
//!  
//!
//! ```text
//!    ( q0, q1, q2, q3 )
//! ```
//!
//!  can be transformed to the alternate style
//!
//!  
//!
//! ```text
//!    ( q0', q1', q2', q3' )
//! ```
//!
//!  by the equations
//!
//!  
//!
//! ```text
//!    q0'  =  -q1
//!    q1'  =  -q2
//!    q2'  =  -q3
//!    q3'  =   q0
//! ```
//!
//!  These equations also indicate how to transform the alternate quaternion
//!    style to the NAIF style.
//!
//!  
//!
//!
//!  
//! ###  NAIF matrix--quaternion conversion appears incorrect
//!
//!  The formulas for conversion between quaternions and matrices depend on
//!    the quaternion style in use. See the section above.
//!
//!  
//!
//!
//!  
//! ##  Reference frames
//!
//!  
//!
//!
//!  
//! ###  Problem: EME50 vectors from SPICE appear incorrect
//!
//!  See discussion of reference frames in "Getting it Right."
//!
//!  
//!
//!
//!  
//! ###  Problem: Vectors in body-fixed frame appear incorrect
//!
//!  See "Body-Fixed Frames."
//!
//!  
//!
//!
//!  
//! ##  Software Application Integration
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE routine names conflict with application's names
//!
//!  NAIF cannot change the names of routines in its published interfaces.
//!    However, NAIF can supply on request a special version of a library
//!    having special suffixed or prefixed names that are unlikely to collide
//!    with names used elsewhere.
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE code is not thread safe.
//!
//!  No mechanisms to ensure thread safe behavior exist in standard ANSI C or
//!    FORTRAN 77. NAIF has no plans to use non-standard features in SPICELIB.
//!
//!  
//!
//!
//!  
//! ###  Problem: Application requires SPICE error output to be trapped
//!
//!  See "Error Handling."
//!
//!  
//!
//!
//!  
//! ##  SPK/Ephemeris/States
//!
//!  
//!
//!
//!  
//! ###  Problem: How can I interactively determine the coverage of an SPK?
//!
//!  Use the SPICE utility program BRIEF to summarize the SPK file.
//!
//!  
//!
//!
//!  
//! ###  Problem: Can't determine what states are computable from SPK files
//!
//!  Good question. SPICE does not currently contain routines that provide a
//!    convenient answer.
//!
//!  It is possible to examine loaded kernels at run time to determine their
//!    coverage. This is done via the DAF search routines. See the DAF Required
//!    Reading, [daf.req](crate::required_reading::daf), or the headers of [DAFBFS](crate::raw::dafbfs), [DAFBBS](crate::raw::dafbbs), [DAFFNA](crate::raw::daffna), [DAFFPA](crate::raw::daffpa),
//!    [DAFGS](crate::raw::dafgs), and [DAFUS](crate::raw::dafus).
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE(SPKINSUFFDATA) error is signaled
//!
//!  The most common reason for this is that the user neglected to load a
//!    necessary SPK file.
//!
//!  Computation of aberration-corrected states requires that sufficient data
//!    be available to compute the observer and target states relative to the
//!    solar system barycenter. The target state must be computable over the
//!    interval from the request time back to the request time minus one-way
//!    light-time. So don't request an aberration-corrected state at or near
//!    the coverage start time of an SPK file.
//!
//!  
//!
//!
//!  
//! ###  Problem: no data found for times near SPK endpoints
//!
//!  SPK coverage boundaries shown by SPACIT are approximate.
//!
//!  Also, if aberration corrections are used, state requests will result in
//!    look-ups of the target state for epochs prior to the request time. See
//!    "SPICE(SPKINSUFFDATA) error is signaled" above.
//!
//!  
//!
//!
//!  
//! ###  Problem: states vary over different program runs
//!
//!  If multiple SPK files contain "competing" data, that is redundant
//!    ephemeris data for a given body, center and time, then the SPK system
//!    selects the data based on the order in which the competing files were
//!    loaded, with files loaded last taking precedence.
//!
//!  Varying the order in which the files were loaded can affect the state
//!    vectors returned by the SPK system.
//!
//!  Of course, any change in the set of kernels used may affect results
//!    computed by the SPICE system.
//!
//!  
//!
//!
//!  
//! ###  Problem: Velocity in rotating frame is incorrect
//!
//!  See "Body-Fixed Frames."
//!
//!  
//!
//!
//!  
//! ###  Problem: SPK file contains clearly invalid data
//!
//!  The file may be corrupted. See "File Transfer."
//!
//!  
//!
//!
//!  
//! ###  Problem: Osculating elements are wrong
//!
//!  Conversion of a single state to osculating elements does not yield mean
//!    elements.
//!
//!  For some orbits, some elements are not easily recovered from state
//!    vectors. For example, argument of periapsis cannot be determined for a
//!    circular orbit.
//!
//!  Check that the central mass is valid.
//!
//!  
//!
//!
//!  
//! ###  Problem: Aberration-corrected states are not as expected
//!
//!  See "Accuracy."
//!
//!  
//!
//!
//!  
//! ###  Problem: System barycenter-relative states are inconsistent
//!
//!  Note that system barycenter locations change as mass estimates change.
//!
//!  The solar system barycenter is very sensitive to mass estimates for the
//!    outer planetary systems. Therefore, state vectors of bodies relative to
//!    the solar system barycenter cannot be expected to compare well across
//!    planetary SPK files based on different integrations (having different
//!    underlying planetary ephemerides).
//!
//!  
//!
//!
//!  
//! ##  System errors
//!
//!  SPICE attempts to catch errors before they result in system-level
//!    exceptions. Some types of errors are beyond SPICE's ability to
//!    intercept.
//!
//!  
//!
//!
//!  
//! ###  Problem: divide by zero
//!
//!  Often due to missing data or uninitialized variables.
//!
//!  
//!
//!
//!  
//! ###  Problem: subscript out of range
//!
//!  May be due to inconsistent input arguments supplied to SPICE routines.
//!    However, SPICE generally has no graceful way of determining that it's
//!    writing beyond the bounds of an array passed in by an application.
//!
//!  
//!
//!
//!  
//! ###  Problem: segmentation fault/memory access violation
//!
//!  Often caused by mismatched argument lists. Applications must supply
//!    arguments that match in data type and dimension with those expected by
//!    SPICE routines.
//!
//!  Sometimes this error results from a constant actual argument being
//!    supplied where an output argument is expected.
//!
//!  When this error occurs on a Unix system immediately upon program
//!    execution, the cause may be that the user stack is too small. See the
//!    Unix "limit" man page.
//!
//!  
//!
//!
//!  
//! ###  Problem: arithmetic overflow
//!
//!  Usually caused by invalid input data.
//!
//!  
//!
//!
//!  
//! ###  Problem: arithmetic underflow
//!
//!  SPICE does not attempt to detect or prevent underflow.
//!
//!  
//!
//!
//!  
//! ##  Time
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE conversion between ET and UTC is incorrect
//!
//!  Possible causes:
//!
//!  
//!
//! * Leapseconds kernel is out of date.
//!
//!  * SPICE parsing of input time string is not as user expects (applicable only
//! to string-to-ET conversion).
//!
//!  * SPICE rounding or truncation is not as user expects.
//!
//!  * user expects to input or output Terrestrial Dynamical Time (TDT); SPICE
//! default is Barycentric Dynamical time (TDB).
//!
//!     
//! ###  Problem: Stepping from start UTC to end UTC in loop fails
//!
//!  Note that TDB and UTC advance at different rates. UTC times that are 10
//!    seconds apart are not 10 TDB seconds apart.
//!
//!  The problem may be solved by converting times to TDT in order to perform
//!    stepping. UTC may be converted to TDT by first calling [STR2ET](crate::raw::str2et) to produce
//!    TDB, then calling [UNITIM](crate::raw::unitim) to convert TDB to TDT. Also, be sure to account
//!    for round-off in the loop termination test.
//!
//!  
//!
//!
//!  
//! ###  Problem: SPICE time strings do not have the desired format
//!
//!  SPICE supports an enormous variety of input and output formats. See the
//!    Time Required Reading, [time.req](crate::required_reading::time), and the headers of the routines [STR2ET](crate::raw::str2et)
//!    and [TIMOUT](crate::raw::timout).
//!
//!  
//!
//!
//!  
//! ###  Problem: conversion between ET and SCLK fails
//!
//!  Possible causes:
//!
//!  
//!
//! * SCLK or leapseconds kernel is out of date.
//!
//!  * SPICE parsing of input time string is not as user expects.
//!
//!  * Input string lacks partition information and is ambiguous without it.
//!
//!  * SPICE rounding is not as user expects. When converting ET to discrete
//! ticks, SPICE rounds. Some alternate algorithms truncate.
//!
//!  * ET time value is out of range covered by SCLK kernel.
//!
//!     
//! ###  Problem: conversion of SCLK string to encoded SCLK fails
//!
//!  Possible causes:
//!
//!  
//!
//! * Conversion was not done using the SPICELIB routine [SCENCD](crate::raw::scencd). Alternate
//! conversion methods may not be reliable.
//!
//!     
//! ###  Problem: SCLK string is misinterpreted
//!
//!  Some SCLK string formats look like floating point constants. It's easy
//!    to mistake the least significant SCLK field for a decimal fraction; that
//!    interpretation is usually not correct. See SCLK Required Reading,
//!    [sclk.req](crate::required_reading::sclk), for a discussion of SCLK strings.
//!
//!  
//!
//!
//!  
//! ##  Icy
//!
//!  
//!
//!
//!  
//! ###  Problem: IDL segmentation fault
//!
//!  When using the N59 or N60 distributions of Icy 1.2, the IDL executive
//!    command
//!
//!  
//!
//! ```text
//!    .full_reset_session
//! ```
//!
//!  may cause a segmentation fault on OS X, Linux and Windows platforms
//!    (dependent on the version of IDL). The solution requires editing of the
//!    icy.dlm text file. Open icy.dlm, locate the consecutive description
//!    entries:
//!
//!  
//!
//! ```text
//!    PROCEDURE   CSPICE_RECSPH     0 15
//!    PROCEDURE   CSPICE_REMOVD     0 15
//! ```
//!
//!  Add a new entry for CSPICE_REMOVC.
//!
//!  
//!
//! ```text
//!    PROCEDURE   CSPICE_RECSPH     0 15
//!    PROCEDURE   CSPICE_REMOVC     0 15
//!    PROCEDURE   CSPICE_REMOVD     0 15
//! ```
//!
//!  Then save and close icy.dlm.
//!
//!  NAIF corrected the problem for Icy 1.3, the N61 distribution.
//!
//!  
//!
//!
//!  
//! #  Appendix A: Revisions
//!
//!  
//!
//!
//!  
//! ##  2017 MAR 14 by E. D. Wright
//!
//!  Edits to correct several minor typos.
//!
//!  
//!
//!
//!  
//! ##  2007 FEB 11 by E. D. Wright
//!
//!  
//!
//!
//!  
//! ##  2006 NOV 22 by B. V. Semenov.
//!
//!  
//!
//!
//!     
