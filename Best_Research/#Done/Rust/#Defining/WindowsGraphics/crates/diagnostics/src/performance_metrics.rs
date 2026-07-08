use std::time::{Duration, Instant};
use windows::{
    core::Result,
    Win32::{
        Foundation::FILETIME,
        System::Threading::{GetCurrentProcess, GetProcessTimes},
    },
};

// data structures
/// Aggregates rendered frames and process CPU time between performance samples.
pub struct PerformanceMetrics
{
    sampled_at: Instant,
    sampled_process_cpu_time_in_100_nanoseconds: ProcessTimeIn100Nanoseconds,
    rendered_frame_count: u32,
}

/// A calculated frame-rate, frame-time, and process CPU-usage sample.
pub struct PerformanceSample
{
    /// Rendered frames per second during the sampling interval.
    pub frames_per_second: f32,

    /// Average duration of one rendered frame during the sampling interval.
    pub frame_time_in_milliseconds: f32,

    /// CPU time used by this process, expressed as a percentage of one logical processor.
    pub process_cpu_usage_percentage: f32,
}
// data structures

// domain constants
const METRICS_SAMPLE_INTERVAL: Duration = Duration::from_millis(250);
const ONE_HUNDRED_NANOSECONDS_PER_SECOND: f32 = 10_000_000.0;
const MINIMUM_FRAMES_PER_SECOND: f32 = 0.001;
// domain constants

// semantic type aliases
type ProcessTimeIn100Nanoseconds = u64;
// semantic type aliases

impl PerformanceMetrics
{
    /// Creates a performance sampler using the current process CPU time as its baseline.
    pub fn new() -> Result<Self>
    {
        return Ok(Self {
            sampled_at: Instant::now(),
            sampled_process_cpu_time_in_100_nanoseconds: process_cpu_time_in_100_nanoseconds()?,
            rendered_frame_count: 0,
        });
    }

    /// Records one completed rendered frame.
    pub fn record_rendered_frame(&mut self)
    {
        self.rendered_frame_count += 1;
    }

    /// Returns a new sample after the configured sampling interval has elapsed.
    ///
    /// Returns `Ok(None)` while the current interval is still collecting data.
    pub fn sample(&mut self) -> Result<Option<PerformanceSample>>
    {
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(self.sampled_at);

        if elapsed < METRICS_SAMPLE_INTERVAL
        {
            return Ok(None);
        }

        let current_process_cpu_time = process_cpu_time_in_100_nanoseconds()?;
        let elapsed_seconds = elapsed.as_secs_f32();
        let used_cpu_seconds = (current_process_cpu_time - self.sampled_process_cpu_time_in_100_nanoseconds) as f32
            / ONE_HUNDRED_NANOSECONDS_PER_SECOND;
        let frames_per_second = self.rendered_frame_count as f32 / elapsed_seconds;
        let process_cpu_usage_percentage = used_cpu_seconds / elapsed_seconds * 100.0;

        self.sampled_at = current_time;
        self.sampled_process_cpu_time_in_100_nanoseconds = current_process_cpu_time;
        self.rendered_frame_count = 0;

        return Ok(Some(PerformanceSample {
            frames_per_second,
            frame_time_in_milliseconds: 1000.0 / frames_per_second.max(MINIMUM_FRAMES_PER_SECOND),
            process_cpu_usage_percentage,
        }));
    }
}

fn process_cpu_time_in_100_nanoseconds() -> Result<ProcessTimeIn100Nanoseconds>
{
    unsafe
    {
        let mut created_at = FILETIME::default();
        let mut exited_at = FILETIME::default();
        let mut kernel_time = FILETIME::default();
        let mut user_time = FILETIME::default();
        GetProcessTimes(
            GetCurrentProcess(),
            &mut created_at,
            &mut exited_at,
            &mut kernel_time,
            &mut user_time,
        )?;

        return Ok(
            file_time_as_process_time_in_100_nanoseconds(kernel_time)
                + file_time_as_process_time_in_100_nanoseconds(user_time),
        );
    }
}

fn file_time_as_process_time_in_100_nanoseconds(file_time: FILETIME) -> ProcessTimeIn100Nanoseconds
{
    return (u64::from(file_time.dwHighDateTime) << 32) | u64::from(file_time.dwLowDateTime);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn file_time_conversion_preserves_high_and_low_components()
    {
        let file_time = FILETIME {
            dwLowDateTime: 0x89AB_CDEF,
            dwHighDateTime: 0x0123_4567,
        };
        let process_time = file_time_as_process_time_in_100_nanoseconds(file_time);

        assert_eq!(process_time, 0x0123_4567_89AB_CDEF);
    }
}
