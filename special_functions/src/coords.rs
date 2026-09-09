//! Classical curvilinear coordinate systems: bidirectional, checked
//! maps between Cartesian coordinates and the eleven classical
//! three-dimensional systems plus the three classical two-dimensional
//! systems (conventions per Morse & Feshbach, *Methods of Theoretical
//! Physics* ch. 5, and Arfken & Weber ch. 2; stated explicitly on
//! every function).
//!
//! # Error policy
//!
//! Everything returns `Result<_, String>`, the crate's standard. An
//! error is returned for: non-finite input components; invalid scale
//! parameters (`a <= 0`, or an ellipsoidal triple that is not
//! `a > b > c > 0`); coordinates outside their defining ranges where
//! the range is load-bearing (negative radii, negative `mu`/`u`/`tau`
//! where the chart requires them non-negative); and points ON a
//! genuine singular locus of a chart (the foci of the bipolar family,
//! the focal ring of toroidal coordinates, `sigma = tau = 0` in the
//! bipolar family, which is the point at infinity; the coordinate
//! planes for ellipsoidal coordinates, whose chart covers open
//! octants). Where a value is merely *conventional* rather than
//! undefined-and-unbounded — the azimuth on the polar axis, the angles
//! at the origin — the universal `atan2(0, 0) = 0` convention is used
//! and documented, so every map round-trips exactly on its domain.
//!
//! # Sign conventions
//!
//! Angles from `atan2` lie in `(-pi, pi]`; polar angles from `acos`
//! lie in `[0, pi]` (arguments clamped to `[-1, 1]` against roundoff).
//! Ellipsoidal coordinates encode squares, so their inverse map
//! returns the positive-octant point; the chart is degenerate on the
//! coordinate planes and that is reported as an error, not glossed.

/// Rejects non-finite components before any arithmetic runs.
fn finite3(what: &str, p: [f64; 3]) -> Result<(), String> {
    if p.iter().all(|v| v.is_finite()) {
        Ok(())
    } else {
        Err(format!("{what}: input components must be finite, got [{}, {}, {}]", p[0], p[1], p[2]))
    }
}

fn finite2(what: &str, p: [f64; 2]) -> Result<(), String> {
    if p.iter().all(|v| v.is_finite()) {
        Ok(())
    } else {
        Err(format!("{what}: input components must be finite, got [{}, {}]", p[0], p[1]))
    }
}

/// A positive scale parameter (the focal distance / focal ring radius).
fn positive_a(what: &str, a: f64) -> Result<(), String> {
    if a.is_finite() && a > 0.0 {
        Ok(())
    } else {
        Err(format!("{what}: the scale parameter a must be finite and > 0, got {a}"))
    }
}

fn clamp1(x: f64) -> f64 {
    x.clamp(-1.0, 1.0)
}

// ======================================================================
// 1. Spherical-polar coordinates (r, theta, phi)
// ======================================================================

/// Cartesian → spherical-polar `(r, theta, phi)`: physics (ISO)
/// convention — `r >= 0`, `theta` in `[0, pi]` measured from the +z
/// axis, `phi = atan2(y, x)` in `(-pi, pi]`.
///
/// At the origin `r = 0` both angles are conventional and this returns
/// `[0, 0, 0]` (which `spherical_to_cart` maps back exactly). On the
/// polar axis `phi` is conventional: `atan2(0, 0) = 0`.
pub fn cart_to_spherical(p: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("cart_to_spherical", p)?;
    let [x, y, z] = p;
    let r = (x * x + y * y + z * z).sqrt();
    if r == 0.0 {
        return Ok([0.0, 0.0, 0.0]);
    }
    Ok([r, clamp1(z / r).acos(), y.atan2(x)])
}

/// Spherical-polar `(r, theta, phi)` → Cartesian. Requires `r >= 0`.
pub fn spherical_to_cart(q: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("spherical_to_cart", q)?;
    let [r, theta, phi] = q;
    if r < 0.0 {
        return Err(format!("spherical_to_cart: r must be >= 0, got {r}"));
    }
    let (st, ct) = (theta.sin(), theta.cos());
    Ok([r * st * phi.cos(), r * st * phi.sin(), r * ct])
}

// ======================================================================
// 2. Circular cylindrical coordinates (rho, phi, z)
// ======================================================================

/// Cartesian → cylindrical `(rho, phi, z)`: `rho = hypot(x, y) >= 0`,
/// `phi = atan2(y, x)`. On the axis `phi` is conventional (0).
pub fn cart_to_cylindrical(p: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("cart_to_cylindrical", p)?;
    let [x, y, z] = p;
    Ok([x.hypot(y), y.atan2(x), z])
}

/// Cylindrical `(rho, phi, z)` → Cartesian. Requires `rho >= 0`.
pub fn cylindrical_to_cart(q: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("cylindrical_to_cart", q)?;
    let [rho, phi, z] = q;
    if rho < 0.0 {
        return Err(format!("cylindrical_to_cart: rho must be >= 0, got {rho}"));
    }
    Ok([rho * phi.cos(), rho * phi.sin(), z])
}

// ======================================================================
// 3. Ellipsoidal (confocal) coordinates (lambda, mu, nu; a > b > c > 0)
// ======================================================================

fn ellipsoidal_params(what: &str, a: f64, b: f64, c: f64) -> Result<(), String> {
    // One conjunction (De Morgan of the original two-clause form, which
    // clippy 1.91 flags as `nonminimal_bool` under deny(warnings)).
    if !(a.is_finite() && b.is_finite() && c.is_finite() && a > b && b > c && c > 0.0) {
        return Err(format!(
            "{what}: ellipsoidal coordinates need semi-axes a > b > c > 0, got a = {a}, b = {b}, c = {c}"
        ));
    }
    Ok(())
}

/// The confocal cubic `f(s) = x^2/(a^2+s) + y^2/(b^2+s) + z^2/(c^2+s) - 1`,
/// strictly decreasing between its poles, so each root brackets cleanly.
fn confocal_f(s: f64, x2: f64, y2: f64, z2: f64, a2: f64, b2: f64, c2: f64) -> f64 {
    x2 / (a2 + s) + y2 / (b2 + s) + z2 / (c2 + s) - 1.0
}

/// One bisection root of the confocal cubic on the open interval
/// `(lo, hi)` where `f(lo+) = +inf` and `f(hi-) < 0` (or `-inf`).
/// (Eight scalar arguments mirror the cubic's six coefficients plus the
/// bracket; bundling them would only add a struct — clippy's
/// `too_many_arguments` is waived here, nothing else changes.)
#[allow(clippy::too_many_arguments)]
fn confocal_root(
    mut lo: f64,
    mut hi: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    a2: f64,
    b2: f64,
    c2: f64,
) -> f64 {
    /* f is strictly decreasing on each interval between its poles, so
     * plain bisection is unconditionally safe; 200 halvings exhaust
     * f64 resolution on any interval. */
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break; // interval no longer representable
        }
        if confocal_f(mid, x2, y2, z2, a2, b2, c2) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// Cartesian → ellipsoidal `(lambda, mu, nu)` for semi-axes
/// `a > b > c > 0`: the three real roots of
/// `x^2/(a^2+s) + y^2/(b^2+s) + z^2/(c^2+s) = 1`, interlaced as
/// `lambda > -c^2 > mu > -b^2 > nu > -a^2` (Morse & Feshbach §5.1).
///
/// The chart is degenerate on the coordinate planes — ellipsoidal
/// coordinates encode `x^2, y^2, z^2` only — so a point with any zero
/// component is refused rather than silently folded into an octant.
pub fn cart_to_ellipsoidal(p: [f64; 3], a: f64, b: f64, c: f64) -> Result<[f64; 3], String> {
    finite3("cart_to_ellipsoidal", p)?;
    ellipsoidal_params("cart_to_ellipsoidal", a, b, c)?;
    let [x, y, z] = p;
    if x == 0.0 || y == 0.0 || z == 0.0 {
        return Err(format!(
            "cart_to_ellipsoidal: the chart is degenerate on the coordinate planes; \
             every component must be nonzero, got [{x}, {y}, {z}]"
        ));
    }
    let (x2, y2, z2) = (x * x, y * y, z * z);
    let (a2, b2, c2) = (a * a, b * b, c * c);
    /* Upper bracket for lambda: f(r^2) < 0 because every denominator
     * there exceeds r^2 = x^2 + y^2 + z^2. */
    let hi = x2 + y2 + z2;
    let lambda = confocal_root(-c2, hi.max(-c2 + 1.0), x2, y2, z2, a2, b2, c2);
    let mu = confocal_root(-b2, -c2, x2, y2, z2, a2, b2, c2);
    let nu = confocal_root(-a2, -b2, x2, y2, z2, a2, b2, c2);
    Ok([lambda, mu, nu])
}

/// Ellipsoidal `(lambda, mu, nu)` → Cartesian (positive octant — the
/// coordinates encode squares, so the signs of `x, y, z` are not part
/// of the chart). Requires the interlacing
/// `lambda > -c^2 >= mu >= -b^2 >= nu >= -a^2`; violations are
/// reported with the expected ordering spelled out.
pub fn ellipsoidal_to_cart(q: [f64; 3], a: f64, b: f64, c: f64) -> Result<[f64; 3], String> {
    finite3("ellipsoidal_to_cart", q)?;
    ellipsoidal_params("ellipsoidal_to_cart", a, b, c)?;
    let [lambda, mu, nu] = q;
    let (a2, b2, c2) = (a * a, b * b, c * c);
    if !(lambda > -c2 && (-b2..=-c2).contains(&mu) && (-a2..=-b2).contains(&nu)) {
        return Err(format!(
            "ellipsoidal_to_cart: coordinates must interlace as \
             lambda > {} > mu > {} > nu > {}, got lambda = {lambda}, mu = {mu}, nu = {nu}",
            -c2, -b2, -a2
        ));
    }
    let x2 = (a2 + lambda) * (a2 + mu) * (a2 + nu) / ((a2 - b2) * (a2 - c2));
    let y2 = (b2 + lambda) * (b2 + mu) * (b2 + nu) / ((b2 - a2) * (b2 - c2));
    let z2 = (c2 + lambda) * (c2 + mu) * (c2 + nu) / ((c2 - a2) * (c2 - b2));
    /* Exact-boundary triples can push a square to -0.0 through
     * roundoff; anything genuinely negative was rejected above. */
    Ok([x2.max(0.0).sqrt(), y2.max(0.0).sqrt(), z2.max(0.0).sqrt()])
}

// ======================================================================
// 4. Oblate spheroidal coordinates (mu, nu, phi; focal ring radius a)
// ======================================================================

/// Cartesian → oblate spheroidal `(mu, nu, phi)` with focal ring
/// `rho = a, z = 0`: `x = a cosh(mu) cos(nu) cos(phi)`,
/// `y = a cosh(mu) cos(nu) sin(phi)`, `z = a sinh(mu) sin(nu)`;
/// `mu >= 0`, `nu` in `[-pi/2, pi/2]` with the sign of `nu` following
/// `z`. The inverse follows from `rho + i z = a cosh(mu + i nu)`: with
/// the ring distances `d± = sqrt((rho ± a)^2 + z^2)`,
/// `cosh(mu) = (d+ + d-)/(2a)` and `cos(nu) = (d+ - d-)/(2a)`
/// (non-negative because `rho >= 0`, so `|nu| = acos(...) <= pi/2`).
///
/// On the disk `z = 0, rho < a` the chart has `mu = 0` and `±nu`
/// coincide; the non-negative `nu` is returned (round-trips exactly).
pub fn cart_to_oblate(p: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("cart_to_oblate", p)?;
    positive_a("cart_to_oblate", a)?;
    let [x, y, z] = p;
    let rho = x.hypot(y);
    let d1 = ((rho + a) * (rho + a) + z * z).sqrt();
    let d2 = ((rho - a) * (rho - a) + z * z).sqrt();
    let mu = (((d1 + d2) / (2.0 * a)).max(1.0)).acosh();
    /* d1 >= d2 because rho >= 0, so cos(nu) >= 0 and acos lands in
     * [0, pi/2]; the sign of nu is the sign of z. */
    let nu_mag = clamp1((d1 - d2) / (2.0 * a)).acos();
    let nu_mag = std::f64::consts::FRAC_PI_2.min(nu_mag);
    let nu = if z < 0.0 { -nu_mag } else { nu_mag };
    Ok([mu, nu, y.atan2(x)])
}

/// Oblate spheroidal `(mu, nu, phi)` → Cartesian. Requires `mu >= 0`
/// and `nu` in `[-pi/2, pi/2]`.
pub fn oblate_to_cart(q: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("oblate_to_cart", q)?;
    positive_a("oblate_to_cart", a)?;
    let [mu, nu, phi] = q;
    if mu < 0.0 {
        return Err(format!("oblate_to_cart: mu must be >= 0, got {mu}"));
    }
    if !(-std::f64::consts::FRAC_PI_2..=std::f64::consts::FRAC_PI_2).contains(&nu) {
        return Err(format!("oblate_to_cart: nu must lie in [-pi/2, pi/2], got {nu}"));
    }
    let rho = a * mu.cosh() * nu.cos();
    Ok([rho * phi.cos(), rho * phi.sin(), a * mu.sinh() * nu.sin()])
}

// ======================================================================
// 5. Prolate spheroidal coordinates (mu, nu, phi; foci at z = ±a)
// ======================================================================

/// Cartesian → prolate spheroidal `(mu, nu, phi)` with foci at
/// `(0, 0, ±a)`: `x = a sinh(mu) sin(nu) cos(phi)`,
/// `y = a sinh(mu) sin(nu) sin(phi)`, `z = a cosh(mu) cos(nu)`;
/// `mu >= 0`, `nu` in `[0, pi]`. Via the focal distances
/// `d± = sqrt(rho^2 + (z ∓ a)^2)`:
/// `cosh(mu) = (d- + d+)/(2a)`, `cos(nu) = (d- - d+)/(2a)`.
pub fn cart_to_prolate(p: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("cart_to_prolate", p)?;
    positive_a("cart_to_prolate", a)?;
    let [x, y, z] = p;
    let rho2 = x * x + y * y;
    let d_plus = (rho2 + (z - a) * (z - a)).sqrt();
    let d_minus = (rho2 + (z + a) * (z + a)).sqrt();
    let mu = (((d_minus + d_plus) / (2.0 * a)).max(1.0)).acosh();
    let nu = clamp1((d_minus - d_plus) / (2.0 * a)).acos();
    Ok([mu, nu, y.atan2(x)])
}

/// Prolate spheroidal `(mu, nu, phi)` → Cartesian. Requires `mu >= 0`
/// and `nu` in `[0, pi]`.
pub fn prolate_to_cart(q: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("prolate_to_cart", q)?;
    positive_a("prolate_to_cart", a)?;
    let [mu, nu, phi] = q;
    if mu < 0.0 {
        return Err(format!("prolate_to_cart: mu must be >= 0, got {mu}"));
    }
    if !(0.0..=std::f64::consts::PI).contains(&nu) {
        return Err(format!("prolate_to_cart: nu must lie in [0, pi], got {nu}"));
    }
    let rho = a * mu.sinh() * nu.sin();
    Ok([rho * phi.cos(), rho * phi.sin(), a * mu.cosh() * nu.cos()])
}

// ======================================================================
// 6. Paraboloidal (rotational parabolic) coordinates (u, v, phi)
// ======================================================================

/// Cartesian → paraboloidal `(u, v, phi)` (the rotationally symmetric
/// parabolic system): `x = u v cos(phi)`, `y = u v sin(phi)`,
/// `z = (u^2 - v^2)/2`, with `u, v >= 0`. Inverse:
/// `u = sqrt(r + z)`, `v = sqrt(r - z)` with `r = sqrt(rho^2 + z^2)`.
pub fn cart_to_paraboloidal(p: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("cart_to_paraboloidal", p)?;
    let [x, y, z] = p;
    let r = (x * x + y * y + z * z).sqrt();
    Ok([(r + z).max(0.0).sqrt(), (r - z).max(0.0).sqrt(), y.atan2(x)])
}

/// Paraboloidal `(u, v, phi)` → Cartesian. Requires `u, v >= 0`.
pub fn paraboloidal_to_cart(q: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("paraboloidal_to_cart", q)?;
    let [u, v, phi] = q;
    if u < 0.0 || v < 0.0 {
        return Err(format!("paraboloidal_to_cart: u and v must be >= 0, got u = {u}, v = {v}"));
    }
    let rho = u * v;
    Ok([rho * phi.cos(), rho * phi.sin(), 0.5 * (u * u - v * v)])
}

// ======================================================================
// 7. Elliptic cylindrical coordinates (u, v, z; foci at x = ±a, y = 0)
// ======================================================================

/// Cartesian → elliptic cylindrical `(u, v, z)` with in-plane foci
/// `(±a, 0)`: `x = a cosh(u) cos(v)`, `y = a sinh(u) sin(v)`;
/// `u >= 0`, `v` in `(-pi, pi]` with the sign of `v` following `y`.
/// Via `d± = sqrt((x ± a)^2 + y^2)`:
/// `cosh(u) = (d+ + d-)/(2a)`, `cos(v) = (d+ - d-)/(2a)`.
///
/// On the focal segment `|x| < a, y = 0` the chart has `u = 0` and
/// `±v` coincide; the non-negative `v` is returned.
pub fn cart_to_elliptic_cyl(p: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("cart_to_elliptic_cyl", p)?;
    positive_a("cart_to_elliptic_cyl", a)?;
    let [x, y, z] = p;
    let d1 = ((x + a) * (x + a) + y * y).sqrt();
    let d2 = ((x - a) * (x - a) + y * y).sqrt();
    let u = (((d1 + d2) / (2.0 * a)).max(1.0)).acosh();
    let v = clamp1((d1 - d2) / (2.0 * a)).acos();
    let v = if y < 0.0 { -v } else { v };
    Ok([u, v, z])
}

/// Elliptic cylindrical `(u, v, z)` → Cartesian. Requires `u >= 0`.
pub fn elliptic_cyl_to_cart(q: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("elliptic_cyl_to_cart", q)?;
    positive_a("elliptic_cyl_to_cart", a)?;
    let [u, v, z] = q;
    if u < 0.0 {
        return Err(format!("elliptic_cyl_to_cart: u must be >= 0, got {u}"));
    }
    Ok([a * u.cosh() * v.cos(), a * u.sinh() * v.sin(), z])
}

// ======================================================================
// 8. Parabolic cylindrical coordinates (sigma, tau, z)
// ======================================================================

/// Cartesian → parabolic cylindrical `(sigma, tau, z)`:
/// `x = sigma tau`, `y = (tau^2 - sigma^2)/2`, `z = z`, with
/// `tau >= 0` and the sign of `sigma` following `x`. Inverse:
/// `tau = sqrt(r + y)`, `|sigma| = sqrt(r - y)`, `r = hypot(x, y)`.
pub fn cart_to_parabolic_cyl(p: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("cart_to_parabolic_cyl", p)?;
    let [x, y, z] = p;
    let r = x.hypot(y);
    let tau = (r + y).max(0.0).sqrt();
    let sigma_abs = (r - y).max(0.0).sqrt();
    let sigma = if x < 0.0 { -sigma_abs } else { sigma_abs };
    Ok([sigma, tau, z])
}

/// Parabolic cylindrical `(sigma, tau, z)` → Cartesian. Requires
/// `tau >= 0`.
pub fn parabolic_cyl_to_cart(q: [f64; 3]) -> Result<[f64; 3], String> {
    finite3("parabolic_cyl_to_cart", q)?;
    let [sigma, tau, z] = q;
    if tau < 0.0 {
        return Err(format!("parabolic_cyl_to_cart: tau must be >= 0, got {tau}"));
    }
    Ok([sigma * tau, 0.5 * (tau * tau - sigma * sigma), z])
}

// ======================================================================
// 9. Bipolar cylindrical coordinates (sigma, tau, z; foci at x = ±a)
// ======================================================================

/// Cartesian → bipolar cylindrical `(sigma, tau, z)` with in-plane
/// foci `(±a, 0)`:
/// `x = a sinh(tau)/D`, `y = a sin(sigma)/D`, `D = cosh(tau) - cos(sigma)`;
/// `tau = (1/2) ln(d+^2 / d-^2)` with `d± = sqrt((x ± a)^2 + y^2)`,
/// `sigma = atan2(2 a y, x^2 + y^2 - a^2)` in `(-pi, pi]`.
///
/// The foci themselves are singular (`tau -> ±inf`) and are refused.
pub fn cart_to_bipolar_cyl(p: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("cart_to_bipolar_cyl", p)?;
    positive_a("cart_to_bipolar_cyl", a)?;
    let [x, y, z] = p;
    let d1sq = (x + a) * (x + a) + y * y;
    let d2sq = (x - a) * (x - a) + y * y;
    if d1sq == 0.0 || d2sq == 0.0 {
        return Err(format!(
            "cart_to_bipolar_cyl: ({x}, {y}) is a focus (±{a}, 0) — a singular point of \
             bipolar coordinates (tau diverges there)"
        ));
    }
    let tau = 0.5 * (d1sq / d2sq).ln();
    let sigma = (2.0 * a * y).atan2(x * x + y * y - a * a);
    Ok([sigma, tau, z])
}

/// Bipolar cylindrical `(sigma, tau, z)` → Cartesian.
/// `sigma = tau = 0` is the point at infinity and is refused.
pub fn bipolar_cyl_to_cart(q: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("bipolar_cyl_to_cart", q)?;
    positive_a("bipolar_cyl_to_cart", a)?;
    let [sigma, tau, z] = q;
    let d = tau.cosh() - sigma.cos();
    if d == 0.0 {
        return Err(
            "bipolar_cyl_to_cart: cosh(tau) - cos(sigma) = 0 (sigma = tau = 0) is the \
             point at infinity"
                .to_string(),
        );
    }
    Ok([a * tau.sinh() / d, a * sigma.sin() / d, z])
}

// ======================================================================
// 10. Toroidal coordinates (sigma, tau, phi; focal ring rho = a, z = 0)
// ======================================================================

/// Cartesian → toroidal `(sigma, tau, phi)` with focal ring
/// `rho = a, z = 0`:
/// `rho = a sinh(tau)/D`, `z = a sin(sigma)/D`, `D = cosh(tau) - cos(sigma)`;
/// `tau = (1/2) ln(((rho+a)^2 + z^2) / ((rho-a)^2 + z^2)) >= 0`,
/// `sigma = atan2(2 a z, rho^2 + z^2 - a^2)` in `(-pi, pi]`,
/// `phi = atan2(y, x)`.
///
/// The focal ring itself is singular (`tau -> inf`) and is refused.
pub fn cart_to_toroidal(p: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("cart_to_toroidal", p)?;
    positive_a("cart_to_toroidal", a)?;
    let [x, y, z] = p;
    let rho = x.hypot(y);
    let d1sq = (rho + a) * (rho + a) + z * z;
    let d2sq = (rho - a) * (rho - a) + z * z;
    if d2sq == 0.0 {
        return Err(format!(
            "cart_to_toroidal: (rho, z) = ({a}, 0) is on the focal ring — a singular \
             locus of toroidal coordinates (tau diverges there)"
        ));
    }
    let tau = 0.5 * (d1sq / d2sq).ln();
    let sigma = (2.0 * a * z).atan2(rho * rho + z * z - a * a);
    Ok([sigma, tau, y.atan2(x)])
}

/// Toroidal `(sigma, tau, phi)` → Cartesian. Requires `tau >= 0`;
/// `sigma = tau = 0` is the point at infinity and is refused.
pub fn toroidal_to_cart(q: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("toroidal_to_cart", q)?;
    positive_a("toroidal_to_cart", a)?;
    let [sigma, tau, phi] = q;
    if tau < 0.0 {
        return Err(format!("toroidal_to_cart: tau must be >= 0, got {tau}"));
    }
    let d = tau.cosh() - sigma.cos();
    if d == 0.0 {
        return Err(
            "toroidal_to_cart: cosh(tau) - cos(sigma) = 0 (sigma = tau = 0) is the point \
             at infinity"
                .to_string(),
        );
    }
    let rho = a * tau.sinh() / d;
    Ok([rho * phi.cos(), rho * phi.sin(), a * sigma.sin() / d])
}

// ======================================================================
// 11. Bispherical coordinates (sigma, tau, phi; foci at z = ±a)
// ======================================================================

/// Cartesian → bispherical `(sigma, tau, phi)` with foci `(0, 0, ±a)`:
/// `rho = a sin(sigma)/D`, `z = a sinh(tau)/D`, `D = cosh(tau) - cos(sigma)`;
/// `tau = (1/2) ln((rho^2 + (z+a)^2) / (rho^2 + (z-a)^2))`,
/// `sigma = atan2(2 a rho, rho^2 + z^2 - a^2)` in `[0, pi]`
/// (non-negative because `rho >= 0`), `phi = atan2(y, x)`.
///
/// The two foci are singular (`tau -> ±inf`) and are refused.
pub fn cart_to_bispherical(p: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("cart_to_bispherical", p)?;
    positive_a("cart_to_bispherical", a)?;
    let [x, y, z] = p;
    let rho = x.hypot(y);
    let d1sq = rho * rho + (z + a) * (z + a);
    let d2sq = rho * rho + (z - a) * (z - a);
    if d1sq == 0.0 || d2sq == 0.0 {
        return Err(format!(
            "cart_to_bispherical: (0, 0, ±{a}) is a focus — a singular point of \
             bispherical coordinates (tau diverges there)"
        ));
    }
    let tau = 0.5 * (d1sq / d2sq).ln();
    let sigma = (2.0 * a * rho).atan2(rho * rho + z * z - a * a);
    Ok([sigma, tau, y.atan2(x)])
}

/// Bispherical `(sigma, tau, phi)` → Cartesian. Requires `sigma` in
/// `[0, pi]`; `sigma = tau = 0` is the point at infinity and is
/// refused.
pub fn bispherical_to_cart(q: [f64; 3], a: f64) -> Result<[f64; 3], String> {
    finite3("bispherical_to_cart", q)?;
    positive_a("bispherical_to_cart", a)?;
    let [sigma, tau, phi] = q;
    if !(0.0..=std::f64::consts::PI).contains(&sigma) {
        return Err(format!("bispherical_to_cart: sigma must lie in [0, pi], got {sigma}"));
    }
    let d = tau.cosh() - sigma.cos();
    if d == 0.0 {
        return Err(
            "bispherical_to_cart: cosh(tau) - cos(sigma) = 0 (sigma = tau = 0) is the \
             point at infinity"
                .to_string(),
        );
    }
    let rho = a * sigma.sin() / d;
    Ok([rho * phi.cos(), rho * phi.sin(), a * tau.sinh() / d])
}

// ======================================================================
// 12. Plane polar coordinates (r, phi) — two-dimensional
// ======================================================================

/// 2-D Cartesian → plane polar `(r, phi)`: `r = hypot(x, y)`,
/// `phi = atan2(y, x)`. At the origin `phi` is conventional (0).
pub fn cart2_to_polar(p: [f64; 2]) -> Result<[f64; 2], String> {
    finite2("cart2_to_polar", p)?;
    let [x, y] = p;
    Ok([x.hypot(y), y.atan2(x)])
}

/// Plane polar `(r, phi)` → 2-D Cartesian. Requires `r >= 0`.
pub fn polar_to_cart2(q: [f64; 2]) -> Result<[f64; 2], String> {
    finite2("polar_to_cart2", q)?;
    let [r, phi] = q;
    if r < 0.0 {
        return Err(format!("polar_to_cart2: r must be >= 0, got {r}"));
    }
    Ok([r * phi.cos(), r * phi.sin()])
}

// ======================================================================
// 13. Elliptic coordinates (u, v; foci at x = ±a) — two-dimensional
// ======================================================================

/// 2-D Cartesian → elliptic `(u, v)` with foci `(±a, 0)`:
/// `x = a cosh(u) cos(v)`, `y = a sinh(u) sin(v)`; `u >= 0`, `v` in
/// `(-pi, pi]` with the sign of `v` following `y`. (The planar section
/// of elliptic cylindrical coordinates.)
pub fn cart2_to_elliptic(p: [f64; 2], a: f64) -> Result<[f64; 2], String> {
    finite2("cart2_to_elliptic", p)?;
    let q = cart_to_elliptic_cyl([p[0], p[1], 0.0], a)?;
    Ok([q[0], q[1]])
}

/// Elliptic `(u, v)` → 2-D Cartesian. Requires `u >= 0`.
pub fn elliptic_to_cart2(q: [f64; 2], a: f64) -> Result<[f64; 2], String> {
    finite2("elliptic_to_cart2", q)?;
    let p = elliptic_cyl_to_cart([q[0], q[1], 0.0], a)?;
    Ok([p[0], p[1]])
}

// ======================================================================
// 14. Bipolar coordinates (sigma, tau; foci at x = ±a) — two-dimensional
//     (the special conformal extension)
// ======================================================================

/// 2-D Cartesian → bipolar `(sigma, tau)` with foci `(±a, 0)`. This is
/// the planar bipolar system,
/// `x = a sinh(tau)/D`, `y = a sin(sigma)/D`, `D = cosh(tau) - cos(sigma)`,
/// and it is CONFORMAL: it is the real form of the analytic map
/// `x + i y = a coth((tau - i sigma)/2)` — the "special conformal
/// extension" of the two-center geometry — so angles between crossing
/// curves are preserved everywhere off the foci (checked numerically
/// in the tests). The foci are singular and refused.
pub fn cart2_to_bipolar(p: [f64; 2], a: f64) -> Result<[f64; 2], String> {
    finite2("cart2_to_bipolar", p)?;
    let q = cart_to_bipolar_cyl([p[0], p[1], 0.0], a)?;
    Ok([q[0], q[1]])
}

/// Bipolar `(sigma, tau)` → 2-D Cartesian. `sigma = tau = 0` is the
/// point at infinity and is refused.
pub fn bipolar_to_cart2(q: [f64; 2], a: f64) -> Result<[f64; 2], String> {
    finite2("bipolar_to_cart2", q)?;
    let p = bipolar_cyl_to_cart([q[0], q[1], 0.0], a)?;
    Ok([p[0], p[1]])
}

// ======================================================================
// The dispatcher the DATASET MAP command drives
// ======================================================================

/// A named, parameterized 3-D coordinate map (Cartesian → curvilinear).
#[derive(Clone, Debug, PartialEq)]
pub enum Map3 {
    Spherical,
    Cylindrical,
    Ellipsoidal { a: f64, b: f64, c: f64 },
    Oblate { a: f64 },
    Prolate { a: f64 },
    Paraboloidal,
    EllipticCyl { a: f64 },
    ParabolicCyl,
    BipolarCyl { a: f64 },
    Toroidal { a: f64 },
    Bispherical { a: f64 },
}

/// A named, parameterized 2-D coordinate map (Cartesian → curvilinear),
/// applied in the plane `z = 0`.
#[derive(Clone, Debug, PartialEq)]
pub enum Map2 {
    Polar,
    Elliptic { a: f64 },
    Bipolar { a: f64 },
}

impl Map3 {
    pub fn forward(&self, p: [f64; 3]) -> Result<[f64; 3], String> {
        match *self {
            Map3::Spherical => cart_to_spherical(p),
            Map3::Cylindrical => cart_to_cylindrical(p),
            Map3::Ellipsoidal { a, b, c } => cart_to_ellipsoidal(p, a, b, c),
            Map3::Oblate { a } => cart_to_oblate(p, a),
            Map3::Prolate { a } => cart_to_prolate(p, a),
            Map3::Paraboloidal => cart_to_paraboloidal(p),
            Map3::EllipticCyl { a } => cart_to_elliptic_cyl(p, a),
            Map3::ParabolicCyl => cart_to_parabolic_cyl(p),
            Map3::BipolarCyl { a } => cart_to_bipolar_cyl(p, a),
            Map3::Toroidal { a } => cart_to_toroidal(p, a),
            Map3::Bispherical { a } => cart_to_bispherical(p, a),
        }
    }
}

impl Map2 {
    pub fn forward(&self, p: [f64; 2]) -> Result<[f64; 2], String> {
        match *self {
            Map2::Polar => cart2_to_polar(p),
            Map2::Elliptic { a } => cart2_to_elliptic(p, a),
            Map2::Bipolar { a } => cart2_to_bipolar(p, a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    const TOL: f64 = 1e-11;

    fn close3(a: [f64; 3], b: [f64; 3], scale: f64) -> bool {
        a.iter().zip(b.iter()).all(|(p, q)| (p - q).abs() <= TOL * scale.max(1.0))
    }

    fn close2(a: [f64; 2], b: [f64; 2], scale: f64) -> bool {
        a.iter().zip(b.iter()).all(|(p, q)| (p - q).abs() <= TOL * scale.max(1.0))
    }

    /// cart -> curvilinear -> cart is the identity on a grid of generic
    /// points, for every 3-D system.
    #[test]
    fn three_dimensional_round_trips_hold_on_a_generic_grid() {
        let pts: Vec<[f64; 3]> = vec![
            [1.3, 0.7, 2.1],
            [-0.9, 1.8, -0.4],
            [2.5, -1.1, 0.6],
            [-1.7, -2.3, 1.9],
            [0.31, 0.77, -2.6],
            [3.4, 1.2, -1.5],
        ];
        let a = 1.25;
        for &p in &pts {
            let scale = p.iter().map(|v| v.abs()).fold(0.0, f64::max);
            let cases: Vec<(&str, [f64; 3])> = vec![
                ("spherical", spherical_to_cart(cart_to_spherical(p).unwrap()).unwrap()),
                ("cylindrical", cylindrical_to_cart(cart_to_cylindrical(p).unwrap()).unwrap()),
                ("oblate", oblate_to_cart(cart_to_oblate(p, a).unwrap(), a).unwrap()),
                ("prolate", prolate_to_cart(cart_to_prolate(p, a).unwrap(), a).unwrap()),
                ("paraboloidal", paraboloidal_to_cart(cart_to_paraboloidal(p).unwrap()).unwrap()),
                (
                    "elliptic_cyl",
                    elliptic_cyl_to_cart(cart_to_elliptic_cyl(p, a).unwrap(), a).unwrap(),
                ),
                (
                    "parabolic_cyl",
                    parabolic_cyl_to_cart(cart_to_parabolic_cyl(p).unwrap()).unwrap(),
                ),
                (
                    "bipolar_cyl",
                    bipolar_cyl_to_cart(cart_to_bipolar_cyl(p, a).unwrap(), a).unwrap(),
                ),
                ("toroidal", toroidal_to_cart(cart_to_toroidal(p, a).unwrap(), a).unwrap()),
                (
                    "bispherical",
                    bispherical_to_cart(cart_to_bispherical(p, a).unwrap(), a).unwrap(),
                ),
            ];
            for (name, back) in cases {
                assert!(close3(back, p, scale), "{name} round trip at {p:?}: got {back:?}");
            }
            /* Ellipsoidal encodes squares: round trip in the positive
             * octant. */
            let pa = [p[0].abs(), p[1].abs(), p[2].abs()];
            let back =
                ellipsoidal_to_cart(cart_to_ellipsoidal(pa, 3.0, 2.0, 1.0).unwrap(), 3.0, 2.0, 1.0)
                    .unwrap();
            assert!(close3(back, pa, scale), "ellipsoidal round trip at {pa:?}: got {back:?}");
        }
    }

    /// curvilinear -> cart -> curvilinear is the identity on in-range
    /// coordinates (the direction that exercises the range checks).
    #[test]
    fn inverse_round_trips_hold_inside_the_charts() {
        let a = 0.8;
        let q3 = [0.9, 0.6, -1.1]; // generic in-range triple
        assert!(close3(
            cart_to_spherical(spherical_to_cart([2.0, 0.7, -0.6]).unwrap()).unwrap(),
            [2.0, 0.7, -0.6],
            2.0
        ));
        assert!(close3(
            cart_to_cylindrical(cylindrical_to_cart([1.4, 2.2, -0.3]).unwrap()).unwrap(),
            [1.4, 2.2, -0.3],
            2.2
        ));
        assert!(close3(cart_to_oblate(oblate_to_cart(q3, a).unwrap(), a).unwrap(), q3, 1.1));
        assert!(close3(
            cart_to_prolate(prolate_to_cart([0.9, 0.6, -1.1], a).unwrap(), a).unwrap(),
            q3,
            1.1
        ));
        assert!(close3(
            cart_to_paraboloidal(paraboloidal_to_cart([1.2, 0.5, 2.0]).unwrap()).unwrap(),
            [1.2, 0.5, 2.0],
            2.0
        ));
        assert!(close3(
            cart_to_elliptic_cyl(elliptic_cyl_to_cart([0.7, 2.1, 0.4], a).unwrap(), a).unwrap(),
            [0.7, 2.1, 0.4],
            2.1
        ));
        assert!(close3(
            cart_to_parabolic_cyl(parabolic_cyl_to_cart([-0.8, 1.3, 0.2]).unwrap()).unwrap(),
            [-0.8, 1.3, 0.2],
            1.3
        ));
        assert!(close3(
            cart_to_bipolar_cyl(bipolar_cyl_to_cart([1.9, -0.7, 0.3], a).unwrap(), a).unwrap(),
            [1.9, -0.7, 0.3],
            1.9
        ));
        assert!(close3(
            cart_to_toroidal(toroidal_to_cart([1.1, 0.8, 2.4], a).unwrap(), a).unwrap(),
            [1.1, 0.8, 2.4],
            2.4
        ));
        assert!(close3(
            cart_to_bispherical(bispherical_to_cart([1.0, -0.9, 0.5], a).unwrap(), a).unwrap(),
            [1.0, -0.9, 0.5],
            1.0
        ));
        // ellipsoidal, interlaced for a,b,c = 3,2,1
        let e = [1.7, -2.5, -6.1];
        assert!(close3(
            cart_to_ellipsoidal(ellipsoidal_to_cart(e, 3.0, 2.0, 1.0).unwrap(), 3.0, 2.0, 1.0)
                .unwrap(),
            e,
            6.1
        ));
    }

    /// Two-dimensional systems round-trip both ways.
    #[test]
    fn two_dimensional_round_trips_hold() {
        let a = 1.5;
        let pts: [[f64; 2]; 4] = [[1.2, -0.8], [-2.1, 0.4], [0.6, 2.9], [-0.3, -1.7]];
        for &p in &pts {
            let scale = p[0].abs().max(p[1].abs());
            assert!(close2(polar_to_cart2(cart2_to_polar(p).unwrap()).unwrap(), p, scale));
            assert!(close2(
                elliptic_to_cart2(cart2_to_elliptic(p, a).unwrap(), a).unwrap(),
                p,
                scale
            ));
            assert!(close2(
                bipolar_to_cart2(cart2_to_bipolar(p, a).unwrap(), a).unwrap(),
                p,
                scale
            ));
        }
        assert!(close2(
            cart2_to_polar(polar_to_cart2([2.5, -1.9]).unwrap()).unwrap(),
            [2.5, -1.9],
            2.5
        ));
        assert!(close2(
            cart2_to_elliptic(elliptic_to_cart2([0.8, 1.1], a).unwrap(), a).unwrap(),
            [0.8, 1.1],
            1.1
        ));
        assert!(close2(
            cart2_to_bipolar(bipolar_to_cart2([2.2, 0.9], a).unwrap(), a).unwrap(),
            [2.2, 0.9],
            2.2
        ));
    }

    /// Hand-checked anchor values, one per family.
    #[test]
    fn known_values_match_the_stated_conventions() {
        // Spherical: (0, 0, 2) -> r = 2, theta = 0.
        let s = cart_to_spherical([0.0, 0.0, 2.0]).unwrap();
        assert!(close3(s, [2.0, 0.0, 0.0], 2.0));
        // Cylindrical: (0, 3, -1) -> rho 3, phi pi/2.
        let c = cart_to_cylindrical([0.0, 3.0, -1.0]).unwrap();
        assert!(close3(c, [3.0, FRAC_PI_2, -1.0], 3.0));
        // Oblate: the z axis at height h has sinh(mu) = h/a, nu = pi/2.
        let a = 2.0;
        let o = cart_to_oblate([0.0, 0.0, 1.5], a).unwrap();
        assert!((o[0] - (1.5f64 / a).asinh()).abs() < 1e-12, "mu = {}", o[0]);
        assert!((o[1] - FRAC_PI_2).abs() < 1e-12, "nu = {}", o[1]);
        // Prolate: on the +z axis beyond the focus, nu = 0, cosh(mu) = z/a.
        let p = cart_to_prolate([0.0, 0.0, 3.0], a).unwrap();
        assert!((p[0] - (3.0f64 / a).acosh()).abs() < 1e-12);
        assert!(p[1].abs() < 1e-12);
        // Paraboloidal: (0, 0, z>0) has v = 0, u = sqrt(2 z).
        let q = cart_to_paraboloidal([0.0, 0.0, 4.5]).unwrap();
        assert!((q[0] - 3.0).abs() < 1e-12 && q[1].abs() < 1e-12);
        // Bipolar (2-D): sigma = pi/2, tau = 0 is the point (0, a).
        let b = bipolar_to_cart2([FRAC_PI_2, 0.0], a).unwrap();
        assert!(close2(b, [0.0, a], a));
        let back = cart2_to_bipolar([0.0, a], a).unwrap();
        assert!(close2(back, [FRAC_PI_2, 0.0], 1.0));
        // Toroidal: the origin has tau = 0, sigma = pi.
        let t = cart_to_toroidal([0.0, 0.0, 0.0], a).unwrap();
        assert!((t[0] - PI).abs() < 1e-12 && t[1].abs() < 1e-12);
        // Bispherical: the origin has tau = 0, sigma = pi.
        let bs = cart_to_bispherical([0.0, 0.0, 0.0], a).unwrap();
        assert!((bs[0] - PI).abs() < 1e-12 && bs[1].abs() < 1e-12);
        // Ellipsoidal: the roots interlace and satisfy the defining cubic.
        let (aa, bb, cc) = (3.0, 2.0, 1.0);
        let pt = [1.1, 0.7, 0.4];
        let [l, m, n] = cart_to_ellipsoidal(pt, aa, bb, cc).unwrap();
        assert!(
            l > -cc * cc
                && (-bb * bb..-cc * cc).contains(&m)
                && (-aa * aa..-bb * bb).contains(&n)
        );
        for s in [l, m, n] {
            let f = pt[0] * pt[0] / (aa * aa + s)
                + pt[1] * pt[1] / (bb * bb + s)
                + pt[2] * pt[2] / (cc * cc + s)
                - 1.0;
            assert!(f.abs() < 1e-10, "confocal residual {f} at root {s}");
        }
    }

    /// Every documented error path actually errors, with a message that
    /// names the problem.
    #[test]
    fn error_paths_are_reported_not_glossed() {
        // Non-finite input.
        assert!(cart_to_spherical([f64::NAN, 0.0, 0.0]).unwrap_err().contains("finite"));
        assert!(cart2_to_polar([f64::INFINITY, 0.0]).unwrap_err().contains("finite"));
        // Invalid scale parameters.
        assert!(cart_to_toroidal([1.0, 0.0, 0.0], 0.0).unwrap_err().contains("a must be"));
        assert!(cart_to_oblate([1.0, 0.0, 0.0], -2.0).unwrap_err().contains("a must be"));
        assert!(cart_to_ellipsoidal([1.0, 1.0, 1.0], 1.0, 2.0, 3.0)
            .unwrap_err()
            .contains("a > b > c"));
        // Range violations on the curvilinear side.
        assert!(spherical_to_cart([-1.0, 0.0, 0.0]).unwrap_err().contains("r must be >= 0"));
        assert!(cylindrical_to_cart([-0.1, 0.0, 0.0]).unwrap_err().contains("rho must be >= 0"));
        assert!(oblate_to_cart([-0.1, 0.0, 0.0], 1.0).unwrap_err().contains("mu must be >= 0"));
        assert!(prolate_to_cart([0.5, 4.0, 0.0], 1.0).unwrap_err().contains("[0, pi]"));
        assert!(paraboloidal_to_cart([-1.0, 1.0, 0.0]).unwrap_err().contains(">= 0"));
        assert!(parabolic_cyl_to_cart([1.0, -1.0, 0.0])
            .unwrap_err()
            .contains("tau must be >= 0"));
        assert!(toroidal_to_cart([0.5, -0.2, 0.0], 1.0).unwrap_err().contains("tau must be >= 0"));
        assert!(bispherical_to_cart([4.0, 0.5, 0.0], 1.0).unwrap_err().contains("[0, pi]"));
        assert!(ellipsoidal_to_cart([0.0, 0.0, 0.0], 3.0, 2.0, 1.0)
            .unwrap_err()
            .contains("interlace"));
        // Singular loci.
        assert!(cart_to_bipolar_cyl([1.0, 0.0, 0.0], 1.0).unwrap_err().contains("focus"));
        assert!(cart_to_toroidal([1.0, 0.0, 0.0], 1.0).unwrap_err().contains("focal ring"));
        assert!(cart_to_bispherical([0.0, 0.0, 1.0], 1.0).unwrap_err().contains("focus"));
        assert!(cart2_to_bipolar([-1.5, 0.0], 1.5).unwrap_err().contains("focus"));
        assert!(bipolar_cyl_to_cart([0.0, 0.0, 0.0], 1.0).unwrap_err().contains("infinity"));
        assert!(toroidal_to_cart([0.0, 0.0, 0.0], 1.0).unwrap_err().contains("infinity"));
        assert!(bispherical_to_cart([0.0, 0.0, 0.0], 1.0).unwrap_err().contains("infinity"));
        // Ellipsoidal chart degeneracy on the coordinate planes.
        assert!(cart_to_ellipsoidal([0.0, 1.0, 1.0], 3.0, 2.0, 1.0)
            .unwrap_err()
            .contains("degenerate"));
    }

    /// The 2-D bipolar map is conformal: the images of the sigma and
    /// tau coordinate lines cross at right angles with equal scale
    /// factors — checked numerically.
    #[test]
    fn planar_bipolar_is_conformal() {
        let a = 1.0;
        let h = 1e-6;
        for &(sigma, tau) in &[(0.7, 0.9), (-1.3, 0.4), (2.0, -1.1)] {
            let f = |s: f64, t: f64| bipolar_to_cart2([s, t], a).unwrap();
            let p0 = f(sigma, tau);
            let ds = f(sigma + h, tau);
            let dt = f(sigma, tau + h);
            let e1 = [(ds[0] - p0[0]) / h, (ds[1] - p0[1]) / h];
            let e2 = [(dt[0] - p0[0]) / h, (dt[1] - p0[1]) / h];
            let dot = e1[0] * e2[0] + e1[1] * e2[1];
            let n1 = (e1[0] * e1[0] + e1[1] * e1[1]).sqrt();
            let n2 = (e2[0] * e2[0] + e2[1] * e2[1]).sqrt();
            assert!((dot / (n1 * n2)).abs() < 1e-5, "not orthogonal at ({sigma}, {tau})");
            assert!((n1 - n2).abs() / n1 < 1e-4, "scale factors differ at ({sigma}, {tau})");
        }
    }

    /// The dispatcher names resolve to the same functions.
    #[test]
    fn the_map_dispatchers_agree_with_the_named_functions() {
        let p = [1.3, 0.7, 2.1];
        assert_eq!(Map3::Spherical.forward(p).unwrap(), cart_to_spherical(p).unwrap());
        assert_eq!(
            Map3::Toroidal { a: 1.25 }.forward(p).unwrap(),
            cart_to_toroidal(p, 1.25).unwrap()
        );
        assert_eq!(
            Map3::Ellipsoidal { a: 3.0, b: 2.0, c: 1.0 }.forward(p).unwrap(),
            cart_to_ellipsoidal(p, 3.0, 2.0, 1.0).unwrap()
        );
        let q = [0.4, -0.9];
        assert_eq!(Map2::Polar.forward(q).unwrap(), cart2_to_polar(q).unwrap());
        assert_eq!(
            Map2::Bipolar { a: 2.0 }.forward(q).unwrap(),
            cart2_to_bipolar(q, 2.0).unwrap()
        );
    }
}
