extern crate winapi;
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, WPARAM},
        ntdef::LPCWSTR,
        windef::{HBRUSH, HWND, RECT},
    },
    um::{
        libloaderapi::GetModuleHandleW,
        wingdi::{CreateSolidBrush, RGB},
        winuser::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW,
            PostQuitMessage, RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW,
            CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, SW_SHOW, WM_DESTROY, WM_PAINT,
            WNDCLASSW, WS_OVERLAPPEDWINDOW,
        },
    },
};

const CLASS_NAME: &str = "MyWindowClass\0";

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut rect: RECT = std::mem::zeroed();
            if winapi::um::winuser::GetClientRect(hwnd, &mut rect) == 0 {
                return 0;
            }

            let hdc = winapi::um::winuser::BeginPaint(hwnd, std::ptr::null_mut());
            let brush = winapi::um::wingdi::CreateSolidBrush(RGB(255, 0, 0)); // Red color brush

            winapi::um::winuser::FillRect(hdc, &rect, brush);

            winapi::um::winuser::EndPaint(hwnd, std::ptr::null_mut());
        }

        WM_DESTROY => {
            winapi::um::winuser::PostQuitMessage(0);
        }

        _ => return winapi::um::winuser::DefWindowProcW(hwnd, msg, w_param, l_param),
    }

    return 0;
}

fn main() {
    unsafe {
        let h_instance = GetModuleHandleW(std::ptr::null());

        let window_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: CLASS_NAME.as_ptr() as LPCWSTR,
            ..std::mem::zeroed()
        };

        RegisterClassW(&window_class);

        let hwnd = CreateWindowExW(
            0,
            CLASS_NAME.as_ptr() as LPCWSTR,
            "Rust Window\0".as_ptr() as LPCWSTR,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        if hwnd.is_null() {
            return;
        }

        ShowWindow(hwnd, SW_SHOW);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}