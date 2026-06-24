use std::ptr::null_mut;
use windows::{
    core::{w, Error, Result, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, PostQuitMessage,
            RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
            MSG, PM_REMOVE, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WM_QUIT, WNDCLASSW,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

// data structures
pub struct ApplicationWindow
{
    window_handle: HWND,
}
// data structures

// domain constants
const WINDOW_CLASS_NAME: PCWSTR = w!("WindowsRsSpinningCubeWindow");
const WINDOW_TITLE: PCWSTR = w!("Windows-rs Direct3D 11 Spinning Cube");
const MINIMUM_WINDOW_DIMENSION: i32 = 1;
// domain constants

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

pub fn process_pending_messages(message: &mut MSG) -> bool
{
    unsafe
    {
        while PeekMessageW(message, None, 0, 0, PM_REMOVE).as_bool()
        {
            if message.message == WM_QUIT
            {
                return true;
            }

            let _ = TranslateMessage(message);
            DispatchMessageW(message);
        }
    }

    return false;
}

impl ApplicationWindow
{
    pub fn handle(&self) -> HWND
    {
        return self.window_handle;
    }
}

unsafe fn create_window_internal(
    window_width: i32,
    window_height: i32,
) -> Result<ApplicationWindow>
{
    let instance = GetModuleHandleW(None)?;
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
        window_width,
        window_height,
        None,
        None,
        Some(instance.into()),
        Some(null_mut()),
    )?;

    let _ = ShowWindow(window_handle, SW_SHOW);

    return Ok(ApplicationWindow {
        window_handle,
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
