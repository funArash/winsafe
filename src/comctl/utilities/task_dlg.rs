//! Provides high-level abstractions to
//! [`TaskDialogIndirect`](crate::TaskDialogIndirect) and
//! [`HWND::TaskDialog`](crate::prelude::comctl_Hwnd::TaskDialog) functions.

use crate::co;
use crate::decl::*;
use crate::prelude::*;

/// Displays an error modal window with an OK button.
///
/// High-level abstraction over
/// [`TaskDialogIndirect`](crate::TaskDialogIndirect).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let hparent: w::HWND; // initialized somewhere
/// # let hparent = w::HWND::NULL;
///
/// w::task_dlg::error(
///     &hparent,
///     "Error",
///     Some("The error"),
///     "Operation failed to complete.",
/// )?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
pub fn error(
	hparent: &HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
) -> HrResult<()>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::OK, co::TD_ICON::ERROR)
		.map(|_| ())
}

/// Displays a warning modal window with an OK button.
///
/// High-level abstraction over
/// [`TaskDialogIndirect`](crate::TaskDialogIndirect).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let hparent: w::HWND; // initialized somewhere
/// # let hparent = w::HWND::NULL;
///
/// w::task_dlg::warn(
///     &hparent,
///     "Warning",
///     Some("The warning"),
///     "Something is not quite right.",
/// )?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
pub fn warn(
	hparent: &HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
) -> HrResult<()>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::OK, co::TD_ICON::WARNING)
		.map(|_| ())
}

/// Displays an information modal window with an OK button.
///
/// High-level abstraction over
/// [`TaskDialogIndirect`](crate::TaskDialogIndirect).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let hparent: w::HWND; // initialized somewhere
/// # let hparent = w::HWND::NULL;
///
/// w::task_dlg::info(
///     &hparent,
///     "Information",
///     Some("The information"),
///     "Operation completed successfully.",
/// )?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
pub fn info(
	hparent: &HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
) -> HrResult<()>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::OK, co::TD_ICON::INFORMATION)
		.map(|_| ())
}

/// Displays a question modal window with OK and Cancel buttons. The text of the
/// OK button can be customized.
///
/// Returns `true` if the user clicked OK.
///
/// High-level abstraction over
/// [`TaskDialogIndirect`](crate::TaskDialogIndirect).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let hparent: w::HWND; // initialized somewhere
/// # let hparent = w::HWND::NULL;
///
/// let got_ok = w::task_dlg::ok_cancel(
///     &hparent,
///     "Question",
///     Some("The question"),
///     "Do you want to proceed with this operation?",
///     Some("Proceed"),
/// )?;
///
/// if got_ok {
///     // ...
/// } else {
///     // ...
/// }
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn ok_cancel(
	hparent: &HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
	ok_text: Option<&str>,
) -> HrResult<bool>
{
	let mut btns = co::TDCBF::CANCEL;
	if ok_text.is_none() {
		btns |= co::TDCBF::OK;
	}

	generate(hparent, title, header, body, ok_text,
		btns, co::TD_ICON::WARNING)
		.map(|dlg_id| dlg_id == co::DLGID::OK)
}

/// Displays a question modal window with Yes and No buttons.
///
/// Returns `true` if the user clicked Yes.
///
/// High-level abstraction over
/// [`TaskDialogIndirect`](crate::TaskDialogIndirect).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let hparent: w::HWND; // initialized somewhere
/// # let hparent = w::HWND::NULL;
///
/// let got_yes = w::task_dlg::yes_no(
///     &hparent,
///     "Question",
///     Some("The question"),
///     "Do you want to proceed with this operation?",
/// )?;
///
/// if got_yes {
///     // ...
/// } else {
///     // ...
/// }
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn yes_no(
	hparent: &HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
) -> HrResult<bool>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::YES | co::TDCBF::NO, co::TD_ICON::WARNING)
		.map(|dlg_id| dlg_id == co::DLGID::YES)
}

/// Displays a question modal window with Yes, No and Cancel buttons.
///
/// High-level abstraction over
/// [`TaskDialogIndirect`](crate::TaskDialogIndirect).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let hparent: w::HWND; // initialized somewhere
/// # let hparent = w::HWND::NULL;
///
/// let ret_code = w::task_dlg::yes_no_cancel(
///     &hparent,
///     "Question",
///     Some("The question"),
///     "Do you want to proceed with this operation?",
/// )?;
///
/// if ret_code == co::DLGID::YES {
///     // ...
/// } else if ret_code == co::DLGID::NO {
///     // ...
/// } else {
///     // ...
/// }
/// # Ok::<_, co::HRESULT>(())
/// ```
#[must_use]
pub fn yes_no_cancel(
	hparent: &HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
) -> HrResult<co::DLGID>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::YES | co::TDCBF::NO | co::TDCBF::CANCEL, co::TD_ICON::WARNING)
}

fn generate(
	hparent: &HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
	ok_text: Option<&str>,
	btns: co::TDCBF,
	ico: co::TD_ICON,
) -> HrResult<co::DLGID>
{
	let mut ok_text_buf = WString::from_opt_str(ok_text);
	let mut custom_btns = if ok_text.is_some() {
		let mut td_btn = TASKDIALOG_BUTTON::default();
		td_btn.set_nButtonID(co::DLGID::OK.into());
		td_btn.set_pszButtonText(Some(&mut ok_text_buf));

		let mut custom_btns = Vec::with_capacity(1);
		custom_btns.push(td_btn);
		custom_btns
	} else {
		Vec::<TASKDIALOG_BUTTON>::default()
	};

	let mut tdc = TASKDIALOGCONFIG::default();
	tdc.hwndParent = unsafe { hparent.raw_copy() };
	tdc.dwFlags = co::TDF::ALLOW_DIALOG_CANCELLATION | co::TDF::POSITION_RELATIVE_TO_WINDOW;
	tdc.dwCommonButtons = btns;
	tdc.set_pszMainIcon(IconIdTdicon::Tdicon(ico));

	if ok_text.is_some() {
		tdc.set_pButtons(Some(&mut custom_btns));
	}

	let mut title_buf = WString::from_str(title);
	tdc.set_pszWindowTitle(Some(&mut title_buf));

	let mut header_buf = WString::from_opt_str(header);
	if header.is_some() {
		tdc.set_pszMainInstruction(Some(&mut header_buf));
	}

	let mut body_buf = WString::from_str(body);
	tdc.set_pszContent(Some(&mut body_buf));

	TaskDialogIndirect(&tdc, None)
		.map(|(dlg_id, _)| dlg_id)
}
