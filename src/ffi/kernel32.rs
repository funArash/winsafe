//! Raw bindings to kernel32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};

#[link(name = "kernel32")]
extern "system" {
	pub fn CloseHandle(_: HANDLE) -> BOOL;
	pub fn CopyFileW(_: PCSTR, _: PCSTR, _: BOOL) -> BOOL;
	pub fn CreateFileMappingW(_: HANDLE, _: PVOID, _: u32, _: u32, _: u32, _: PCSTR) -> HANDLE;
	pub fn CreateFileW(_: PCSTR, _: u32, _: u32, _: PVOID, _: u32, _: u32, _: HANDLE) -> HANDLE;
	pub fn CreatePipe(_: *mut HANDLE, _: *mut HANDLE, _: PVOID, _: u32) -> BOOL;
	pub fn CreateProcessW(_: PCSTR, _: PSTR, _: PVOID, _: PVOID, _: BOOL, _: u32, _: PVOID, _: PCSTR, _: PVOID, _: PVOID) -> BOOL;
	pub fn CreateThread(_: PVOID, _: u64, _: PVOID, _: PVOID, _: u32, _: &mut u32) -> HANDLE;
	pub fn DeleteFileW(_: PCSTR) -> BOOL;
	pub fn ExitProcess(_: u32);
	pub fn ExitThread(_: u32);
	pub fn ExpandEnvironmentStringsW(_: PCSTR, _: PSTR, _: u32) -> u32;
	pub fn FileTimeToSystemTime(_: PCVOID, _: PVOID) -> BOOL;
	pub fn FlushInstructionCache(_: HANDLE, _: PCVOID, _: u64) -> BOOL;
	pub fn FlushProcessWriteBuffers();
	pub fn FormatMessageW(_: u32, _: PCVOID, _: u32, _: u32, _: PSTR, _: u32, _: PVOID) -> u32;
	pub fn FreeEnvironmentStringsW(_: HANDLE) -> BOOL;
	pub fn GetEnvironmentStringsW() -> HANDLE;
	pub fn GetExitCodeProcess(_: HANDLE, _: *mut u32) -> BOOL;
	pub fn GetExitCodeThread(_: HANDLE, _: *mut u32) -> BOOL;
	pub fn GetFileAttributesW(_: PCSTR) -> u32;
	pub fn GetFileInformationByHandle(_: HANDLE, _: PVOID) -> BOOL;
	pub fn GetFileSizeEx(_: HANDLE, _: *mut i64) -> BOOL;
	pub fn GetFileType(_: HANDLE) -> u32;
	pub fn GetLargePageMinimum() -> u64;
	pub fn GetLastError() -> u32;
	pub fn GetLogicalDriveStringsW(_: u32, _: PSTR) -> u32;
	pub fn GetModuleHandleW(_: PCSTR) -> HANDLE;
	pub fn GetSystemTime(_: PVOID);
	pub fn GetSystemTimeAsFileTime(_: PVOID);
	pub fn GetSystemTimePreciseAsFileTime(_: PVOID);
	pub fn GetTempPathW(_: u32, _: PSTR) -> u32;
	pub fn GetTickCount64() -> u64;
	pub fn GlobalAlloc(_: u32, _: u64) -> HANDLE;
	pub fn GlobalFlags(_: HANDLE) -> u32;
	pub fn GlobalFree(_: HANDLE) -> HANDLE;
	pub fn GlobalLock(_: HANDLE) -> PVOID;
	pub fn GlobalMemoryStatusEx(_: PVOID) -> BOOL;
	pub fn GlobalReAlloc(_: HANDLE, _: u64, _: u32) -> HANDLE;
	pub fn GlobalSize(_: HANDLE) -> u64;
	pub fn GlobalUnlock(_: HANDLE) -> BOOL;
	pub fn LocalFree(_: HANDLE) -> HANDLE;
	pub fn LocalSize(_: HANDLE) -> u64;
	pub fn LockFile(_: HANDLE, _: u32, _: u32, _: u32, _: u32) -> BOOL;
	pub fn lstrlenW(_: PCSTR) -> i32;
	pub fn MapViewOfFile(_: HANDLE, _: u32, _: u32, _: u32, _: i64) -> PVOID;
	pub fn MoveFileW(_: PCSTR, _: PCSTR) -> BOOL;
	pub fn MulDiv(_: i32, _: i32, _: i32) -> i32;
	pub fn MultiByteToWideChar(_: u32, _: u32, _: *const u8, _: i32, _: PSTR, _: i32) -> i32;
	pub fn OutputDebugStringW(_: PCSTR);
	pub fn ReadFile(_: HANDLE, _: PVOID, _: u32, _: *mut u32, _: PVOID) -> BOOL;
	pub fn SetEndOfFile(_: HANDLE) -> BOOL;
	pub fn SetFilePointerEx(_: HANDLE, _: i64, _: *mut i64, _: u32) -> BOOL;
	pub fn SetLastError(_: u32);
	pub fn Sleep(_: u32);
	pub fn SystemTimeToFileTime(_: PCVOID, _: PVOID) -> BOOL;
	pub fn SystemTimeToTzSpecificLocalTime(_: PCVOID, _: PCVOID, _: PVOID) -> BOOL;
	pub fn UnlockFile(_: HANDLE, _: u32, _: u32, _: u32, _: u32) -> BOOL;
	pub fn UnmapViewOfFile(_: PCVOID) -> BOOL;
	pub fn VerifyVersionInfoW(_: PVOID, _: u32, _: u64) -> BOOL;
	pub fn VerSetConditionMask(_: u64, _: u32, _: u8) -> u64;
	pub fn WideCharToMultiByte(_: u32, _: u32, _: PCSTR, _: i32, _: PSTR, _: i32, _: *const u8, _: *mut BOOL) -> i32;
	pub fn WriteFile(_: HANDLE, _: PCVOID, _: u32, _: *mut u32, _: PVOID) -> BOOL;
}
