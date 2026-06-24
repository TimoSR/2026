use std::mem::size_of;
use windows::{
    core::Result,
    Win32::Graphics::Direct3D11::{
        D3D11_ASYNC_GETDATA_DONOTFLUSH, D3D11_QUERY, D3D11_QUERY_DATA_TIMESTAMP_DISJOINT,
        D3D11_QUERY_DESC, D3D11_QUERY_TIMESTAMP, D3D11_QUERY_TIMESTAMP_DISJOINT,
        ID3D11Device, ID3D11DeviceContext, ID3D11Query,
    },
};

// data structures
pub(crate) struct GpuTiming
{
    query_sets: [GpuTimestampQuerySet; 3],
    next_query_set_index: usize,
    active_query_set_index: Option<usize>,
    last_frame_time_in_milliseconds: Option<f32>,
}

struct GpuTimestampQuerySet
{
    disjoint_query: ID3D11Query,
    start_timestamp_query: ID3D11Query,
    end_timestamp_query: ID3D11Query,
    is_pending: bool,
}
// data structures

impl GpuTiming
{
    pub(crate) unsafe fn create(device: &ID3D11Device) -> Result<Self>
    {
        return Ok(Self {
            query_sets: [
                GpuTimestampQuerySet::create(device)?,
                GpuTimestampQuerySet::create(device)?,
                GpuTimestampQuerySet::create(device)?,
            ],
            next_query_set_index: 0,
            active_query_set_index: None,
            last_frame_time_in_milliseconds: None,
        });
    }

    pub(crate) fn last_frame_time_in_milliseconds(&self) -> Option<f32>
    {
        return self.last_frame_time_in_milliseconds;
    }

    pub(crate) unsafe fn begin_frame(&mut self, device_context: &ID3D11DeviceContext)
    {
        self.collect_completed_queries(device_context);

        let query_set = &mut self.query_sets[self.next_query_set_index];

        if query_set.is_pending
        {
            self.active_query_set_index = None;
            return;
        }

        device_context.Begin(&query_set.disjoint_query);
        device_context.End(&query_set.start_timestamp_query);
        query_set.is_pending = true;
        self.active_query_set_index = Some(self.next_query_set_index);
        self.next_query_set_index = (self.next_query_set_index + 1) % self.query_sets.len();
    }

    pub(crate) unsafe fn end_frame(&mut self, device_context: &ID3D11DeviceContext)
    {
        let active_query_set_index = match self.active_query_set_index
        {
            Some(active_query_set_index) => active_query_set_index,
            None => return,
        };
        let query_set = &self.query_sets[active_query_set_index];

        device_context.End(&query_set.end_timestamp_query);
        device_context.End(&query_set.disjoint_query);
        self.active_query_set_index = None;
    }

    unsafe fn collect_completed_queries(&mut self, device_context: &ID3D11DeviceContext)
    {
        let mut query_set_index = 0;

        while query_set_index < self.query_sets.len()
        {
            let query_set = &mut self.query_sets[query_set_index];

            if !query_set.is_pending
            {
                query_set_index += 1;
                continue;
            }

            let frame_time = read_frame_time(device_context, query_set);

            if let Some(frame_time) = frame_time
            {
                self.last_frame_time_in_milliseconds = Some(frame_time);
                query_set.is_pending = false;
            }

            query_set_index += 1;
        }
    }
}

impl GpuTimestampQuerySet
{
    unsafe fn create(device: &ID3D11Device) -> Result<Self>
    {
        return Ok(Self {
            disjoint_query: create_query(device, D3D11_QUERY_TIMESTAMP_DISJOINT)?,
            start_timestamp_query: create_query(device, D3D11_QUERY_TIMESTAMP)?,
            end_timestamp_query: create_query(device, D3D11_QUERY_TIMESTAMP)?,
            is_pending: false,
        });
    }
}

unsafe fn create_query(device: &ID3D11Device, query_type: D3D11_QUERY) -> Result<ID3D11Query>
{
    let description = D3D11_QUERY_DESC {
        Query: query_type,
        MiscFlags: 0,
    };
    let mut query = None;
    device.CreateQuery(&description, Some(&mut query))?;
    return query.ok_or_else(windows::core::Error::from_thread);
}

unsafe fn read_frame_time(
    device_context: &ID3D11DeviceContext,
    query_set: &GpuTimestampQuerySet,
) -> Option<f32>
{
    let mut disjoint_data = D3D11_QUERY_DATA_TIMESTAMP_DISJOINT::default();

    if device_context.GetData(
        &query_set.disjoint_query,
        Some((&mut disjoint_data as *mut D3D11_QUERY_DATA_TIMESTAMP_DISJOINT).cast()),
        size_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>() as u32,
        D3D11_ASYNC_GETDATA_DONOTFLUSH.0 as u32,
    ).is_err()
    {
        return None;
    }

    if disjoint_data.Disjoint.as_bool() || disjoint_data.Frequency == 0
    {
        return Some(0.0);
    }

    let mut start_timestamp = 0_u64;
    let mut end_timestamp = 0_u64;

    if device_context.GetData(
        &query_set.start_timestamp_query,
        Some((&mut start_timestamp as *mut u64).cast()),
        size_of::<u64>() as u32,
        D3D11_ASYNC_GETDATA_DONOTFLUSH.0 as u32,
    ).is_err()
    {
        return None;
    }

    if device_context.GetData(
        &query_set.end_timestamp_query,
        Some((&mut end_timestamp as *mut u64).cast()),
        size_of::<u64>() as u32,
        D3D11_ASYNC_GETDATA_DONOTFLUSH.0 as u32,
    ).is_err()
    {
        return None;
    }

    let timestamp_delta = end_timestamp.saturating_sub(start_timestamp);
    return Some(timestamp_delta as f64 as f32 / disjoint_data.Frequency as f32 * 1000.0);
}
