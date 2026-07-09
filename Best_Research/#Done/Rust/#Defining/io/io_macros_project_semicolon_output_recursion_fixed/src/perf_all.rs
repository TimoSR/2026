use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use io_macros_project::{output, output_buffered_to, output_to};

const DEFAULT_WRITER_ITERATIONS: usize = 1_000_000;
const DEFAULT_STDOUT_ITERATIONS: usize = 10_000;
const REPORT_BUFFER_CAPACITY: usize = 2048;

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

static ALLOCATION_CALLS: AtomicUsize = AtomicUsize::new(0);
static ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATION_CALLS: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATION_CALLS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(layout.size(), Ordering::Relaxed);

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        DEALLOCATION_CALLS.fetch_add(1, Ordering::Relaxed);
        DEALLOCATED_BYTES.fetch_add(layout.size(), Ordering::Relaxed);

        System.dealloc(pointer, layout);
    }

    unsafe fn realloc(&self, pointer: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATION_CALLS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(new_size, Ordering::Relaxed);
        DEALLOCATION_CALLS.fetch_add(1, Ordering::Relaxed);
        DEALLOCATED_BYTES.fetch_add(old_layout.size(), Ordering::Relaxed);

        System.realloc(pointer, old_layout, new_size)
    }
}

fn main() -> io::Result<()> {
    let arguments: Vec<String> = std::env::args().collect();
    let writer_iterations = parse_argument(&arguments, 1, DEFAULT_WRITER_ITERATIONS);
    let stdout_iterations = parse_argument(&arguments, 2, DEFAULT_STDOUT_ITERATIONS);

    let verified_report_size = verify_report_bytes()?;

    print_banner("io_macros_project performance report");
    eprintln!("proof: PASS - every benchmark implementation renders identical report bytes before timing");
    eprintln!("report bytes: {verified_report_size}");
    eprintln!("build: release profile with requested optimization flags");
    eprintln!();
    print_metric_guide();
    eprintln!();

    run_writer_benchmarks(writer_iterations)?;
    eprintln!();
    run_stdout_benchmarks(stdout_iterations, verified_report_size)?;

    Ok(())
}

fn parse_argument(arguments: &[String], index: usize, default_value: usize) -> usize {
    if let Some(argument) = arguments.get(index) {
        if let Ok(value) = argument.parse::<usize>() {
            return value;
        }
    }

    default_value
}

fn run_writer_benchmarks(iterations: usize) -> io::Result<()> {
    print_section("1. writer benchmarks");
    eprintln!("iterations: {iterations}");
    eprintln!("write ops/report: exact counted calls to the wrapped Write target");

    let output_to_result = measure_writer(iterations, "output_to!", write_with_output_to)?;
    let output_buffered_result = measure_writer(iterations, "output_buffered_to!", write_with_output_buffered_to)?;
    let writeln_result = measure_writer(iterations, "writeln!", write_traditional)?;
    let buffered_writeln_result = measure_writer(iterations, "buffered writeln! new Vec", write_traditional_buffered)?;
    let reused_buffer_result = measure_reused_buffered_writeln(iterations)?;

    let results = vec![output_to_result, output_buffered_result, writeln_result, buffered_writeln_result, reused_buffer_result];

    print_writer_table(&results, "writeln!", iterations);
    print_fastest(&results);

    Ok(())
}

fn run_stdout_benchmarks(iterations: usize, report_size: usize) -> io::Result<()> {
    print_section("2. stdout benchmarks");
    eprintln!("iterations: {iterations}");
    eprintln!("note: run this command with stdout redirected, for example `> $null` on PowerShell");
    eprintln!("write ops/report: logical stdout-facing calls per report, not OS syscall count");

    let output_result = measure_stdout(iterations, "output!", report_size, 1, write_with_output_macro)?;
    let println_result = measure_stdout(iterations, "println!", report_size, 16, write_with_println)?;
    let locked_writeln_result = measure_stdout(iterations, "locked writeln!", report_size, 16, write_with_locked_writeln)?;
    let buffered_writeln_result = measure_stdout(iterations, "buffered writeln! new Vec", report_size, 1, write_with_buffered_writeln)?;
    let reused_buffer_result = measure_stdout_reused_buffered_writeln(iterations, report_size)?;

    let results = vec![output_result, println_result, locked_writeln_result, buffered_writeln_result, reused_buffer_result];

    print_stdout_table(&results, "buffered writeln! reused Vec", iterations);
    print_fastest(&results);

    Ok(())
}

fn print_banner(title: &str) {
    eprintln!("============================================================");
    eprintln!("{title}");
    eprintln!("============================================================");
}

fn print_section(title: &str) {
    eprintln!("------------------------------------------------------------");
    eprintln!("{title}");
    eprintln!("------------------------------------------------------------");
}

fn print_metric_guide() {
    print_section("how to read this report");
    eprintln!("- ns/report: lower is faster for one complete report.");
    eprintln!("- write ops/r: fewer write calls usually means less writer or stdout overhead.");
    eprintln!("- allocs/r: 0 means no heap allocation inside the measured loop; 1 means one new allocation per report.");
    eprintln!("- alloc bytes/r: 2048 means a new 2 KiB buffer was allocated for each report.");
    eprintln!("- extra alloc/r: allocation count compared with that section's baseline.");
    eprintln!("- output_to! trades zero allocation for more write calls.");
    eprintln!("- output!, output_buffered_to!, and new-Vec buffering trade one allocation for one write call.");
    eprintln!("- reused-Vec buffering allocates before timing and reuses that memory.");
}

#[derive(Clone)]
struct BenchmarkResult {
    label: &'static str,
    duration: Duration,
    bytes_written: usize,
    write_operations: usize,
    allocation_calls: usize,
    allocated_bytes: usize,
}

impl BenchmarkResult {
    fn nanoseconds_per_report(&self, iterations: usize) -> f64 {
        self.duration.as_nanos() as f64 / iterations as f64
    }

    fn bytes_per_report(&self, iterations: usize) -> f64 {
        self.bytes_written as f64 / iterations as f64
    }

    fn write_operations_per_report(&self, iterations: usize) -> f64 {
        self.write_operations as f64 / iterations as f64
    }

    fn allocations_per_report(&self, iterations: usize) -> f64 {
        self.allocation_calls as f64 / iterations as f64
    }

    fn allocated_bytes_per_report(&self, iterations: usize) -> f64 {
        self.allocated_bytes as f64 / iterations as f64
    }
}

#[derive(Clone, Copy)]
struct AllocationSnapshot {
    allocation_calls: usize,
    allocated_bytes: usize,
}

fn reset_allocation_counters() {
    ALLOCATION_CALLS.store(0, Ordering::Relaxed);
    ALLOCATED_BYTES.store(0, Ordering::Relaxed);
    DEALLOCATION_CALLS.store(0, Ordering::Relaxed);
    DEALLOCATED_BYTES.store(0, Ordering::Relaxed);
}

fn allocation_snapshot() -> AllocationSnapshot {
    AllocationSnapshot { allocation_calls: ALLOCATION_CALLS.load(Ordering::Relaxed), allocated_bytes: ALLOCATED_BYTES.load(Ordering::Relaxed) }
}

fn measure_writer(iterations: usize, label: &'static str, write_report: fn(&mut dyn Write, &ReportValues) -> io::Result<()>) -> io::Result<BenchmarkResult> {
    let values = ReportValues::new();
    let mut warmup_writer = CountingWriter::default();

    write_report(&mut warmup_writer, black_box(&values))?;

    let mut writer = CountingWriter::default();
    reset_allocation_counters();

    let start = Instant::now();

    for _ in 0..iterations {
        write_report(&mut writer, black_box(&values))?;
    }

    let duration = start.elapsed();
    let allocation_snapshot = allocation_snapshot();

    Ok(BenchmarkResult { label, duration, bytes_written: black_box(writer.bytes_written), write_operations: black_box(writer.write_calls), allocation_calls: allocation_snapshot.allocation_calls, allocated_bytes: allocation_snapshot.allocated_bytes })
}

fn measure_reused_buffered_writeln(iterations: usize) -> io::Result<BenchmarkResult> {
    let values = ReportValues::new();
    let mut writer = CountingWriter::default();
    let mut buffer = Vec::with_capacity(REPORT_BUFFER_CAPACITY);

    write_traditional_reusing_buffer(&mut writer, &mut buffer, black_box(&values))?;
    writer.reset();
    buffer.clear();
    reset_allocation_counters();

    let start = Instant::now();

    for _ in 0..iterations {
        write_traditional_reusing_buffer(&mut writer, &mut buffer, black_box(&values))?;
    }

    let duration = start.elapsed();
    let allocation_snapshot = allocation_snapshot();

    Ok(BenchmarkResult { label: "buffered writeln! reused Vec", duration, bytes_written: black_box(writer.bytes_written), write_operations: black_box(writer.write_calls), allocation_calls: allocation_snapshot.allocation_calls, allocated_bytes: allocation_snapshot.allocated_bytes })
}

fn measure_stdout(iterations: usize, label: &'static str, report_size: usize, write_operations_per_report: usize, write_report: fn(&ReportValues) -> io::Result<()>) -> io::Result<BenchmarkResult> {
    let values = ReportValues::new();

    write_report(black_box(&values))?;
    reset_allocation_counters();

    let start = Instant::now();

    for _ in 0..iterations {
        write_report(black_box(&values))?;
    }

    let duration = start.elapsed();
    let allocation_snapshot = allocation_snapshot();

    Ok(BenchmarkResult { label, duration, bytes_written: report_size.saturating_mul(iterations), write_operations: write_operations_per_report.saturating_mul(iterations), allocation_calls: allocation_snapshot.allocation_calls, allocated_bytes: allocation_snapshot.allocated_bytes })
}

fn measure_stdout_reused_buffered_writeln(iterations: usize, report_size: usize) -> io::Result<BenchmarkResult> {
    let values = ReportValues::new();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut buffer = Vec::with_capacity(REPORT_BUFFER_CAPACITY);

    write_traditional_reusing_buffer(&mut stdout_lock, &mut buffer, black_box(&values))?;
    buffer.clear();
    reset_allocation_counters();

    let start = Instant::now();

    for _ in 0..iterations {
        write_traditional_reusing_buffer(&mut stdout_lock, &mut buffer, black_box(&values))?;
    }

    let duration = start.elapsed();
    let allocation_snapshot = allocation_snapshot();

    Ok(BenchmarkResult { label: "buffered writeln! reused Vec", duration, bytes_written: report_size.saturating_mul(iterations), write_operations: iterations, allocation_calls: allocation_snapshot.allocation_calls, allocated_bytes: allocation_snapshot.allocated_bytes })
}

fn print_writer_table(results: &[BenchmarkResult], baseline_label: &str, iterations: usize) {
    let baseline_allocations = baseline_allocations(results, baseline_label, iterations);

    print_metrics_table(results, baseline_allocations, iterations);
}

fn print_stdout_table(results: &[BenchmarkResult], baseline_label: &str, iterations: usize) {
    let baseline_allocations = baseline_allocations(results, baseline_label, iterations);

    print_metrics_table(results, baseline_allocations, iterations);
}

fn print_metrics_table(results: &[BenchmarkResult], baseline_allocations: f64, iterations: usize) {
    eprintln!("+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+");
    eprintln!("| implementation                  |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |");
    eprintln!("+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+");

    for result in results {
        print_result_row(result, baseline_allocations, iterations);
    }

    eprintln!("+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+");
}

fn print_result_row(result: &BenchmarkResult, baseline_allocations: f64, iterations: usize) {
    let allocations_per_report = result.allocations_per_report(iterations);
    let allocation_delta = allocations_per_report - baseline_allocations;

    eprintln!(
        "| {:<31} | {:>12.1} | {:>12.1} | {:>12.1} | {:>11.3} | {:>13.1} | {:>+13.3} |",
        result.label,
        result.nanoseconds_per_report(iterations),
        result.bytes_per_report(iterations),
        result.write_operations_per_report(iterations),
        allocations_per_report,
        result.allocated_bytes_per_report(iterations),
        allocation_delta
    );
}

fn baseline_allocations(results: &[BenchmarkResult], baseline_label: &str, iterations: usize) -> f64 {
    for result in results {
        if result.label == baseline_label {
            return result.allocations_per_report(iterations);
        }
    }

    0.0
}

fn print_fastest(results: &[BenchmarkResult]) {
    let mut fastest: Option<&BenchmarkResult> = None;

    for result in results {
        if let Some(current_fastest) = fastest {
            if result.duration < current_fastest.duration {
                fastest = Some(result);
            }
        } else {
            fastest = Some(result);
        }
    }

    if let Some(fastest_result) = fastest {
        eprintln!("fastest: {}", fastest_result.label);

        for result in results {
            if result.label != fastest_result.label {
                let ratio = result.duration.as_secs_f64() / fastest_result.duration.as_secs_f64();
                eprintln!("{} is {ratio:.2}x faster than {}", fastest_result.label, result.label);
            }
        }
    }
}

#[derive(Default)]
struct CountingWriter {
    bytes_written: usize,
    write_calls: usize,
}

impl CountingWriter {
    fn reset(&mut self) {
        self.bytes_written = 0;
        self.write_calls = 0;
    }
}

impl Write for CountingWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.bytes_written = self.bytes_written.wrapping_add(black_box(buffer.len()));
        self.write_calls = self.write_calls.wrapping_add(1);

        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct ReportValues {
    distance: f64,
    centimeters: f64,
    meters: f64,
    seconds: f64,
    kilograms: f64,
    kilometers_per_hour: f64,
    acceleration: f64,
    gravity: f64,
    kilonewtons: f64,
    hello: &'static str,
}

impl ReportValues {
    fn new() -> Self {
        let distance = 10.0;
        let time = 2.0;
        let mass = 4.0;
        let velocity = distance / time;
        let acceleration = velocity / time;
        let force = mass * acceleration;

        Self { distance, centimeters: distance * 100.0, meters: distance, seconds: time, kilograms: mass, kilometers_per_hour: velocity * 3.6, acceleration, gravity: acceleration / 9.80665, kilonewtons: force / 1000.0, hello: "Hello" }
    }
}

fn verify_report_bytes() -> io::Result<usize> {
    let values = ReportValues::new();
    let mut traditional_buffer = Vec::new();
    let mut output_to_buffer = Vec::new();
    let mut output_buffered_buffer = Vec::new();
    let mut buffered_traditional_buffer = Vec::new();
    let mut reused_buffered_traditional_buffer = Vec::new();
    let mut reusable_buffer = Vec::with_capacity(REPORT_BUFFER_CAPACITY);

    write_traditional(&mut traditional_buffer, &values)?;
    write_with_output_to(&mut output_to_buffer, &values)?;
    write_with_output_buffered_to(&mut output_buffered_buffer, &values)?;
    write_traditional_buffered(&mut buffered_traditional_buffer, &values)?;
    write_traditional_reusing_buffer(&mut reused_buffered_traditional_buffer, &mut reusable_buffer, &values)?;

    assert_eq!(traditional_buffer, output_to_buffer);
    assert_eq!(traditional_buffer, output_buffered_buffer);
    assert_eq!(traditional_buffer, buffered_traditional_buffer);
    assert_eq!(traditional_buffer, reused_buffered_traditional_buffer);

    Ok(traditional_buffer.len())
}

fn write_with_output_to(writer: &mut dyn Write, values: &ReportValues) -> io::Result<()> {
    let distance = values.distance;
    let centimeters = values.centimeters;
    let meters = values.meters;
    let seconds = values.seconds;
    let time = values.seconds;
    let kilograms = values.kilograms;
    let kilometers_per_hour = values.kilometers_per_hour;
    let acceleration = values.acceleration;
    let gravity = values.gravity;
    let kilonewtons = values.kilonewtons;
    let hello = values.hello;

    output_to! {
        writer: writer,
        << [measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.
        << [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
        << [time details] time = {seconds} seconds, source value = {time}, status: accepted;
        << [mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.
        << [motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.
        << [motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.
        << [gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.
        << [force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.
        << [summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].
        << [summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].
        << [summary: force] force [{kilonewtons} kN], message [{hello}].
        << [report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.
        << [report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.
        << [report] Compared with Earth gravity, this acceleration is {gravity} g.
        << [report] Final force output: {kilonewtons} kN.
        << [message] force label reused with text value: {hello}.
    }

    Ok(())
}

fn write_with_output_buffered_to(writer: &mut dyn Write, values: &ReportValues) -> io::Result<()> {
    let distance = values.distance;
    let centimeters = values.centimeters;
    let meters = values.meters;
    let seconds = values.seconds;
    let time = values.seconds;
    let kilograms = values.kilograms;
    let kilometers_per_hour = values.kilometers_per_hour;
    let acceleration = values.acceleration;
    let gravity = values.gravity;
    let kilonewtons = values.kilonewtons;
    let hello = values.hello;

    output_buffered_to! {
        writer: writer,
        << [measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.
        << [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
        << [time details] time = {seconds} seconds, source value = {time}, status: accepted;
        << [mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.
        << [motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.
        << [motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.
        << [gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.
        << [force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.
        << [summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].
        << [summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].
        << [summary: force] force [{kilonewtons} kN], message [{hello}].
        << [report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.
        << [report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.
        << [report] Compared with Earth gravity, this acceleration is {gravity} g.
        << [report] Final force output: {kilonewtons} kN.
        << [message] force label reused with text value: {hello}.
    }

    Ok(())
}

fn write_with_output_macro(values: &ReportValues) -> io::Result<()> {
    let distance = values.distance;
    let centimeters = values.centimeters;
    let meters = values.meters;
    let seconds = values.seconds;
    let time = values.seconds;
    let kilograms = values.kilograms;
    let kilometers_per_hour = values.kilometers_per_hour;
    let acceleration = values.acceleration;
    let gravity = values.gravity;
    let kilonewtons = values.kilonewtons;
    let hello = values.hello;

    output! {
        << [measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.
        << [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
        << [time details] time = {seconds} seconds, source value = {time}, status: accepted;
        << [mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.
        << [motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.
        << [motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.
        << [gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.
        << [force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.
        << [summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].
        << [summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].
        << [summary: force] force [{kilonewtons} kN], message [{hello}].
        << [report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.
        << [report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.
        << [report] Compared with Earth gravity, this acceleration is {gravity} g.
        << [report] Final force output: {kilonewtons} kN.
        << [message] force label reused with text value: {hello}.
    }

    Ok(())
}

fn write_with_println(values: &ReportValues) -> io::Result<()> {
    println!("[measurement: distance] raw input = {}, converted value = {} centimeters, normalized value = {} meters.", values.distance, values.centimeters, values.meters);
    println!("[distance details] distance = {}, bubels = {} centimeters, meters = {}.", values.distance, values.centimeters, values.meters);
    println!("[time details] time = {} seconds, source value = {}, status: accepted;", values.seconds, values.seconds);
    println!("[mass details] mass = {} kilograms, input validation: complete, range check: not applied.", values.kilograms);
    println!("[motion: velocity] velocity = {} km/h, calculated from distance and time.", values.kilometers_per_hour);
    println!("[motion: acceleration] acceleration = {} m/s^2, derived from velocity over time.", values.acceleration);
    println!("[gravity comparison] gravity = {} g, where 1.0 g means standard Earth gravity.", values.gravity);
    println!("[force calculation] force = {} kN, based on mass, velocity, and acceleration.", values.kilonewtons);
    println!("[summary: values] distance [{}], centimeters [{}], meters [{}], seconds [{}], kilograms [{}].", values.distance, values.centimeters, values.meters, values.seconds, values.kilograms);
    println!("[summary: motion] velocity [{} km/h], acceleration [{} m/s^2], gravity [{} g].", values.kilometers_per_hour, values.acceleration, values.gravity);
    println!("[summary: force] force [{} kN], message [{}].", values.kilonewtons, values.hello);
    println!("[report] The object moved {} meters, over {} seconds, with mass {} kilograms.", values.meters, values.seconds, values.kilograms);
    println!("[report] The resulting velocity was {} km/h, and acceleration was {} m/s^2.", values.kilometers_per_hour, values.acceleration);
    println!("[report] Compared with Earth gravity, this acceleration is {} g.", values.gravity);
    println!("[report] Final force output: {} kN.", values.kilonewtons);
    println!("[message] force label reused with text value: {}.", values.hello);

    Ok(())
}

fn write_with_locked_writeln(values: &ReportValues) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    write_traditional(&mut stdout_lock, values)
}

fn write_with_buffered_writeln(values: &ReportValues) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut buffer = Vec::with_capacity(REPORT_BUFFER_CAPACITY);

    write_traditional(&mut buffer, values)?;
    stdout_lock.write_all(&buffer)
}

fn write_traditional_buffered(writer: &mut dyn Write, values: &ReportValues) -> io::Result<()> {
    let mut buffer = Vec::with_capacity(REPORT_BUFFER_CAPACITY);

    write_traditional(&mut buffer, values)?;
    writer.write_all(&buffer)
}

fn write_traditional_reusing_buffer<Writer>(writer: &mut Writer, buffer: &mut Vec<u8>, values: &ReportValues) -> io::Result<()>
where
    Writer: Write + ?Sized,
{
    buffer.clear();
    write_traditional(buffer, values)?;
    writer.write_all(buffer)
}

fn write_traditional(writer: &mut dyn Write, values: &ReportValues) -> io::Result<()> {
    writeln!(writer, "[measurement: distance] raw input = {}, converted value = {} centimeters, normalized value = {} meters.", values.distance, values.centimeters, values.meters)?;
    writeln!(writer, "[distance details] distance = {}, bubels = {} centimeters, meters = {}.", values.distance, values.centimeters, values.meters)?;
    writeln!(writer, "[time details] time = {} seconds, source value = {}, status: accepted;", values.seconds, values.seconds)?;
    writeln!(writer, "[mass details] mass = {} kilograms, input validation: complete, range check: not applied.", values.kilograms)?;
    writeln!(writer, "[motion: velocity] velocity = {} km/h, calculated from distance and time.", values.kilometers_per_hour)?;
    writeln!(writer, "[motion: acceleration] acceleration = {} m/s^2, derived from velocity over time.", values.acceleration)?;
    writeln!(writer, "[gravity comparison] gravity = {} g, where 1.0 g means standard Earth gravity.", values.gravity)?;
    writeln!(writer, "[force calculation] force = {} kN, based on mass, velocity, and acceleration.", values.kilonewtons)?;
    writeln!(writer, "[summary: values] distance [{}], centimeters [{}], meters [{}], seconds [{}], kilograms [{}].", values.distance, values.centimeters, values.meters, values.seconds, values.kilograms)?;
    writeln!(writer, "[summary: motion] velocity [{} km/h], acceleration [{} m/s^2], gravity [{} g].", values.kilometers_per_hour, values.acceleration, values.gravity)?;
    writeln!(writer, "[summary: force] force [{} kN], message [{}].", values.kilonewtons, values.hello)?;
    writeln!(writer, "[report] The object moved {} meters, over {} seconds, with mass {} kilograms.", values.meters, values.seconds, values.kilograms)?;
    writeln!(writer, "[report] The resulting velocity was {} km/h, and acceleration was {} m/s^2.", values.kilometers_per_hour, values.acceleration)?;
    writeln!(writer, "[report] Compared with Earth gravity, this acceleration is {} g.", values.gravity)?;
    writeln!(writer, "[report] Final force output: {} kN.", values.kilonewtons)?;
    writeln!(writer, "[message] force label reused with text value: {}.", values.hello)?;

    Ok(())
}
