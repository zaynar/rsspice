//! #  Geometry Finder Required Reading
//!
//!  Last revised on 2017 JUN 19 by N. J. Bachman.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  The SPICE Geometry Finder (GF) subsystem finds time windows over which
//!    user-specified geometric conditions are met.
//!
//!  
//!
//!
//!  
//! ###  Purpose
//!
//!  This document is a reference guide for the SPICE GF subsystem. Here
//!    you'll find
//!
//!  
//!
//! * A list of the subsystem's API (application programming interface)
//! routines---these are the routines that may be called directly by
//! SPICE-based user application code
//!
//!  * Computational "recipes" for a variety of GF applications
//!
//!  * Discussions of concepts essential to understanding the correct use of the
//! GF subsystem
//!
//!  * Discussion of problems that may arise when using the GF subsystem
//!
//!  * Extensive example programs, including overview discussion, source code,
//! meta-kernels, and program output
//!
//!     
//! ###  Intended Audience
//!
//!  This document addresses the needs of several groups of SPICE users.
//!    Users looking for a basic discussion the capabilities of the SPICE GF
//!    subsystem should read the introduction below. Users planning to write
//!    application code using the GF subsystem may benefit from reading the
//!    entire document, but in any case should read the "GF Concepts"
//!    chapter.
//!
//!  This document assumes you already have a strong understanding of SPICE
//!    concepts and terminology.
//!
//!  
//!
//!
//!  
//! ###  References
//!
//!  The references listed below provide essential background for programmers
//!    intending to use the GF subsystem.
//!
//!  
//!
//! *  1. SPICE Tutorials (available on the NAIF web site)
//!
//!  *  2. Cells Required Reading ([cells.req](crate::required_reading::cells))
//!
//!  *  3. Windows Required Reading ([windows.req](crate::required_reading::windows))
//!
//!     
//! ##  Introduction
//!
//!  Most geometry computations performed with SPICE involve calculating
//!    quantities of interest---such as distances, vectors, angles, or
//!    orientations---for specified times. The GF subsystem solves the inverse
//!    problem: it finds times when specified geometric conditions are met.
//!
//!  For example, the GF subsystem can solve the problem:
//!
//!  
//!
//! ```text
//!    Within the time interval January 1 2009 to January 1 2010,
//!    find the time periods when the light time corrected
//!    distance between the centers of the Earth and
//!    Moon is less than 400000 kilometers.
//! ```
//!
//!  The GF subsystem works with a small set of geometric quantities:
//!
//!  
//!
//! * Angular separation of targets as seen by a specified observer
//!
//!  * Coordinates of position vectors
//!
//!  * Coordinates of sub-observer points
//!
//!  * Coordinates of surface intercept points
//!
//!  * Instrument FOV (Field of view) visibility states (appearance of a specified
//! target within an instrument FOV)
//!
//!  * Illumination angles
//!
//!  * Observer-target distance
//!
//!  * Observer-target range rate
//!
//!  * Occultation states
//!
//!  * Phase angle between observer and target centers with respect to an
//! illumination source
//!
//!  * User-defined geometric quantities, either scalar functions of boolean
//! functions
//!
//!  At the highest level of the SPICE GF subsystem interface, there is a
//!    search subroutine for each geometric quantity. The Fortran and C SPICE
//!    Toolkits contain additional, lower-level routines that provide
//!    functionality such as support for progress reporting and interrupt
//!    handling. The full set of interface routines is discussed in the chapter
//!    titled "GF API Routines."
//!
//!  All language versions of the SPICE Toolkit contain complete example
//!    programs in the GF module headers or corresponding HTML Reference Guide
//!    pages. Extensive example programs are presented at the end of this
//!    document.
//!
//!  Much of the capability of the GF subsystem derives from the wide range
//!    of input data (particularly FK files) and input parameters it supports.
//!    But in many cases it may not be immediately obvious how to select or
//!    create the necessary SPICE kernels and how to apply the small set of GF
//!    API routines to accomplish a given search task. The "GF Computational
//!    Recipes" chapter below provides many short descriptions of how to use
//!    the GF subsystem to search for geometric events that are frequently of
//!    interest.
//!
//!  Because the main function of the GF subsystem is, at its heart, solving
//!    equations, the details of the subsystem's behavior are more complex than
//!    is the case for most other SPICE subsystems. Understanding how to call
//!    the GF routines is not sufficient to guarantee correct results. So SPICE
//!    application programmers are encouraged to read the "GF Concepts"
//!    chapter below.
//!
//!  
//!
//!
//!  
//! ##  Planned enhancements
//!
//!  NAIF expects to expand the set of supported quantities in future
//!    versions of the SPICE Toolkit. Planned additions include, but are not
//!    limited to:
//!
//!  
//!
//! * Eclipse events
//!
//!  * Latitude-longitude boxes
//!
//!     
//! ##  Terminology
//!
//!  Throughout this document we use terms such as SPICE window, root
//!    finding, convergence, etc. We include brief explanations of these terms
//!    below.
//!
//!  
//!
//! *  Absolute extremum
//!
//!
//!  See Global extremum (below).
//!
//!  *  API
//!
//!
//!  "Application programming interface": a set of routines intended to be
//! called directly by SPICE based user application programs. Also an
//! adjective indicating that a designated routine is a member of the set
//! of API routines, for example "[GFPOSC](crate::raw::gfposc) is a GF API routine."
//!
//!  *  Aberration correction
//!
//!
//!  Correction for light time or stellar aberration effects. These
//! corrections can involve adjustment of position or direction vectors,
//! orientation of objects, or times. See the header of [SPKEZR](crate::raw::spkezr) and the
//! Fundamental Concepts tutorial for details.
//!
//!  *  Boolean quantity function
//!
//!
//!  A function whose range is comprised of only two values, for example 0
//! and 1 or "true" and "false." For GF use such a function is
//! implemented as a routine with one independent variable (nominally time)
//! as input and a boolean variable as output.
//!
//!  *  Bounds
//!
//!
//!  Values that constrain the range of values in a specified set of
//! numbers: A is a lower bound for a set S if no member of S is less than
//! A; B is an upper bound for S if no member of S is greater than B. Note
//! that bounds are not equivalent to extrema.
//!
//!  *  Binary state function
//!
//!
//!  See "Boolean quantity function."
//!
//!  *  Boresight
//!
//!
//!  A vector or ray used to indicate the "look direction" of an
//! instrument.
//!
//!  *  Bracket
//!
//!
//!  A number X is bracketed by numbers A and B when X lies between A and B,
//! inclusive.
//!
//!  *  Closed
//!
//!
//!  An interval is "closed" if it contains its endpoints.
//!
//!  *  Confinement window
//!
//!
//!  The time window over which a GF search is to be conducted, or a SPICE
//! window (see below) representing this time window.
//!
//!  *  Converge
//!
//!
//!  A sequence of numbers converges if the sequence tends to a limit.
//!
//!  *  Convergence
//!
//!
//!  The act of converging; progress toward or completion of the process of
//! locating a root.
//!
//!  *  Convergence tolerance
//!
//!
//!  A GF root-finding process is considered to have found a root when the
//! root is bracketed by upper and lower bounds that differ by no more than
//! a specified bound called the "convergence tolerance."
//!
//!  *  Coordinate
//!
//!
//!  A spatial parameter belonging to a coordinate system.
//!
//!  *  Coordinate system
//!
//!
//!  In SPICE documentation, three-dimensional "coordinate systems" are
//! parameterizations of three-dimensional space: they are mappings that
//! label each point in space using an ordered set of three spatial
//! parameters such as (X, Y, Z) or (radius, longitude, latitude). At any
//! point in space, the directions in which the three coordinates increase
//! are mutually orthogonal. Put another way, the Jacobian matrices of
//! these mappings are not orthogonal, but they do have orthogonal sets of
//! rows and columns. (Compare to "reference frame" below.)
//!
//!  *  Coverage
//!
//!
//!  In SPICE documentation, "coverage" refers to the extent of data
//! provided by a set of SPICE kernels: either the time window for which
//! data are available, or less commonly, the set of bodies or instruments
//! for which data are available.
//!
//!  *  Coverage window
//!
//!
//!  The time window over which data of interest are available, or a SPICE
//! window (see below) representing this time window.
//!
//!  *  Disjoint
//!
//!
//!  Non-intersecting. No common elements.
//!
//!  *  Domain
//!
//!
//!  The set of points on which a function acts: a function "maps"
//! elements of its domain to a set called the "range" of the function.
//!
//!  *  DSN
//!
//!
//!  Deep Space Network.
//!
//!  *  Eclipse
//!
//!
//!  An object is "eclipsed" or "in eclipse" when it intersects the
//! shadow created by the Sun and another object.
//!
//!  *  Endpoints
//!
//!
//!  The boundary values of an interval on the real line. The left endpoint
//! of an interval is its smallest value; the right endpoint is its
//! greatest.
//!
//!  *  Ephemeris object
//!
//!
//!  Any entity whose position and velocity, relative to a specified center
//! of motion, are given by an SPK file.
//!
//!  *  Extended Object
//!
//!
//!  Also extended body or extended target. An object of finite size; an
//! object consisting of more than a single point. In SPICE applications,
//! extended objects are often represented by ellipsoids.
//!
//!  *  Extrema
//!
//!
//!  Plural of extremum.
//!
//!  *  Extremum
//!
//!
//!  The minimum or maximum value attained by a function. See Global
//! Extremum and Local Extremum.
//!
//!  *  FOV
//!
//!
//!  Abbreviation of "field of view."
//!
//!  *  Field of view
//!
//!
//!  The spatial region that can be viewed by a remote sensing instrument,
//! or a mathematical model of this region. Often an instrument's field of
//! view is modeled by a cone or a pyramid having a polygonal cross
//! section.
//!
//!  *  Global extremum
//!
//!
//!  A global maximum or minimum: the unique greatest or least value
//! attained by a function. It is possible for a function to have multiple
//! locations in its domain at which a global extremum occurs.
//!
//!  *  Instrument
//!
//!
//!  In the GF setting, an instrument is usually a camera or other
//! remote-sensing radiation detector whose orientation is given by a
//! reference frame known to the SPICE system and which has a field of view
//! modeled by an IK.
//!
//!  *  Interrupt handler
//!
//!
//!  A routine that performs an action in response to an interrupt signal,
//! such as that generated by a user hitting the "control Y" key
//! combination at a Unix command line.
//!
//!  *  Inverse problem
//!
//!
//!  Inverse problems entail finding times when geometric quantities take on
//! specified values. In general, inverse problems involve finding the set
//! S in the domain of a function such that the function maps S to a
//! specified set.
//!
//!  *  Local extremum
//!
//!
//!  A local maximum or local minimum: the greatest or least value attained
//! by a function in a neighborhood of a point in the function's domain. At
//! a point where a local extremum of a function is attained, there is a
//! region or "neighborhood" enclosing that point over which the function
//! is bounded by that extreme value. For a local maximum, on this region,
//! the function is no greater than the local maximum; for a local minimum,
//! the function is no smaller. A function can have multiple local extrema.
//!
//!  *  Measure
//!
//!
//!  The measure of a SPICE window is the sum of the lengths of the window's
//! intervals. (This definition is valid because the intervals of a SPICE
//! window are disjoint.)
//!
//!  *  Meta-kernel
//!
//!
//!  A SPICE text kernel specifying names of SPICE kernels to load.
//!
//!  *  Number line
//!
//!
//!  The "real line" (see below).
//!
//!  *  Observer
//!
//!
//!  An ephemeris object, the location of which acts as the tail of a
//! position vector. The head of the vector is the location of another
//! ephemeris object called the "target."
//!
//!  *  Observer-target vector
//!
//!
//!  A vector emanating from one ephemeris object (the observer) and
//! terminating at another (the target).
//!
//!  *  Occultation
//!
//!
//!  Blockage of the apparent figure of one object by another, as seen from
//! a specified vantage point.
//!
//!  *  Range
//!
//!
//!  \[1] The set of values attained by a function: a function "maps"
//! elements of its domain to its range. \[2] The Euclidean distance between
//! two objects, usually target and observer.
//!
//!  *  Range rate
//!
//!
//!  The derivative with respect to time of the range between two objects.
//! For GF use, the objects being an observer and a target body.
//!
//!  *  Real line
//!
//!
//!  A line representing the real numbers. The real numbers include zero,
//! all positive and negative fractions, and any number that's a limit of
//! some sequence of fractions. In SPICE documentation, real numbers are
//! restricted to those representable by the double precision floating
//! point data type, excluding distinguished values such as +/- Inf and
//! NaN.
//!
//!  *  Reference frame
//!
//!
//!  A set of three mutually orthogonal directions in space and an
//! associated center. See the Fundamental Concepts, FK, and Using Frames
//! tutorials, as well as the Frames Required Reading, [frames.req](crate::required_reading::frames), for
//! details. (Compare to "coordinate system" above.)
//!
//!  *  Result window
//!
//!
//!  In the GF setting, a SPICE window (see below) representing the time
//! window over which a specified geometric condition is satisfied. A
//! result window is an output window returned by a SPICE GF API search
//! routine.
//!
//!  *  Root
//!
//!
//!  Solution of an equation; point satisfying given constraints. In the GF
//! setting, roots are times at which state transitions of interest occur,
//! for example times when a specified occultation starts or stops, or the
//! time at which the distance between two ephemeris objects attains a
//! local minimum. Roots are endpoints of SPICE windows representing search
//! results.
//!
//!  *  Root Finding
//!
//!
//!  The process of locating roots; searching for roots.
//!
//!  *  Scalar quantity function
//!
//!
//!  A function that returns a scalar value. For GF use such a function is
//! implemented as a routine with one independent variable (nominally time)
//! as input and the scalar variable as output.
//!
//!  *  SCLK
//!
//!
//!  Spacecraft clock. See the "LSK and SCLK" tutorial and the SCLK
//! Required Reading, [sclk.req](crate::required_reading::sclk), for details.
//!
//!  *  Search window
//!
//!
//!  A confinement window (see above).
//!
//!  *  Singleton
//!
//!
//!  A set consisting of a single point. Also short for "singleton
//! interval."
//!
//!  *  Singleton interval
//!
//!
//!  An interval having equal left and right endpoints.
//!
//!  *  Singularity
//!
//!
//!  A point or region in the domain of a function at which the function is
//! "badly behaved": the function is not defined, not continuous, or not
//! differentiable. For example, longitude has a singularity at pi radians.
//! In three dimensional space the singular region of longitude is the
//! half-plane for which Y = 0 and X \<= 0.
//!
//!  *  SPICE window
//!
//!
//!  Also SPICELIB window. An abstract data type used to represent
//! collections of intervals on the real line, especially collections of
//! time intervals; also, an instance of this type. A SPICE window
//! represents a union of zero or more disjoint intervals, arranged in
//! increasing order: the right endpoint of one constituent interval of a
//! window is strictly less than the left endpoint of the next interval.
//! Intervals in a SPICE window may be singletons. SPICE window can be
//! empty. SPICE windows are implemented as structured arrays in Fortran
//! and MATLAB; they're implemented as structures in C and IDL. See the
//! Windows Required Reading, [windows.req](crate::required_reading::windows), for details.
//!
//!  *  Step size
//!
//!
//!  The duration between times at which a function is sampled.
//!
//!  *  Sub-observer point
//!
//!
//!  The point on the surface of an extended target that is, depending on
//! the user's specification, either closest to the observer, or lies on
//! the line connecting the observer and the target's center.
//!
//!  *  Surface intercept
//!
//!
//!  An intersection of a ray and a specified surface. When the vertex of
//! the ray is associated with an observer, usually the surface intercept
//! is understood to be the point of intersection closest to the observer.
//!
//!  *  Target
//!
//!
//!  Ephemeris object, the location of which acts as the head of a position
//! vector. The tail of the vector is the location of another ephemeris
//! object called the "observer."
//!
//!  *  TDB
//!
//!
//!  Barycentric Dynamical Time. The independent variable used in SPK, PCK,
//! and dynamic FK files and all of the SPICE API routines, except for the
//! CK readers and some time conversion routines. See the Time Required
//! Reading, [time.req](crate::required_reading::time), and the Fundamental Concepts tutorial for details.
//!
//!  *  In SPICE Toolkit documentation, any reference to ET (ephemeris time)
//! means a TDB time.
//!
//!  *  Ticks
//!
//!
//!  Encoded SCLK. Used as the independent variable in CK files. See the
//! "LSK and SCLK" tutorial and the SCLK Required Reading, [sclk.req](crate::required_reading::sclk), for
//! details.
//!
//!  *  Time interval
//!
//!
//!  The set of times between a start time and a stop time, inclusive. The
//! start and stop times are also called "endpoints."
//!
//!  *  Time window
//!
//!
//!  A set of zero or more closed, disjoint time intervals arranged in
//! increasing order. Also a SPICE window (see above) representing such a
//! set of time intervals.
//!
//!  *  Tolerance
//!
//!
//!  See convergence tolerance.
//!
//!  *  Window
//!
//!
//!  A set of zero or more closed, disjoint intervals on the real line,
//! arranged in increasing order. Also a SPICE window (see above). Windows
//! frequently represent time but may be used for other purposes, for
//! example to represent sets of angular intervals on the unit circle.
//!
//!     
//! #  GF Concepts
//!
//!  
//!
//!
//!  
//! ##  Time windows
//!
//!  Every GF search is performed over a time period represented by a SPICE
//!    window called the "confinement window." Every successful GF search
//!    produces as a result a SPICE window called the "result window."
//!
//!  In SPICE documentation, a "time window" is a set of zero or more
//!    closed, disjoint time intervals, arranged in increasing order. The
//!    intervals may be singletons: they can have equal left and right
//!    endpoints.
//!
//!  The term "SPICE window" refers to both the abstract data type used to
//!    represent time windows and instances of this type. In Fortran and
//!    MATLAB, SPICE windows are implemented via structured arrays (arrays
//!    whose internal organization adheres to certain rules); in C and IDL they
//!    are represented by structures.
//!
//!  By "closed" we mean that the intervals of a SPICE window are
//!    topologically closed: that is, the intervals always include their
//!    endpoints.
//!
//!  We'll use diagrams like the one below to depict time windows. The dashed
//!    line represents the real line; the bracketed regions signify the time
//!    intervals comprising the window.
//!
//!  
//!
//! ```text
//!    --[----------------------]-------[----]--[----------------]--
//! ```
//!
//!     
//! ###  Window manipulation and arithmetic
//!
//!  The SPICE Toolkit provides a set of routines that manipulate SPICE
//!    windows. These are described in the Windows Required Reading
//!    [windows.req](crate::required_reading::windows). Among the supported window operations are "set
//!    arithmetic" functions such as union, intersection, difference, and
//!    complementing with respect to an interval.
//!
//!  Arithmetic on SPICE windows differs a bit from standard set arithmetic
//!    because all windows resulting from window operations remain closed. For
//!    example, when you subtract a SPICE window from another, the result is a
//!    union of closed intervals. Standard set arithmetic would produce a
//!    result containing half-open or open intervals.
//!
//!  Window arithmetic is used to solve for logical combinations of geometric
//!    conditions. For example:
//!
//!  
//!
//! * To find times within a given confinement window when a target is not
//! occulted, use [GFOCLT](crate::raw::gfoclt) to find the times when the target is occulted, then
//! subtract the result window from the confinement window.
//!
//!  * To find times when a target is visible in either of the FOVs of two
//! instruments, conduct visibility searches for each instrument using [GFTFOV](crate::raw::gftfov),
//! then compute the union of the result windows from the two searches.
//!
//!  It is often convenient to use the result window produced by one GF
//!    search as the input confinement window of another. Often this is both
//!    simpler and faster than computing two searches on the original
//!    confinement window and then intersecting the result windows.
//!
//!  
//!
//!
//!  
//! ###  Result windows are approximate
//!
//!  Since result windows are created by a mathematical root finding process,
//!    the endpoints---that is, the start and stop times---of the intervals
//!    comprising these windows are always approximate. The errors in these
//!    endpoint times are due not only to errors in input data and round-off
//!    errors introduced by finite-precision arithmetic, but to the fact that
//!    the endpoints are determined by an approximation process that terminates
//!    when the endpoints are found to be correct within a "convergence
//!    tolerance."
//!
//!  A consequence of the errors in the computed endpoints is that the
//!    geometric constraint that is supposed to be satisfied for every time
//!    within the result window FREQUENTLY IS NOT SATISFIED at one or more
//!    endpoints of the intervals of this window. In fact, it is common for
//!    there to be a small time region surrounding an interval endpoint on
//!    which the constraint of interest is not satisfied.
//!
//!  For the same reason, it is just as likely that the constraint of
//!    interest is satisfied on a small time region extending beyond an
//!    interval endpoint of the result window. This is perhaps a less obvious
//!    error, but it is nevertheless an error because the result window is
//!    ideally the exact set of times, within the confinement window, on which
//!    the constraint is satisfied. In this case the result window is not the
//!    maximal subset of the confinement window on which the constraint is
//!    satisfied.
//!
//!  One application for which result window errors are particularly striking
//!    is that of searches for time windows satisfying longitude or right
//!    ascension constraints. For example, a small error in the window over
//!    which a given longitude is between -180 and -150 degrees can easily
//!    include some times at which the longitude is between 179 and 180
//!    degrees.
//!
//!  
//!
//!
//!  
//! ###  Working around result window errors
//!
//!  SPICE window "contraction" is an operation in which the left endpoints
//!    of each of a window's intervals are moved to the right and the right
//!    endpoints are moved to the left. Use the SPICE routine [WNCOND](crate::raw::wncond) to
//!    contract a SPICE window.
//!
//!  In many cases, it makes sense to contract a result window slightly to
//!    remove portions of the window on which a constraint is not satisfied.
//!    Usually it suffices to contract a window by an amount on the order of
//!    the convergence tolerance. In the case of result windows produced by
//!    longitude or right ascension searches, a somewhat larger contraction is
//!    needed because these result windows are actually the product of multiple
//!    sub-searches.
//!
//!  When an application performs set arithmetic on result windows, usually
//!    contraction should be performed only on the final result. Contracting
//!    intermediate results can be a mistake. For example, contracting a window
//!    before computing its complement introduces an error in the complement:
//!    the complement then includes more of the original window than just its
//!    endpoints.
//!
//!  Contraction should not be performed on result windows comprised of
//!    singleton intervals: the result of such contractions would be an empty
//!    window. Searches for local or absolute extrema are examples of the type
//!    that produces a window of singleton intervals.
//!
//!  
//!
//!
//!  
//! ##  Events
//!
//!  In GF documentation, an instance of a geometric quantity satisfying a
//!    specified condition is called an "event." An event can be
//!    instantaneous, such as an observer attaining its minimum distance to a
//!    target, or it can have finite duration, as does an occultation.
//!
//!  Geometric quantities supported by the GF subsystem either have binary
//!    states or are numeric functions of time.
//!
//!  "Binary state" quantities are logical-valued functions of time;
//!    they're either true or false for a given time value. For example,
//!    "target A is fully occulted by target B as seen from observer C" is
//!    either true or false at any given time. Occultation and FOV visibility
//!    are binary state quantities.
//!
//!  "Numeric" quantities are scalar-valued functions of time. Distance,
//!    angular separation, and coordinates are numeric quantities.
//!
//!  
//!
//!
//!  
//! ###  Constraints
//!
//!  Constraints are logical conditions that are specified by a calling
//!    SPICE-based application and satisfied over the result window produced by
//!    a GF search.
//!
//!  The only supported constraint applicable to binary state quantities is
//!    "the state is true." Note that SPICE window arithmetic serves to
//!    produce the window on which a binary state is false.
//!
//!  Supported constraints on numeric quantities are mathematical relations,
//!    such as equalities, inequalities, and attainment of local or global
//!    maxima or minima. These are often called "numeric constraints"
//!    "scalar constraints," or "relational constraints." Specifically,
//!    these relations are:
//!
//!  
//!
//! *  =
//!
//!
//!  The quantity is equal to a specified value, called the "reference
//! value.'
//!
//!  *  \<
//!
//!
//!  The quantity is less than a specified value, called the "reference
//! value.'
//!
//!  *  >
//!
//!
//!  The quantity is greater than a specified value, called the "reference
//! value.'
//!
//!  *  ABSMAX
//!
//!
//!  The quantity attains its absolute (global) maximum.
//!
//!  *  ABSMIN
//!
//!
//!  The quantity attains its absolute (global) minimum.
//!
//!  *  LOCMAX
//!
//!
//!  The quantity attains a local maximum.
//!
//!  *  LOCMIN
//!
//!
//!  The quantity attains a local minimum.
//!
//!  *  ABSMAX, ADJUST !=0
//!
//!
//!  The quantity is within the adjustment amount ADJUST of its absolute
//! (global) maximum.
//!
//!  *  ABSMIN, ADJUST !=0
//!
//!
//!  The quantity is within the adjustment amount ADJUST of its absolute
//! (global) minimum.
//!
//!  For a numeric quantity search, the result window is the set of times at
//!    which the quantity satisfies the specified relation.
//!
//!  Note that the "greater than or equal to (>=)" and "less than or
//!    equal to (\<=)" operators are not supported. Since result windows are
//!    approximate, the distinction between the solutions that could be found
//!    using these operators and those found using strict inequality operators
//!    is usually not meaningful. The case where there is a significant
//!    distinction is that in which a function takes on the constant value X on
//!    one or more intervals, and the reference value is set to X. However, as
//!    discussed below, the GF subsystem cannot solve for this constraint.
//!
//!  
//!
//!
//!  
//! ##  Root finding
//!
//!  A search for a specified event comes down to finding the start and stop
//!    times of the intervals, within a given confinement window, over which
//!    the event occurs---that is, over which the geometric quantity of
//!    interest satisfies a constraint specified by the calling application.
//!    These start and stop times are the "roots" found by a GF search.
//!
//!  Because GF searches are "global" in the sense that they attempt to
//!    find all roots within the confinement window, each search involves two
//!    basic steps: bracketing the roots and refining the roots.
//!
//!  Note that the most elementary root finding techniques deal with finding
//!    roots that are already bracketed.
//!
//!  Searches for roots are conducted independently over each interval
//!    comprising the confinement window. For simplicity, and without loss of
//!    generality, we'll describe the processes below for confinement windows
//!    consisting of a single interval.
//!
//!  
//!
//!
//!  
//! ###  Search step size
//!
//!  Root bracketing consists of sampling the geometric quantity of interest
//!    at evenly spaced times throughout an interval.
//!
//!  An example is shown below: we have a confinement window consisting of an
//!    interval having a start time of 2 seconds past J2000 TDB, a stop time of
//!    57 seconds past J2000 TDB, and a step size of 10 TDB seconds.
//!
//!  Sampling with 10-second step:
//!
//!  
//!
//! ```text
//!  
//!      2         12        22        32        42        52   57
//!      |         |         |         |         |         |    |
//!      v         v         v         v         v         v    v
//!  
//!    --[------------------------------------------------------]---
//!      ^                                                      ^
//!      2                                                      57
//! ```
//!
//!  Note that a sample is always taken at the end of the interval.
//!
//!  The reader may note that the unlikely TDB time values used here
//!    correspond to the zero-based column counts of the dashes in the diagram.
//!
//!  Suppose the quantity we're sampling is of the binary-state variety. Each
//!    sample has the value "true" or "false." Suppose the diagram below
//!    indicates the state of the quantity as a function of time. At the top of
//!    the diagram are the values of the state samples:
//!
//!  
//!
//! ```text
//!      F         T         T         T         F         F    F
//!  
//!      2         12        22        32        42        52   57
//!      |         |         |         |         |         |    |
//!      v         v         v         v         v         v    v
//!  
//!         TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT
//!      FFF                                    FFFFFFFFFFFFFFFFF
//!  
//!    --[------------------------------------------------------]---
//!      ^                                                      ^
//!      2                                                      57
//! ```
//!
//!  Above, the samples indicate that state transitions must occur between
//!    the times 2 and 12 TDB seconds past J2000 TDB, and also between 32 and
//!    42 seconds past J2000 TDB. So these pairs of times bracket,
//!    respectively, the start and stop times of our "event."
//!
//!  Given the bracketing times, the GF system can refine the actual times of
//!    the state transitions, producing estimates that are accurate to within a
//!    given convergence tolerance.
//!
//!  
//!
//!
//!  
//! ###  Binary state step size selection problems
//!
//!  It's clear that for most searches, choosing an extremely small step size
//!    will result in a large number of samples being taken. This will result
//!    in very---probably unacceptably---slow search execution.
//!
//!  Step sizes that are too large may result in fast search completion, but
//!    they'll produce erroneous results.
//!
//!  As an example, suppose we repeat the previous search using a 40 second
//!    step. The samples we'd find are shown below.
//!
//!  
//!
//! ```text
//!      F                                       F              F
//!  
//!      2                                       42             57
//!      |                                       |              |
//!      v                                       v              v
//!  
//!         TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT
//!      FFF                                    FFFFFFFFFFFFFFFFF
//!  
//!    --[------------------------------------------------------]---
//!      ^                                                      ^
//!      2                                                      57
//! ```
//!
//!  Above, the samples indicate that no state transitions occur: the state
//!    is always "false." The GF subsystem will fail to detect the event and
//!    will return an empty result window.
//!
//!  Another example: suppose we use a 10 second step size and our binary
//!    state quantity has the profile shown below:
//!
//!  
//!
//! ```text
//!      F         T         T         T         T         F    F
//!  
//!      2         12        22        32        42        52   57
//!      |         |         |         |         |         |    |
//!      v         v         v         v         v         v    v
//!  
//!         TTTTTTTTTTTTTTTTTTT       TTTTTTTTTTTTTT
//!      FFF                   FFFFFFF              FFFFFFFFFFFFF
//!  
//!    --[------------------------------------------------------]---
//!      ^                                                      ^
//!      2                                                      57
//! ```
//!
//!  Above, the samples indicate that state transitions must occur between
//!    the times 2 and 12 TDB seconds past J2000 TDB, and also between 42 and
//!    52 seconds past J2000 TDB. The GF subsystem thinks that only one long
//!    event has occurred because the state transitions in the middle of the
//!    search interval were missed.
//!
//!  We can conclude that for binary state searches, the step size must be
//!    short enough to capture the relevant behavior of the underlying
//!    geometric quantity: the step size must be shorter than any event of
//!    interest, and it must be shorter than any gap between events of
//!    interest.
//!
//!  
//!
//!
//!  
//! ###  Numeric quantity step size selection problems
//!
//!  The step size issues discussed above apply to numeric quantity searches
//!    as well, because each numeric quantity search involves a binary state
//!    search to determine times, within the confinement window, when the
//!    quantity is decreasing. The state transition times found by this search
//!    are times when local extrema are attained.
//!
//!  So for numeric quantity searches, the step size must be small enough so
//!    that all (relevant) local extrema can be found: the step size must be
//!    smaller than the minimum time between consecutive epochs at which local
//!    extrema of the numeric quantity occur.
//!
//!  
//!
//!
//!  
//! ###  Search convergence
//!
//!  Once a root has been bracketed, a refinement process is used to narrow
//!    down the time interval, \[t1, t2] with t2 >= t1, within which the root
//!    must lie. This refinement process terminates when the location of the
//!    root has been determined to within an error margin called then
//!    "convergence tolerance."
//!
//!  The high-level GF search routines use a fixed tolerance in units of
//!    seconds CNVTOL which is defined in the include file
//!
//!  
//!
//! ```text
//!    gf.inc
//! ```
//!
//!  The default value is "tight" so that the tolerance doesn't become the
//!    limiting factor in the accuracy of solutions. In general the accuracy of
//!    input data will be the limiting factor.
//!
//!  To use a different tolerance value, mid-level GF search routines
//!    (available only in the Fortran and C SPICE Toolkits) must be called.
//!    Making the tolerance tighter than the default is unlikely to be useful,
//!    since the results are unlikely to be more accurate. Making the tolerance
//!    looser will speed up searches somewhat, since a few convergence steps
//!    will be omitted. However, in most cases, the step size is likely to have
//!    a much greater effect on processing time than would the convergence
//!    tolerance.
//!
//!  Please remember the condition satisfying convergence
//!
//!  
//!
//! ```text
//!    || t2 - t1 || =< CNVTOL
//! ```
//!
//!  does not imply (ignoring incompatibility of units)
//!
//!  
//!
//! ```text
//!    || f(t2) - f(t1) || =< CNVTOL
//! ```
//!
//!  for scalar function "f(t)." The GF subsystem measures convergence
//!    using time (units of TDB seconds) not using the scalar quantity (units
//!    of kilometers or radians or whatever).
//!
//!  
//!
//!
//!  
//! ##  An important numeric event limitation
//!
//!  The algorithm currently used by the GF subsystem to search for numeric
//!    events makes a very strong assumption about the underlying numeric
//!    quantities:
//!
//!  
//!
//! ```text
//!    Each numeric quantity is piecewise monotone.
//! ```
//!
//!  That is, each interval of the confinement window can be divided into a
//!    finite set of intervals over which the quantity is always increasing or
//!    always decreasing.
//!
//!  The authors believe this is a reasonable assumption for most numeric
//!    quantities involving solar system geometry.
//!
//!  However, this not a valid assumption for all numeric quantities
//!    supported by SPICE. For example, spacecraft orientation definitely can,
//!    and often does, violate this assumption.
//!
//!  There are two practical consequences of this assumption:
//!
//!  
//!
//! * The GF subsystem cannot correctly solve for times when the numeric quantity
//! of interest takes on a constant value X, if the quantity takes on the value
//! X over a finite (non singleton) interval. The GF subsystem can solve for
//! equality constraints only when the solution consists of a finite set of
//! points.
//!
//!  * Searches for local extrema may yield extraneous solutions if the numeric
//! quantity of interest is constant on a finite (non singleton) interval. If
//! the search step size is shorter than such an interval, and if the quantity
//! exhibits any noise (such as that caused by round-off errors), then at least
//! one local extremum will be found in the interval.
//!
//!  GF users must consider the impact of this assumption on the validity of
//!    planned GF applications.
//!
//!  
//!
//!
//!  
//! ##  Workspace
//!
//!  GF scalar quantity searches require memory to store intermediate
//!    results; this memory is called "workspace." Note that GF binary state
//!    searches don't require workspace.
//!
//!  Workspace is used to store multiple SPICE windows, all of which have the
//!    same size. The windows' size requirement is determined by the number of
//!    time intervals they must be able to hold.
//!
//!  GF users decide the amount of workspace to provide: in Fortran, callers
//!    of the GF search API routines declare workspace arrays, while GF APIs of
//!    SPICE Toolkits for other languages dynamically allocate memory based on
//!    the workspace window interval count specified by calling applications
//!    via an input argument.
//!
//!  
//!
//!
//!  
//! ###  Workspace window counts
//!
//!  Fortran SPICE Toolkit users must declare workspace using two dimensions:
//!    workspace window size and workspace window count---the count is the
//!    number of windows the workspace can hold. Parameters giving recommended
//!    workspace window counts are declared in the SPICELIB include file
//!
//!  
//!
//! ```text
//!    gf.inc
//! ```
//!
//!  Declaring workspace window counts to be larger than the actual required
//!    number is not an error.
//!
//!  Readers may note that the SPICELIB GF interfaces could have relied on
//!    hard-coded workspace window counts. The reason for treating these counts
//!    as passed-in parameters is that this enables run-time error checking on
//!    the counts.
//!
//!  SPICE Toolkits implemented in languages other than Fortran handle
//!    workspace window counts automatically. However, users of these Toolkits
//!    may wish to be aware of these window count requirements because they
//!    affect the total amount of dynamically allocated memory used by the GF
//!    API routines. Parameters giving workspace window counts are declared in
//!    the CSPICE header file
//!
//!  
//!
//! ```text
//!    SpiceGF.h
//! ```
//!
//!     
//! ###  Workspace window interval counts
//!
//!  While workspace window count requirements are determined by parameters,
//!    maximum workspace window interval counts must be selected by SPICE
//!    users.
//!
//!  For most searches, it's safe to choose a workspace interval count that's
//!    much larger than the actual requirement. For example, one can choose an
//!    interval count of 200000 for a search that really requires only 200
//!    intervals. This approach is used in most GF example programs that appear
//!    in SPICE documentation.
//!
//!  The only drawback to the approach of picking a large, default workspace
//!    size is that if it's taken to extremes, applications may use so much
//!    memory so that they fail to link or run, or so that they run
//!    inefficiently.
//!
//!  If an initial guess at the workspace size requirement fails, one usually
//!    can simply increase the workspace size and repeat the search.
//!
//!  However, some applications call for a more accurate method of estimating
//!    workspace interval count requirements. The actual requirement is that
//!    the interval count must be large enough to hold the windows, restricted
//!    to the confinement interval, over which the quantity of interest is
//!    monotonically increasing or decreasing. Note that the number of
//!    intervals comprising the confinement window affects the amount of
//!    required space.
//!
//!  
//!
//!
//!  
//! ###  Estimating the workspace interval count requirement
//!
//!  If a confinement window is comprised of N intervals and has measure M
//!    seconds, and the search step size is STEP seconds, then a rule of thumb
//!    for the number of required workspace intervals NINTVLS is
//!
//!  
//!
//! ```text
//!    NINTVLS  =  2*N  +  ( M / STEP )
//! ```
//!
//!  In many cases the actual number of intervals needed is much smaller than
//!    this estimate.
//!
//!  
//!
//!
//!  
//! #  GF API Routines
//!
//!  
//!
//!
//!  
//! ##  High-level GF search routines
//!
//!  The high-level GF search routines constitute the principal application
//!    programming interface to the GF subsystem.
//!
//!  The routines described here are supported in all language versions of
//!    the SPICE Toolkit.
//!
//!  All of the routines listed below have extensive header documentation.
//!    Each header describes all input and output arguments and includes one or
//!    more example programs accompanied by example meta-kernels and
//!    corresponding program outputs.
//!
//!  Additional, more extensive code examples are presented at the end of
//!    this document.
//!
//!  The "GF Computational Recipes" chapter below provides hints on how to
//!    solve various geometric search problems using these routines.
//!
//!  The high-level GF search routines are:
//!
//!  
//!
//! *  [GFDIST](crate::raw::gfdist)
//!
//!
//!  Distance search: find time windows when a given observer-target
//! distance constraint is met.
//!
//!  *  [GFILUM](crate::raw::gfilum)
//!
//!
//!  Illumination angles: find time window over which a constraint on the
//! observed phase, solar incidence, or emission angle at a specified
//! target body surface point is met.
//!
//!  *  [GFOCLT](crate::raw::gfoclt)
//!
//!
//!  Occultation or transit search: find time windows when a given type of
//! occultation or transit is in progress.
//!
//!  *  [GFPA](crate::raw::gfpa)
//!
//!
//!  Phase angle: find time windows when a observer-target-illuminator phase
//! angle constraint is met.
//!
//!  *  [GFPOSC](crate::raw::gfposc)
//!
//!
//!  Observer-target position vector coordinate search: find time windows
//! when a given constraint on a specified coordinate (e.g. Cartesian X, Y,
//! Z or planetocentric radius, longitude, or latitude) of an
//! observer-target position vector is met.
//!
//!  *  [GFRFOV](crate::raw::gfrfov)
//!
//!
//!  Ray in instrument field of view search: find time windows when a given
//! ray emanating from an observer is contained in a specified instrument's
//! field of view.
//!
//!  *  [GFRR](crate::raw::gfrr)
//!
//!
//!  Range rate: find time windows when a given constraint on the range rate
//! of an observer to target position vector is met.
//!
//!  *  [GFSEP](crate::raw::gfsep)
//!
//!
//!  Angular separation search: find time windows when a given constraint on
//! the angular separation of two targets as seen by a specified observer
//! is met.
//!
//!  *  [GFSNTC](crate::raw::gfsntc)
//!
//!
//!  Ray-surface intercept coordinate search: find time windows when a
//! specified constraint on a coordinate of the surface intercept of a
//! specified ray on a target body is met.
//!
//!  *  [GFSTOL](crate::raw::gfstol)
//!
//!
//!  Set the GF subsystem convergence tolerance. The high level GF routines
//! use the default tolerance for the search. The user may change the
//! convergence tolerance from the default value by calling [GFSTOL](crate::raw::gfstol). All
//! subsequent searches using the high level routines will use the updated
//! tolerance value.
//!
//!  *  [GFSUBC](crate::raw::gfsubc)
//!
//!
//!  Sub-observer point coordinate search: find time windows when a
//! specified constraint on a coordinate of the sub-observer point on a
//! specified target body is met.
//!
//!  *  [GFTFOV](crate::raw::gftfov)
//!
//!
//!  Target body in instrument field of view search: find time windows when
//! a given target body appears in a specified instrument's field of view.
//!
//!  *  [GFUDB](crate::raw::gfudb)
//!
//!
//!  User-defined boolean quantity function: find time windows when a given
//! user-defined boolean value function equals true.
//!
//!  *  [GFUDS](crate::raw::gfuds)
//!
//!
//!  User-defined scalar quantity function: find time windows when a given
//! constraint on a user-defined scalar value function is met.
//!
//!     
//! ##  Mid-level GF search routines
//!
//!  The routines discussed here are provided only in the Fortran and C SPICE
//!    Toolkits. (Practical means of implementing these routines in IDL and
//!    MATLAB have not been found.)
//!
//!  The mid-level GF search routines are:
//!
//!  
//!
//! *  [GFEVNT](crate::raw::gfevnt)
//!
//!
//!  Scalar quantity search: find times when specified constraints on any
//! scalar quantity, such as distance or angular separation, are met.
//!
//!  *  [GFFOVE](crate::raw::gffove)
//!
//!
//!  FOV search: find times when a specified target appears in a specified
//! instrument FOV. This routine provides the functionality of both [GFTFOV](crate::raw::gftfov)
//! and [GFRFOV](crate::raw::gfrfov).
//!
//!  *  [GFOCCE](crate::raw::gfocce)
//!
//!
//!  Occultation or transit search: find times when a specified target body
//! occults or is in transit across another, as seen by a specified
//! observer.
//!
//!  These routines provide all of the functionality of the high-level search
//!    routines, plus several additional features:
//!
//!  
//!
//! *  Progress reporting
//!
//!
//!  Applications can control whether a "percent complete" progress report
//! is displayed during a GF search. By default, the report is displayed
//! via console I/O. Applications can override the default display by
//! passing custom progress reporting routines as input arguments to
//! mid-level GF search routines.
//!
//!  *  Interrupt handling
//!
//!
//!  Applications can control whether mid-level GF search routines test for
//! issuance of an interrupt command and abort if such a command is
//! detected. Due to the lack of interrupt handling support in ANSI
//! standard Fortran 77, applications can make use of this capability only
//! by passing a custom (non-standard) interrupt detection routine as an
//! input argument to mid-level GF search routines.
//!
//!  *  Set search step function
//!
//!
//!  Applications can override the default search step behavior by passing a
//! custom step size routine as an input argument to mid-level GF search
//! routines.
//!
//!  *  Set refinement function
//!
//!
//!  Applications can override the default root refinement algorithm (binary
//! search) by passing a custom root refinement routine as an input
//! argument to mid-level GF search routines.
//!
//!  *  Set convergence tolerance
//!
//!
//!  Convergence tolerance is an input argument to mid-level GF search
//! routines.
//!
//!     
//! ###  Rationale for calling mid-level GF search routines
//!
//!  The mid-level GF search routines are more complex than their high-level
//!    counterparts (considerably so in the case of [GFEVNT](crate::raw::gfevnt)). The main reason to
//!    use the mid-level routines is to take advantage of their progress
//!    reporting and interrupt handling capabilities.
//!
//!  GF searches can take a long time to complete, particularly when the
//!    confinement window is large and the step size is small. It may not be
//!    obvious to a user whether a running search is making progress at a
//!    reasonable rate. The default GF progress report, which when enabled
//!    updates approximately once per second, helps to answer this question.
//!
//!  If progress reporting is used in a GUI application, reports produced by
//!    the default mechanism are probably undesirable. In this case,
//!    application programs can pass custom progress reporting routines to the
//!    mid-level GF search routines.
//!
//!  When an interactive SPICE-based application runs a GF search, the user
//!    may want to abort the search without terminating the program, possibly
//!    because terminating the program would result in substantial loss of
//!    work. The GF interrupt handling capability allows an application program
//!    to quickly abort GF searches and have the GF system return control to
//!    the application.
//!
//!  
//!
//!
//!  
//! ###  Supporting utility routines
//!
//!  When the default GF progress reporting, interrupt handling, and
//!    root-finding functionality is desired, a calling application can call
//!    existing GF utility routines and, where applicable, pass them as actual
//!    input arguments to the mid-level search routines. These utilities are:
//!
//!  
//!
//! *  [GFSSTP](crate::raw::gfsstp)
//!
//!
//!  Set search step. This routine sets the step size that will be returned
//! by [GFSTEP](crate::raw::gfstep).
//!
//!  *  [GFSTEP](crate::raw::gfstep)
//!
//!
//!  Get search step. This routine returns step size that was last set by
//! [GFSSTP](crate::raw::gfsstp).
//!
//!  *  [GFREFN](crate::raw::gfrefn)
//!
//!
//!  Refine root bracketing interval. This routine returns the midpoint of
//! the input times; this behavior supports root finding by bisection.
//!
//!  *  [GFREPI](crate::raw::gfrepi)
//!
//!
//!  Initialize progress report.
//!
//!  *  [GFREPU](crate::raw::gfrepu)
//!
//!
//!  Update progress report.
//!
//!  *  [GFREPF](crate::raw::gfrepf)
//!
//!
//!  Finalize progress report.
//!
//!  *  [GFBAIL](crate::raw::gfbail)
//!
//!
//!  Detect interrupt. This function returns a logical value indicating
//! whether an interrupt has been detected.
//!
//!  Since ANSI standard Fortran 77 (unlike ANSI C) doesn't provide support
//!    for interrupt handling, the SPICELIB function [GFBAIL](crate::raw::gfbail) is a placeholder;
//!    SPICELIB users must override this function with their own custom routine
//!    to implement interrupt handling.
//!
//!  
//!
//!
//!  
//! ###  Overriding default behavior
//!
//!  Progress Reporting
//!
//!  To override the default progress reporting capability provided by the
//!    mid-level GF search routines, an application must pass in custom
//!    routines in place of [GFREPI](crate::raw::gfrepi), GRREPU, and [GFREPF](crate::raw::gfrepf). Each of the custom
//!    routines must have an argument list that exactly matches that of the
//!    default routine it overrides; see the headers of the default routines
//!    for details. These routines are entry points in the umbrella routine
//!    [GFRPRT](crate::raw::gfrprt). Note that the application must override all of the progress
//!    reporting routines in a given call, if it overrides any one of them.
//!
//!  To override the default interrupt handling capability, an application
//!    must pass in a custom routine in place of [GFBAIL](crate::raw::gfbail). The custom routine
//!    must have an argument list that exactly matches that of [GFBAIL](crate::raw::gfbail).
//!
//!  Step Size and Refinement Functions
//!
//!  The need to override the step size and refinement functions is expected
//!    to be quite unusual; it should be attempted only by programmers having a
//!    detailed knowledge of the GF search algorithms and the code that
//!    implements them.
//!
//!  
//!
//!
//!  
//! #  GF Computational Recipes
//!
//!  Below we provide terse descriptions of computational approaches for
//!    solving common geometric search problems.
//!
//!  The "recipes" below are very abbreviated; they're intended to be
//!    helpful to experienced SPICE users. New users are encouraged to first
//!    familiarize themselves with the example programs in the GF API headers
//!    and in this document.
//!
//!  Users should consult the headers of the pertinent SPICE routines for
//!    details on the use of those routines.
//!
//!  Note that for valid comparison of GF results against those obtained by
//!    alternate means, inputs such as kernel data, aberration corrections,
//!    reference frames, coordinate systems, confinement windows, and time
//!    systems used to represent time windows must be compatible.
//!
//!  
//!
//!
//!  
//! ##  Required SPICE kernels
//!
//!  With a few exceptions, the recipes below don't discuss the SPICE kernels
//!    required to carry out the described computations. Some general
//!    requirements are summarized here:
//!
//!  
//!
//! * SPK files containing ephemeris data for targets and observers are almost
//! always required; the only exception is the star visibility case where the
//! star's location is modeled as a direction rather than as a position vector.
//!
//!  *  When aberration corrections are used, sufficient ephemeris data must be
//! available to propagate states of the observer and targets to the solar
//! system barycenter. The states of the targets must be calculable at light
//! time corrected epochs, so the required coverage will extend beyond the
//! confinement window.
//!
//!  *  When stellar aberration corrections are used, coverage for the observer
//! must be available on a window whose intervals are expanded by one second
//! (in both directions) relative to the confinement window.
//!
//!  * Computations involving target body-fixed, body-centered reference frames
//! require PCK files providing orientation data for those reference frames.
//! Such computations often require PCK files containing size and shape data
//! for the target body as well. In many cases one PCK file can provide both
//! the necessary orientation and size/shape data.
//!
//!  *  When required body-fixed, body-centered reference frame specifications are
//! not built into the SPICE system, those specifications must be provided by
//! FK files.
//!
//!  * Computations involving topocentric reference frames centered at surface
//! points on extended objects require both SPK and FK files providing state
//! data for the surface point and topocentric frame orientation, respectively.
//! Usually these computations also require a PCK file providing orientation of
//! the extended object.
//!
//!  * Computations involving instrument pointing and FOV specifications normally
//! require all of the following: CK files, SCLK kernels, LSK files, FK files,
//! and IK files.
//!
//!     
//! ##  A note about CK data availability
//!
//!  CK files, particularly those containing reconstructed attitude data,
//!    often have coverage gaps. A SPICE-based application program can obtain
//!    the time window over which CK data are available by calling the SPICE
//!    routine [CKCOV](crate::raw::ckcov).
//!
//!  When the caller of [CKCOV](crate::raw::ckcov) requests that interval endpoints in the CK
//!    coverage time window be expressed as TDB seconds, [CKCOV](crate::raw::ckcov) must convert
//!    these endpoints from encoded SCLK (ticks) to TDB. Due to round-off
//!    errors, and in some cases, to discontinuities in the TDB-to-ticks
//!    mapping, the TDB values obtained via this call may not be translatable
//!    to tick values within the actual coverage window of the CK file.
//!
//!  For safety, applications obtaining TDB coverage windows via [CKCOV](crate::raw::ckcov) should
//!    call the SPICE window routine [WNCOND](crate::raw::wncond) to contract those windows by a
//!    duration large enough to ensure that the entire, contracted coverage
//!    window is usable.
//!
//!  For an SCLK kernel that provides a continuous TDB-to-ticks mapping, a
//!    contraction duration (having units of TDB seconds) equivalent to one
//!    tick normally should suffice, as long as the nominal tick duration is at
//!    least one microsecond.
//!
//!  For SCLK kernels having discontinuities, the required contraction
//!    duration can be determined by analyzing the possible mapping errors
//!    caused by those discontinuities; alternatively, it can be determined by
//!    trial and error. If a search is performed and required CK data are
//!    unavailable, SPICE routines will signal an error.
//!
//!  
//!
//!
//!  
//! ##  Geometric constraint searches
//!
//!  
//!
//!
//!  
//! ###  Periapse/Apoapse
//!
//!  To find the unique closest approach of an observer to a target over a
//!    specified time window, call [GFDIST](crate::raw::gfdist), specifying the
//!
//!  
//!
//! ```text
//!    'ABSMIN'
//! ```
//!
//!  (absolute minimum) relational operator. To find all of the "close
//!    approaches" of an observer to a target over a specified time window,
//!    use the
//!
//!  
//!
//! ```text
//!    'LOCMIN'
//! ```
//!
//!  (local minimum) relational operator.
//!
//!  For apoapse events, use the absolute or local maximum operators instead:
//!
//!  
//!
//! ```text
//!    'ABSMAX'
//!    'LOCMAX'
//! ```
//!
//!  See the example program MEDLEY below for details.
//!
//!  
//!
//!
//!  
//! ###  View periods
//!
//!  View periods may be defined as time intervals, within a confinement
//!    window, during which a target body has elevation greater than a
//!    specified limit with respect to the local horizontal plane at a given
//!    point on the surface of an extended body.
//!
//!  Compute view periods using [GFPOSC](crate::raw::gfposc). See the example program MEDLEY below
//!    for details.
//!
//!  In the [GFPOSC](crate::raw::gfposc) call, aberration corrections should be set to be
//!    compatible with the direction of radiation travel: either "reception"
//!    or "transmission" corrections can be selected. Normally both light
//!    time and stellar aberration corrections should be used; the aberration
//!    correction input string should be either of
//!
//!  
//!
//! ```text
//!    'LT+S'
//!    'XLT+S'
//! ```
//!
//!  To find the time window when the target is "visible" for both
//!    reception and transmission, run the search twice, using both aberration
//!    correction choices. The result window from the first search can be used
//!    as the confinement window for the second.
//!
//!  SPICE doesn't have the capability of modeling atmospheric effects, so
//!    for observers on bodies having atmospheres, view periods found using the
//!    GF subsystem will be subject to errors due to this deficiency.
//!
//!  "Usable" view periods may be a subset of those found by GF searches,
//!    since there may be pointing limitations on the antenna or instrument
//!    viewing the target.
//!
//!  
//!
//!
//!  
//! ###  Sub-observer point
//!
//!  Use [GFSUBC](crate::raw::gfsubc) to find times when the sub-observer point on an extended
//!    target satisfies the constraints
//!
//!  
//!
//! ```text
//!    min_lon < sub-observer longitude < max_lon
//!    min_lat < sub-observer latitude  < max_lat
//! ```
//!
//!  Four searches are required: one for each constraint.
//!
//!  The searches can be cascaded: the result window for one search can be
//!    used as the confinement window for the next.
//!
//!  If the longitude interval of interest includes 180 degrees, then
//!
//!  
//!
//! ```text
//!          min_lon > max_lon
//! ```
//!
//!  and the corresponding longitude constraints have the form
//!
//!  
//!
//! ```text
//!          min_lon < sub-observer longitude
//!    OR    max_lon > sub-observer longitude
//! ```
//!
//!  In this case the solution window for the longitude constraints is the
//!    union of the solution windows for the two constraints shown above; use [WNUNID](crate::raw::wnunid) to compute this union. The union can then be used as the
//!    confinement window for a latitude search.
//!
//!  The case of a right ascension interval containing 0 degrees is handled
//!    analogously.
//!
//!  The order of the searches can be important: often constraints on one of
//!    the coordinates produce a smaller result window than constraints on the
//!    other. For example, for a polar orbiter, latitude constraints may be
//!    satisfied over a small fraction of the search window, so searching for
//!    times when the latitude constraints are met would yield a small window
//!    over which the longitude searches would be conducted. For an equatorial
//!    orbiter, the situation would be reversed.
//!
//!  
//!
//!
//!  
//! ###  Instrument boresight intercept
//!
//!  Use [GFSNTC](crate::raw::gfsntc) to find times when the intercept on an extended target body
//!    of a ray emanating from an observing instrument's location and aligned
//!    with the instrument's boresight satisfies the constraints
//!
//!  
//!
//! ```text
//!    min_lon < intercept longitude < max_lon
//!    min_lat < intercept latitude  < max_lat
//! ```
//!
//!  Four searches are required: one for each constraint.
//!
//!  See the discussion of alternate longitude constraints and of search
//!    order above in the section titled "Sub-observer point."
//!
//!  Note that pointing stability can be an issue for boresight intercept
//!    searches: the pointing must be stable enough so that the GF system can
//!    compute the time window, within the confinement window, during which the
//!    ray-surface intercept exists. High-frequency pointing excursions can
//!    cause this "existence window" computation to produce invalid results,
//!    which in turn will cause the requested coordinate constraint searches to
//!    either fail before completion or to complete but produce invalid
//!    results.
//!
//!  [GFSNTC](crate::raw::gfsntc) should not be used for near-tangent ray direction cases. GFSNTC
//!    contracts the existence window described above by a fraction of a second
//!    to avoid geometric singularities; this affords more robust search
//!    behavior for normal cases but prevents [GFSNTC](crate::raw::gfsntc) from producing accurate
//!    results for near-tangent ray pointing.
//!
//!  
//!
//!
//!  
//! ###  Planet in instrument field of view
//!
//!  Use [GFTFOV](crate::raw::gftfov) to find times when an ephemeris object is in the FOV of an
//!    instrument, provided this FOV can be modeled as one of the shapes
//!    supported by the SPICE routine [GETFOV](crate::raw::getfov). The target shape can be treated
//!    as an ellipsoid or a point.
//!
//!  [GFTFOV](crate::raw::gftfov) may not be suitable for FOV searches involving push-broom
//!    cameras. For an alternate approach, see the example program ROVER below
//!    for a demonstration of a search involving the MRO HIRISE camera.
//!
//!  
//!
//!
//!  
//! ###  Star in instrument field of view
//!
//!  Use [GFRFOV](crate::raw::gfrfov) to find times when a target modeled as a ray (that is, the
//!    direction to the target is available, the distance to the target is not)
//!    is in the FOV of an instrument, provided this FOV can be modeled as one
//!    of the shapes supported by the SPICE routine [GETFOV](crate::raw::getfov).
//!
//!  
//!
//!
//!  
//! ###  Spacecraft occultation or transit
//!
//!  Use [GFOCLT](crate::raw::gfoclt) to search for spacecraft occultations or transits. If the
//!    spacecraft is the target, the spacecraft shape can be modeled as a
//!    point. The blocking body must be modeled as an ellipsoid.
//!
//!  [GFOCLT](crate::raw::gfoclt) assumes straight-line light paths for occultation searches. This
//!    assumption may not be suitable for high-accuracy work.
//!
//!  
//!
//!
//!  
//! ###  Natural satellite occultation or transit
//!
//!  Use [GFOCLT](crate::raw::gfoclt) to search for natural satellite occultations or transits.
//!    Both satellite and planet should be modeled as ellipsoids.
//!
//!  
//!
//!
//!  
//! ###  Spacecraft eclipse
//!
//!  Defining a spacecraft eclipse as the presence of the spacecraft in the
//!    shadow created by the Sun and a blocking body, one can observe that
//!    eclipses are equivalent to occultations, where the spacecraft is the
//!    observer, the Sun is the "back" body, and the blocking body is the
//!    "front" body.
//!
//!  Use [GFOCLT](crate::raw::gfoclt) to search for spacecraft eclipses.
//!
//!  Both the Sun and the blocking body should be modeled as ellipsoids.
//!
//!  Set the occultation type to
//!
//!  
//!
//! ```text
//!    'ANY'
//! ```
//!
//!  to search for times when the spacecraft is in penumbral or umbral
//!    eclipse; set the occultation type to
//!
//!  
//!
//! ```text
//!    'FULL'
//! ```
//!
//!  to search for times when the spacecraft is in umbral eclipse.
//!
//!  [GFOCLT](crate::raw::gfoclt) assumes straight-line light paths for occultation searches. This
//!    assumption may not be suitable for high-accuracy work.
//!
//!  
//!
//!
//!  
//! ###  Surface point eclipse
//!
//!  Searches for eclipses of a surface point on an extended object can be
//!    conducted using [GFOCLT](crate::raw::gfoclt), as long as the position of the surface point is
//!    given by an SPK file. Use the SPICE utility PINPOINT to create an SPK
//!    file for the surface point if necessary; then proceed as described in
//!    the above "Spacecraft eclipse" discussion.
//!
//!  
//!
//!
//!  
//! ###  Equator crossing
//!
//!  Use [GFPOSC](crate::raw::gfposc) to find times when one body crosses the equatorial plane of
//!    another.
//!
//!  The reference frame should be the body-fixed, body-centered frame
//!    associated with the body whose equatorial plane is of interest.
//!
//!  The coordinate system and coordinate can be set, respectively, to
//!
//!  
//!
//! ```text
//!    'RECTANGULAR'
//!    'Z'
//! ```
//!
//!  Use the relational description "Z = 0" for the search.
//!
//!  Other choices such as
//!
//!  
//!
//! ```text
//!    'LATITUDINAL'
//!    'LATITUDE'
//! ```
//!
//!  will yield the same results, up to round-off errors.
//!
//!  Use the relational description "LATITUDE = 0" for the search.
//!
//!  Note that for a given pair of bodies, when aberration corrections are
//!    used, the choice of observer and target affects the result, since
//!    aberration corrections are not anti-symmetric functions of target and
//!    observer.
//!
//!  See the Fundamental Concepts SPICE tutorial and the header of [SPKEZR](crate::raw::spkezr) for
//!    further information on aberration corrections.
//!
//!  
//!
//!
//!  
//! ###  Meridian crossing
//!
//!  Use [GFPOSC](crate::raw::gfposc) to find times when one body crosses a given meridian of the
//!    body-fixed, body-centered reference frame of another.
//!
//!  Care must be taken to identify the appropriate coordinate system: is
//!    longitude positive East or positive West?
//!
//!  For the positive East longitude case, the coordinate system and
//!    coordinate can be set to
//!
//!  
//!
//! ```text
//!    'LATITUDINAL'
//!    'LONGITUDE'
//! ```
//!
//!  respectively.
//!
//!  For the positive West longitude case, planetographic longitude can be
//!    used, but in some cases, additional set-up is required.
//!
//!  If the central body is not the Earth, Moon, Sun, or a body with
//!    retrograde spin, the selections
//!
//!  
//!
//! ```text
//!    'PLANETOGRAPHIC'
//!    'LONGITUDE'
//! ```
//!
//!  can be used as is.
//!
//!  Use the relational description "LONGITUDE = value" for the search, where
//!    value is the angular value for the meridian, expressed in radians.
//!
//!  The Earth, Moon, Sun, and bodies with retrograde spin are special cases,
//!    because for these objects planetographic longitude is positive East by
//!    default. However, this default can be overridden via kernel pool
//!    assignments: an application can force planetographic longitude for a
//!    given body to increase in the desired sense. See the header of [RECPGR](crate::raw::recpgr)
//!    for details. If these assignments are made, then the above choices of
//!    coordinate system and coordinate will work for these special cases as
//!    well.
//!
//!  See the notes on aberration corrections in the section titled "Equator
//!    crossing" above.
//!
//!  
//!
//!
//!  
//! ###  Elongation
//!
//!  Use [GFSEP](crate::raw::gfsep) to find times when target body elongation constraints are met,
//!    given a target body and observer. The Sun is the second target.
//!
//!  
//!
//!
//!  
//! ###  Orbital longitude of a satellite
//!
//!  This recipe requires the user to create two dynamic reference frame
//!    specifications in a frame kernel. See the "Dynamic Frames" tutorial
//!    and the Frames Required Reading, [frames.req](crate::required_reading::frames), for detailed discussions of
//!    this topic.
//!
//!  The participants in this geometric relationship are an observer, a
//!    central body, and a satellite orbiting the central body. For this
//!    geometric case, "orbital longitude" is measured in the orbital plane
//!    of the satellite, in the positive sense about the satellite's angular
//!    velocity vector, with the zero longitude direction aligned with the
//!    orthogonal projection of the observer-central body vector onto the
//!    satellite's orbital plane. This definition is applicable, for example,
//!    when the Earth is the observer, Mars is the central body, and Phobos is
//!    the satellite.
//!
//!  There is a different definition of orbital longitude for the case where
//!    the target is a planet, asteroid, or comet and the Earth is the
//!    observer: for this case, the Sun-Earth vector points in direction of
//!    zero longitude. We won't address this case, but it can be handled by a
//!    simple modification of the ORBITAL_LONG_FRAME we describe below.
//!
//!  The first step is to specify a two-vector dynamic frame ORBIT_FRAME
//!    whose primary axis is aligned with the central body-satellite position
//!    vector; this is the frame's +X axis. Associate the secondary axis with
//!    the central body-satellite velocity vector; this is the frame's +Y axis.
//!    The +Z axis of ORBIT_FRAME is then aligned with the instantaneous
//!    angular velocity of the satellite's orbit.
//!
//!  Next, specify a two-vector dynamic frame ORBITAL_LONG_FRAME whose
//!    primary axis is aligned with the +Z axis of ORBIT_FRAME; this is the +Z
//!    axis of ORBITAL_LONG_FRAME. Associate the secondary axis of
//!    ORBITAL_LONG_FRAME with the observer-central body position vector; this
//!    is the +X axis of ORBITAL_LONG_FRAME.
//!
//!  Finally, call [GFPOSC](crate::raw::gfposc) to search for times when the satellite's orbital
//!    longitude satisfies constraints of interest. For these searches, the
//!    observer is the central body, the target is the satellite, the reference
//!    frame is ORBITAL_LONG_FRAME, and the coordinate system and coordinate
//!    are, respectively, set to:
//!
//!  
//!
//! ```text
//!    'LATITUDINAL'
//!    'LONGITUDE'
//! ```
//!
//!  Aberration corrections should not be used for this application.
//!
//!  
//!
//!
//!  
//! ###  Approximate times of Cassini Saturn ring occultations
//!
//!  An approximation of Saturn ring occultation ingress and egress times for
//!    the Cassini orbiter, as viewed by a given Deep Space Network (DSN)
//!    station, can be found using [GFOCLT](crate::raw::gfoclt).
//!
//!  For the purpose of this search, the ring boundaries can be approximated
//!    using two extremely flat spheroids, both of which are aligned, as is
//!    Saturn, with the IAU_SATURN reference frame. The large and small
//!    spheroids have equatorial radii equal to, respectively, the radii of the
//!    outer and inner ring boundaries. The polar radii can be set to 1 cm.
//!
//!  Two searches using [GFOCLT](crate::raw::gfoclt) are required: the first search finds the time
//!    window when the orbiter is occulted by the larger spheroid. The result
//!    window from that search can be used as the confinement window for the
//!    second search, which finds the time window when the orbiter is occulted
//!    by the inner spheroid.
//!
//!  Subtracting the result window of the second search from that of the
//!    first yields a window representing the time period of the ring
//!    occultations.
//!
//!  To avoid having to create new SPICE kernels representing the
//!    trajectories, orientations, sizes, shapes, names, and ID codes of the
//!    spheroids, one simply creates them by temporarily changing the radii of
//!    Saturn.
//!
//!  The radii of the spheroids can be set before each [GFOCLT](crate::raw::gfoclt) call by either:
//!
//!  
//!
//! * Loading a text kernel assigning the desired radii to the kernel variable
//! BODY699_RADII
//!
//!  * Calling [PDPOOL](crate::raw::pdpool) to assign the desired radii to the kernel variable
//! BODY699_RADII
//!
//!  Normally the application should restore Saturn's original radii after
//!    the second search has been completed.
//!
//!  For each of the searches, the DSN station is the observer, Saturn (with
//!    modified radii) is the "front" target body, and the Cassini orbiter is
//!    the "back" target body. The aberration correction should be set to
//!
//!  
//!
//! ```text
//!    'CN'
//! ```
//!
//!  The method described here will not work for edge-on or nearly edge-on
//!    viewing geometry: the ray-spheroid intercept computation fails to model
//!    the real occultation geometry in the first case and is too unstable to
//!    provide accurate results in the second.
//!
//!  The assumption of straight-line radiation paths may also be unsuitable
//!    for very high-accuracy work.
//!
//!  
//!
//!
//!  
//! ###  Angular offset between instrument boresight and velocity
//!
//!  Although the GF subsystem doesn't directly support searches involving
//!    coordinates of velocity vectors, one can use [GFPOSC](crate::raw::gfposc) to find times when
//!    the angular separation of a spacecraft-mounted instrument's boresight
//!    vector and the instrument's (inertially referenced) velocity satisfies
//!    specified constraints.
//!
//!  The first step is to create an SPK file for an artificial object whose
//!    position relative to the spacecraft's center of mass is parallel to the
//!    instrument's boresight direction. The SPICE utility PINPOINT can be used
//!    to create such an SPK file.
//!
//!  Next, create a specification for a dynamic reference frame whose +Z axis
//!    is aligned with the spacecraft velocity vector. The view frame example
//!    in the Dynamic Frames tutorial demonstrates this.
//!
//!  The colatitude of the vector from the spacecraft to the artificial
//!    object, expressed in the view frame, is the desired angular separation.
//!    An application program can call [GFPOSC](crate::raw::gfposc) with the coordinate system and
//!    coordinate, respectively, set to
//!
//!  
//!
//! ```text
//!    'SPHERICAL'
//!    'COLATITUDE'
//! ```
//!
//!  to conduct the search.
//!
//!  
//!
//!
//!  
//! #  Common GF Problems
//!
//!  Here we discuss some common problems that may arise when SPICE-based
//!    applications the use the GF subsystem.
//!
//!  
//!
//!
//!  
//! ##  A challenge
//!
//!  One noteworthy difference between debugging GF search problems and other
//!    types of computational problems is that GF searches don't assist the
//!    programmer by returning invalid geometric parameters; they just return
//!    time windows. While it can be obvious that a given distance or angle is
//!    incorrect, it's often much harder to determine, without much
//!    investigative work, that a given set of time intervals is incorrect.
//!
//!  The conclusion to draw is that preventing problems by correctly setting
//!    up one's work is even more important for GF searches than for other
//!    types of computations.
//!
//!  
//!
//!
//!  
//! ##  Wrong SPICE kernels
//!
//!  This is not a GF-specific issue, but it's one of the most common
//!    problems that occurs in SPICE applications. Using the correct SPICE
//!    kernel versions can make all the difference when trying to determine
//!    event times.
//!
//!  
//!
//!
//!  
//! ##  Insufficient kernel data
//!
//!  As with most work performed with SPICE, it's not uncommon for GF
//!    searches to terminate due to missing kernel data.
//!
//!  Some of the common short error messages indicating missing data are:
//!
//!  
//!
//! ```text
//!    SPICE(NOTRANSLATION)
//!    SPICE(NOFRAME)
//!    SPICE(NOFRAMECONNECT)
//!    SPICE(FRAMEDATANOTFOUND)
//!    SPICE(SPKINSUFFDATA)
//!    SPICE(KERNELVARNOTFOUND)
//! ```
//!
//!  In many cases, a careful reading of the SPICE long error message will
//!    indicate the cause of the problem.
//!
//!  Since it can be frustrating (or worse) to have a search run for a long
//!    time, and then have the search terminate due to missing data, we
//!    recommend that users verify that the required data are present before
//!    starting a search.
//!
//!  The section titled "Required SPICE kernels" in the chapter "GF
//!    Computational Recipes" may be helpful.
//!
//!  Often it's worthwhile to manually verify the coverage of the SPK and CK
//!    files intended to be used in a search; this can be done using the SPICE
//!    utilities BRIEF and CKBRIEF. See the user's guides [brief.ug](crate::raw::brief.ug) and
//!    [ckbrief.ug](crate::raw::ckbrief.ug) for details.
//!
//!  It can be very useful for an application to determine a time window over
//!    which required SPK and CK data are available. See the discussion and
//!    example code dealing with this task in the ROVER code example below.
//!
//!  
//!
//!
//!  
//! ##  Missed events
//!
//!  Here are some simple reasons why a GF search might fail to find events
//!    that you know did occur:
//!
//!  
//!
//! * Kernel versions are wrong. For example, an out-of-date predict SPK or CK
//! file can yield completely wrong viewing geometry.
//!
//!  * The step size is too long. See the discussion of search step size in the
//! "GF Concepts" chapter.
//!
//!  *  Note that proper understanding of the underlying geometry is crucial for
//! correct step size selection. For example, incorrect assumptions about the
//! period of a numeric quantity can lead to selecting a step size that's too
//! large to capture all of the local extrema of the quantity.
//!
//!  * The confinement window is incorrect. If the event does occur, but not
//! during the confinement window you're passing to the GF search routine, it
//! won't be found.
//!
//!     
//! ##  Slow performance
//!
//!  Slow performance may be due to an excessively small step size. See the
//!    step size discussion in the "GF concepts" chapter to get an idea of
//!    the step size requirements for your search.
//!
//!  Slow performance is not necessarily indicative of an error.
//!
//!  For a long search, it may not be evident just how slow the performance
//!    really is; one may only know that whatever fraction of the search has
//!    been completed has already taken a long time.
//!
//!  Users of the Fortran and C SPICE Toolkits can use the mid-level search
//!    routines and enable progress reporting to determine a search's rate of
//!    progress.
//!
//!  All SPICE users can shorten the confinement window until a search
//!    completes in a short time, then extrapolate the time required for the
//!    entire search.
//!
//!  
//!
//!
//!  
//! ##  Constraints not met on result window
//!
//!  New GF users may be surprised to learn that constraints are not
//!    necessarily met by times at the endpoints of, of even slightly inside,
//!    the intervals comprising the result window.
//!
//!  See the discussion of time windows and window contraction in the "GF
//!    Concepts" chapter.
//!
//!  
//!
//!
//!  
//! ##  Result window intervals appear invalid
//!
//!  There are a number of reasons why a GF search can return a result window
//!    that appears "just plain wrong."
//!
//!  Possible causes include:
//!
//!  
//!
//! * Invalid SPICE kernels---bad data or wrong versions.
//!
//!  * Step size is too long, causing events to be missed or multiple events to be
//! seen as a single event.
//!
//!  * The search is attempting to extract results from noisy data. For example,
//! it's difficult to find correct local extrema of the light-time corrected
//! range rate (note: not yet implemented in SPICE) of the Moon relative to the
//! Earth; near the times when the extrema occur, the variation of the
//! quantity, as SPICE computes it, is on the same scale as the noise in the
//! quantity.
//!
//!  * Models used by SPICE differ from those expected or those used in a search
//! done using means other than SPICE. For example, in some cases, occultation
//! times computed with spherical target models can differ by tens of minutes
//! from those computed with ellipsoidal models.
//!
//!     
//! #  GF Example Programs
//!
//!  The next several sections present example programs that illustrate use
//!    of GF routines to solve realistic geometry problems.
//!
//!  All routines used in the examples are from SPICE.
//!
//!  The numerical results shown for these examples may differ across
//!    platforms. The results depend on the SPICE kernels used as input, the
//!    compiler and supporting libraries, and the machine specific arithmetic
//!    implementation.
//!
//!  
//!
//!
//!  
//! ##  Program MEDLEY: Searches for Periapse, Occultation, Rise/Set
//!
//!  
//!
//!
//!  
//! ###  Overview
//!
//!  This example program demonstrates use of the GF subsystem to perform
//!    three relatively simple tasks:
//!
//!  
//!
//! * Find times of periapse of the Earth relative to the Sun over a specified
//! decade.
//!
//!  * Find times when Titan is at least partially occulted by Saturn as seen from
//! DSS-14, on a specified day. Occultations of duration less than ten minutes
//! are ignored.
//!
//!  * Find times when Saturn is visible from DSS-14, over a specified 5-day
//! period. Saturn is considered to be visible when its elevation is above 6
//! degrees. These periods of visibility are sometimes called "view periods."
//!
//!  *  The SPICE system doesn't support modeling of atmospheric effects such as
//! refraction, so the target rise and set times found by this search are
//! approximate.
//!
//!  In the interest of brevity, both of the example code and of the
//!    discussion, the example program below combines the solutions of the
//!    above (unrelated) problems.
//!
//!  
//!
//!
//!  
//! ###  Aberration corrections
//!
//!  For the Earth-Sun periapse computation, the goal is to find the local
//!    minima of distance given by the planetary ephemeris, as opposed to the
//!    apparent local minima, so no aberration corrections are used.
//!
//!  For the occultation search, only light time corrections are needed.
//!    Normally computations involving apparent geometry of extended objects
//!    require correction of target positions for light time and stellar
//!    aberration, so the aberration correction flag
//!
//!  
//!
//! ```text
//!    'LT+S'
//! ```
//!
//!  would be used. However, stellar aberration corrections are unnecessary
//!    for occultation computations, since the respective stellar aberration
//!    corrections for the two targets are identical at the point of tangency
//!    of the figures of the targets. For this reason the GF occultation
//!    routine [GFOCLT](crate::raw::gfoclt) ignores the stellar aberration correction token
//!
//!  
//!
//! ```text
//!    '+S'
//! ```
//!
//!  if it's provided.
//!
//!  For the view period search, the apparent position of Saturn is used, so
//!    both light time and stellar aberration corrections are applied.
//!
//!  
//!
//!
//!  
//! ###  SPICE kernels
//!
//!  The meta-kernel used for this example is shown below.
//!
//!  
//!
//! ```text
//!  
//!    KPL/MK
//!  
//!       File: medley.tm
//!  
//!       Meta-kernel for example program MEDLEY.
//!  
//!       This meta-kernel is intended to support operation of SPICE
//!       example programs. The kernels shown here should not be
//!       assumed to contain adequate or correct versions of data
//!       required by a user's own SPICE-based applications.
//!  
//!       In order for an application to use this meta-kernel, the
//!       kernels referenced here must be present in the user's
//!       current working directory.
//!  
//!       The names and contents of the kernels referenced
//!       by this meta-kernel are as follows:
//!  
//!          File name                        Contents
//!          ---------                        --------
//!          naif0009.tls                     Leapseconds
//!          pck00008.tpc                     Planet orientation and
//!                                           radii
//!          de421.bsp                        Planetary ephemeris
//!          sat288.bsp                       Saturn satellite ephemeris
//!          earthstns_itrf93_050714.bsp      DSN station locations
//!          earth_topo_050714.tf             DSN station topocentric
//!                                           frame specifications
//!          earth_070425_370426_predict.bpc  Long term, low-accuracy
//!                                           Earth orientation
//!  
//!       Version 1.0.0 23-JAN-2009 (NJB)
//!  
//!    \begindata
//!  
//!       KERNELS_TO_LOAD = (
//!                           'naif0009.tls'
//!                           'pck00008.tpc'
//!                           'de421.bsp'
//!                           'sat288.bsp'
//!                           'earthstns_itrf93_050714.bsp'
//!                           'earth_topo_050714.tf'
//!                           'earth_070425_370426_predict.bpc'
//!                         )
//!    \begintext
//!  
//!    [End of kernel]
//!  
//! ```
//!
//!     
//! ###  Source code
//!
//!  Example source code begins here.
//!
//!  
//!
//! ```text
//!          PROGRAM MEDLEY
//!          IMPLICIT NONE
//!  
//!    C
//!    C     SPICELIB functions
//!    C
//!          DOUBLE PRECISION      RPD
//!          DOUBLE PRECISION      SPD
//!  
//!          INTEGER               WNCARD
//!  
//!    C
//!    C     Global parameters
//!    C
//!          INCLUDE 'gf.inc'
//!  
//!    C
//!    C     Local parameters
//!    C
//!          CHARACTER*(*)         META
//!          PARAMETER           ( META   = 'medley.tm' )
//!  
//!          CHARACTER*(*)         TIMFMT
//!          PARAMETER           ( TIMFMT =
//!         .                'YYYY MON DD HR:MN:SC.###### TDB::RND::TDB')
//!  
//!          INTEGER               BDNMLN
//!          PARAMETER           ( BDNMLN = 36 )
//!  
//!          INTEGER               FRNMLN
//!          PARAMETER           ( FRNMLN = 32 )
//!  
//!          INTEGER               CORLEN
//!          PARAMETER           ( CORLEN = 10 )
//!  
//!          INTEGER               CRDLEN
//!          PARAMETER           ( CRDLEN = 25 )
//!  
//!          INTEGER               LBCELL
//!          PARAMETER           ( LBCELL = -5 )
//!  
//!          INTEGER               LNSIZE
//!          PARAMETER           ( LNSIZE = 78 )
//!  
//!          INTEGER               MAXWIN
//!          PARAMETER           ( MAXWIN = 200000 )
//!  
//!          INTEGER               RLTLEN
//!          PARAMETER           ( RLTLEN = 10 )
//!  
//!          INTEGER               SYSLEN
//!          PARAMETER           ( SYSLEN = 25 )
//!  
//!    C
//!    C     Local variables
//!    C
//!          CHARACTER*(CORLEN)    ABCORR
//!          CHARACTER*(FRNMLN)    BACK
//!          CHARACTER*(FRNMLN)    BFRAME
//!          CHARACTER*(SHPLEN)    BSHAPE
//!          CHARACTER*(CRDLEN)    COORD
//!          CHARACTER*(SYSLEN)    CRDSYS
//!          CHARACTER*(FRNMLN)    FFRAME
//!          CHARACTER*(FRNMLN)    FRAME
//!          CHARACTER*(BDNMLN)    FRONT
//!          CHARACTER*(SHPLEN)    FSHAPE
//!          CHARACTER*(BDNMLN)    OBSRVR
//!          CHARACTER*(LNSIZE)    OUTLIN
//!          CHARACTER*(RLTLEN)    RELATE
//!          CHARACTER*(BDNMLN)    TARGET
//!  
//!          DOUBLE PRECISION      ADJUST
//!          DOUBLE PRECISION      CNFINE ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      ET0
//!          DOUBLE PRECISION      ET1
//!          DOUBLE PRECISION      FINISH
//!          DOUBLE PRECISION      REFVAL
//!          DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      START
//!          DOUBLE PRECISION      STEP
//!          DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWSEP )
//!  
//!          INTEGER               I
//!  
//!  
//!  
//!    C
//!    C     Set up: load kernels for all tasks.
//!    C
//!          CALL FURNSH ( META )
//!  
//!  
//!    C ******************************************************************
//!    C     First task: find closest approaches of the Earth
//!    C     to the Sun during the time period 2009-2019.
//!    C ******************************************************************
//!  
//!    C
//!    C     Initialize windows.
//!    C
//!          CALL SSIZED ( MAXWIN, CNFINE )
//!          CALL SSIZED ( MAXWIN, RESULT )
//!  
//!    C
//!    C     Create a confinement window for the distance
//!    C     search. This window contains the start and stop times
//!    C     of the search interval.
//!    C
//!          CALL STR2ET ( '2009 JAN 1', ET0 )
//!          CALL STR2ET ( '2019 JAN 1', ET1 )
//!  
//!          CALL WNINSD ( ET0, ET1, CNFINE )
//!  
//!    C
//!    C     Set the observer and target.
//!    C
//!          OBSRVR = 'EARTH'
//!          TARGET = 'SUN'
//!  
//!    C
//!    C     We're looking for the distance given by the planetary
//!    C     ephemeris, not the apparent distance, so we'll use
//!    C     geometric states.
//!    C
//!          ABCORR = 'NONE'
//!  
//!    C
//!    C     The relational operator for this search is "local
//!    C     minimum." The reference value is unused; simply
//!    C     initialize it to zero.
//!    C
//!          RELATE = 'LOCMIN'
//!          REFVAL = 0.D0
//!  
//!    C
//!    C     Set the step size for this search. The step must
//!    C     be shorter than the shortest interval over which
//!    C     the distance is increasing or decreasing.
//!    C     We pick a conservative value: 100 days. Units
//!    C     expected by SPICE are TDB seconds.
//!    C
//!          STEP   = 100 * SPD()
//!  
//!    C
//!    C     The adjustment value isn't used for this search;
//!    C     set it to 0.
//!    C
//!          ADJUST = 0.D0
//!  
//!    C
//!    C     The work space array has dimensions
//!    C
//!    C         ( LBCELL : MAXWIN,  NWDIST )
//!    C
//!    C     where NWDIST is defined in gf.inc. We supply
//!    C     the upper bounds MAXWIN and NWDIST as input
//!    C     arguments to GFDIST.
//!    C
//!    C     Execute the search.
//!    C
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Starting distance search.'
//!  
//!          CALL GFDIST ( TARGET, ABCORR, OBSRVR, RELATE,
//!         .              REFVAL, ADJUST, STEP,   CNFINE,
//!         .              MAXWIN, NWDIST, WORK,   RESULT )
//!  
//!          WRITE (*,*) 'Done.'
//!  
//!    C
//!    C     Display the times of the local minima of distance.
//!    C
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Times of closest approach of Earth to Sun:'
//!          WRITE (*,*) ' '
//!  
//!          DO I = 1, WNCARD(RESULT)
//!  
//!             OUTLIN = ' '
//!    C
//!    C        Fetch the start and stop times of the Ith
//!    C        interval from the window RESULT.
//!    C
//!             CALL WNFETD ( RESULT, I, START, FINISH )
//!  
//!    C
//!    C        The result window's intervals are singletons,
//!    C        so we display only the start times.
//!    C
//!             CALL TIMOUT ( START, TIMFMT, OUTLIN(3: ) )
//!  
//!             WRITE (*,*) OUTLIN
//!  
//!          END DO
//!  
//!  
//!  
//!    C ******************************************************************
//!    C     Second task: find occultations of Titan by Saturn,
//!    C     as seen from DSS-14, for the time period January, 2009.
//!    C ******************************************************************
//!  
//!    C
//!    C     Find times when Titan is at least partially occulted
//!    C     by Saturn as seen by the observer. The occultation
//!    C     type 'ANY' indicates that any overlap of the back
//!    C     target by the front will be considered an occultation.
//!    C
//!    C     Create a confinement window for the view period
//!    C     search. This window contains the start and stop times
//!    C     of the search interval.
//!    C
//!    C     Empty the window CNFINE, then insert the new time bounds.
//!    C
//!          CALL SCARDD ( 0, CNFINE )
//!  
//!          CALL STR2ET ( '2009 JAN 1',  ET0 )
//!          CALL STR2ET ( '2010 JAN 1',  ET1 )
//!  
//!          CALL WNINSD ( ET0, ET1, CNFINE )
//!  
//!    C     The step size for the occultation search must be
//!    C     short enough to catch any occultation of interest.
//!    C     We'll look for occultations lasting at least
//!    C     one hour. Units are seconds.
//!    C
//!          STEP   = 3600.D0
//!  
//!    C
//!    C     Set the observer for the occultation search.
//!    C
//!          OBSRVR = 'DSS-14'
//!  
//!    C
//!    C     Set the front and back targets, their shapes,
//!    C     and their body-fixed reference frame names.
//!    C
//!          FRONT  = 'SATURN'
//!          FSHAPE = 'ELLIPSOID'
//!          FFRAME = 'IAU_SATURN'
//!  
//!          BACK   = 'TITAN'
//!          BSHAPE = 'ELLIPSOID'
//!          BFRAME = 'IAU_TITAN'
//!  
//!    C
//!    C     Occultations occur when one apparent object is
//!    C     behind another. Normally we'd use light time and
//!    C     stellar aberration corrections for this case, but
//!    C     stellar aberration corrections are not needed for
//!    C     accurate occultation computations, since at ingress
//!    C     or egress, the respective corrections for target
//!    C     and observer are equal along the direction from
//!    C     the observer to the point of tangency of the
//!    C     figures of the targets. So only light time
//!    C     corrections are used.
//!    C
//!          ABCORR = 'LT'
//!  
//!    C
//!    C     Note that GFOCLT, like the other GF binary
//!    C     state search routines, doesn't use a workspace
//!    C     array, hence there are no workspace dimension
//!    C     inputs.
//!    C
//!          WRITE (*,*) ' '
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Starting Titan occultation search.'
//!  
//!          CALL GFOCLT ( 'ANY',
//!         .              FRONT,  FSHAPE, FFRAME,
//!         .              BACK,   BSHAPE, BFRAME,
//!         .              ABCORR, OBSRVR, STEP,
//!         .              CNFINE, RESULT         )
//!  
//!          WRITE (*,*) 'Done.'
//!          WRITE (*,*) ' '
//!  
//!          IF ( WNCARD(RESULT) .EQ. 0 ) THEN
//!  
//!             WRITE (*,*) 'No occultations were found.'
//!          ELSE
//!             WRITE (*,*) 'Times of occultation of Titan by Saturn:'
//!             WRITE (*,*) ' '
//!  
//!             DO I = 1, WNCARD(RESULT)
//!  
//!                OUTLIN = ' '
//!    C
//!    C           Fetch the start and stop times of the Ith
//!    C           interval from the window RESULT.
//!    C
//!                CALL WNFETD ( RESULT, I, START, FINISH )
//!  
//!                CALL TIMOUT ( START,  TIMFMT, OUTLIN(3: ) )
//!                CALL TIMOUT ( FINISH, TIMFMT, OUTLIN(37:) )
//!  
//!                WRITE (*,*) OUTLIN
//!  
//!             END DO
//!  
//!          END IF
//!  
//!  
//!  
//!    C ******************************************************************
//!    C     Third task: find view periods (periods of visibility)
//!    C     for Saturn, as seen from DSS-14, for the time period
//!    C     January 1-5, 2009.
//!    C ******************************************************************
//!  
//!    C
//!    C     We'll consider Saturn to be visible from DSS-14 when
//!    C     Saturn has elevation above 6 degrees in the DSS-14
//!    C     topocentric reference frame DSS-14_TOPO.
//!    C
//!    C     Create a confinement window for the view period
//!    C     search. This window contains the start and stop times
//!    C     of the search interval.
//!    C
//!    C     Empty the window CNFINE, then insert the new time bounds.
//!    C
//!          CALL SCARDD ( 0, CNFINE )
//!  
//!          CALL STR2ET ( '2009 JAN 1', ET0 )
//!          CALL STR2ET ( '2009 JAN 5', ET1 )
//!  
//!          CALL WNINSD ( ET0, ET1, CNFINE )
//!  
//!    C
//!    C     Set the observer, target and reference frame.
//!    C
//!          OBSRVR = 'DSS-14'
//!          TARGET = 'SATURN'
//!          FRAME  = 'DSS-14_TOPO'
//!  
//!    C
//!    C     The coordinate system is latitudinal; in this system,
//!    C     in the DSS-14_TOPO frame, the coordinate "latitude"
//!    C     is equivalent to elevation.
//!    C
//!          CRDSYS = 'LATITUDINAL'
//!          COORD  = 'LATITUDE'
//!  
//!    C
//!    C     The relational operator for this search is "greater
//!    C     than" and the reference value is 6 degrees (converted
//!    C     to radians).
//!    C
//!          RELATE = '>'
//!          REFVAL = 6.D0 * RPD()
//!  
//!    C
//!    C     We're looking for the apparent position of Saturn,
//!    C     so apply corrections for light time and stellar
//!    C     aberration.
//!    C
//!          ABCORR = 'LT+S'
//!  
//!    C
//!    C     Set the step size for this search. The step must
//!    C     be shorter than the shortest interval over which
//!    C     the elevation is increasing or decreasing.
//!    C     We pick a conservative value: 6 hours. Units
//!    C     expected by SPICE are TDB seconds.
//!    C
//!          STEP   =  SPD() / 4
//!  
//!    C
//!    C     The adjustment value isn't used for this search;
//!    C     set it to 0.
//!    C
//!          ADJUST = 0.D0
//!  
//!    C
//!    C     The work space array has dimensions
//!    C
//!    C         ( LBCELL : MAXWIN,  NWMAX )
//!    C
//!    C     where NWMAX is defined in gf.inc. We supply
//!    C     the upper bounds MAXWIN and NWMAX as input
//!    C     arguments to GFDIST.
//!    C
//!    C     Execute the search.
//!    C
//!          WRITE (*,*) ' '
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Starting elevation search.'
//!  
//!          CALL GFPOSC ( TARGET, FRAME,  ABCORR, OBSRVR,
//!         .              CRDSYS, COORD,  RELATE, REFVAL,
//!         .              ADJUST, STEP,   CNFINE, MAXWIN,
//!         .              NWMAX,  WORK,   RESULT         )
//!  
//!          WRITE (*,*) 'Done.'
//!  
//!    C
//!    C     Display the times of rise and set.
//!    C
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Times of Saturn rise/set as seen from DSS-14:'
//!          WRITE (*,*) ' '
//!  
//!          DO I = 1, WNCARD(RESULT)
//!  
//!             OUTLIN = ' '
//!    C
//!    C        Fetch the start and stop times of the Ith
//!    C        interval from the window RESULT.
//!    C
//!             CALL WNFETD ( RESULT, I, START, FINISH )
//!  
//!             CALL TIMOUT ( START,  TIMFMT, OUTLIN(3: ) )
//!             CALL TIMOUT ( FINISH, TIMFMT, OUTLIN(37:) )
//!  
//!             WRITE (*,*) OUTLIN
//!  
//!          END DO
//!  
//!          WRITE (*,*) ' '
//!  
//!          END
//!  
//! ```
//!
//!     
//! ###  Results
//!
//!  Any numerical results shown for this example may differ between
//!    platforms as the results depend on the SPICE kernels used as input and
//!    the machine specific arithmetic implementation.
//!
//!  The output from this program was as follows:
//!
//!  
//!
//! ```text
//!  
//!     Starting distance search.
//!     Done.
//!  
//!     Times of closest approach of Earth to Sun:
//!  
//!       2009 JAN 04 15:30:45.589082 TDB
//!       2010 JAN 03 00:10:21.610041 TDB
//!       2011 JAN 03 18:33:04.989377 TDB
//!       2012 JAN 05 00:32:57.166524 TDB
//!       2013 JAN 02 04:38:41.978883 TDB
//!       2014 JAN 04 11:59:41.025358 TDB
//!       2015 JAN 04 06:37:17.796385 TDB
//!       2016 JAN 02 22:49:53.333439 TDB
//!       2017 JAN 04 14:18:58.873657 TDB
//!       2018 JAN 03 05:35:52.459640 TDB
//!  
//!  
//!     Starting Titan occultation search.
//!     Done.
//!  
//!     Times of occultation of Titan by Saturn:
//!  
//!       2009 JAN 15 17:17:13.408673 TDB   2009 JAN 15 23:24:45.666928 TDB
//!       2009 JAN 31 15:31:34.392257 TDB   2009 JAN 31 21:17:02.978691 TDB
//!       2009 FEB 16 13:38:51.079254 TDB   2009 FEB 16 18:34:44.780780 TDB
//!       2009 MAR 04 12:01:10.277826 TDB   2009 MAR 04 15:11:39.545971 TDB
//!       2009 JUL 25 23:54:05.774967 TDB   2009 JUL 26 04:00:27.167482 TDB
//!       2009 AUG 10 23:28:28.728724 TDB   2009 AUG 11 05:13:21.654337 TDB
//!       2009 AUG 26 23:41:29.894421 TDB   2009 AUG 27 06:07:25.788109 TDB
//!       2009 SEP 12 00:24:15.048030 TDB   2009 SEP 12 06:43:07.257580 TDB
//!       2009 SEP 28 01:35:28.489195 TDB   2009 SEP 28 06:53:19.855589 TDB
//!       2009 OCT 14 03:32:20.159136 TDB   2009 OCT 14 06:11:58.766312 TDB
//!  
//!  
//!     Starting elevation search.
//!     Done.
//!  
//!     Times of Saturn rise/set as seen from DSS-14:
//!  
//!       2009 JAN 01 06:52:14.372881 TDB   2009 JAN 01 18:20:41.050047 TDB
//!       2009 JAN 02 06:48:17.641267 TDB   2009 JAN 02 18:16:45.859623 TDB
//!       2009 JAN 03 06:44:20.383435 TDB   2009 JAN 03 18:12:50.385687 TDB
//!       2009 JAN 04 06:40:22.601451 TDB   2009 JAN 04 18:08:54.628325 TDB
//!  
//! ```
//!
//!     
//! ##  Program CASCADE: Fast Search for Solar Eclipse
//!
//!  
//!
//!
//!  
//! ###  Overview
//!
//!  This example demonstrates a search for a solar eclipse as seen from a
//!    specified location on the Earth's surface, during the year 2008. The
//!    eclipse search speed is increased by a factor of over 100 by use of a
//!    preliminary search to determine a time window during which the apparent
//!    angular separation of the Sun and Moon is small enough so that an
//!    eclipse could occur.
//!
//!  The price we pay to achieve this speed-up is that we must perform a
//!    little analysis of the observation geometry in order to decide how to
//!    perform the preliminary search.
//!
//!  In this example, we use DSN station DSS-14 as the observer. We have an
//!    SPK file providing the geocentric station location in the ITRF93
//!    terrestrial reference frame, so we're able to treat the observer as a
//!    SPICE ephemeris object. For an arbitrary surface point, we could use the
//!    SPICE utility PINPOINT to create an SPK file containing that point's
//!    geocentric location.
//!
//!  We consider a solar eclipse to be any (partial or full) occultation of
//!    the apparent Sun by the apparent Moon, so we perform the eclipse search
//!    using the GF occultation search routine [GFOCLT](crate::raw::gfoclt). We're interested in
//!    detecting any occultation lasting a minute or more, so we use a step
//!    size of 60 seconds for this search. Since we're searching over a time
//!    span of one year, this search, if performed over the entire search
//!    interval, would require over 31 million occultation tests.
//!
//!  To accelerate the search, we'll first narrow down the search period
//!    using a more rapid search---one for which we can use a step size of
//!    days, not seconds. We know an occultation can occur only when the
//!    angular separation of the Sun and Moon as seen from DSS-14 is small. If
//!    we can quickly find the time window over which the angular separation of
//!    the apparent figures of the Sun and Moon is less than a small upper
//!    bound, we can then gain speed by performing the slower occultation
//!    search only over this small window.
//!
//!  
//!
//!
//!  
//! ###  Specifying the angular separation search parameters
//!
//!  In order to perform the angular separation search, we'll need to decide
//!    on the search step size and the upper bound of the angular separation.
//!    We'll also choose a convenient observation point relative to which the
//!    angular separation is defined.
//!
//!  Recall that GF searches involving a scalar quantity, such as angular
//!    separation, have search step size requirements based on the separation
//!    in time of the local extrema of the quantity: except for longitude
//!    searches, the step must be smaller than the minimum time separation
//!    between the epochs of the extrema (minima and maxima) of the quantity,
//!    taken over the search interval. When these extrema are widely separated,
//!    a large step size can be used.
//!
//!  So that we can pick a useful lower bound on the time separation of the
//!    extrema of angular separation, we want to define angular separation in
//!    such a way that this function is easy to analyze.
//!
//!  There are two candidate observers we could use to define the angular
//!    separation of Sun and Moon: DSS-14 and the center of the Earth. If we
//!    use the center of the Earth, the relative angular velocity of the
//!    targets has only small relative variations in magnitude, except in the
//!    vicinity of its extrema, and we can be confident that we won't find any
//!    unexpected extrema of angular separation; however the angular separation
//!    we compute is slightly different than what we'd find using DSS-14 as the
//!    observer. If we use DSS-14 as the observer, we must consider whether the
//!    motion of the station relative to the center of the Earth introduces any
//!    additional extrema of angular separation beyond those occurring when the
//!    observer is the Earth's center.
//!
//!  Since we can easily bound the angular separation error caused by using
//!    the Earth's center as the observer, we'll choose this observer, thus
//!    simplifying our analysis. The maximum angular separation error caused by
//!    this choice is roughly 1 degree; we'll conservatively pick 2 degrees as
//!    the error bound. If we pick a generous limit of 1 degree for angular
//!    separation of the figures of the Sun and Moon as seen from DSS-14,
//!    adding 2 degrees to this yields the 3 degree bound we'll use for the
//!    angular separation search.
//!
//!  The angular separation of Sun and Moon as seen from the center of the
//!    Earth has a period of about four weeks. The local minima and maxima of
//!    the separation are separated by roughly two weeks. Since we don't want
//!    to perform a detailed analysis of the minimum time separation of the
//!    extrema, we simply pick a value that's guaranteed to be smaller than
//!    this minimum duration but large enough to be helpful: 5 days.
//!
//!  If we were to perform this search repeatedly, it could be useful to
//!    analyze the problem further in order to compute a tighter angular
//!    separation bound and a smaller step size.
//!
//!  
//!
//!
//!  
//! ###  Aberration corrections
//!
//!  Normally computations involving apparent geometry of extended objects
//!    require correcting target positions for light time and stellar
//!    aberration, so the aberration correction flag
//!
//!  
//!
//! ```text
//!    'LT+S'
//! ```
//!
//!  would be used. However, stellar aberration corrections are unnecessary
//!    for occultation computations, since the respective stellar aberration
//!    corrections for the two targets are identical at the point of tangency
//!    of the figures of the targets. For this reason the GF occultation
//!    routine [GFOCLT](crate::raw::gfoclt) ignores the stellar aberration correction token
//!
//!  
//!
//! ```text
//!    '+S'
//! ```
//!
//!  if it's provided. Only light time corrections are needed for the
//!    occultation search.
//!
//!  
//!
//!
//!  
//! ###  SPICE kernels
//!
//!  The meta-kernel used for this example is shown below.
//!
//!  
//!
//! ```text
//!  
//!    KPL/MK
//!  
//!       File: cascade.tm
//!  
//!       Meta-kernel for example program CASCADE.
//!  
//!       This meta-kernel is intended to support operation of SPICE
//!       example programs. The kernels shown here should not be
//!       assumed to contain adequate or correct versions of data
//!       required by a user's SPICE-based applications.
//!  
//!       In order for an application to use this meta-kernel, the
//!       kernels referenced here must be present in the user's
//!       current working directory.
//!  
//!       The names and contents of the kernels referenced
//!       by this meta-kernel are as follows:
//!  
//!          File name                        Contents
//!          ---------                        --------
//!          de421.bsp                        Planetary ephemeris
//!          pck00008.tpc                     Planet orientation and
//!                                           radii
//!          naif0009.tls                     Leapseconds
//!          earthstns_itrf93_050714.bsp      DSN station locations
//!          earth_070425_370426_predict.bpc  Long term, low-accuracy
//!                                           Earth orientation
//!  
//!       Version 1.0.0 13-JAN-2009 (NJB)
//!  
//!    \begindata
//!  
//!       KERNELS_TO_LOAD = (
//!                           'naif0009.tls'
//!                           'pck00008.tpc'
//!                           'de421.bsp'
//!                           'earthstns_itrf93_050714.bsp'
//!                           'earth_070425_370426_predict.bpc'
//!                         )
//!    \begintext
//!  
//!    [End of kernel]
//!  
//! ```
//!
//!     
//! ###  Source code
//!
//!  Example source code begins here.
//!
//!  
//!
//! ```text
//!  
//!          PROGRAM CASCADE
//!          IMPLICIT NONE
//!  
//!    C
//!    C     SPICELIB functions
//!    C
//!          DOUBLE PRECISION      RPD
//!          DOUBLE PRECISION      SPD
//!  
//!          INTEGER               WNCARD
//!  
//!    C
//!    C     Global parameters
//!    C
//!          INCLUDE 'gf.inc'
//!  
//!    C
//!    C     Local parameters
//!    C
//!          CHARACTER*(*)         META
//!          PARAMETER           ( META   = 'cascade.tm' )
//!  
//!          CHARACTER*(*)         TIMFMT
//!          PARAMETER           ( TIMFMT =
//!         .                'YYYY MON DD HR:MN:SC.###### TDB::RND::TDB')
//!  
//!          INTEGER               BDNMLN
//!          PARAMETER           ( BDNMLN = 36 )
//!  
//!          INTEGER               CORLEN
//!          PARAMETER           ( CORLEN = 10 )
//!  
//!          INTEGER               LBCELL
//!          PARAMETER           ( LBCELL = -5 )
//!  
//!          INTEGER               LNSIZE
//!          PARAMETER           ( LNSIZE = 78 )
//!  
//!          INTEGER               MAXWIN
//!          PARAMETER           ( MAXWIN = 200000 )
//!  
//!          INTEGER               RLTLEN
//!          PARAMETER           ( RLTLEN = 10 )
//!  
//!    C
//!    C     Local variables
//!    C
//!          CHARACTER*(CORLEN)    ABCORR
//!          CHARACTER*(BDNMLN)    OBSRVR
//!          CHARACTER*(LNSIZE)    OUTLIN
//!          CHARACTER*(RLTLEN)    RELATE
//!  
//!          DOUBLE PRECISION      ADJUST
//!          DOUBLE PRECISION      AVG
//!          DOUBLE PRECISION      CNFINE ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      ET0
//!          DOUBLE PRECISION      ET1
//!          DOUBLE PRECISION      FINISH
//!          DOUBLE PRECISION      LIMIT
//!          DOUBLE PRECISION      MEASUR ( 2 )
//!          DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      START
//!          DOUBLE PRECISION      STDDEV
//!          DOUBLE PRECISION      STEP
//!          DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWSEP )
//!  
//!          INTEGER               I
//!          INTEGER               LONG
//!          INTEGER               SHORT
//!  
//!    C
//!    C     Load kernels.
//!    C
//!          CALL FURNSH ( META )
//!  
//!    C
//!    C     Initialize windows.
//!    C
//!          CALL SSIZED ( MAXWIN, CNFINE )
//!          CALL SSIZED ( MAXWIN, RESULT )
//!  
//!    C
//!    C     Create a confinement window for an angular separation
//!    C     search. This window contains the start and stop times
//!    C     of the search interval.
//!    C
//!          CALL STR2ET ( '2008 JAN 1', ET0 )
//!          CALL STR2ET ( '2009 JAN 1', ET1 )
//!  
//!          CALL WNINSD ( ET0, ET1, CNFINE )
//!  
//!    C
//!    C     Save the measure of this window.
//!    C
//!          MEASUR(1) = ET1 - ET0
//!  
//!    C
//!    C     Set the observer for the angular separation search.
//!    C
//!          OBSRVR = 'EARTH'
//!  
//!    C
//!    C     We don't need high precision for the angular
//!    C     separation search: we could use uncorrected states,
//!    C     which are computed more quickly than aberration-
//!    C     corrected states. But for simplicity of the code,
//!    C     we'll use the same aberration corrections for the
//!    C     angular separation and occultation searches.
//!    C
//!    C     Use light time correction. Stellar aberration correction
//!    C     is not helpful for occultation searches, so the
//!    C     stellar aberration flag '+S' is ignored by GFOCLT.
//!    C
//!          ABCORR = 'LT'
//!  
//!    C
//!    C     Find times when the angular separation of the Sun and
//!    C     Moon is below the specified limit, as seen by the
//!    C     observer. We can use the centers of the objects
//!    C     for this search.
//!    C
//!    C     Set the angular separation limit of 3 degrees. Units
//!    C     accepted by SPICE are radians, so do the conversion
//!    C     here.
//!    C
//!          LIMIT = 3.D0 * RPD()
//!  
//!    C
//!    C     The relational operator for this search is "less than."
//!    C
//!          RELATE = '<'
//!  
//!    C
//!    C     Set the step size for this search. The step must
//!    C     be shorter than the shortest interval over which
//!    C     the angular separation is increasing or decreasing.
//!    C     We pick a conservative value: 5 days. Units
//!    C     expected by SPICE are TDB seconds.
//!    C
//!          STEP   = 5.D0 * SPD()
//!  
//!    C
//!    C     The adjustment value isn't used for this search;
//!    C     set it to 0.
//!    C
//!          ADJUST = 0.D0
//!  
//!    C
//!    C     Execute the search. Note that we can leave the
//!    C     body-fixed frame arguments blank, since they're
//!    C     not used for point targets.
//!    C
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Starting angular separation search.'
//!  
//!          CALL GFSEP ( 'MOON', 'POINT', ' ',
//!         .             'SUN',  'POINT', ' ',
//!         .             ABCORR, OBSRVR,  RELATE, LIMIT,
//!         .             ADJUST, STEP,    CNFINE, MAXWIN,
//!         .             NWSEP,  WORK,    RESULT          )
//!  
//!          WRITE (*,*) 'Done.'
//!  
//!    C
//!    C     Use the result window from this search as the
//!    C     confinement window for the occultation search.
//!    C
//!          CALL COPYD ( RESULT, CNFINE )
//!  
//!    C
//!    C     Save the measure of this window. This window
//!    C     contains multiple intervals, so we sum their
//!    C     lengths. We could do this in a loop, but it's
//!    C     even easier to call the window summary routine
//!    C     WNSUMD.
//!    C
//!          CALL WNSUMD ( CNFINE, MEASUR(2), AVG, STDDEV, SHORT, LONG )
//!  
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Ratio of measure of short confinement '
//!         .//          'window to original:'
//!  
//!          IF ( MEASUR(1) .EQ. 0.D0 ) THEN
//!             CALL SIGERR ( 'SPICE(DIVIDEBYZERO' )
//!          END IF
//!  
//!          WRITE (*,*) MEASUR(2) / MEASUR(1)
//!  
//!    C
//!    C     Find times when the Sun is at least partially occulted
//!    C     by the Moon as seen by the observer. The occultation
//!    C     type 'ANY' indicates that any overlap of the back
//!    C     target by the front will be considered an occultation.
//!    C
//!    C     The step size for the occultation search must be
//!    C     short enough to catch any occultation of interest.
//!    C     We choose 60 seconds.
//!    C
//!          STEP   = 60.D0
//!  
//!    C
//!    C     Set the observer for the occultation search.
//!    C
//!          OBSRVR = 'DSS-14'
//!  
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Starting occultation search.'
//!  
//!          CALL GFOCLT ( 'ANY',
//!         .              'MOON', 'ELLIPSOID', 'IAU_MOON',
//!         .              'SUN',  'ELLIPSOID', 'IAU_SUN',
//!         .              ABCORR, OBSRVR,      STEP,
//!         .              CNFINE, RESULT                  )
//!  
//!          WRITE (*,*) 'Done.'
//!          WRITE (*,*) ' '
//!  
//!          IF ( WNCARD(RESULT) .EQ. 0 ) THEN
//!  
//!             WRITE (*,*) 'No occultations were found.'
//!          ELSE
//!             WRITE (*,*) 'Occultations:'
//!  
//!             DO I = 1, WNCARD(RESULT)
//!  
//!                OUTLIN = ' '
//!    C
//!    C           Fetch the start and stop times of the Ith
//!    C           interval from the window RESULT.
//!    C
//!                CALL WNFETD ( RESULT, I, START, FINISH )
//!  
//!                CALL TIMOUT ( START,  TIMFMT, OUTLIN(3: ) )
//!                CALL TIMOUT ( FINISH, TIMFMT, OUTLIN(37:) )
//!  
//!                WRITE (*,*) OUTLIN
//!  
//!             END DO
//!  
//!          END IF
//!  
//!          WRITE (*,*) ' '
//!  
//!          END
//!  
//! ```
//!
//!     
//! ###  Results
//!
//!  Any numerical results shown for this example may differ between
//!    platforms as the results depend on the SPICE kernels used as input and
//!    the machine specific arithmetic implementation.
//!
//!  The output from this program was as follows:
//!
//!  
//!
//! ```text
//!  
//!     Starting angular separation search.
//!     Done.
//!  
//!     Ratio of measure of short confinement window to original:
//!      0.00497163966
//!  
//!     Starting occultation search.
//!     Done.
//!  
//!     Occultations:
//!       2008 AUG 01 08:40:50.967887 TDB   2008 AUG 01 10:00:42.048379 TDB
//!  
//! ```
//!
//!  On this platform, the (wall clock) run time was about 0.95 seconds.
//!
//!  When the angular separation search was removed (this can be done by
//!    commenting out the [COPYD](crate::raw::copyd) call in the source code), the run time was
//!    about 140 seconds.
//!
//!  
//!
//!
//!  
//! ##  Program ROVER: Mars Reconnaissance Orbiter photographs MER-1
//!
//!  
//!
//!
//!  
//! ###  Overview
//!
//!  This program finds an approximate time window, during the month November
//!    2006, over which the MER-1 ("Opportunity") rover is visible within the
//!    Mars Reconnaissance Orbiter (MRO) HIRISE field of view (FOV). Since
//!    HIRISE was used to photograph MER-1 during this time period, the timing
//!    results from this example program can be compared against actual data.
//!
//!  
//!
//!
//!  
//! ###  Determining SPK and CK coverage at run time
//!
//!  This example involves multiple CK and SPK files. Because the coverage of
//!    the CK files has numerous gaps, and because we want the program to
//!    determine the times of coverage for all required data, the SPICE CK and
//!    SPK coverage routines [CKCOV](crate::raw::ckcov) and [SPKCOV](crate::raw::spkcov) are used. To ensure availability
//!    of data, certain modifications of the coverage windows found by these
//!    routines are required:
//!
//!  
//!
//! * Within CK files, CK coverage bounds are represented by encoded SCLK time.
//! In order to conveniently work with these time bounds, they must be
//! converted to Barycentric Dynamical Time (TDB). Each such conversion
//! introduces a small amount of round-off error. These errors may prevent the
//! TDB values from being converted back to encoded SCLK values within the CK
//! coverage window.
//!
//!  *  So the MRO spacecraft bus orientation coverage window is contracted
//! slightly (that is, the left endpoint of each interval of the window is
//! moved to the right, and the right endpoint of each interval is moved to the
//! left) to eliminate any CK look-up failures that could result from these
//! round-off errors.
//!
//!  * The intervals comprising the MER-1 SPK coverage window are contracted on
//! the left to compensate for one-way light time between MER-1 and MRO. This
//! ensures that times at the beginning of these intervals can be adjusted by
//! one-way light time and still be within the actual coverage window for the
//! MER-1 SPK files.
//!
//!  * The intervals comprising the MRO SPK coverage window are contracted by
//! slightly more than one second on both sides to ensure data availability for
//! stellar aberration computations. Even though we're performing searches
//! involving constraints on observer-target position vectors, the GF subsystem
//! uses the corresponding velocities to conduct these searches. The SPK
//! subsystem's stellar aberration correction velocity computation requires
//! observer acceleration with respect to the solar system barycenter. The
//! acceleration at a given epoch ET is computed by discrete differentiation
//! using samples taken at ET +/- one second.
//!
//!  *  For the problem at hand, it happens that this contraction isn't needed
//! because MRO SPK coverage is not the limiting factor determining the overall
//! coverage window. The contraction is demonstrated in the interest of safety
//! and broader applicability of the example.
//!
//!  The intersection of the modified coverage windows yields a window over
//!    which all required data are available.
//!
//!  
//!
//!
//!  
//! ###  Speeding up the search
//!
//!  Because of the minute angular extent of the MRO HIRISE field of view in
//!    the MRO downtrack direction, a simple search of the data availability
//!    window using the GF "is target in instrument FOV?" routine [GFTFOV](crate::raw::gftfov) would
//!    be prohibitively slow. So the search is performed in three steps:
//!
//!  
//!
//! *  1. The data availability window is searched for times when the observer and
//! target are separated by no more than 500 km. Since the nominal altitude of
//! MRO above Mars' surface is about 300 km, this limit allows for a
//! substantial pointing offset relative to the nadir direction. The result of
//! this search is the "distance window" DISTWN.
//!
//!  *  The step size for this search can be large, since the epochs of the extrema
//! of the observer-target distance are separated by almost an hour. For
//! safety, a half-hour step is used.
//!
//!  *  2. Since the MRO spacecraft's downtrack direction is nominally aligned with
//! the MRO_HIRISE_LOOK_DIRECTION frame's +X axis, the distance window is
//! searched for times when the MER-1 rover crosses the
//! MRO_HIRISE_LOOK_DIRECTION frame's Y-Z plane.
//!
//!  *  This search produces a non-empty window of measure zero: the contents of
//! the window are singleton intervals, some of which may lie in the time
//! window during which MER-1 is in the MRO HIRISE FOV.
//!
//!  *  3. For each singleton interval in the result window of the Y-Z plane crossing
//! search, we find the angular separation of the MRO-rover vector (which at
//! the epochs of comparison lies in the camera's Y-Z plane) and the HIRISE +Z
//! vector. We compare this angle to the angular half-width of the HIRISE
//! nominal FOV; if the angle is smaller than the half-width, we consider the
//! rover to be visible.
//!
//!     
//! ###  Pointing issues
//!
//!  With nominal nadir pointing, the target moves in the downtrack direction
//!    with a period matching that of MRO's orbit, so extrema of the target's X
//!    coordinate in the HIRISE frame are almost an hour apart. However, if the
//!    spacecraft were to rotate rapidly, this effect could dominate that of
//!    the spacecraft's orbital motion, creating new extrema.
//!
//!  Substantial deviation from the nominal nadir-pointed spacecraft
//!    orientation could also prevent the HIRISE FOV from "seeing" the
//!    target.
//!
//!  Based on prior knowledge, we expect this search to find two solutions.
//!    The results of the search will show that the solutions are the ones we
//!    want: we have near-nadir pointing at the visibility epochs in each case.
//!
//!  In a more realistic setting, we would need to ensure that no valid
//!    solutions were missed. This could be done by reducing the step size for
//!    the MRO_HIRISE_LOOK_DIRECTION frame's Y-Z plane crossing search.
//!    Alternatively, the spacecraft pointing could be analyzed for the time
//!    window over which the Y-Z plane crossing search is performed.
//!
//!  
//!
//!
//!  
//! ###  Aberration corrections
//!
//!  The searches described above involve apparent target geometry, so in all
//!    but the distance search, which need not produce highly accurate results,
//!    light time and stellar aberration corrections are used. The flag
//!    indicating these aberration corrections is
//!
//!  
//!
//! ```text
//!    'LT+S'
//! ```
//!
//!     
//! ###  SPICE kernels
//!
//!  SPICE kernels for MRO and MER-1 referenced below were obtained from the
//!    NAIF PDS archive.
//!
//!  The meta-kernel used for this example is shown below.
//!
//!  
//!
//! ```text
//!  
//!    KPL/MK
//!  
//!       File: rover.tm
//!  
//!       Meta-kernel for example program ROVER.
//!  
//!       This meta-kernel is intended to support operation of SPICE
//!       example programs. The kernels shown here should not be
//!       assumed to contain adequate or correct versions of data
//!       required by a user's SPICE-based application.
//!  
//!       In order for an application to use this meta-kernel, the
//!       kernels referenced here must be present in the user's
//!       current working directory.
//!  
//!       The names and contents of the kernels referenced
//!       by this meta-kernel are as follows:
//!  
//!          File name                        Contents
//!          ---------                        --------
//!          de421.bsp                        Planetary ephemeris
//!          pck00008.tpc                     Planet orientation and
//!                                           radii
//!          naif0009.tls                     Leapseconds
//!          mro_v11.tf                       MRO frame specifications
//!          mro_hirise_v10.ti                MRO HIRISE instrument
//!                                           parameters
//!          mro_sc_psp_061031_061106.bc      MRO orientation
//!          mro_sc_psp_061107_061113.bc      MRO orientation
//!          mro_sc_psp_061114_061120.bc      MRO orientation
//!          mro_sc_psp_061121_061127.bc      MRO orientation
//!          mro_sc_psp_061128_061204.bc      MRO orientation
//!          mro_sclkscet_00026_65536.tsc     MRO SCLK parameters and
//!                                           correlation data
//!          mro_psp1.bsp                     MRO ephemeris
//!          mer1_v10.tf                      MER-1 frame specifications
//!          mer1_surf_rover_ext10_v1.bsp     MER-1 ephemeris
//!          mer1_surf_rover_ext11_v1.bsp     MER-1 ephemeris
//!          mer1_ls_040128_iau2000_v1.bsp    MER-1 landing site location
//!  
//!  
//!       Version 1.0.0 25-JAN-2009 (NJB)
//!  
//!    \begindata
//!  
//!       KERNELS_TO_LOAD = (
//!                           'naif0009.tls'
//!                           'pck00008.tpc'
//!                           'de421.bsp'
//!                           'mro_v11.tf'
//!                           'mro_hirise_v10.ti'
//!                           'mro_sc_psp_061031_061106.bc'
//!                           'mro_sc_psp_061107_061113.bc'
//!                           'mro_sc_psp_061114_061120.bc'
//!                           'mro_sc_psp_061121_061127.bc'
//!                           'mro_sc_psp_061128_061204.bc'
//!                           'mro_sclkscet_00026_65536.tsc'
//!                           'mro_psp1.bsp'
//!                           'mer1_v10.tf'
//!                           'mer1_surf_rover_ext10_v1.bsp'
//!                           'mer1_surf_rover_ext11_v1.bsp'
//!                           'mer1_ls_040128_iau2000_v1.bsp'
//!                         )
//!    \begintext
//!  
//!    [End of kernel]
//!  
//! ```
//!
//!     
//! ###  Source code
//!
//!  Example source code begins here.
//!
//!  
//!
//! ```text
//!          PROGRAM ROVER
//!          IMPLICIT NONE
//!  
//!    C
//!    C     SPICELIB functions
//!    C
//!          DOUBLE PRECISION      VNORM
//!          INTEGER               WNCARD
//!  
//!    C
//!    C     Global parameters
//!    C
//!          INCLUDE 'gf.inc'
//!  
//!    C
//!    C     Local parameters
//!    C
//!          INTEGER               FILSIZ
//!          PARAMETER           ( FILSIZ = 255 )
//!  
//!          CHARACTER*(*)         META
//!          PARAMETER           ( META   = 'rover.tm' )
//!  
//!          CHARACTER*(*)         TIMFMT
//!          PARAMETER           ( TIMFMT =
//!         .                'YYYY MON DD HR:MN:SC.###### TDB::RND::TDB')
//!  
//!          INTEGER               BDNMLN
//!          PARAMETER           ( BDNMLN = 36 )
//!  
//!          INTEGER               FRNMLN
//!          PARAMETER           ( FRNMLN = 32 )
//!  
//!          INTEGER               SYSLEN
//!          PARAMETER           ( SYSLEN = 25 )
//!  
//!          INTEGER               CRDLEN
//!          PARAMETER           ( CRDLEN = 25 )
//!  
//!          INTEGER               CORLEN
//!          PARAMETER           ( CORLEN = 10 )
//!  
//!          INTEGER               LBCELL
//!          PARAMETER           ( LBCELL = -5 )
//!  
//!          INTEGER               LNSIZE
//!          PARAMETER           ( LNSIZE = 78 )
//!  
//!          INTEGER               MAXWIN
//!          PARAMETER           ( MAXWIN = 200000 )
//!  
//!          INTEGER               RLTLEN
//!          PARAMETER           ( RLTLEN = 10 )
//!  
//!          INTEGER               TYPLEN
//!          PARAMETER           ( TYPLEN = 10 )
//!  
//!          INTEGER               UNTLEN
//!          PARAMETER           ( UNTLEN = 25 )
//!  
//!    C
//!    C     Local variables
//!    C
//!          CHARACTER*(CORLEN)    ABCORR
//!          CHARACTER*(FILSIZ)    CKNAME
//!          CHARACTER*(CRDLEN)    COORD
//!          CHARACTER*(SYSLEN)    CRDSYS
//!          CHARACTER*(FRNMLN)    FRAME
//!          CHARACTER*(TYPLEN)    FTYPE
//!          CHARACTER*(BDNMLN)    OBSRVR
//!          CHARACTER*(LNSIZE)    OUTLIN
//!          CHARACTER*(RLTLEN)    RELATE
//!          CHARACTER*(FILSIZ)    SOURCE
//!          CHARACTER*(FILSIZ)    SPKNAM
//!          CHARACTER*(BDNMLN)    TARGET
//!          CHARACTER*(UNTLEN)    UNITS
//!  
//!          DOUBLE PRECISION      ADJUST
//!          DOUBLE PRECISION      AVG
//!          DOUBLE PRECISION      CKWMER ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      CKWMRO ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      CNFINE ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      DISTWN ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      FINISH
//!          DOUBLE PRECISION      HFOV
//!          DOUBLE PRECISION      LT
//!          DOUBLE PRECISION      MEASUR
//!          DOUBLE PRECISION      NUDGE
//!          DOUBLE PRECISION      REFANG
//!          DOUBLE PRECISION      REFVAL
//!          DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      SPWMER ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      SPWMRO ( LBCELL : MAXWIN )
//!          DOUBLE PRECISION      START
//!          DOUBLE PRECISION      STDDEV
//!          DOUBLE PRECISION      STEP
//!          DOUBLE PRECISION      TRGPOS ( 3 )
//!  
//!          DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWSEP )
//!  
//!          INTEGER               HANDLE
//!          INTEGER               I
//!          INTEGER               LONG
//!          INTEGER               MERCDE
//!          INTEGER               MROBUS
//!          INTEGER               MROCDE
//!          INTEGER               N
//!          INTEGER               SHORT
//!  
//!          LOGICAL               FOUND
//!  
//!    C
//!    C     Load kernels.
//!    C
//!          CALL FURNSH ( META )
//!  
//!    C
//!    C     Initialize windows.
//!    C
//!          CALL SSIZED ( MAXWIN, CKWMER )
//!          CALL SSIZED ( MAXWIN, CKWMRO )
//!          CALL SSIZED ( MAXWIN, SPWMER )
//!          CALL SSIZED ( MAXWIN, SPWMRO )
//!          CALL SSIZED ( MAXWIN, CNFINE )
//!          CALL SSIZED ( MAXWIN, DISTWN )
//!          CALL SSIZED ( MAXWIN, RESULT )
//!  
//!    C
//!    C     Get the count of loaded CKs.
//!    C
//!          CALL KTOTAL ( 'CK', N )
//!  
//!    C
//!    C     For each loaded CK, get the coverage, if any, for
//!    C     the MRO s/c bus. Combine this coverage with that
//!    C     already found.
//!    C
//!          MROBUS = -74000
//!  
//!          DO I = 1, N
//!  
//!             CALL KDATA ( I,      'CK',   CKNAME, FTYPE,
//!         .                SOURCE, HANDLE, FOUND         )
//!    C
//!    C        Get coverage at the interpolation interval level.
//!    C        Angular velocity is not required. Tolerance
//!    C        is 0 seconds. Return the window times as TDB values.
//!    C
//!             CALL CKCOV ( CKNAME,     MROBUS, .FALSE.,
//!         .                'INTERVAL', 0.D0,   'TDB',   CKWMRO )
//!  
//!          END DO
//!  
//!    C
//!    C     Contract each interval of the coverage window
//!    C     by 1 microsecond  on both sides to protect
//!    C     against round-off error in the SCLK-to-TDB
//!    C     conversion performed by CKCOV.
//!    C
//!          NUDGE = 1.D-6
//!          CALL WNCOND ( NUDGE, NUDGE, CKWMRO )
//!  
//!    C
//!    C     Get coverage of both the MRO and MER-1 SPK files.
//!    C
//!          CALL KTOTAL ( 'SPK', N )
//!  
//!          CALL BODN2C ( 'MRO', MROCDE, FOUND )
//!  
//!          IF ( .NOT. FOUND ) THEN
//!             CALL SETMSG ( 'Could not map MRO to an ID code' )
//!             CALL SIGERR ( 'SPICE(NOTRANSLATION)'            )
//!          END IF
//!  
//!          CALL BODN2C ( 'MER-1', MERCDE, FOUND )
//!  
//!          IF ( .NOT. FOUND ) THEN
//!             CALL SETMSG ( 'Could not map MER-1 to an ID code' )
//!             CALL SIGERR ( 'SPICE(NOTRANSLATION)'              )
//!          END IF
//!  
//!          DO I = 1, N
//!  
//!             CALL KDATA ( I,      'SPK',  SPKNAM, FTYPE,
//!         .                SOURCE, HANDLE, FOUND         )
//!  
//!             CALL SPKCOV ( SPKNAM, MROCDE, SPWMRO )
//!             CALL SPKCOV ( SPKNAM, MERCDE, SPWMER )
//!  
//!          END DO
//!  
//!    C
//!    C     Contract the intervals of the MER-1 SPK
//!    C     window on their left sides to account
//!    C     for light time correction. Note that we may look up the
//!    C     position of MER-1 relative to MRO even when MER-1 is not
//!    C     visible, so the contraction amount must be large enough
//!    C     to ensure data availability when MRO and MER-1 are on
//!    C     opposite sides of Mars.
//!    C
//!          NUDGE = 5.D-2
//!          CALL WNCOND ( NUDGE, 0.D0, SPWMER )
//!  
//!    C
//!    C     Let the confinement window be the intersection of
//!    C     the CK and SPK kernel coverage windows.
//!    C
//!          CALL WNINTD ( CKWMRO, SPWMRO, RESULT )
//!          CALL WNINTD ( SPWMER, RESULT, CNFINE )
//!  
//!    C
//!    C     Contract the confinement window by a bit more than 1 second
//!    C     on both sides to account for the times at which
//!    C     data will be required to compute observer acceleration.
//!    C
//!          NUDGE = 1.001D0
//!          CALL WNCOND ( NUDGE, NUDGE, CNFINE )
//!  
//!          WRITE (*,*) ' '
//!  
//!          IF ( WNCARD(CNFINE) .EQ. 0 ) THEN
//!  
//!             WRITE (*,*) 'The coverage window is empty.'
//!          ELSE
//!             WRITE (*,*) 'Common MRO CK, MRO SPK and MER SPK coverage:'
//!  
//!             DO I = 1, WNCARD(CNFINE)
//!  
//!                OUTLIN = ' '
//!    C
//!    C           Fetch the start and stop times of the Ith
//!    C           interval from the window RESULT.
//!    C
//!                CALL WNFETD ( CNFINE, I, START, FINISH )
//!  
//!                CALL TIMOUT ( START,  TIMFMT, OUTLIN(3: ) )
//!                CALL TIMOUT ( FINISH, TIMFMT, OUTLIN(37:) )
//!  
//!                WRITE (*,*) OUTLIN
//!  
//!             END DO
//!  
//!          END IF
//!  
//!          CALL WNSUMD ( CNFINE, MEASUR, AVG, STDDEV, SHORT, LONG )
//!  
//!          WRITE (*,*) 'Measure of coverage window (sec): ', MEASUR
//!  
//!    C
//!    C     Find times during our coverage window when the
//!    C     distance between MER-1 and MRO is less than
//!    C     500 km. We're not interested in other viewing
//!    C     opportunities.
//!    C
//!          TARGET = 'MER-1'
//!          OBSRVR = 'MRO'
//!          ABCORR = 'NONE'
//!          RELATE = '<'
//!          REFVAL = 500.D0
//!          ADJUST = 0.D0
//!  
//!    C
//!    C     Pick a time step smaller than half the orbital
//!    C     period, but large enough for a fast search.
//!    C     Units are seconds. Store the resulting window
//!    C     in DISTWN.
//!    C
//!          STEP = 1800.D0
//!  
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Starting distance search.'
//!          CALL GFDIST ( TARGET, ABCORR, OBSRVR, RELATE,
//!         .              REFVAL, ADJUST, STEP,   CNFINE,
//!         .              MAXWIN, NWMAX,  WORK,   DISTWN )
//!          WRITE (*,*) 'Done.'
//!  
//!          CALL WNSUMD ( DISTWN, MEASUR, AVG, STDDEV, SHORT, LONG )
//!  
//!          WRITE (*,*) 'Measure of distance window (sec): ', MEASUR
//!  
//!    C
//!    C     Find times during the window DISTWN when the
//!    C     apparent position of MER-1 relative to MRO lies on the
//!    C     Y-Z plane of the MRO_HIRISE_LOOK_DIRECTION frame.
//!    C
//!          TARGET = 'MER-1'
//!          OBSRVR = 'MRO'
//!          FRAME  = 'MRO_HIRISE_LOOK_DIRECTION'
//!          ABCORR = 'LT+S'
//!          CRDSYS = 'RECTANGULAR'
//!          COORD  = 'X'
//!          RELATE = '='
//!          REFVAL = 0.D0
//!          ADJUST = 0.D0
//!  
//!    C
//!    C     Pick a time step small enough so that the
//!    C     search is unlikely to miss the events,
//!    C     but large enough for a fast search.
//!    C
//!    C     Set the step to 1/2 hour. Units are seconds.
//!    C
//!          STEP = 1800.D0
//!  
//!          WRITE (*,*) ' '
//!          WRITE (*,*) 'Starting MRO_HIRISE_LOOK_DIRECTION frame''s'
//!          WRITE (*,*) 'Y-Z plane crossing search.'
//!  
//!          CALL GFPOSC ( TARGET, FRAME,  ABCORR, OBSRVR,
//!         .              CRDSYS, COORD,  RELATE, REFVAL,
//!         .              ADJUST, STEP,   DISTWN, MAXWIN,
//!         .              NWMAX,  WORK,   RESULT          )
//!          WRITE (*,*) 'Done.'
//!  
//!    C
//!    C     Display the Y-Z plane crossings for which the magnitude
//!    C     of the target's Y angular offset from the camera frame's
//!    C     X-Z plane is less than the angular half-width of the HIRISE
//!    C     nominal FOV. Look up this half-width here.
//!    C
//!          CALL GDPOOL ( 'INS-74699_FOV_REF_ANGLE', 1,      1,
//!         .              N,                         REFANG, FOUND )
//!  
//!          IF ( .NOT. FOUND ) THEN
//!             WRITE (*,*) 'Could not find data for HIRISE nominal FOV.'
//!             STOP
//!          END IF
//!    C
//!    C     Look up units for the angle; convert the angle to radians.
//!    C
//!          CALL GCPOOL ( 'INS-74699_FOV_ANGLE_UNITS', 1,      1,
//!         .              N,                           UNITS,  FOUND )
//!          IF ( .NOT. FOUND ) THEN
//!             WRITE (*,*) 'Could not find units for HIRISE nominal FOV.'
//!             STOP
//!          END IF
//!  
//!          CALL CONVRT ( REFANG, UNITS, 'RADIANS', HFOV )
//!  
//!  
//!          IF ( WNCARD(RESULT) .EQ. 0 ) THEN
//!  
//!             WRITE (*,*) 'The visibility window is empty.'
//!          ELSE
//!             WRITE (*,*) ' '
//!             WRITE (*,*) 'Times of MER-1 visibility within '
//!         .   //          'MRO HIRISE nominal FOV swath:'
//!             WRITE (*,*) ' '
//!  
//!             DO I = 1, WNCARD(RESULT)
//!  
//!                OUTLIN = ' '
//!    C
//!    C           Fetch the start and stop times of the Ith
//!    C           interval from the window RESULT.
//!    C
//!                CALL WNFETD ( RESULT, I, START, FINISH )
//!  
//!                CALL SPKPOS ( TARGET, START,  FRAME, ABCORR,
//!         .                    OBSRVR, TRGPOS, LT            )
//!  
//!  
//!                IF (      ABS( ATAN2( TRGPOS(2), TRGPOS(3) ) )
//!         .           .LT. HFOV                                  ) THEN
//!    C
//!    C              The target lies within the nominal HIRISE swath.
//!    C
//!                   OUTLIN = ' '
//!                   CALL TIMOUT ( START,  TIMFMT, OUTLIN(4: ) )
//!  
//!                   WRITE (*,*) OUTLIN
//!                   WRITE (*,*) ' '
//!                   WRITE (*,*) '     Frame: '//FRAME
//!                   WRITE (*,*) ' '
//!  
//!                   WRITE (*,*) '     Target X-coordinate (km): ',
//!         .                     TRGPOS(1)
//!                   WRITE (*,*) '     Target Y-coordinate (km): ',
//!         .                     TRGPOS(2)
//!                   WRITE (*,*) '     Target Z-coordinate (km): ',
//!         .                     TRGPOS(3)
//!                   WRITE (*,*) '     Target range        (km): ',
//!         .                     VNORM(TRGPOS)
//!                   WRITE (*,*) ' '
//!                   WRITE (*,*) ' '
//!  
//!                END IF
//!  
//!             END DO
//!  
//!          END IF
//!  
//!          END
//! ```
//!
//!     
//! ###  Results
//!
//!  Any numerical results shown for this example may differ between
//!    platforms as the results depend on the SPICE kernels used as input and
//!    the machine specific arithmetic implementation.
//!
//!  The output from this program was as follows:
//!
//!  
//!
//! ```text
//!  
//!     Common MRO CK, MRO SPK and MER SPK coverage:
//!       2006 OCT 31 00:01:06.180062 TDB   2006 NOV 06 22:21:31.968150 TDB
//!       2006 NOV 06 22:24:36.379454 TDB   2006 NOV 15 16:38:42.527264 TDB
//!       2006 NOV 15 16:45:46.551315 TDB   2006 NOV 15 16:46:00.550204 TDB
//!       2006 NOV 15 16:53:51.675814 TDB   2006 NOV 15 23:09:04.754778 TDB
//!       2006 NOV 15 23:13:33.469771 TDB   2006 DEC 05 00:02:04.052144 TDB
//!     Measure of coverage window (sec):   3022709.6
//!  
//!     Starting distance search.
//!     Done.
//!     Measure of distance window (sec):   6455.26201
//!  
//!     Starting MRO_HIRISE_LOOK_DIRECTION frame's
//!     Y-Z plane crossing search.
//!     Done.
//!  
//!     Times of MER-1 visibility within MRO HIRISE nominal FOV swath:
//!  
//!        2006 NOV 14 15:41:02.511527 TDB
//!  
//!          Frame: MRO_HIRISE_LOOK_DIRECTION
//!  
//!          Target X-coordinate (km):  -1.22204848E-06
//!          Target Y-coordinate (km):  -0.893623145
//!          Target Z-coordinate (km):   278.011537
//!          Target range        (km):   278.012973
//!  
//!  
//!        2006 NOV 30 01:39:40.509680 TDB
//!  
//!          Frame: MRO_HIRISE_LOOK_DIRECTION
//!  
//!          Target X-coordinate (km):   1.21573781E-06
//!          Target Y-coordinate (km):  -0.577714696
//!          Target Z-coordinate (km):   267.423792
//!          Target range        (km):   267.424416
//!  
//!  
//! ```
//!
//!     
//! #  Appendix A --- Summary of GF Routines
//!
//!  
//!
//!
//!  
//! ##  Summary of Mnemonics
//!
//!  The following is a complete list of GF API mnemonics and translations,
//!    in alphabetical order.
//!
//!  A few of the routines listed are entry points of another routine. If a
//!    routine is an entry point, the parent routine's name will be listed
//!    inside brackets preceding the mnemonic translation.
//!
//!  
//!
//! ```text
//!    GFBAIL               Test for interrupt
//!    GFDIST               Distance search
//!    GFEVNT               Mid-level scalar constraint search
//!    GFFOVE               Mid-level FOV intersection search
//!    GFILUM               Illumination angle search
//!    GFOCCE               Mid-level occultation search
//!    GFOCLT               Find occultation
//!    GFPA                 Phase angle search
//!    GFPOSC               Position coordinate search
//!    GFREFN               Refine solution bounds
//!    GFREPF [GFRPRT]      Finalize progress report
//!    GFREPI [GFRPRT]      Initialize progress report
//!    GFREPU [GFRPRT]      Update progress report
//!    GFRFOV               Ray-FOV intersection search
//!    GFRR                 Range rate search
//!    GFSEP                Angular separation search
//!    GFSNTC               Surface intercept coordinate search
//!    GFSSTP [GFSTEP]      Set search step size
//!    GFSTEP [GFSTEP]      Get search step size
//!    GFSTOL               Set/reset GF search tolerance
//!    GFSUBC               Sub-observer coordinate search
//!    GFTFOV               Target-FOV intersection search
//!    GFUDB                User defined boolean function search
//!    GFUDS                User defined scalar function search
//! ```
//!
//!     
//! #  Appendix B --- Revision History
//!
//!  
//!
//!
//!  
//! ###  2017 JUN 19 by N. J. Bachman
//!
//!  Corrected typo.
//!
//!  
//!
//!
//!  
//! ###  2012 OCT 01 by E. D. Wright.
//!
//!  Documentation expanded to include descriptions of the illumination
//!    angles, body center phase angle, GF tolerance adjustment, and user
//!    defined boolean search routine capabilities.
//!
//!  Edits to description of orbital longitude.
//!
//!  
//!
//!
//!  
//! ###  2010 MAY 13 by E. D. Wright.
//!
//!  Documentation expanded to include descriptions of the range rate and
//!    user defined scalar search routine capabilities.
//!
//!  
//!
//!
//!  
//! ###  2009 APR 15 by N. J. Bachman.
//!
//!  Initial release.
//!
//!  
//!
//!
//!     
