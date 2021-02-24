//! Raw bindings to user32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PFUNC, PSTR, PVOID};

#[link(name = "user32")]
extern "system" {
	pub fn AdjustWindowRectEx(lpRect: PVOID, dwStyle: u32, bMenu: BOOL, dwExStyle: u32) -> BOOL;
	pub fn AppendMenuW(hMenu: HANDLE, uFlags: u32, uIDNewItem: usize, lpNewItem: PCSTR) -> BOOL;
	pub fn ArrangeIconicWindows(hWnd: HANDLE) -> u32;
	pub fn BeginDeferWindowPos(nNumWindows: i32) -> HANDLE;
	pub fn BeginPaint(hWnd: HANDLE, lpPaint: PVOID) -> HANDLE;
	pub fn BringWindowToTop(hWnd: HANDLE) -> BOOL;
	pub fn CheckMenuItem(hMenu: HANDLE, uIDCheckItem: u32, uCheck: u32) -> i32;
	pub fn ClientToScreen(hWnd: HANDLE, lpPoint: PVOID) -> BOOL;
	pub fn CloseWindow(hWnd: HANDLE) -> BOOL;
	pub fn CreateAcceleratorTableW(paccel: PVOID, cAccel: i32) -> HANDLE;
	pub fn CreateDialogParamW(hInstance: HANDLE, lpTemplateName: PCSTR, hWndParent: HANDLE, lpDialogFunc: PFUNC, dwInitParam: isize) -> HANDLE;
	pub fn CreateMenu() -> HANDLE;
	pub fn CreatePopupMenu() -> HANDLE;
	pub fn CreateWindowExW(dwExStyle: u32, lpClassName: PCSTR, lpWindowName: PCSTR, dwStyle: u32, X: i32, Y: i32, nWidth: i32, nHeight: i32, hWndParent: HANDLE, hMenu: HANDLE, hInstance: HANDLE, lpParam: PVOID) -> HANDLE;
	pub fn DeferWindowPos(hWinPosInfo: HANDLE, hWnd: HANDLE, hWndInsertAfter: HANDLE, X: i32, Y: i32, cx: i32, cy: i32, uFlags: u32) -> HANDLE;
	pub fn DefWindowProcW(hWnd: HANDLE, Msg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn DeleteMenu(hMenu: HANDLE, uPosition: u32, uFlags: u32) -> BOOL;
	pub fn DestroyAcceleratorTable(hAccel: HANDLE) -> BOOL;
	pub fn DestroyIcon(hIcon: HANDLE) -> BOOL;
	pub fn DestroyMenu(hMenu: HANDLE) -> BOOL;
	pub fn DestroyWindow(hWnd: HANDLE) -> BOOL;
	pub fn DialogBoxParamW(hInstance: HANDLE, lpTemplateName: PCSTR, hWndParent: HANDLE, lpDialogFunc: PFUNC, dwInitParam: isize) -> isize;
	pub fn DispatchMessageW(lpMsg: PCVOID) -> isize;
	pub fn EnableMenuItem(hMenu: HANDLE, uIDEnableItem: u32, uEnable: u32) -> BOOL;
	pub fn EnableWindow(hWnd: HANDLE, bEnable: BOOL) -> BOOL;
	pub fn EndDeferWindowPos(hWinPosInfo: HANDLE) -> BOOL;
	pub fn EndDialog(hDlg: HANDLE, nResult: isize) -> BOOL;
	pub fn EndPaint(hWnd: HANDLE, lpPaint: PCVOID) -> BOOL;
	pub fn EnumChildWindows(hWndParent: HANDLE, lpEnumFunc: PFUNC, lParam: isize) -> BOOL;
	pub fn FindWindowW(lpClassName: PCSTR, lpWindowName: PCSTR) -> HANDLE;
	pub fn GetActiveWindow() -> HANDLE;
	pub fn GetAncestor(hwnd: HANDLE, gaFlags: u32) -> HANDLE;
	pub fn GetAsyncKeyState(vKey: i32) -> i16;
	pub fn GetCapture() -> HANDLE;
	pub fn GetClassInfoExW(hInstance: HANDLE, lpszClass: PCSTR, lpwcx: PVOID) -> BOOL;
	pub fn GetClassLongPtrW(hWnd: HANDLE, nIndex: i32) -> usize;
	pub fn GetClientRect(hWnd: HANDLE, lpRect: PVOID) -> BOOL;
	pub fn GetDC(hWnd: HANDLE) -> HANDLE;
	pub fn GetDesktopWindow() -> HANDLE;
	pub fn GetDialogBaseUnits() -> i32;
	pub fn GetDlgCtrlID(hWnd: HANDLE) -> i32;
	pub fn GetDlgItem(hDlg: HANDLE, nIDDlgItem: i32) -> HANDLE;
	pub fn GetDoubleClickTime() -> u32;
	pub fn GetFocus() -> HANDLE;
	pub fn GetForegroundWindow() -> HANDLE;
	pub fn GetMenuInfo(hMenu: HANDLE, lpmi: PVOID) -> BOOL;
	pub fn GetMenuItemCount(hMenu: HANDLE) -> i32;
	pub fn GetMenuItemID(hMenu: HANDLE, nPos: i32) -> i32;
	pub fn GetMessageW(lpMsg: PVOID, hWnd: HANDLE, wMsgFilterMin: u32, wMsgFilterMax: u32) -> BOOL;
	pub fn GetNextDlgGroupItem(hDlg: HANDLE, hCtl: HANDLE, bPrevious: BOOL) -> HANDLE;
	pub fn GetNextDlgTabItem(hDlg: HANDLE, hCtl: HANDLE, bPrevious: BOOL) -> HANDLE;
	pub fn GetParent(hWnd: HANDLE) -> HANDLE;
	pub fn GetQueueStatus(flags: u32) -> u32;
	pub fn GetSubMenu(hMenu: HANDLE, nPos: i32) -> HANDLE;
	pub fn GetSysColor(nIndex: i32) -> u32;
	pub fn GetSystemMetrics(nIndex: i32) -> i32;
	pub fn GetUpdateRgn(hWnd: HANDLE, hRgn: HANDLE, bErase: BOOL) -> i32;
	pub fn GetWindow(hWnd: HANDLE, uCmd: u32) -> HANDLE;
	pub fn GetWindowDC(hWnd: HANDLE) -> HANDLE;
	pub fn GetWindowDisplayAffinity(hWnd: HANDLE, pdwAffinity: PVOID) -> BOOL;
	pub fn GetWindowInfo(hwnd: HANDLE, pwi: PVOID) -> BOOL;
	pub fn GetWindowLongPtrW(hWnd: HANDLE, nIndex: i32) -> isize;
	pub fn GetWindowPlacement(hWnd: HANDLE, lpwndpl: PVOID) -> BOOL;
	pub fn GetWindowRect(hWnd: HANDLE, lpRect: PVOID) -> BOOL;
	pub fn GetWindowRgn(hWnd: HANDLE, hRgn: HANDLE) -> i32;
	pub fn GetWindowRgnBox(hWnd: HANDLE, lprc: PVOID) -> i32;
	pub fn GetWindowTextLengthW(hWnd: HANDLE) -> i32;
	pub fn GetWindowTextW(hWnd: HANDLE, lpString: PSTR, nMaxCount: i32) -> i32;
	pub fn HiliteMenuItem(hWnd: HANDLE, hMenu: HANDLE, uIDHiliteItem: u32, uHilite: u32) -> BOOL;
	pub fn InsertMenuItemW(hmenu: HANDLE, item: u32, fByPosition: BOOL, lpmi: PCVOID) -> BOOL;
	pub fn InsertMenuW(hMenu: HANDLE, uPosition: u32, uFlags: u32, uIDNewItem: usize, lpNewItem: PCSTR) -> BOOL;
	pub fn InvalidateRect(hWnd: HANDLE, lpRect: PCVOID, bErase: BOOL) -> BOOL;
	pub fn InvalidateRgn(hWnd: HANDLE, hRgn: HANDLE, bErase: BOOL) -> BOOL;
	pub fn IsChild(hWndParent: HANDLE, hWnd: HANDLE) -> BOOL;
	pub fn IsDialogMessageW(hDlg: HANDLE, lpMsg: PVOID) -> BOOL;
	pub fn IsGUIThread(bConvert: BOOL) -> BOOL;
	pub fn IsIconic(hWnd: HANDLE) -> BOOL;
	pub fn IsMenu(hMenu: HANDLE) -> BOOL;
	pub fn IsWindow(hWnd: HANDLE) -> BOOL;
	pub fn IsWindowEnabled(hWnd: HANDLE) -> BOOL;
	pub fn IsWindowVisible(hWnd: HANDLE) -> BOOL;
	pub fn IsZoomed(hWnd: HANDLE) -> BOOL;
	pub fn LoadAcceleratorsW(hInstance: HANDLE, lpTableName: PCSTR) -> HANDLE;
	pub fn LoadCursorW(hInstance: HANDLE, lpCursorName: PCSTR) -> HANDLE;
	pub fn LoadIconW(hInstance: HANDLE, lpIconName: PCSTR) -> HANDLE;
	pub fn LoadImageW(hInst: HANDLE, name: PCSTR, utype: u32, cx: i32, cy: i32, fuLoad: u32) -> HANDLE;
	pub fn LockSetForegroundWindow(uLockCode: u32) -> BOOL;
	pub fn MapDialogRect(hDlg: HANDLE, lpRect: PVOID) -> BOOL;
	pub fn MessageBoxW(hWnd: HANDLE, lpText: PCSTR, lpCaption: PCSTR, uType: u32) -> i32;
	pub fn PeekMessageW(lpMsg: PVOID, hWnd: HANDLE, wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMsg: u32) -> BOOL;
	pub fn PostMessageW(hWnd: HANDLE, Msg: u32, wParam: usize, lParam: isize) -> BOOL;
	pub fn PostQuitMessage(nExitCode: i32);
	pub fn RedrawWindow(hWnd: HANDLE, lprcUpdate: PCVOID, hrgnUpdate: HANDLE, flags: u32) -> BOOL;
	pub fn RegisterClassExW(lpwcx: PCVOID) -> u16;
	pub fn ReleaseDC(hWnd: HANDLE, hDC: HANDLE) -> i32;
	pub fn RemoveMenu(hMenu: HANDLE, uPosition: u32, uFlags: u32) -> BOOL;
	pub fn ScreenToClient(hWnd: HANDLE, lpPoint: PVOID) -> BOOL;
	pub fn SendMessageW(hWnd: HANDLE, Msg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn SetCaretBlinkTime(uMSeconds: u32) -> BOOL;
	pub fn SetCaretPos(X: i32, Y: i32) -> BOOL;
	pub fn SetCursorPos(X: i32, Y: i32) -> BOOL;
	pub fn SetFocus(hWnd: HANDLE) -> HANDLE;
	pub fn SetMenuInfo(hMenu: HANDLE, mii: PCVOID) -> BOOL;
	pub fn SetMenuItemInfoW(hmenu: HANDLE, item: u32, fByPosition: BOOL, lpmii: PCVOID) -> BOOL;
	pub fn SetParent(hWndChild: HANDLE, hWndNewParent: HANDLE) -> HANDLE;
	pub fn SetProcessDPIAware() -> BOOL;
	pub fn SetSystemCursor(hcur: HANDLE, id: u32) -> BOOL;
	pub fn SetWindowDisplayAffinity(hWnd: HANDLE, dwAffinity: u32) -> BOOL;
	pub fn SetWindowLongPtrW(hWnd: HANDLE, nIndex: i32, dwNewLong: isize) -> isize;
	pub fn SetWindowPlacement(hWnd: HANDLE, lpwndpl: PCVOID) -> BOOL;
	pub fn SetWindowPos(hWnd: HANDLE, hWndInsertAfter: HANDLE, X: i32, Y: i32, cx: i32, cy: i32, uFlags: u32) -> BOOL;
	pub fn SetWindowRgn(hWnd: HANDLE, hRgn: HANDLE, bRedraw: BOOL) -> i32;
	pub fn SetWindowsHookExW(idHook: i32, lpfn: PFUNC, hmod: HANDLE, dwThreadId: u32) -> HANDLE;
	pub fn SetWindowTextW(hWnd: HANDLE, lpString: PCSTR) -> BOOL;
	pub fn ShowCaret(hWnd: HANDLE) -> BOOL;
	pub fn ShowCursor(bShow: BOOL) -> i32;
	pub fn ShowWindow(hWnd: HANDLE, nCmdShow: i32) -> BOOL;
	pub fn SystemParametersInfoW(uiAction: u32, uiParam: u32, pvParam: PVOID, fWinIni: u32) -> BOOL;
	pub fn TrackMouseEvent(lpEventTrack: PVOID) -> BOOL;
	pub fn TrackPopupMenu(hMenu: HANDLE, uFlags: u32, x: i32, y: i32, nReserved: i32, hWnd: HANDLE, prcRect: PCVOID) -> BOOL;
	pub fn TranslateAcceleratorW(hWnd: HANDLE, hAccTable: HANDLE, lpMsg: PVOID) -> i32;
	pub fn TranslateMessage(lpMsg: PCVOID) -> BOOL;
	pub fn UnregisterClassW(lpClassName: PCSTR, hInstance: HANDLE) -> BOOL;
	pub fn UpdateWindow(hWnd: HANDLE) -> BOOL;
	pub fn ValidateRect(hWnd: HANDLE, lpRect: PCVOID) -> BOOL;
	pub fn ValidateRgn(hWnd: HANDLE, hRgn: HANDLE) -> BOOL;
}
