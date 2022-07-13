var sourcesIndex = {};
sourcesIndex["winsafe"] = {"name":"","dirs":[{"name":"advapi","dirs":[{"name":"handles","files":["hkey.rs","mod.rs"]}],"files":["co.rs","enums.rs","ffi.rs","funcs.rs","mod.rs","privs.rs"]},{"name":"comctl","dirs":[{"name":"handles","files":["himagelist.rs","hwnd.rs","mod.rs"]},{"name":"messages","files":["bcm.rs","cb.rs","dtm.rs","em.rs","hdm.rs","lvm.rs","mcm.rs","mod.rs","pbm.rs","sb.rs","stm.rs","tbm.rs","trbm.rs","tvm.rs","wm.rs"]}],"files":["aliases.rs","co.rs","enums.rs","ffi.rs","funcs.rs","mod.rs","privs.rs","structs.rs"]},{"name":"comctl_gdi","dirs":[{"name":"messages","files":["dtm.rs","mod.rs"]}],"files":["mod.rs","structs.rs"]},{"name":"comctl_ole","dirs":[{"name":"handles","files":["hwnd.rs","mod.rs"]},{"name":"messages","files":["lvm.rs","mod.rs","tvm.rs"]}],"files":["aliases.rs","enums.rs","ffi.rs","funcs.rs","mod.rs","structs.rs"]},{"name":"comctl_shell","dirs":[{"name":"handles","files":["himagelist.rs","mod.rs"]}],"files":["mod.rs"]},{"name":"comdlg","dirs":[{"name":"co","files":["cderr.rs","consts.rs","mod.rs"]}],"files":["aliases.rs","ffi.rs","funcs.rs","mod.rs","structs.rs"]},{"name":"dshow","dirs":[{"name":"co","files":["consts.rs","guids.rs","hresult.rs","mod.rs"]},{"name":"com_interfaces","files":["ibasefilter.rs","ienumfilters.rs","ifilesinkfilter.rs","ifiltergraph.rs","igraphbuilder.rs","imediacontrol.rs","imediafilter.rs","imediaseeking.rs","imfgetservice.rs","imfvideodisplaycontrol.rs","ipin.rs","mod.rs"]}],"files":["mod.rs","structs.rs"]},{"name":"dshow_gdi","dirs":[{"name":"handles","files":["imfvideodisplaycontrol.rs","mod.rs"]}],"files":["mod.rs"]},{"name":"gdi","dirs":[{"name":"handles","files":["hbitmap.rs","hbrush.rs","hdc.rs","hfont.rs","hgdiobj.rs","hpen.rs","hrgn.rs","mod.rs"]},{"name":"messages","files":["mod.rs","wm.rs"]}],"files":["co.rs","ffi.rs","mod.rs","privs.rs","structs.rs"]},{"name":"gdi_oleaut","dirs":[{"name":"com_interfaces","files":["ipicture.rs","mod.rs"]},{"name":"handles","files":["hdc.rs","mod.rs"]}],"files":["mod.rs","privs.rs"]},{"name":"gui","dirs":[{"name":"events","files":["base_events_proxy.rs","button_events.rs","combo_box_events.rs","date_time_picker_events.rs","edit_events.rs","func_store.rs","label_events.rs","list_box_events.rs","list_view_events.rs","mod.rs","month_calendar_events.rs","radio_group_events.rs","status_bar_events.rs","trackbar_events.rs","tree_view_events.rs","window_events.rs","window_events_all.rs"]},{"name":"native_controls","files":["base_native_control.rs","button.rs","check_box.rs","combo_box.rs","combo_box_items.rs","date_time_picker.rs","edit.rs","label.rs","list_box.rs","list_box_items.rs","list_view.rs","list_view_columns.rs","list_view_item.rs","list_view_items.rs","mod.rs","month_calendar.rs","progress_bar.rs","radio_button.rs","radio_group.rs","status_bar.rs","status_bar_parts.rs","trackbar.rs","tree_view.rs","tree_view_item.rs","tree_view_items.rs"]}],"files":["base.rs","dlg_base.rs","dlg_control.rs","dlg_main.rs","dlg_modal.rs","gui_traits.rs","layout_arranger.rs","mod.rs","privs.rs","raw_base.rs","raw_control.rs","raw_main.rs","raw_modal.rs","runtime_error.rs","very_unsafe_cell.rs","window_control.rs","window_main.rs","window_modal.rs"]},{"name":"kernel","dirs":[{"name":"co","files":["consts.rs","error.rs","mod.rs"]},{"name":"handles","files":["haccesstoken.rs","handle.rs","hevent.rs","hfile.rs","hfilemap.rs","hfilemapview.rs","hfindfile.rs","hglobal.rs","hinstance.rs","hlocal.rs","hpipe.rs","hprocess.rs","hprocesslist.rs","hthread.rs","hupdatesrc.rs","mod.rs"]},{"name":"utilities","files":["file.rs","file_mapped.rs","ini.rs","mod.rs","path.rs","w_string.rs"]}],"files":["aliases.rs","co_traits.rs","enums.rs","ffi.rs","funcs.rs","mod.rs","privs.rs","structs.rs"]},{"name":"macros","files":["com.rs","consts.rs","ffis.rs","gui_events.rs","handles.rs","messages.rs","mod.rs","seq_ids.rs","structs.rs"]},{"name":"msimg","dirs":[{"name":"handles","files":["hdc.rs","mod.rs"]}],"files":["ffi.rs","mod.rs"]},{"name":"ole","dirs":[{"name":"co","files":["consts.rs","guids.rs","hresult.rs","mod.rs"]},{"name":"com_interfaces","files":["iunknown.rs","mod.rs"]}],"files":["aliases.rs","ffi.rs","funcs.rs","mod.rs","privs.rs","structs.rs"]},{"name":"oleaut","dirs":[{"name":"com_interfaces","files":["idispatch.rs","ipicture.rs","ipropertystore.rs","itypeinfo.rs","mod.rs"]},{"name":"structs","files":["bstr.rs","mod.rs","others.rs","variant.rs"]}],"files":["co.rs","ffi.rs","funcs.rs","mod.rs"]},{"name":"shell","dirs":[{"name":"co","files":["consts.rs","guids.rs","mod.rs"]},{"name":"com_interfaces","files":["ibindctx.rs","ienumshellitems.rs","ifiledialog.rs","ifileopendialog.rs","ifilesavedialog.rs","imodalwindow.rs","ipersist.rs","isequentialstream.rs","ishellitem.rs","ishellitemarray.rs","ishelllink.rs","istream.rs","itaskbarlist.rs","itaskbarlist2.rs","itaskbarlist3.rs","itaskbarlist4.rs","mod.rs"]},{"name":"handles","files":["hdrop.rs","hwnd.rs","mod.rs"]},{"name":"messages","files":["mod.rs","wm.rs"]}],"files":["ffi.rs","funcs.rs","mod.rs","privs.rs","structs.rs"]},{"name":"user","dirs":[{"name":"handles","files":["haccel.rs","hcursor.rs","hdc.rs","hdwp.rs","hhook.rs","hicon.rs","hinstance.rs","hmenu.rs","hmonitor.rs","hprocess.rs","hwnd.rs","mod.rs"]},{"name":"messages","files":["bm.rs","cb.rs","em.rs","lb.rs","mod.rs","wm.rs","wnd_msg.rs"]}],"files":["aliases.rs","co.rs","enums.rs","ffi.rs","funcs.rs","mod.rs","msg_traits.rs","privs.rs","structs.rs"]},{"name":"uxtheme","dirs":[{"name":"co","files":["consts.rs","mod.rs","vs.rs"]},{"name":"handles","files":["htheme.rs","hwnd.rs","mod.rs"]}],"files":["ffi.rs","funcs.rs","mod.rs"]},{"name":"version","files":["co.rs","ffi.rs","funcs.rs","mod.rs","resource_info.rs","structs.rs"]}],"files":["ffi_types.rs","lib.rs"]};
createSourceSidebar();
