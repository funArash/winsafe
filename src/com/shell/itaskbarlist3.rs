#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{ComVtbl, HWND, IID};
use crate::co::ERROR;
use crate::shell::{co, ITaskbarList2, ITaskbarList2Vtbl};

type PPVtbl = *const *const ITaskbarList3Vtbl;

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
#[repr(C)]
pub struct ITaskbarList3Vtbl {
	iTaskbarList2Vtbl: ITaskbarList2Vtbl,
	SetProgressValue: fn(PPVtbl, *const c_void, u64, u64) -> u32,
	SetProgressState: fn(PPVtbl, *const c_void, u32) -> u32,
	RegisterTab: fn(PPVtbl, *const c_void, *const c_void) -> u32,
	UnregisterTab: fn(PPVtbl, *const c_void) -> u32,
	SetTabOrder: fn(PPVtbl, *const c_void, *const c_void) -> u32,
	SetTabActive: fn(PPVtbl, *const c_void, *const c_void, u32) -> u32,
	ThumbBarAddButtons: fn(PPVtbl, *const c_void, u32, *const c_void) -> u32,
	ThumbBarUpdateButtons: fn(PPVtbl, *const c_void, u32, *const c_void) -> u32,
	ThumbBarSetImageList: fn(PPVtbl, *const c_void, *const c_void) -> u32,
	SetOverlayIcon: fn(PPVtbl, *const c_void, *const c_void, *const u16) -> u32,
	SetThumbnailTooltip: fn(PPVtbl, *const c_void, *const u16) -> u32,
	SetThumbnailClip: fn(PPVtbl, *const c_void, *const c_void) -> u32,
}

impl ComVtbl for ITaskbarList3Vtbl {
	fn IID() -> IID {
		IID::new(0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf)
	}
}

//------------------------------------------------------------------------------

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList2`](crate::shell::ITaskbarList2);
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// let mut obj: w::shell::ITaskbarList3 = w::CoCreateInstance(
///   &w::shell::clsid::TaskbarList,
///   None,
///   w::co::CLSCTX::INPROC_SERVER,
/// );
/// ```
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
pub struct ITaskbarList3 {
	/// Methods of base interface
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2).
	pub ITaskbarList2: ITaskbarList2,
}

impl From<PPVtbl> for ITaskbarList3 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl) -> Self {
		Self {
			ITaskbarList2: ITaskbarList2::from(ppv as *const *const ITaskbarList2Vtbl),
		}
	}
}

impl ITaskbarList3 {
	/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	pub fn RegisterTab(
		&self, hwndTab: HWND, hwndMDI: HWND) -> Result<(), ERROR>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match ERROR::from(
				((*(*ppv)).RegisterTab)(ppv, hwndTab.as_ptr(), hwndMDI.as_ptr()),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	pub fn SetProgressState(&self, hwnd: HWND, tbpfFlags: co::TBPF) -> Result<(), ERROR> {
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match ERROR::from(
				((*(*ppv)).SetProgressState)(
					ppv, hwnd.as_ptr(), tbpfFlags.into(),
				),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	pub fn SetProgressValue(
		&self, hwnd: HWND,
		ullCompleted: u64, ullTotal: u64) -> Result<(), ERROR>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match ERROR::from(
				((*(*ppv)).SetProgressValue)(
					ppv, hwnd.as_ptr(), ullCompleted, ullTotal,
				),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	pub fn SetTabActive(
		&self, hwndTab: HWND, hwndMDI: HWND) -> Result<(), ERROR>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match ERROR::from(
				((*(*ppv)).SetTabActive)(ppv, hwndTab.as_ptr(), hwndMDI.as_ptr(), 0),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	pub fn SetTabOrder(
		&self, hwndTab: HWND, hwndInsertBefore: HWND) -> Result<(), ERROR>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match ERROR::from(
				((*(*ppv)).SetTabOrder)(
					ppv, hwndTab.as_ptr(), hwndInsertBefore.as_ptr(),
				),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}
}