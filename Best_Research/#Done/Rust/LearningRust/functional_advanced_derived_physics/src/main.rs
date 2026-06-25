use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// ============================================================
// Core units
// ============================================================

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Length(pub f64); // meters

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Time(pub f64); // seconds

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Mass(pub f64); // kilograms

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Velocity(pub f64); // meters / second

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Acceleration(pub f64); // meters / second^2

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Force(pub f64); // newtons = kg * m / s^2

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Damping(pub f64); // kg / s

// ============================================================
// Constructors
// ============================================================

#[inline]
pub fn meters(value: f64) -> Length {
    return Length(value);
}

#[inline]
pub fn seconds(value: f64) -> Time {
    return Time(value);
}

#[inline]
pub fn kilograms(value: f64) -> Mass {
    return Mass(value);
}

#[inline]
pub fn meters_per_second(value: f64) -> Velocity {
    return Velocity(value);
}

#[inline]
pub fn meters_per_second_squared(value: f64) -> Acceleration {
    return Acceleration(value);
}

#[inline]
pub fn newtons(value: f64) -> Force {
    return Force(value);
}

#[inline]
pub fn damping(value: f64) -> Damping {
    return Damping(value);
}

// ============================================================
// Pretty printing
// ============================================================

macro_rules! impl_display {
    ($type_name:ident, $unit:literal) => {
        impl fmt::Display for $type_name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                return write!(formatter, "{} {}", self.0, $unit);
            }
        }
    };
}

impl_display!(Length, "m");
impl_display!(Time, "s");
impl_display!(Mass, "kg");
impl_display!(Velocity, "m/s");
impl_display!(Acceleration, "m/s^2");
impl_display!(Force, "N");
impl_display!(Damping, "kg/s");

// ============================================================
// Same-type arithmetic
// ============================================================

macro_rules! impl_add_sub_same {
    ($type_name:ident) => {
        impl Add<$type_name> for $type_name {
            type Output = $type_name;

            #[inline]
            fn add(self, rhs: $type_name) -> Self::Output {
                return $type_name(self.0 + rhs.0);
            }
        }

        impl Sub<$type_name> for $type_name {
            type Output = $type_name;

            #[inline]
            fn sub(self, rhs: $type_name) -> Self::Output {
                return $type_name(self.0 - rhs.0);
            }
        }
    };
}

impl_add_sub_same!(Length);
impl_add_sub_same!(Time);
impl_add_sub_same!(Mass);
impl_add_sub_same!(Velocity);
impl_add_sub_same!(Acceleration);
impl_add_sub_same!(Force);
impl_add_sub_same!(Damping);

// ============================================================
// Scalar arithmetic
// ============================================================

macro_rules! impl_scalar_ops {
    ($type_name:ident) => {
        impl Mul<f64> for $type_name {
            type Output = $type_name;

            #[inline]
            fn mul(self, rhs: f64) -> Self::Output {
                return $type_name(self.0 * rhs);
            }
        }

        impl Div<f64> for $type_name {
            type Output = $type_name;

            #[inline]
            fn div(self, rhs: f64) -> Self::Output {
                return $type_name(self.0 / rhs);
            }
        }

        impl Mul<$type_name> for f64 {
            type Output = $type_name;

            #[inline]
            fn mul(self, rhs: $type_name) -> Self::Output {
                return $type_name(self * rhs.0);
            }
        }
    };
}

impl_scalar_ops!(Length);
impl_scalar_ops!(Time);
impl_scalar_ops!(Mass);
impl_scalar_ops!(Velocity);
impl_scalar_ops!(Acceleration);
impl_scalar_ops!(Force);
impl_scalar_ops!(Damping);

// ============================================================
// Cross-unit arithmetic
// ============================================================

// velocity = length / time
impl Div<Time> for Length {
    type Output = Velocity;

    #[inline]
    fn div(self, rhs: Time) -> Self::Output {
        return Velocity(self.0 / rhs.0);
    }
}

// acceleration = velocity / time
impl Div<Time> for Velocity {
    type Output = Acceleration;

    #[inline]
    fn div(self, rhs: Time) -> Self::Output {
        return Acceleration(self.0 / rhs.0);
    }
}

// length = velocity * time
impl Mul<Time> for Velocity {
    type Output = Length;

    #[inline]
    fn mul(self, rhs: Time) -> Self::Output {
        return Length(self.0 * rhs.0);
    }
}

// length = time * velocity
impl Mul<Velocity> for Time {
    type Output = Length;

    #[inline]
    fn mul(self, rhs: Velocity) -> Self::Output {
        return Length(self.0 * rhs.0);
    }
}

// velocity = acceleration * time
impl Mul<Time> for Acceleration {
    type Output = Velocity;

    #[inline]
    fn mul(self, rhs: Time) -> Self::Output {
        return Velocity(self.0 * rhs.0);
    }
}

// velocity = time * acceleration
impl Mul<Acceleration> for Time {
    type Output = Velocity;

    #[inline]
    fn mul(self, rhs: Acceleration) -> Self::Output {
        return Velocity(self.0 * rhs.0);
    }
}

// force = mass * acceleration
impl Mul<Acceleration> for Mass {
    type Output = Force;

    #[inline]
    fn mul(self, rhs: Acceleration) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

impl Mul<Mass> for Acceleration {
    type Output = Force;

    #[inline]
    fn mul(self, rhs: Mass) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

// acceleration = force / mass
impl Div<Mass> for Force {
    type Output = Acceleration;

    #[inline]
    fn div(self, rhs: Mass) -> Self::Output {
        return Acceleration(self.0 / rhs.0);
    }
}

// force = damping * velocity
impl Mul<Velocity> for Damping {
    type Output = Force;

    #[inline]
    fn mul(self, rhs: Velocity) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

impl Mul<Damping> for Velocity {
    type Output = Force;

    #[inline]
    fn mul(self, rhs: Damping) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

// velocity = force / damping
impl Div<Damping> for Force {
    type Output = Velocity;

    #[inline]
    fn div(self, rhs: Damping) -> Self::Output {
        return Velocity(self.0 / rhs.0);
    }
}

// ============================================================
// ODE core
// ============================================================
//
// These names stay close to the mathematics:
//
//   y' = f(t, y)
//
// - state
// - derivative
// - system
// - time step
//
// ============================================================

pub trait StateDerivative: Copy {
    fn plus(self, rhs: Self) -> Self;
    fn scale_by(self, scalar: f64) -> Self;
}

pub trait OdeState: Copy {
    type Derivative: StateDerivative;

    fn apply_derivative(self, derivative: Self::Derivative, time_step: Time) -> Self;
}

pub trait OdeSystem {
    type State: OdeState;

    fn derivative(&self, time: Time, state: Self::State) -> <Self::State as OdeState>::Derivative;
}

// ============================================================
// Example state for 1D motion
//
// position' = velocity
// velocity' = acceleration
// ============================================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MotionState {
    pub position: Length,
    pub velocity: Velocity,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MotionDerivative {
    pub dposition_dt: Velocity,
    pub dvelocity_dt: Acceleration,
}

impl StateDerivative for MotionDerivative {
    fn plus(self, rhs: Self) -> Self {
        return Self {
            dposition_dt: self.dposition_dt + rhs.dposition_dt,
            dvelocity_dt: self.dvelocity_dt + rhs.dvelocity_dt,
        };
    }

    fn scale_by(self, scalar: f64) -> Self {
        return Self {
            dposition_dt: self.dposition_dt * scalar,
            dvelocity_dt: self.dvelocity_dt * scalar,
        };
    }
}

impl OdeState for MotionState {
    type Derivative = MotionDerivative;

    fn apply_derivative(self, derivative: Self::Derivative, time_step: Time) -> Self {
        return Self {
            position: self.position + derivative.dposition_dt * time_step,
            velocity: self.velocity + derivative.dvelocity_dt * time_step,
        };
    }
}

// ============================================================
// RK4 integrator
// ============================================================

pub fn rk4_step<S>(system: &S, time: Time, state: S::State, time_step: Time) -> S::State
where
    S: OdeSystem,
{
    let half_step = time_step * 0.5;

    let k1 = system.derivative(time, state);
    let k2 = system.derivative(time + half_step, state.apply_derivative(k1, half_step));
    let k3 = system.derivative(time + half_step, state.apply_derivative(k2, half_step));
    let k4 = system.derivative(time + time_step, state.apply_derivative(k3, time_step));

    let weighted_derivative = k1
        .plus(k2.scale_by(2.0))
        .plus(k3.scale_by(2.0))
        .plus(k4)
        .scale_by(1.0 / 6.0);

    return state.apply_derivative(weighted_derivative, time_step);
}

// ============================================================
// Solution / trajectory
// ============================================================

#[derive(Copy, Clone, Debug)]
pub struct Sample<State> {
    pub time: Time,
    pub state: State,
}

#[derive(Clone, Debug)]
pub struct Solution<State> {
    samples: Vec<Sample<State>>,
}

impl<State> Solution<State> {
    pub fn samples(&self) -> &[Sample<State>] {
        return &self.samples;
    }

    pub fn first(&self) -> Option<&Sample<State>> {
        return self.samples.first();
    }

    pub fn last(&self) -> Option<&Sample<State>> {
        return self.samples.last();
    }

    pub fn len(&self) -> usize {
        return self.samples.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.samples.is_empty();
    }
}

pub fn integrate<S>(
    system: &S,
    initial_time: Time,
    initial_state: S::State,
    time_step: Time,
    steps: usize,
) -> Solution<S::State>
where
    S: OdeSystem,
    S::State: Copy,
{
    let mut samples = Vec::with_capacity(steps + 1);

    let mut time = initial_time;
    let mut state = initial_state;

    samples.push(Sample { time, state });

    for _ in 0..steps {
        state = rk4_step(system, time, state, time_step);
        time = time + time_step;

        samples.push(Sample { time, state });
    }

    return Solution { samples };
}

// ============================================================
// Falling body with linear drag
//
// position' = velocity
// velocity' = gravity - (damping / mass) * velocity
//
// Positive position is upward.
// Positive velocity is upward.
// Gravity is usually negative.
// ============================================================

#[derive(Copy, Clone, Debug)]
pub struct FallingBodySystem {
    pub mass: Mass,
    pub damping: Damping,
    pub gravity: Acceleration,
}

impl FallingBodySystem {
    // ---------------------------
    // Real-world interpretation
    // ---------------------------

    pub fn height(&self, state: MotionState) -> Length {
        return state.position;
    }

    pub fn speed(&self, state: MotionState) -> Velocity {
        return Velocity(state.velocity.0.abs());
    }

    pub fn drag_force(&self, state: MotionState) -> Force {
        return self.damping * state.velocity;
    }

    pub fn drag_force_magnitude(&self, state: MotionState) -> Force {
        return Force((self.damping * state.velocity).0.abs());
    }

    pub fn gravity_force(&self) -> Force {
        return self.mass * self.gravity;
    }

    pub fn net_force(&self, state: MotionState) -> Force {
        return self.gravity_force() - self.drag_force(state);
    }

    pub fn acceleration(&self, state: MotionState) -> Acceleration {
        return self.net_force(state) / self.mass;
    }

    pub fn terminal_velocity(&self) -> Velocity {
        return self.gravity_force() / self.damping;
    }
}

impl OdeSystem for FallingBodySystem {
    type State = MotionState;

    fn derivative(&self, _time: Time, state: Self::State) -> MotionDerivative {
        return MotionDerivative {
            dposition_dt: state.velocity,
            dvelocity_dt: self.acceleration(state),
        };
    }
}

// ============================================================
// Solution analysis for falling-body trajectories
// ============================================================

impl Solution<MotionState> {
    pub fn maximum_height(&self) -> Option<Length> {
        let first_sample = self.samples.first()?;
        let mut maximum = first_sample.state.position;

        for sample in &self.samples {
            if sample.state.position.0 > maximum.0 {
                maximum = sample.state.position;
            }
        }

        return Some(maximum);
    }

    pub fn minimum_height(&self) -> Option<Length> {
        let first_sample = self.samples.first()?;
        let mut minimum = first_sample.state.position;

        for sample in &self.samples {
            if sample.state.position.0 < minimum.0 {
                minimum = sample.state.position;
            }
        }

        return Some(minimum);
    }

    pub fn maximum_speed(&self) -> Option<Velocity> {
        let first_sample = self.samples.first()?;
        let mut maximum = Velocity(first_sample.state.velocity.0.abs());

        for sample in &self.samples {
            let speed = Velocity(sample.state.velocity.0.abs());

            if speed.0 > maximum.0 {
                maximum = speed;
            }
        }

        return Some(maximum);
    }

    pub fn time_of_impact(&self) -> Option<Time> {
        let (left, right) = self.first_ground_crossing_segment()?;

        let left_height = left.state.position.0;
        let right_height = right.state.position.0;

        let interpolation = left_height / (left_height - right_height);
        let impact_time_value = left.time.0 + interpolation * (right.time.0 - left.time.0);

        return Some(Time(impact_time_value));
    }

    pub fn impact_velocity(&self) -> Option<Velocity> {
        let (left, right) = self.first_ground_crossing_segment()?;

        let left_height = left.state.position.0;
        let right_height = right.state.position.0;

        let interpolation = left_height / (left_height - right_height);

        let impact_velocity_value =
            left.state.velocity.0 + interpolation * (right.state.velocity.0 - left.state.velocity.0);

        return Some(Velocity(impact_velocity_value));
    }

    pub fn impact_state(&self) -> Option<MotionState> {
        let impact_time = self.time_of_impact()?;
        let impact_velocity = self.impact_velocity()?;

        return Some(MotionState {
            position: meters(0.0),
            velocity: impact_velocity,
        })
        .map(|state| {
            let _ = impact_time;
            return state;
        });
    }

    fn first_ground_crossing_segment(&self) -> Option<(Sample<MotionState>, Sample<MotionState>)> {
        if self.samples.len() < 2 {
            return None;
        }

        for window in self.samples.windows(2) {
            let left = window[0];
            let right = window[1];

            let left_height = left.state.position.0;
            let right_height = right.state.position.0;

            if left_height > 0.0 && right_height <= 0.0 {
                return Some((left, right));
            }

            if left_height == 0.0 {
                return Some((left, left));
            }
        }

        return None;
    }
}

// ============================================================
// Demo
// ============================================================

fn main() {
    let system = FallingBodySystem {
        mass: kilograms(80.0),
        damping: damping(12.5),
        gravity: meters_per_second_squared(-9.81),
    };

    let initial_state = MotionState {
        position: meters(100.0),
        velocity: meters_per_second(0.0),
    };

    let initial_time = seconds(0.0);
    let time_step = seconds(0.05);
    let steps = 200;

    let solution = integrate(&system, initial_time, initial_state, time_step, steps);

    println!("initial height = {}", system.height(initial_state));
    println!("initial speed = {}", system.speed(initial_state));
    println!("terminal velocity = {}", system.terminal_velocity());
    println!();

    for sample in solution.samples().iter().step_by(20) {
        let state = sample.state;

        println!(
            "t = {:>5.2} s | height = {:>9.4} m | velocity = {:>9.4} m/s | speed = {:>9.4} m/s | acceleration = {:>9.4} m/s^2",
            sample.time.0,
            state.position.0,
            state.velocity.0,
            system.speed(state).0,
            system.acceleration(state).0,
        );
    }

    println!();

    if let Some(maximum_height) = solution.maximum_height() {
        println!("maximum height = {}", maximum_height);
    }

    if let Some(maximum_speed) = solution.maximum_speed() {
        println!("maximum speed = {}", maximum_speed);
    }

    if let Some(time_of_impact) = solution.time_of_impact() {
        println!("time of impact = {}", time_of_impact);
    } else {
        println!("time of impact = not reached during simulated time span");
    }

    if let Some(impact_velocity) = solution.impact_velocity() {
        println!("impact velocity = {}", impact_velocity);
    } else {
        println!("impact velocity = not reached during simulated time span");
    }

    // Uncomment to see a type error:
    // let invalid = meters(1.0) + seconds(1.0);
}