#![allow(non_snake_case)]

use crate::co;
use crate::ffi::{HANDLE, kernel32};
use crate::funcs::GetLastError;

handle_type! {
	/// Handle to a
	/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
	/// Exposes methods.
	HLOCAL
}

impl HLOCAL {
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	pub fn LocalFree(self) -> Result<(), co::ERROR> {
		match ptr_as_opt!(kernel32::LocalFree(self.0)) {
			Some(_) => Err(GetLastError()),
			None => Ok(()),
		}
	}
}