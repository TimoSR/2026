use std::alloc::{GlobalAlloc, Layout, System};
use std::error::Error;
use std::fmt;
use std::hint::black_box;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

// ============================================================
// Allocation tracking
// ============================================================

struct CountingAllocator;

static TOTAL_ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);
static TOTAL_DEALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = System.alloc(layout);

        if !pointer.is_null() {
            TOTAL_ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }

        return pointer;
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = System.alloc_zeroed(layout);

        if !pointer.is_null() {
            TOTAL_ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }

        return pointer;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        TOTAL_DEALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        let pointer = System.realloc(ptr, old_layout, new_size);

        if !pointer.is_null() {
            if new_size >= old_layout.size() {
                TOTAL_ALLOCATED_BYTES.fetch_add((new_size - old_layout.size()) as u64, Ordering::Relaxed);
            } else {
                TOTAL_DEALLOCATED_BYTES.fetch_add((old_layout.size() - new_size) as u64, Ordering::Relaxed);
            }
        }

        return pointer;
    }
}

#[inline]
fn total_allocated_bytes() -> u64 {
    return TOTAL_ALLOCATED_BYTES.load(Ordering::Relaxed);
}

#[inline]
fn total_deallocated_bytes() -> u64 {
    return TOTAL_DEALLOCATED_BYTES.load(Ordering::Relaxed);
}

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
// Motion state
// ============================================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MotionState {
    pub position: Length,
    pub velocity: Velocity,
}

impl MotionState {
    #[inline]
    pub fn new(position: Length, velocity: Velocity) -> Self {
        return Self { position, velocity };
    }

    #[inline]
    pub fn at_rest(position: Length) -> Self {
        return Self {
            position,
            velocity: meters_per_second(0.0),
        };
    }

    #[inline]
    pub fn height(&self) -> Length {
        return self.position;
    }

    #[inline]
    pub fn signed_velocity(&self) -> Velocity {
        return self.velocity;
    }

    #[inline]
    pub fn speed(&self) -> Velocity {
        return Velocity(self.velocity.0.abs());
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MotionDerivative {
    pub dposition_dt: Velocity,
    pub dvelocity_dt: Acceleration,
}

impl MotionDerivative {
    #[inline]
    pub fn new(dposition_dt: Velocity, dvelocity_dt: Acceleration) -> Self {
        return Self {
            dposition_dt,
            dvelocity_dt,
        };
    }
}

impl StateDerivative for MotionDerivative {
    #[inline]
    fn plus(self, rhs: Self) -> Self {
        return Self {
            dposition_dt: self.dposition_dt + rhs.dposition_dt,
            dvelocity_dt: self.dvelocity_dt + rhs.dvelocity_dt,
        };
    }

    #[inline]
    fn scale_by(self, scalar: f64) -> Self {
        return Self {
            dposition_dt: self.dposition_dt * scalar,
            dvelocity_dt: self.dvelocity_dt * scalar,
        };
    }
}

impl OdeState for MotionState {
    type Derivative = MotionDerivative;

    #[inline]
    fn apply_derivative(self, derivative: Self::Derivative, time_step: Time) -> Self {
        return Self {
            position: self.position + derivative.dposition_dt * time_step,
            velocity: self.velocity + derivative.dvelocity_dt * time_step,
        };
    }
}

// ============================================================
// Generic integration
// ============================================================

#[derive(Copy, Clone, Debug)]
pub struct Sample<State> {
    pub time: Time,
    pub state: State,
}

impl<State> Sample<State>
where
    State: Copy,
{
    #[inline]
    pub fn new(time: Time, state: State) -> Self {
        return Self { time, state };
    }
}

#[derive(Clone, Debug)]
pub struct Solution<State> {
    samples: Vec<Sample<State>>,
}

impl<State> Solution<State> {
    #[inline]
    pub fn new(samples: Vec<Sample<State>>) -> Self {
        return Self { samples };
    }

    #[inline]
    pub fn samples(&self) -> &[Sample<State>] {
        return &self.samples;
    }

    #[inline]
    pub fn first(&self) -> Option<&Sample<State>> {
        return self.samples.first();
    }

    #[inline]
    pub fn last(&self) -> Option<&Sample<State>> {
        return self.samples.last();
    }

    #[inline]
    pub fn len(&self) -> usize {
        return self.samples.len();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        return self.samples.is_empty();
    }
}

#[inline]
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

    samples.push(Sample::new(time, state));

    for _ in 0..steps {
        state = rk4_step(system, time, state, time_step);
        time = time + time_step;
        samples.push(Sample::new(time, state));
    }

    return Solution::new(samples);
}

// ============================================================
// Timeline
// ============================================================

#[derive(Copy, Clone, Debug)]
pub struct SimulationTimeline {
    start_time: Time,
    time_step: Time,
    steps: usize,
}

#[derive(Debug)]
pub enum SimulationTimelineError {
    NonPositiveTimeStep(Time),
    ZeroSteps,
    NegativeDuration(Time),
}

impl fmt::Display for SimulationTimelineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveTimeStep(time_step) => {
                return write!(
                    formatter,
                    "simulation time step must be greater than zero, got {}",
                    time_step
                );
            }
            Self::ZeroSteps => {
                return write!(formatter, "simulation steps must be greater than zero");
            }
            Self::NegativeDuration(duration) => {
                return write!(
                    formatter,
                    "simulation duration must be zero or greater, got {}",
                    duration
                );
            }
        }
    }
}

impl Error for SimulationTimelineError {}

impl SimulationTimeline {
    pub fn from_step_count(
        start_time: Time,
        time_step: Time,
        steps: usize,
    ) -> Result<Self, SimulationTimelineError> {
        if time_step.0 <= 0.0 {
            return Err(SimulationTimelineError::NonPositiveTimeStep(time_step));
        }

        if steps == 0 {
            return Err(SimulationTimelineError::ZeroSteps);
        }

        return Ok(Self {
            start_time,
            time_step,
            steps,
        });
    }

    pub fn from_duration(
        start_time: Time,
        time_step: Time,
        duration: Time,
    ) -> Result<Self, SimulationTimelineError> {
        if time_step.0 <= 0.0 {
            return Err(SimulationTimelineError::NonPositiveTimeStep(time_step));
        }

        if duration.0 < 0.0 {
            return Err(SimulationTimelineError::NegativeDuration(duration));
        }

        let steps = ((duration.0 / time_step.0).ceil() as usize).max(1);

        return Ok(Self {
            start_time,
            time_step,
            steps,
        });
    }

    #[inline]
    pub fn start_time(&self) -> Time {
        return self.start_time;
    }

    #[inline]
    pub fn time_step(&self) -> Time {
        return self.time_step;
    }

    #[inline]
    pub fn steps(&self) -> usize {
        return self.steps;
    }

    #[inline]
    pub fn duration(&self) -> Time {
        return self.time_step * self.steps as f64;
    }
}

// ============================================================
// Falling body system
// ============================================================

#[derive(Copy, Clone, Debug)]
pub struct FallingBodySystem {
    pub mass: Mass,
    pub damping: Damping,
    pub gravity: Acceleration,
}

#[derive(Debug)]
pub enum FallingBodySystemError {
    NonPositiveMass(Mass),
    NegativeDamping(Damping),
    NonFiniteGravity(Acceleration),
}

impl fmt::Display for FallingBodySystemError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveMass(mass) => {
                return write!(formatter, "mass must be greater than zero, got {}", mass);
            }
            Self::NegativeDamping(damping_value) => {
                return write!(formatter, "damping must be zero or greater, got {}", damping_value);
            }
            Self::NonFiniteGravity(gravity) => {
                return write!(formatter, "gravity must be finite, got {}", gravity);
            }
        }
    }
}

impl Error for FallingBodySystemError {}

impl FallingBodySystem {
    pub fn new(
        mass: Mass,
        damping: Damping,
        gravity: Acceleration,
    ) -> Result<Self, FallingBodySystemError> {
        if mass.0 <= 0.0 {
            return Err(FallingBodySystemError::NonPositiveMass(mass));
        }

        if damping.0 < 0.0 {
            return Err(FallingBodySystemError::NegativeDamping(damping));
        }

        if !gravity.0.is_finite() {
            return Err(FallingBodySystemError::NonFiniteGravity(gravity));
        }

        return Ok(Self {
            mass,
            damping,
            gravity,
        });
    }

    #[inline]
    pub fn gravity_force(&self) -> Force {
        return self.mass * self.gravity;
    }

    #[inline]
    pub fn drag_force(&self, state: MotionState) -> Force {
        return self.damping * state.velocity;
    }

    #[inline]
    pub fn drag_force_magnitude(&self, state: MotionState) -> Force {
        return Force(self.drag_force(state).0.abs());
    }

    #[inline]
    pub fn net_force(&self, state: MotionState) -> Force {
        return self.gravity_force() - self.drag_force(state);
    }

    #[inline]
    pub fn acceleration(&self, state: MotionState) -> Acceleration {
        return self.net_force(state) / self.mass;
    }

    pub fn terminal_velocity(&self) -> Option<Velocity> {
        if self.damping.0 == 0.0 {
            return None;
        }

        return Some(self.gravity_force() / self.damping);
    }

    pub fn simulate_from_state(
        &self,
        initial_state: MotionState,
        timeline: SimulationTimeline,
    ) -> FallingBodySimulation {
        let solution = integrate(
            self,
            timeline.start_time(),
            initial_state,
            timeline.time_step(),
            timeline.steps(),
        );

        return FallingBodySimulation::new(*self, timeline, initial_state, solution);
    }

    pub fn simulate_drop_from_rest(
        &self,
        initial_height: Length,
        timeline: SimulationTimeline,
    ) -> FallingBodySimulation {
        return self.simulate_from_state(MotionState::at_rest(initial_height), timeline);
    }

    pub fn simulate_drop_from_height(
        &self,
        initial_height: Length,
        initial_velocity: Velocity,
        timeline: SimulationTimeline,
    ) -> FallingBodySimulation {
        return self.simulate_from_state(
            MotionState::new(initial_height, initial_velocity),
            timeline,
        );
    }
}

impl OdeSystem for FallingBodySystem {
    type State = MotionState;

    #[inline]
    fn derivative(&self, _time: Time, state: Self::State) -> MotionDerivative {
        return MotionDerivative::new(state.velocity, self.acceleration(state));
    }
}

// ============================================================
// Falling body simulation
// ============================================================

#[derive(Clone, Debug)]
pub struct FallingBodySimulation {
    system: FallingBodySystem,
    timeline: SimulationTimeline,
    initial_state: MotionState,
    solution: Solution<MotionState>,
}

impl FallingBodySimulation {
    pub fn new(
        system: FallingBodySystem,
        timeline: SimulationTimeline,
        initial_state: MotionState,
        solution: Solution<MotionState>,
    ) -> Self {
        return Self {
            system,
            timeline,
            initial_state,
            solution,
        };
    }

    #[inline]
    pub fn system(&self) -> &FallingBodySystem {
        return &self.system;
    }

    #[inline]
    pub fn timeline(&self) -> SimulationTimeline {
        return self.timeline;
    }

    #[inline]
    pub fn initial_state(&self) -> MotionState {
        return self.initial_state;
    }

    #[inline]
    pub fn initial_height(&self) -> Length {
        return self.initial_state.height();
    }

    #[inline]
    pub fn initial_velocity(&self) -> Velocity {
        return self.initial_state.signed_velocity();
    }

    #[inline]
    pub fn initial_speed(&self) -> Velocity {
        return self.initial_state.speed();
    }

    pub fn terminal_velocity(&self) -> Option<Velocity> {
        return self.system.terminal_velocity();
    }

    #[inline]
    pub fn samples(&self) -> &[Sample<MotionState>] {
        return self.solution.samples();
    }

    #[inline]
    pub fn first_sample(&self) -> Option<&Sample<MotionState>> {
        return self.solution.first();
    }

    #[inline]
    pub fn last_sample(&self) -> Option<&Sample<MotionState>> {
        return self.solution.last();
    }

    #[inline]
    pub fn speed_at(&self, state: MotionState) -> Velocity {
        return state.speed();
    }

    #[inline]
    pub fn acceleration_at(&self, state: MotionState) -> Acceleration {
        return self.system.acceleration(state);
    }

    #[inline]
    pub fn drag_force_at(&self, state: MotionState) -> Force {
        return self.system.drag_force(state);
    }

    #[inline]
    pub fn net_force_at(&self, state: MotionState) -> Force {
        return self.system.net_force(state);
    }

    pub fn maximum_height(&self) -> Option<Length> {
        let first_sample = self.samples().first()?;
        let mut maximum = first_sample.state.position;

        for sample in self.samples() {
            if sample.state.position.0 > maximum.0 {
                maximum = sample.state.position;
            }
        }

        return Some(maximum);
    }

    pub fn minimum_height(&self) -> Option<Length> {
        let first_sample = self.samples().first()?;
        let mut minimum = first_sample.state.position;

        for sample in self.samples() {
            if sample.state.position.0 < minimum.0 {
                minimum = sample.state.position;
            }
        }

        return Some(minimum);
    }

    pub fn maximum_speed(&self) -> Option<Velocity> {
        let first_sample = self.samples().first()?;
        let mut maximum = first_sample.state.speed();

        for sample in self.samples() {
            let speed = sample.state.speed();

            if speed.0 > maximum.0 {
                maximum = speed;
            }
        }

        return Some(maximum);
    }

    pub fn time_of_impact(&self) -> Option<Time> {
        let (left, right) = self.first_ground_crossing_segment()?;

        if left.time.0 == right.time.0 {
            return Some(left.time);
        }

        let left_height = left.state.position.0;
        let right_height = right.state.position.0;
        let interpolation = left_height / (left_height - right_height);
        let impact_time_value = left.time.0 + interpolation * (right.time.0 - left.time.0);

        return Some(Time(impact_time_value));
    }

    pub fn impact_velocity(&self) -> Option<Velocity> {
        let (left, right) = self.first_ground_crossing_segment()?;

        if left.time.0 == right.time.0 {
            return Some(left.state.velocity);
        }

        let left_height = left.state.position.0;
        let right_height = right.state.position.0;
        let interpolation = left_height / (left_height - right_height);

        let impact_velocity_value =
            left.state.velocity.0 + interpolation * (right.state.velocity.0 - left.state.velocity.0);

        return Some(Velocity(impact_velocity_value));
    }

    pub fn impact_speed(&self) -> Option<Velocity> {
        let impact_velocity = self.impact_velocity()?;
        return Some(Velocity(impact_velocity.0.abs()));
    }

    pub fn impact_state(&self) -> Option<MotionState> {
        let impact_velocity = self.impact_velocity()?;

        return Some(MotionState {
            position: meters(0.0),
            velocity: impact_velocity,
        });
    }

    pub fn impact_sample(&self) -> Option<Sample<MotionState>> {
        let impact_time = self.time_of_impact()?;
        let impact_state = self.impact_state()?;

        return Some(Sample::new(impact_time, impact_state));
    }

    pub fn every_nth_sample(&self, step: usize) -> impl Iterator<Item = &Sample<MotionState>> {
        let clamped_step = step.max(1);
        return self.samples().iter().step_by(clamped_step);
    }

    pub fn describe_sample(&self, sample: &Sample<MotionState>) -> String {
        let state = sample.state;

        return format!(
            "t = {:>5.2} s | height = {:>9.4} m | velocity = {:>9.4} m/s | speed = {:>9.4} m/s | acceleration = {:>9.4} m/s^2",
            sample.time.0,
            state.position.0,
            state.velocity.0,
            self.speed_at(state).0,
            self.acceleration_at(state).0,
        );
    }

    pub fn sample_report_lines_every(&self, step: usize) -> Vec<String> {
        let mut lines = Vec::new();

        for sample in self.every_nth_sample(step) {
            lines.push(self.describe_sample(sample));
        }

        return lines;
    }

    pub fn summary(&self) -> String {
        match self.terminal_velocity() {
            Some(terminal_velocity) => {
                return format!(
                    "initial height = {}\ninitial speed = {}\nterminal velocity = {}",
                    self.initial_height(),
                    self.initial_speed(),
                    terminal_velocity,
                );
            }
            None => {
                return format!(
                    "initial height = {}\ninitial speed = {}\nterminal velocity = undefined (zero damping)",
                    self.initial_height(),
                    self.initial_speed(),
                );
            }
        }
    }

    fn first_ground_crossing_segment(&self) -> Option<(Sample<MotionState>, Sample<MotionState>)> {
        if self.samples().len() < 2 {
            return None;
        }

        for window in self.samples().windows(2) {
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
// Benchmark harness
// ============================================================

#[derive(Clone, Debug)]
pub struct BenchmarkResult {
    pub name: &'static str,
    pub iterations: usize,
    pub total_elapsed: Duration,
    pub average_nanoseconds: f64,
    pub operations_per_second: f64,
    pub allocated_bytes: u64,
    pub net_allocated_bytes: i128,
}

impl BenchmarkResult {
    #[inline]
    pub fn average_microseconds(&self) -> f64 {
        return self.average_nanoseconds / 1_000.0;
    }

    #[inline]
    pub fn average_milliseconds(&self) -> f64 {
        return self.average_nanoseconds / 1_000_000.0;
    }
}

pub fn run_benchmark<F>(name: &'static str, iterations: usize, mut action: F) -> BenchmarkResult
where
    F: FnMut(),
{
    assert!(iterations > 0, "iterations must be greater than zero");

    let warmup_iterations = iterations.min(16);

    for _ in 0..warmup_iterations {
        action();
    }

    let before_allocated = total_allocated_bytes();
    let before_deallocated = total_deallocated_bytes();

    let started_at = Instant::now();

    for _ in 0..iterations {
        action();
    }

    let total_elapsed = started_at.elapsed();

    let after_allocated = total_allocated_bytes();
    let after_deallocated = total_deallocated_bytes();

    let allocated_bytes = after_allocated.saturating_sub(before_allocated);
    let net_before = before_allocated as i128 - before_deallocated as i128;
    let net_after = after_allocated as i128 - after_deallocated as i128;
    let net_allocated_bytes = net_after - net_before;

    let average_nanoseconds = total_elapsed.as_secs_f64() * 1_000_000_000.0 / iterations as f64;
    let operations_per_second = iterations as f64 / total_elapsed.as_secs_f64();

    return BenchmarkResult {
        name,
        iterations,
        total_elapsed,
        average_nanoseconds,
        operations_per_second,
        allocated_bytes,
        net_allocated_bytes,
    };
}

#[inline]
fn format_grouped_u64(value: u64) -> String {
    let digits = value.to_string();
    let mut result = String::with_capacity(digits.len() + digits.len() / 3);
    let digit_count = digits.len();

    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digit_count - index) % 3 == 0 {
            result.push('.');
        }

        result.push(character);
    }

    return result;
}

#[inline]
fn format_grouped_usize(value: usize) -> String {
    return format_grouped_u64(value as u64);
}

#[inline]
fn format_grouped_i128(value: i128) -> String {
    if value < 0 {
        return format!("-{}", format_grouped_u64(value.unsigned_abs() as u64));
    }

    return format_grouped_u64(value as u64);
}

#[inline]
fn bytes_to_mb(bytes: u64) -> f64 {
    return bytes as f64 / 1_000_000.0;
}

#[inline]
fn bytes_to_mb_signed(bytes: i128) -> f64 {
    return bytes as f64 / 1_000_000.0;
}

pub fn print_benchmark_result(result: &BenchmarkResult) {
    println!("name                : {}", result.name);
    println!("iterations          : {}", format_grouped_usize(result.iterations));
    println!(
        "total elapsed       : {:.3} ms",
        result.total_elapsed.as_secs_f64() * 1_000.0
    );
    println!("avg / iteration     : {:.3} ns", result.average_nanoseconds);
    println!("avg / iteration     : {:.3} µs", result.average_microseconds());
    println!("avg / iteration     : {:.6} ms", result.average_milliseconds());
    println!(
        "ops / second        : {}",
        format_grouped_u64(result.operations_per_second.round() as u64)
    );
    println!(
        "allocated total     : {:.3} MB ({} B)",
        bytes_to_mb(result.allocated_bytes),
        format_grouped_u64(result.allocated_bytes)
    );
    println!(
        "allocated / iter    : {:.6} MB ({:.3} B)",
        bytes_to_mb(result.allocated_bytes) / result.iterations as f64,
        result.allocated_bytes as f64 / result.iterations as f64
    );
    println!(
        "net allocated delta : {:.3} MB ({} B)",
        bytes_to_mb_signed(result.net_allocated_bytes),
        format_grouped_i128(result.net_allocated_bytes)
    );
    println!();
}

fn run_demo() -> Result<(), Box<dyn Error>> {
    let system = FallingBodySystem::new(
        kilograms(80.0),
        damping(12.5),
        meters_per_second_squared(-9.81),
    )?;

    let timeline = SimulationTimeline::from_step_count(
        seconds(0.0),
        seconds(0.05),
        200,
    )?;

    let simulation = system.simulate_drop_from_height(
        meters(100.0),
        meters_per_second(0.0),
        timeline,
    );

    println!("{}", simulation.summary());
    println!();

    for line in simulation.sample_report_lines_every(20) {
        println!("{}", line);
    }

    println!();

    if let Some(maximum_height) = simulation.maximum_height() {
        println!("maximum height = {}", maximum_height);
    }

    if let Some(maximum_speed) = simulation.maximum_speed() {
        println!("maximum speed = {}", maximum_speed);
    }

    if let Some(time_of_impact) = simulation.time_of_impact() {
        println!("time of impact = {}", time_of_impact);
    } else {
        println!("time of impact = not reached during simulated time span");
    }

    if let Some(impact_velocity) = simulation.impact_velocity() {
        println!("impact velocity = {}", impact_velocity);
    } else {
        println!("impact velocity = not reached during simulated time span");
    }

    if let Some(impact_speed) = simulation.impact_speed() {
        println!("impact speed = {}", impact_speed);
    } else {
        println!("impact speed = not reached during simulated time span");
    }

    return Ok(());
}

fn run_benchmarks() -> Result<(), Box<dyn Error>> {
    println!("Run with: cargo run --release -- --benchmark");
    println!();

    let system = FallingBodySystem::new(
        kilograms(80.0),
        damping(12.5),
        meters_per_second_squared(-9.81),
    )?;

    let short_timeline = SimulationTimeline::from_step_count(
        seconds(0.0),
        seconds(0.05),
        200,
    )?;

    let long_timeline = SimulationTimeline::from_step_count(
        seconds(0.0),
        seconds(0.05),
        20_000,
    )?;

    let initial_state = MotionState::new(
        meters(100.0),
        meters_per_second(0.0),
    );

    let prebuilt_simulation = system.simulate_drop_from_height(
        meters(100.0),
        meters_per_second(0.0),
        short_timeline,
    );

    let result = run_benchmark("RK4 single step", 1_000_000, || {
        let state = rk4_step(&system, seconds(0.0), initial_state, seconds(0.05));
        black_box(state);
    });
    print_benchmark_result(&result);

    let result = run_benchmark("Integrate 200 steps", 100_000, || {
        let solution = integrate(
            &system,
            short_timeline.start_time(),
            initial_state,
            short_timeline.time_step(),
            short_timeline.steps(),
        );
        black_box(solution);
    });
    print_benchmark_result(&result);

    let result = run_benchmark("Integrate 20,000 steps", 1_000, || {
        let solution = integrate(
            &system,
            long_timeline.start_time(),
            initial_state,
            long_timeline.time_step(),
            long_timeline.steps(),
        );
        black_box(solution);
    });
    print_benchmark_result(&result);

    let result = run_benchmark("SimulateDropFromHeight", 100_000, || {
        let simulation = system.simulate_drop_from_height(
            meters(100.0),
            meters_per_second(0.0),
            short_timeline,
        );
        black_box(simulation);
    });
    print_benchmark_result(&result);

    let result = run_benchmark("MaximumSpeed query", 1_000_000, || {
        let maximum_speed = prebuilt_simulation.maximum_speed();
        black_box(maximum_speed);
    });
    print_benchmark_result(&result);

    let result = run_benchmark("TimeOfImpact query", 1_000_000, || {
        let time_of_impact = prebuilt_simulation.time_of_impact();
        black_box(time_of_impact);
    });
    print_benchmark_result(&result);

    let result = run_benchmark("SampleReportLinesEvery(20)", 100_000, || {
        let lines = prebuilt_simulation.sample_report_lines_every(20);
        black_box(lines);
    });
    print_benchmark_result(&result);

    let result = run_benchmark("Summary", 1_000_000, || {
        let summary = prebuilt_simulation.summary();
        black_box(summary);
    });
    print_benchmark_result(&result);

    return Ok(());
}

// ============================================================
// Entry point
// ============================================================

fn has_benchmark_flag() -> bool {
    for argument in std::env::args().skip(1) {
        if argument == "--benchmark" || argument == "-b" {
            return true;
        }
    }

    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    if has_benchmark_flag() {
        return run_benchmarks();
    }

    return run_demo();
}