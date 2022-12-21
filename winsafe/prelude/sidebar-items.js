window.SIDEBAR_ITEMS = {"trait":[["FormattedError","A system error which can be formatted with `FormatMessage`."],["GuiChild","Any child window."],["GuiChildFocus","Any child window which can be focused."],["GuiEvents","Exposes the basic window message methods of `WindowEvents`."],["GuiNativeControl","Any native control, which can be subclassed."],["GuiNativeControlEvents","Events of a native control."],["GuiParent","Any window which can host child controls."],["GuiThread","Allows a window to spawn new threads which can return errors, and run closures in the original UI thread."],["GuiWindow","Any window. Exposes the underlying window handle."],["GuiWindowText","Any window which can get/set text."],["Handle","A native handle, implemented by all handle types."],["MsgSend","Parameters of a message which can be sent. Implemented by all undefined messages."],["MsgSendRecv","Parameters of a message which can be sent and handled (received). Implemented by WndMsg and all msg::wm messages."],["NativeBitflag","A native typed bitflag constant."],["NativeConst","A native typed constant."],["advapi_Hkey","This trait is enabled with the `advapi` feature, and provides methods for `HKEY`."],["comctl_Himagelist","This trait is enabled with the `comctl` feature, and provides methods for `HIMAGELIST`."],["comctl_Hwnd","This trait is enabled with the `comctl` feature, and provides methods for `HWND`."],["comctl_ole_Hwnd","This trait is enabled with `comctl` and `ole` features, and provides methods for `HWND`."],["comctl_shell_Himagelist","This trait is enabled with `comctl` and `shell` features, and provides methods for `HIMAGELIST`."],["dshow_IBaseFilter","This trait is enabled with the `dshow` feature, and provides methods for `IBaseFilter`."],["dshow_IEnumFilters","This trait is enabled with the `dshow` feature, and provides methods for `IEnumFilters`."],["dshow_IEnumMediaTypes","This trait is enabled with the `dshow` feature, and provides methods for `IEnumMediaTypes`."],["dshow_IFileSinkFilter","This trait is enabled with the `dshow` feature, and provides methods for `IFileSinkFilter`."],["dshow_IFilterGraph","This trait is enabled with the `dshow` feature, and provides methods for `IFilterGraph`."],["dshow_IGraphBuilder","This trait is enabled with the `dshow` feature, and provides methods for `IGraphBuilder`."],["dshow_IMFGetService","This trait is enabled with the `dshow` feature, and provides methods for `IMFGetService`."],["dshow_IMFVideoDisplayControl","This trait is enabled with the `dshow` feature, and provides methods for `IMFVideoDisplayControl`."],["dshow_IMediaControl","This trait is enabled with the `dshow` feature, and provides methods for `IMediaControl`."],["dshow_IMediaFilter","This trait is enabled with the `dshow` feature, and provides methods for `IMediaFilter`."],["dshow_IMediaSeeking","This trait is enabled with the `dshow` feature, and provides methods for `IMediaSeeking`."],["dshow_IPin","This trait is enabled with the `dshow` feature, and provides methods for `IPin`."],["dshow_gdi_IMFVideoDisplayControl","This trait is enabled with `dshow` and `gdi` features, and provides methods for `IMFVideoDisplayControl`."],["gdi_Hbitmap","This trait is enabled with the `gdi` feature, and provides methods for `HBITMAP`."],["gdi_Hbrush","This trait is enabled with the `gdi` feature, and provides methods for `HBRUSH`."],["gdi_Hdc","This trait is enabled with the `gdi` feature, and provides methods for `HDC`."],["gdi_Hfont","This trait is enabled with the `gdi` feature, and provides methods for `HFONT`."],["gdi_Hgdiobj","Any `HGDIOBJ` handle, which is the base handle for GDI objects."],["gdi_Hpen","This trait is enabled with the `gdi` feature, and provides methods for `HPEN`."],["gdi_Hrgn","This trait is enabled with the `gdi` feature, and provides methods for `HRGN`."],["gdi_ole_Hdc","This trait is enabled with `gdi` and `ole` features, and provides methods for `HDC`."],["gdi_ole_IPicture","This trait is enabled with `gdi` and `ole` features, and provides methods for `IPicture`."],["kernel_Haccesstoken","This trait is enabled with the `kernel` feature, and provides methods for `HACCESSTOKEN`."],["kernel_Hfile","This trait is enabled with the `kernel` feature, and provides methods for `HFILE`."],["kernel_Hfilemap","This trait is enabled with the `kernel` feature, and provides methods for `HFILEMAP`."],["kernel_Hfilemapview","This trait is enabled with the `kernel` feature, and provides methods for `HFILEMAPVIEW`."],["kernel_Hfindfile","This trait is enabled with the `kernel` feature, and provides methods for `HFINDFILE`."],["kernel_Hglobal","This trait is enabled with the `kernel` feature, and provides methods for `HGLOBAL`."],["kernel_Hinstance","This trait is enabled with the `kernel` feature, and provides methods for `HINSTANCE`."],["kernel_Hlocal","This trait is enabled with the `kernel` feature, and provides methods for `HLOCAL`."],["kernel_Hpipe","This trait is enabled with the `kernel` feature, and provides methods for `HPIPE`."],["kernel_Hprocess","This trait is enabled with the `kernel` feature, and provides methods for `HPROCESS`."],["kernel_Hprocesslist","This trait is enabled with the `kernel` feature, and provides methods for `HPROCESSLIST`."],["kernel_Hthread","This trait is enabled with the `kernel` feature, and provides methods for `HTHREAD`."],["kernel_Hupdatersrc","This trait is enabled with the `kernel` feature, and provides methods for `HUPDATERSRC`."],["ktm_Htransaction","This trait is enabled with the `ktm` feature, and provides methods for `HTRANSACTION`."],["msimg_Hdc","This trait is enabled with the `msimg` feature, and provides methods for `HDC`."],["ole_Hwnd","This trait is enabled with the `ole` feature, and provides methods for `HWND`."],["ole_IBindCtx","This trait is enabled with the `ole` feature, and provides methods for `IBindCtx`."],["ole_IDataObject","This trait is enabled with the `ole` feature, and provides methods for `IDataObject`."],["ole_IDropTarget","This trait is enabled with the `ole` feature, and provides methods for `IDropTarget`."],["ole_IPersist","This trait is enabled with the `ole` feature, and provides methods for `IPersist`."],["ole_IPicture","This trait is enabled with the `ole` feature, and provides methods for `IPicture`."],["ole_ISequentialStream","This trait is enabled with the `ole` feature, and provides methods for `ISequentialStream`."],["ole_IStream","`IStream` methods from `ole` feature."],["ole_IUnknown","This trait is enabled with the `ole` feature, and provides methods for `IUnknown`. It is the base trait for all COM traits."],["oleaut_IDispatch","This trait is enabled with the `oleaut` feature, and provides methods for `IDispatch`."],["oleaut_IPicture","This trait is enabled with the `oleaut` feature, and provides methods for `IDispatch`."],["oleaut_IPropertyStore","This trait is enabled with the `oleaut` feature, and provides methods for `IPropertyStore`."],["oleaut_ITypeInfo","This trait is enabled with the `oleaut` feature, and provides methods for `ITypeInfo`."],["oleaut_Variant","Methods common to `VARIANT` and `PROPVARIANT` structs."],["shell_Hdrop","This trait is enabled with the `shell` feature, and provides methods for `HDROP`."],["shell_Hwnd","This trait is enabled with the `shell` feature, and provides methods for `HWND`."],["shell_IEnumShellItems","This trait is enabled with the `shell` feature, and provides methods for `IEnumShellItems`."],["shell_IFileDialog","This trait is enabled with the `shell` feature, and provides methods for `IFileDialog`."],["shell_IFileOpenDialog","This trait is enabled with the `shell` feature, and provides methods for `IFileOpenDialog`."],["shell_IFileSaveDialog","This trait is enabled with the `shell` feature, and provides methods for `IFileSaveDialog`."],["shell_IModalWindow","This trait is enabled with the `shell` feature, and provides methods for `IModalWindow`."],["shell_IShellItem","This trait is enabled with the `shell` feature, and provides methods for `IShellItem`."],["shell_IShellItem2","This trait is enabled with the `shell` feature, and provides methods for `IShellItem2`."],["shell_IShellItemArray","This trait is enabled with the `shell` feature, and provides methods for `IShellItemArray`."],["shell_IShellLink","This trait is enabled with the `shell` feature, and provides methods for `IShellLink`."],["shell_IStream","This trait is enabled with the `shell` feature, and provides methods for `IStream`."],["shell_ITaskbarList","This trait is enabled with the `shell` feature, and provides methods for `ITaskbarList`."],["shell_ITaskbarList2","This trait is enabled with the `shell` feature, and provides methods for `ITaskbarList2`."],["shell_ITaskbarList3","This trait is enabled with the `shell` feature, and provides methods for `ITaskbarList3`."],["shell_ITaskbarList4","This trait is enabled with the `shell` feature, and provides methods for `ITaskbarList4`."],["user_Haccel","This trait is enabled with the `user` feature, and provides methods for `HACCEL`."],["user_Hcursor","This trait is enabled with the `user` feature, and provides methods for `HCURSOR`."],["user_Hdc","This trait is enabled with the `user` feature, and provides methods for `HDC`."],["user_Hdwp","This trait is enabled with the `user` feature, and provides methods for `HDWP`."],["user_Hhook","This trait is enabled with the `user` feature, and provides methods for `HHOOK`."],["user_Hicon","This trait is enabled with the `user` feature, and provides methods for `HICON`."],["user_Hinstance","This trait is enabled with the `user` feature, and provides methods for `HINSTANCE`."],["user_Hmenu","This trait is enabled with the `user` feature, and provides methods for `HMENU`."],["user_Hmonitor","This trait is enabled with the `user` feature, and provides methods for `HMONITOR`."],["user_Hprocess","This trait is enabled with the `user` feature, and provides methods for `HPROCESS`."],["user_Hwnd","This trait is enabled with the `user` feature, and provides methods for `HWND`."],["uxtheme_Htheme","This trait is enabled with the `uxtheme` feature, and provides methods for `HTHEME`."],["uxtheme_Hwnd","This trait is enabled with the `uxtheme` feature, and provides methods for `HWND`."]]};