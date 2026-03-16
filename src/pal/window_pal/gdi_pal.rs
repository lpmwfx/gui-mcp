/// GDI bitmap capture worker — SetForegroundWindow + BitBlt + GetDIBits → raw BGRA bytes.
use crate::shared::AppError;
use crate::state::sizes::{CAPTURE_BPP, CAPTURE_BYTES_PER_PIXEL};

#[cfg(windows)]
use windows::{
    Win32::Foundation::HWND,
    Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC,
        DeleteDC, DeleteObject, GetDIBits, GetWindowDC, ReleaseDC,
        SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, HGDIOBJ, SRCCOPY,
    },
    Win32::UI::WindowsAndMessaging::SetForegroundWindow,
};

/// Captures raw BGRA pixels from `hwnd`. Returns `(pixels, width, height)`.
///
/// Calls SetForegroundWindow, reads window geometry via `get_window_rect`,
/// then performs the full GDI BitBlt → GetDIBits pipeline.
pub(crate) fn gdi_capture_raw_pal(hwnd: u64) -> Result<(Vec<u8>, i32, i32), AppError> {
    #[cfg(windows)]
    {
        let hwnd_val = HWND(hwnd as usize as *mut _);

        // SAFETY: hwnd_val is a valid HWND obtained from find_window_by_partial_title.
        unsafe { let _ = SetForegroundWindow(hwnd_val); }

        let (left, top, right, bottom) = super::get_window_rect(hwnd)?;
        let (w, h) = (right - left, bottom - top);
        if w <= 0 || h <= 0 {
            return Err(AppError::CaptureFailed(format!(
                "Invalid window dimensions: {}x{}", w, h
            )));
        }

        // SAFETY: hwnd_val is valid; GetWindowDC returns NULL on failure, checked below.
        let src_dc = unsafe { GetWindowDC(hwnd_val) };
        if src_dc.is_invalid() {
            return Err(AppError::CaptureFailed("GetWindowDC failed".to_string()));
        }

        // SAFETY: src_dc is a valid HDC obtained from GetWindowDC.
        let mem_dc = unsafe { CreateCompatibleDC(src_dc) };
        if mem_dc.is_invalid() {
            // SAFETY: src_dc is valid; ReleaseDC pairs with GetWindowDC.
            unsafe { ReleaseDC(hwnd_val, src_dc); }
            return Err(AppError::CaptureFailed("CreateCompatibleDC failed".to_string()));
        }

        // SAFETY: src_dc is valid; w/h are positive (checked above).
        let bitmap = unsafe { CreateCompatibleBitmap(src_dc, w, h) };
        if bitmap.is_invalid() {
            // SAFETY: mem_dc and src_dc are valid; paired cleanup.
            unsafe {
                let _ = DeleteDC(mem_dc);
                ReleaseDC(hwnd_val, src_dc);
            }
            return Err(AppError::CaptureFailed("CreateCompatibleBitmap failed".to_string()));
        }

        // SAFETY: mem_dc is a valid DC; HGDIOBJ(bitmap.0) is a valid bitmap handle.
        let old_obj = unsafe { SelectObject(mem_dc, HGDIOBJ(bitmap.0)) };

        // SAFETY: mem_dc/src_dc are valid DCs; bitmap selected into mem_dc; dimensions positive.
        if let Err(e) = unsafe { BitBlt(mem_dc, 0, 0, w, h, src_dc, 0, 0, SRCCOPY) } {
            // SAFETY: cleanup GDI resources in reverse order of acquisition.
            unsafe {
                SelectObject(mem_dc, old_obj);
                let _ = DeleteDC(mem_dc);
                let _ = DeleteObject(HGDIOBJ(bitmap.0));
                ReleaseDC(hwnd_val, src_dc);
            }
            return Err(AppError::CaptureFailed(format!("BitBlt failed: {e}")));
        }

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w,
                biHeight: -h, // negative = top-down scan order
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

        // SAFETY: mem_dc and bitmap are valid; pixels buffer is sized w*h*CAPTURE_BYTES_PER_PIXEL.
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

        // SAFETY: cleanup all GDI resources unconditionally in reverse order of acquisition.
        unsafe {
            SelectObject(mem_dc, old_obj);
            let _ = DeleteDC(mem_dc);
            let _ = DeleteObject(HGDIOBJ(bitmap.0));
            ReleaseDC(hwnd_val, src_dc);
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
