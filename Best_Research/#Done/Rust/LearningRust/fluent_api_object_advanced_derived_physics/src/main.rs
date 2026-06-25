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

pub fn meters(value: f64) -> Length {
    return Length(value);
}

pub fn seconds(value: f64) -> Time {
    return Time(value);
}

pub fn kilograms(value: f64) -> Mass {
    return Mass(value);
}

pub fn meters_per_second(value: f64) -> Velocity {
    return Velocity(value);
}

pub fn meters_per_second_squared(value: f64) -> Acceleration {
    return Acceleration(value);
}

pub fn newtons(value: f64) -> Force {
    return Force(value);
}

pub fn damping(value: f64) -> Damping {
    return Damping(value);
}

// ============================================================
// Pretty printing
// ============================================================

impl fmt::Display for Length {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "{} m", self.0);
    }
}

impl fmt::Display for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "{} s", self.0);
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "{} kg", self.0);
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "{} m/s", self.0);
    }
}

impl fmt::Display for Acceleration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "{} m/s^2", self.0);
    }
}

impl fmt::Display for Force {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "{} N", self.0);
    }
}

impl fmt::Display for Damping {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "{} kg/s", self.0);
    }
}

// ============================================================
// Same-type arithmetic
// ============================================================

impl Add<Length> for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        return Length(self.0 + rhs.0);
    }
}

impl Sub<Length> for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        return Length(self.0 - rhs.0);
    }
}

impl Add<Time> for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Self::Output {
        return Time(self.0 + rhs.0);
    }
}

impl Sub<Time> for Time {
    type Output = Time;

    fn sub(self, rhs: Time) -> Self::Output {
        return Time(self.0 - rhs.0);
    }
}

impl Add<Mass> for Mass {
    type Output = Mass;

    fn add(self, rhs: Mass) -> Self::Output {
        return Mass(self.0 + rhs.0);
    }
}

impl Sub<Mass> for Mass {
    type Output = Mass;

    fn sub(self, rhs: Mass) -> Self::Output {
        return Mass(self.0 - rhs.0);
    }
}

impl Add<Velocity> for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Velocity) -> Self::Output {
        return Velocity(self.0 + rhs.0);
    }
}

impl Sub<Velocity> for Velocity {
    type Output = Velocity;

    fn sub(self, rhs: Velocity) -> Self::Output {
        return Velocity(self.0 - rhs.0);
    }
}

impl Add<Acceleration> for Acceleration {
    type Output = Acceleration;

    fn add(self, rhs: Acceleration) -> Self::Output {
        return Acceleration(self.0 + rhs.0);
    }
}

impl Sub<Acceleration> for Acceleration {
    type Output = Acceleration;

    fn sub(self, rhs: Acceleration) -> Self::Output {
        return Acceleration(self.0 - rhs.0);
    }
}

impl Add<Force> for Force {
    type Output = Force;

    fn add(self, rhs: Force) -> Self::Output {
        return Force(self.0 + rhs.0);
    }
}

impl Sub<Force> for Force {
    type Output = Force;

    fn sub(self, rhs: Force) -> Self::Output {
        return Force(self.0 - rhs.0);
    }
}

impl Add<Damping> for Damping {
    type Output = Damping;

    fn add(self, rhs: Damping) -> Self::Output {
        return Damping(self.0 + rhs.0);
    }
}

impl Sub<Damping> for Damping {
    type Output = Damping;

    fn sub(self, rhs: Damping) -> Self::Output {
        return Damping(self.0 - rhs.0);
    }
}

// ============================================================
// Scalar arithmetic
// ============================================================

impl Mul<f64> for Length {
    type Output = Length;

    fn mul(self, rhs: f64) -> Self::Output {
        return Length(self.0 * rhs);
    }
}

impl Div<f64> for Length {
    type Output = Length;

    fn div(self, rhs: f64) -> Self::Output {
        return Length(self.0 / rhs);
    }
}

impl Mul<Length> for f64 {
    type Output = Length;

    fn mul(self, rhs: Length) -> Self::Output {
        return Length(self * rhs.0);
    }
}

impl Mul<f64> for Time {
    type Output = Time;

    fn mul(self, rhs: f64) -> Self::Output {
        return Time(self.0 * rhs);
    }
}

impl Div<f64> for Time {
    type Output = Time;

    fn div(self, rhs: f64) -> Self::Output {
        return Time(self.0 / rhs);
    }
}

impl Mul<Time> for f64 {
    type Output = Time;

    fn mul(self, rhs: Time) -> Self::Output {
        return Time(self * rhs.0);
    }
}

impl Mul<f64> for Mass {
    type Output = Mass;

    fn mul(self, rhs: f64) -> Self::Output {
        return Mass(self.0 * rhs);
    }
}

impl Div<f64> for Mass {
    type Output = Mass;

    fn div(self, rhs: f64) -> Self::Output {
        return Mass(self.0 / rhs);
    }
}

impl Mul<Mass> for f64 {
    type Output = Mass;

    fn mul(self, rhs: Mass) -> Self::Output {
        return Mass(self * rhs.0);
    }
}

impl Mul<f64> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f64) -> Self::Output {
        return Velocity(self.0 * rhs);
    }
}

impl Div<f64> for Velocity {
    type Output = Velocity;

    fn div(self, rhs: f64) -> Self::Output {
        return Velocity(self.0 / rhs);
    }
}

impl Mul<Velocity> for f64 {
    type Output = Velocity;

    fn mul(self, rhs: Velocity) -> Self::Output {
        return Velocity(self * rhs.0);
    }
}

impl Mul<f64> for Acceleration {
    type Output = Acceleration;

    fn mul(self, rhs: f64) -> Self::Output {
        return Acceleration(self.0 * rhs);
    }
}

impl Div<f64> for Acceleration {
    type Output = Acceleration;

    fn div(self, rhs: f64) -> Self::Output {
        return Acceleration(self.0 / rhs);
    }
}

impl Mul<Acceleration> for f64 {
    type Output = Acceleration;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        return Acceleration(self * rhs.0);
    }
}

impl Mul<f64> for Force {
    type Output = Force;

    fn mul(self, rhs: f64) -> Self::Output {
        return Force(self.0 * rhs);
    }
}

impl Div<f64> for Force {
    type Output = Force;

    fn div(self, rhs: f64) -> Self::Output {
        return Force(self.0 / rhs);
    }
}

impl Mul<Force> for f64 {
    type Output = Force;

    fn mul(self, rhs: Force) -> Self::Output {
        return Force(self * rhs.0);
    }
}

impl Mul<f64> for Damping {
    type Output = Damping;

    fn mul(self, rhs: f64) -> Self::Output {
        return Damping(self.0 * rhs);
    }
}

impl Div<f64> for Damping {
    type Output = Damping;

    fn div(self, rhs: f64) -> Self::Output {
        return Damping(self.0 / rhs);
    }
}

impl Mul<Damping> for f64 {
    type Output = Damping;

    fn mul(self, rhs: Damping) -> Self::Output {
        return Damping(self * rhs.0);
    }
}

// ============================================================
// Cross-unit arithmetic
// ============================================================

// velocity = length / time
impl Div<Time> for Length {
    type Output = Velocity;

    fn div(self, rhs: Time) -> Self::Output {
        return Velocity(self.0 / rhs.0);
    }
}

// acceleration = velocity / time
impl Div<Time> for Velocity {
    type Output = Acceleration;

    fn div(self, rhs: Time) -> Self::Output {
        return Acceleration(self.0 / rhs.0);
    }
}

// length = velocity * time
impl Mul<Time> for Velocity {
    type Output = Length;

    fn mul(self, rhs: Time) -> Self::Output {
        return Length(self.0 * rhs.0);
    }
}

// length = time * velocity
impl Mul<Velocity> for Time {
    type Output = Length;

    fn mul(self, rhs: Velocity) -> Self::Output {
        return Length(self.0 * rhs.0);
    }
}

// velocity = acceleration * time
impl Mul<Time> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Time) -> Self::Output {
        return Velocity(self.0 * rhs.0);
    }
}

// velocity = time * acceleration
impl Mul<Acceleration> for Time {
    type Output = Velocity;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        return Velocity(self.0 * rhs.0);
    }
}

// force = mass * acceleration
impl Mul<Acceleration> for Mass {
    type Output = Force;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

impl Mul<Mass> for Acceleration {
    type Output = Force;

    fn mul(self, rhs: Mass) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

// acceleration = force / mass
impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, rhs: Mass) -> Self::Output {
        return Acceleration(self.0 / rhs.0);
    }
}

// force = damping * velocity
impl Mul<Velocity> for Damping {
    type Output = Force;

    fn mul(self, rhs: Velocity) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

impl Mul<Damping> for Velocity {
    type Output = Force;

    fn mul(self, rhs: Damping) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

// velocity = force / damping
impl Div<Damping> for Force {
    type Output = Velocity;

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
// 1D motion state
//
// position' = velocity
// velocity' = acceleration
// ============================================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MotionState {
    pub position: Length,
    pub velocity: Velocity,
}

impl MotionState {
    pub fn new(position: Length, velocity: Velocity) -> Self {
        return Self { position, velocity };
    }

    pub fn at_rest(position: Length) -> Self {
        return Self {
            position,
            velocity: meters_per_second(0.0),
        };
    }

    pub fn height(&self) -> Length {
        return self.position;
    }

    pub fn signed_velocity(&self) -> Velocity {
        return self.velocity;
    }

    pub fn speed(&self) -> Velocity {
        return Velocity(self.velocity.0.abs());
    }

    pub fn is_above_ground(&self) -> bool {
        return self.position.0 > 0.0;
    }

    pub fn is_on_ground(&self) -> bool {
        return self.position.0 == 0.0;
    }

    pub fn is_below_ground(&self) -> bool {
        return self.position.0 < 0.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MotionDerivative {
    pub dposition_dt: Velocity,
    pub dvelocity_dt: Acceleration,
}

impl MotionDerivative {
    pub fn new(dposition_dt: Velocity, dvelocity_dt: Acceleration) -> Self {
        return Self {
            dposition_dt,
            dvelocity_dt,
        };
    }
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
// Generic RK4 integrator
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
    pub fn new(time: Time, state: State) -> Self {
        return Self { time, state };
    }
}

#[derive(Clone, Debug)]
pub struct Solution<State> {
    samples: Vec<Sample<State>>,
}

impl<State> Solution<State> {
    pub fn new(samples: Vec<Sample<State>>) -> Self {
        return Self { samples };
    }

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
// Falling body system with linear drag
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
    pub fn new(mass: Mass, damping: Damping, gravity: Acceleration) -> Self {
        return Self {
            mass,
            damping,
            gravity,
        };
    }

    pub fn scenario(&self) -> FallingBodyScenario {
        return FallingBodyScenario::new(*self);
    }

    pub fn drop_from_height(&self, height: Length) -> FallingBodyScenario {
        return self.scenario().with_initial_height(height);
    }

    pub fn launch_from_height(
        &self,
        height: Length,
        initial_velocity: Velocity,
    ) -> FallingBodyScenario {
        return self
            .scenario()
            .with_initial_height(height)
            .with_initial_velocity(initial_velocity);
    }

    pub fn gravity_force(&self) -> Force {
        return self.mass * self.gravity;
    }

    pub fn drag_force(&self, state: MotionState) -> Force {
        return self.damping * state.velocity;
    }

    pub fn drag_force_magnitude(&self, state: MotionState) -> Force {
        return Force(self.drag_force(state).0.abs());
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
        return MotionDerivative::new(state.velocity, self.acceleration(state));
    }
}

// ============================================================
// Falling body scenario
//
// This owns:
// - the system
// - initial conditions
// - simulation settings
// ============================================================

#[derive(Copy, Clone, Debug)]
pub struct FallingBodyScenario {
    system: FallingBodySystem,
    initial_state: MotionState,
    initial_time: Time,
    time_step: Time,
    steps: usize,
}

impl FallingBodyScenario {
    pub fn new(system: FallingBodySystem) -> Self {
        return Self {
            system,
            initial_state: MotionState::at_rest(meters(0.0)),
            initial_time: seconds(0.0),
            time_step: seconds(0.01),
            steps: 100,
        };
    }

    pub fn with_initial_state(mut self, state: MotionState) -> Self {
        self.initial_state = state;
        return self;
    }

    pub fn with_initial_height(mut self, height: Length) -> Self {
        self.initial_state.position = height;
        return self;
    }

    pub fn with_initial_velocity(mut self, velocity: Velocity) -> Self {
        self.initial_state.velocity = velocity;
        return self;
    }

    pub fn with_start_time(mut self, initial_time: Time) -> Self {
        self.initial_time = initial_time;
        return self;
    }

    pub fn with_time_step(mut self, time_step: Time) -> Self {
        self.time_step = time_step;
        return self;
    }

    pub fn with_steps(mut self, steps: usize) -> Self {
        self.steps = steps;
        return self;
    }

    pub fn system(&self) -> &FallingBodySystem {
        return &self.system;
    }

    pub fn initial_state(&self) -> MotionState {
        return self.initial_state;
    }

    pub fn initial_time(&self) -> Time {
        return self.initial_time;
    }

    pub fn time_step(&self) -> Time {
        return self.time_step;
    }

    pub fn steps(&self) -> usize {
        return self.steps;
    }

    pub fn duration(&self) -> Time {
        return self.time_step * self.steps as f64;
    }

    pub fn simulate(&self) -> FallingBodySimulation {
        let solution = integrate(
            &self.system,
            self.initial_time,
            self.initial_state,
            self.time_step,
            self.steps,
        );

        return FallingBodySimulation::new(*self, solution);
    }
}

// ============================================================
// Falling body simulation
//
// This owns:
// - the scenario
// - the executed trajectory
// - domain-specific analysis helpers
// ============================================================

#[derive(Clone, Debug)]
pub struct FallingBodySimulation {
    scenario: FallingBodyScenario,
    solution: Solution<MotionState>,
}

impl FallingBodySimulation {
    pub fn new(scenario: FallingBodyScenario, solution: Solution<MotionState>) -> Self {
        return Self { scenario, solution };
    }

    pub fn scenario(&self) -> &FallingBodyScenario {
        return &self.scenario;
    }

    pub fn system(&self) -> &FallingBodySystem {
        return self.scenario.system();
    }

    pub fn solution(&self) -> &Solution<MotionState> {
        return &self.solution;
    }

    pub fn samples(&self) -> &[Sample<MotionState>] {
        return self.solution.samples();
    }

    pub fn initial_state(&self) -> MotionState {
        return self.scenario.initial_state();
    }

    pub fn initial_height(&self) -> Length {
        return self.initial_state().height();
    }

    pub fn initial_velocity(&self) -> Velocity {
        return self.initial_state().signed_velocity();
    }

    pub fn initial_speed(&self) -> Velocity {
        return self.initial_state().speed();
    }

    pub fn start_time(&self) -> Time {
        return self.scenario.initial_time();
    }

    pub fn time_step(&self) -> Time {
        return self.scenario.time_step();
    }

    pub fn steps(&self) -> usize {
        return self.scenario.steps();
    }

    pub fn terminal_velocity(&self) -> Velocity {
        return self.system().terminal_velocity();
    }

    pub fn speed_at(&self, state: MotionState) -> Velocity {
        return state.speed();
    }

    pub fn acceleration_at(&self, state: MotionState) -> Acceleration {
        return self.system().acceleration(state);
    }

    pub fn drag_force_at(&self, state: MotionState) -> Force {
        return self.system().drag_force(state);
    }

    pub fn net_force_at(&self, state: MotionState) -> Force {
        return self.system().net_force(state);
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
// Demo
// ============================================================

fn main() {
    let system = FallingBodySystem::new(
        kilograms(80.0),
        damping(12.5),
        meters_per_second_squared(-9.81),
    );

    let simulation = system
        .drop_from_height(meters(100.0))
        .with_initial_velocity(meters_per_second(0.0))
        .with_start_time(seconds(0.0))
        .with_time_step(seconds(0.05))
        .with_steps(200)
        .simulate();

    println!("initial height = {}", simulation.initial_height());
    println!("initial speed = {}", simulation.initial_speed());
    println!("terminal velocity = {}", simulation.terminal_velocity());
    println!();

    for sample in simulation.every_nth_sample(20) {
        let state = sample.state;

        println!(
            "t = {:>5.2} s | height = {:>9.4} m | velocity = {:>9.4} m/s | speed = {:>9.4} m/s | acceleration = {:>9.4} m/s^2",
            sample.time.0,
            state.position.0,
            state.velocity.0,
            simulation.speed_at(state).0,
            simulation.acceleration_at(state).0,
        );
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

    // Uncomment to see a type error:
    // let invalid = meters(1.0) + seconds(1.0);
}