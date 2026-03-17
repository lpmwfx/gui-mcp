/// Finds the deepest child window that can receive text input.
use crate::state::sizes::CLASS_NAME_BUF;

#[cfg(windows)]
use windows::{
    Win32::Foundation::{BOOL, HWND, LPARAM},
    Win32::UI::WindowsAndMessaging::{EnumChildWindows, GetClassNameW},
};

/// Walks the child tree recursively, preferring known edit class names.
#[cfg(windows)]
pub(crate) fn find_input_child_pal(parent: HWND) -> HWND {
    #[allow(non_camel_case_types)]
    struct ChildSearch_pal {
        best: HWND,
        depth: usize,
    }

    // SAFETY: callback is only invoked by EnumChildWindows with a valid lparam pointer.
    unsafe extern "system" fn child_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        // SAFETY: lparam is a valid pointer to ChildSearch_pal, alive for EnumChildWindows.
        let search_pal = &mut *(lparam.0 as *mut ChildSearch_pal);

        let mut class_buf = [0u16; CLASS_NAME_BUF];
        let len = GetClassNameW(hwnd, &mut class_buf);
        if len > 0 {
            let class_name = String::from_utf16_lossy(&class_buf[..len as usize]);
            let lower = class_name.to_lowercase();
            if lower.contains("edit") || lower.contains("richedit") || lower.contains("scintilla") {
                search_pal.best = hwnd;
                return BOOL(0);
            }
        }
        search_pal.depth += 1;
        search_pal.best = hwnd;
        BOOL(1)
    }

    let mut search_pal = ChildSearch_pal { best: parent, depth: 0 };
    // SAFETY: parent is valid HWND; search_pal lives for the call.
    unsafe {
        let _ = EnumChildWindows(
            parent,
            Some(child_callback),
            LPARAM(&mut search_pal as *mut _ as isize),
        );
    }
    if search_pal.best != parent {
        let deeper = find_input_child_pal(search_pal.best);
        if deeper != search_pal.best {
            return deeper;
        }
    }
    search_pal.best
}
