use std::ptr::null_mut;
use windows::{
    core::{w, Error, Result, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE},
            Input::KeyboardAndMouse::VK_TAB,
            WindowsAndMessaging::{
            AdjustWindowRect, CreateWindowExW, DefWindowProcW, DispatchMessageW, IsIconic, PeekMessageW, PostQuitMessage,
            RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
            MSG, PM_REMOVE, SW_SHOW,
            WaitMessage, WINDOW_EX_STYLE, WM_DESTROY, WM_KEYUP, WM_QUIT, WNDCLASSW,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,
            },
        },
    },
};

// data structures
/// A Win32 application window and its pending message state.
pub struct ApplicationWindow
{
    window_handle: HWND,
    message: MSG,
}

/// Events collected from the Win32 message queue since the last poll.
pub struct WindowEvents
{
    /// True when the application should terminate.
    pub should_close: bool,

    /// True when the user released the Tab key during this poll.
    pub was_tab_released: bool,
}
// data structures

// domain constants
const WINDOW_CLASS_NAME: PCWSTR = w!("WindowsRsSpinningCubeWindow");
const WINDOW_TITLE: PCWSTR = w!("Windows-rs Direct3D 11 Spinning Cube");
const MINIMUM_WINDOW_DIMENSION: i32 = 1;
// domain constants

/// Configures the process to use per-monitor DPI awareness.
pub fn enable_per_monitor_dpi_awareness() -> Result<()>
{
    unsafe
    {
        return SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    }
}

/// Creates and displays the demo's Win32 application window.
pub fn create_window(window_width: i32, window_height: i32) -> Result<ApplicationWindow>
{
    if window_width < MINIMUM_WINDOW_DIMENSION
    {
        return Err(Error::new(
            windows::Win32::Foundation::E_INVALIDARG,
            "window_width must be positive.",
        ));
    }

    if window_height < MINIMUM_WINDOW_DIMENSION
    {
        return Err(Error::new(
            windows::Win32::Foundation::E_INVALIDARG,
            "window_height must be positive.",
        ));
    }

    unsafe
    {
        return create_window_internal(window_width, window_height);
    }
}

impl ApplicationWindow
{
    /// Removes and translates all currently pending Win32 messages.
    pub fn process_pending_messages(&mut self) -> WindowEvents
    {
        let mut window_events = WindowEvents {
            should_close: false,
            was_tab_released: false,
        };

        unsafe
        {
            while PeekMessageW(&mut self.message, None, 0, 0, PM_REMOVE).as_bool()
            {
                if self.message.message == WM_QUIT
                {
                    window_events.should_close = true;
                }

                if self.message.message == WM_KEYUP && self.message.wParam.0 == VK_TAB.0 as usize
                {
                    window_events.was_tab_released = true;
                }

                let _ = TranslateMessage(&self.message);
                DispatchMessageW(&self.message);
            }
        }

        return window_events;
    }

    /// Returns the native Win32 handle used to create graphics resources.
    pub fn handle(&self) -> HWND
    {
        return self.window_handle;
    }

    /// Returns whether Windows reports that this window is minimized.
    pub fn is_minimized(&self) -> bool
    {
        unsafe
        {
            return IsIconic(self.window_handle).as_bool();
        }
    }

    /// Blocks until Windows places another message in the queue.
    pub fn wait_for_message(&self) -> Result<()>
    {
        unsafe
        {
            return WaitMessage();
        }
    }
}

unsafe fn create_window_internal(
    window_width: i32,
    window_height: i32,
) -> Result<ApplicationWindow>
{
    let instance = GetModuleHandleW(None)?;
    let mut client_area = RECT {
        left: 0,
        top: 0,
        right: window_width,
        bottom: window_height,
    };
    AdjustWindowRect(&mut client_area, WS_OVERLAPPEDWINDOW, false)?;
    let window_class = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_procedure),
        hInstance: instance.into(),
        lpszClassName: WINDOW_CLASS_NAME,
        ..Default::default()
    };
    let registration_result = RegisterClassW(&window_class);

    if registration_result == 0
    {
        return Err(Error::from_thread());
    }

    let window_handle = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        WINDOW_CLASS_NAME,
        WINDOW_TITLE,
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        client_area.right - client_area.left,
        client_area.bottom - client_area.top,
        None,
        None,
        Some(instance.into()),
        Some(null_mut()),
    )?;
    let _ = ShowWindow(window_handle, SW_SHOW);

    return Ok(ApplicationWindow {
        window_handle,
        message: MSG::default(),
    });
}

unsafe extern "system" fn window_procedure(
    window_handle: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT
{
    match message
    {
        WM_DESTROY =>
        {
            PostQuitMessage(0);
            return LRESULT(0);
        }
        _ =>
        {
            return DefWindowProcW(window_handle, message, wparam, lparam);
        }
    }
}
