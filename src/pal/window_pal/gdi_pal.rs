/// GDI bitmap capture worker  --  PrintWindow + GetDIBits -> raw BGRA bytes.
/// Works on background/hidden windows without SetForegroundWindow.
use crate::shared::AppError;
use crate::state::sizes::{CAPTURE_BPP, CAPTURE_BYTES_PER_PIXEL, PW_FULL_CONTENT};

#[cfg(windows)]
use windows::{
    Win32::Foundation::HWND,
    Win32::Graphics::Gdi::{
        CreateCompatibleBitmap, CreateCompatibleDC,
        DeleteDC, DeleteObject, GetDIBits, GetDC, ReleaseDC,
        SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, HGDIOBJ,
    },
    Win32::Storage::Xps::{PrintWindow, PRINT_WINDOW_FLAGS},
};

/// Captures raw BGRA pixels from `hwnd` using PrintWindow.
/// Works even when the window is behind other windows or on another virtual desktop.
pub(crate) fn gdi_capture_raw_pal(hwnd: u64) -> Result<(Vec<u8>, i32, i32), AppError> {
    #[cfg(windows)]
    {
        let hwnd_val = HWND(hwnd as usize as *mut _);

        let (left, top, right, bottom) = super::get_window_rect(hwnd)?;
        let (w, h) = (right - left, bottom - top);
        if w <= 0 || h <= 0 {
            return Err(AppError::CaptureFailed(format!(
                "Invalid window dimensions: {}x{}", w, h
            )));
        }

        // SAFETY: NULL hwnd = screen DC, always valid.
        let screen_dc = unsafe { GetDC(HWND::default()) };
        if screen_dc.is_invalid() {
            return Err(AppError::CaptureFailed("GetDC(screen) failed".to_string()));
        }

        // SAFETY: screen_dc is a valid HDC.
        let mem_dc = unsafe { CreateCompatibleDC(screen_dc) };
        if mem_dc.is_invalid() {
            // SAFETY: paired cleanup.
            unsafe { ReleaseDC(HWND::default(), screen_dc); }
            return Err(AppError::CaptureFailed("CreateCompatibleDC failed".to_string()));
        }

        // SAFETY: screen_dc is valid; w/h are positive.
        let bitmap = unsafe { CreateCompatibleBitmap(screen_dc, w, h) };
        if bitmap.is_invalid() {
            // SAFETY: paired cleanup.
            unsafe {
                let _ = DeleteDC(mem_dc);
                ReleaseDC(HWND::default(), screen_dc);
            }
            return Err(AppError::CaptureFailed("CreateCompatibleBitmap failed".to_string()));
        }

        // SAFETY: mem_dc valid; bitmap handle valid.
        let old_obj = unsafe { SelectObject(mem_dc, HGDIOBJ(bitmap.0)) };

        // SAFETY: hwnd_val is a valid HWND; mem_dc has bitmap selected; PW_RENDERFULLCONTENT flag.
        let ok = unsafe { PrintWindow(hwnd_val, mem_dc, PRINT_WINDOW_FLAGS(PW_FULL_CONTENT)) };
        if !ok.as_bool() {
            // SAFETY: cleanup in reverse order.
            unsafe {
                SelectObject(mem_dc, old_obj);
                let _ = DeleteDC(mem_dc);
                let _ = DeleteObject(HGDIOBJ(bitmap.0));
                ReleaseDC(HWND::default(), screen_dc);
            }
            return Err(AppError::CaptureFailed("PrintWindow failed".to_string()));
        }

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w,
                biHeight: -h,
                biPlanes: 1,
                biBitCount: CAPTURE_BPP as u16,
                biCompression: BI_RGB.0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [Default::default()],
        };

        let buf_size = (w * h) as usize * CAPTURE_BYTES_PER_PIXEL;
        let mut pixels: Vec<u8> = vec![0u8; buf_size];

        // SAFETY: mem_dc and bitmap valid; pixels sized correctly.
        let scan_lines = unsafe {
            GetDIBits(
                mem_dc,
                bitmap,
                0,
                h as u32,
                Some(pixels.as_mut_ptr() as *mut _),
                &mut bmi,
                DIB_RGB_COLORS,
            )
        };

        // SAFETY: cleanup all GDI resources in reverse order.
        unsafe {
            SelectObject(mem_dc, old_obj);
            let _ = DeleteDC(mem_dc);
            let _ = DeleteObject(HGDIOBJ(bitmap.0));
            ReleaseDC(HWND::default(), screen_dc);
        }

        if scan_lines == 0 {
            return Err(AppError::CaptureFailed(
                "GetDIBits returned 0 scan lines".to_string(),
            ));
        }

        return Ok((pixels, w, h));
    }

    #[cfg(not(windows))]
    {
        let _ = hwnd;
        Err(AppError::CaptureFailed("Windows only".to_string()))
    }
}
