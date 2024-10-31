// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, DirectionType,
    LayoutManager, NotebookPage, NotebookTab, Overflow, PackType, PositionType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkNotebook")]
    pub struct Notebook(Object<ffi::GtkNotebook>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_notebook_get_type(),
    }
}

impl Notebook {
    #[doc(alias = "gtk_notebook_new")]
    pub fn new() -> Notebook {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_notebook_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Notebook`] objects.
    ///
    /// This method returns an instance of [`NotebookBuilder`](crate::builders::NotebookBuilder) which can be used to create [`Notebook`] objects.
    pub fn builder() -> NotebookBuilder {
        NotebookBuilder::new()
    }

    #[doc(alias = "gtk_notebook_detach_tab")]
    pub fn detach_tab(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_notebook_detach_tab(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_get_action_widget")]
    #[doc(alias = "get_action_widget")]
    pub fn action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_action_widget(
                self.to_glib_none().0,
                pack_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_group_name")]
    #[doc(alias = "get_group_name")]
    #[doc(alias = "group-name")]
    pub fn group_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_notebook_get_group_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_menu_label")]
    #[doc(alias = "get_menu_label")]
    pub fn menu_label(&self, child: &impl IsA<Widget>) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_menu_label_text")]
    #[doc(alias = "get_menu_label_text")]
    pub fn menu_label_text(&self, child: &impl IsA<Widget>) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self, child: &impl IsA<Widget>) -> NotebookPage {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_pages")]
    #[doc(alias = "get_pages")]
    pub fn pages(&self) -> gio::ListModel {
        unsafe { from_glib_full(ffi::gtk_notebook_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_scrollable")]
    #[doc(alias = "get_scrollable")]
    #[doc(alias = "scrollable")]
    pub fn is_scrollable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_scrollable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_show_border")]
    #[doc(alias = "get_show_border")]
    #[doc(alias = "show-border")]
    pub fn shows_border(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_show_border(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_show_tabs")]
    #[doc(alias = "get_show_tabs")]
    #[doc(alias = "show-tabs")]
    pub fn shows_tabs(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_show_tabs(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_tab_detachable")]
    #[doc(alias = "get_tab_detachable")]
    pub fn tab_is_detachable(&self, child: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_detachable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_tab_label")]
    #[doc(alias = "get_tab_label")]
    pub fn tab_label(&self, child: &impl IsA<Widget>) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_tab_label_text")]
    #[doc(alias = "get_tab_label_text")]
    pub fn tab_label_text(&self, child: &impl IsA<Widget>) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_tab_pos")]
    #[doc(alias = "get_tab_pos")]
    #[doc(alias = "tab-pos")]
    pub fn tab_pos(&self) -> PositionType {
        unsafe { from_glib(ffi::gtk_notebook_get_tab_pos(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_tab_reorderable")]
    #[doc(alias = "get_tab_reorderable")]
    pub fn tab_is_reorderable(&self, child: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_reorderable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_next_page")]
    pub fn next_page(&self) {
        unsafe {
            ffi::gtk_notebook_next_page(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_popup_disable")]
    pub fn popup_disable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_disable(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_popup_enable")]
    pub fn popup_enable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_enable(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_prev_page")]
    pub fn prev_page(&self) {
        unsafe {
            ffi::gtk_notebook_prev_page(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_set_action_widget")]
    pub fn set_action_widget(&self, widget: &impl IsA<Widget>, pack_type: PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                pack_type.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_group_name")]
    #[doc(alias = "group-name")]
    pub fn set_group_name(&self, group_name: Option<&str>) {
        unsafe {
            ffi::gtk_notebook_set_group_name(self.to_glib_none().0, group_name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_set_menu_label")]
    pub fn set_menu_label(&self, child: &impl IsA<Widget>, menu_label: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_notebook_set_menu_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_menu_label_text")]
    pub fn set_menu_label_text(&self, child: &impl IsA<Widget>, menu_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                menu_text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_scrollable")]
    #[doc(alias = "scrollable")]
    pub fn set_scrollable(&self, scrollable: bool) {
        unsafe {
            ffi::gtk_notebook_set_scrollable(self.to_glib_none().0, scrollable.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_show_border")]
    #[doc(alias = "show-border")]
    pub fn set_show_border(&self, show_border: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_border(self.to_glib_none().0, show_border.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_show_tabs")]
    #[doc(alias = "show-tabs")]
    pub fn set_show_tabs(&self, show_tabs: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_tabs(self.to_glib_none().0, show_tabs.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_detachable")]
    pub fn set_tab_detachable(&self, child: &impl IsA<Widget>, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                detachable.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_label")]
    pub fn set_tab_label(&self, child: &impl IsA<Widget>, tab_label: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_notebook_set_tab_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_label_text")]
    pub fn set_tab_label_text(&self, child: &impl IsA<Widget>, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_pos")]
    #[doc(alias = "tab-pos")]
    pub fn set_tab_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_notebook_set_tab_pos(self.to_glib_none().0, pos.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_reorderable")]
    pub fn set_tab_reorderable(&self, child: &impl IsA<Widget>, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                reorderable.into_glib(),
            );
        }
    }

    #[doc(alias = "enable-popup")]
    pub fn enables_popup(&self) -> bool {
        ObjectExt::property(self, "enable-popup")
    }

    #[doc(alias = "enable-popup")]
    pub fn set_enable_popup(&self, enable_popup: bool) {
        ObjectExt::set_property(self, "enable-popup", enable_popup)
    }

    #[doc(alias = "change-current-page")]
    pub fn connect_change_current_page<F: Fn(&Self, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn change_current_page_trampoline<
            F: Fn(&Notebook, i32) -> bool + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            page: std::ffi::c_int,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), page).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-current-page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    change_current_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_change_current_page(&self, page: i32) -> bool {
        self.emit_by_name("change-current-page", &[&page])
    }

    #[doc(alias = "create-window")]
    pub fn connect_create_window<F: Fn(&Self, &Widget) -> Option<Notebook> + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_window_trampoline<
            F: Fn(&Notebook, &Widget) -> Option<Notebook> + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            page: *mut ffi::GtkWidget,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::GtkNotebook {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page)) /*Not checked*/
                .to_glib_none()
                .0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-window\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    create_window_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "focus-tab")]
    pub fn connect_focus_tab<F: Fn(&Self, NotebookTab) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn focus_tab_trampoline<
            F: Fn(&Notebook, NotebookTab) -> bool + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            tab: ffi::GtkNotebookTab,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(tab)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-tab\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    focus_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_focus_tab(&self, tab: NotebookTab) -> bool {
        self.emit_by_name("focus-tab", &[&tab])
    }

    #[doc(alias = "move-focus-out")]
    pub fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_focus_out_trampoline<
            F: Fn(&Notebook, DirectionType) + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            direction: ffi::GtkDirectionType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(direction))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-focus-out\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    move_focus_out_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_focus_out(&self, direction: DirectionType) {
        self.emit_by_name::<()>("move-focus-out", &[&direction]);
    }

    #[doc(alias = "page-added")]
    pub fn connect_page_added<F: Fn(&Self, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_added_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            child: *mut ffi::GtkWidget,
            page_num: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    page_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-removed")]
    pub fn connect_page_removed<F: Fn(&Self, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_removed_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            child: *mut ffi::GtkWidget,
            page_num: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    page_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-reordered")]
    pub fn connect_page_reordered<F: Fn(&Self, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_reordered_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            child: *mut ffi::GtkWidget,
            page_num: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-reordered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    page_reordered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reorder-tab")]
    pub fn connect_reorder_tab<F: Fn(&Self, DirectionType, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn reorder_tab_trampoline<
            F: Fn(&Notebook, DirectionType, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            direction: ffi::GtkDirectionType,
            move_to_last: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(direction),
                from_glib(move_to_last),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reorder-tab\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    reorder_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_reorder_tab(&self, direction: DirectionType, move_to_last: bool) -> bool {
        self.emit_by_name("reorder-tab", &[&direction, &move_to_last])
    }

    #[doc(alias = "select-page")]
    pub fn connect_select_page<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn select_page_trampoline<F: Fn(&Notebook, bool) -> bool + 'static>(
            this: *mut ffi::GtkNotebook,
            move_focus: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(move_focus)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"select-page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    select_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_select_page(&self, move_focus: bool) -> bool {
        self.emit_by_name("select-page", &[&move_focus])
    }

    #[doc(alias = "switch-page")]
    pub fn connect_switch_page<F: Fn(&Self, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn switch_page_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            page: *mut ffi::GtkWidget,
            page_num: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"switch-page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    switch_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "enable-popup")]
    pub fn connect_enable_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_popup_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-popup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enable_popup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "group-name")]
    pub fn connect_group_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_name_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::group-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_group_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page")]
    pub fn connect_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pages")]
    pub fn connect_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pages\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scrollable")]
    pub fn connect_scrollable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scrollable_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scrollable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_scrollable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-border")]
    pub fn connect_show_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_border_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-border\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_border_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-tabs")]
    pub fn connect_show_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_tabs_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-tabs\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_tabs_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tab-pos")]
    pub fn connect_tab_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_pos_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tab-pos\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tab_pos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Notebook`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct NotebookBuilder {
    builder: glib::object::ObjectBuilder<'static, Notebook>,
}

impl NotebookBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn enable_popup(self, enable_popup: bool) -> Self {
        Self {
            builder: self.builder.property("enable-popup", enable_popup),
        }
    }

    pub fn group_name(self, group_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("group-name", group_name.into()),
        }
    }

    pub fn page(self, page: i32) -> Self {
        Self {
            builder: self.builder.property("page", page),
        }
    }

    pub fn scrollable(self, scrollable: bool) -> Self {
        Self {
            builder: self.builder.property("scrollable", scrollable),
        }
    }

    pub fn show_border(self, show_border: bool) -> Self {
        Self {
            builder: self.builder.property("show-border", show_border),
        }
    }

    pub fn show_tabs(self, show_tabs: bool) -> Self {
        Self {
            builder: self.builder.property("show-tabs", show_tabs),
        }
    }

    pub fn tab_pos(self, tab_pos: PositionType) -> Self {
        Self {
            builder: self.builder.property("tab-pos", tab_pos),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Notebook`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Notebook {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
