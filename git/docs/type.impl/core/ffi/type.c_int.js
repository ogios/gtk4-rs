(function() {
    var type_impls = Object.fromEntries([["gdk4_sys",[]],["gdk4_win32_sys",[]],["gdk4_x11_sys",[]],["gsk4_sys",[]],["gtk4_sys",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[15,22,20,16,16]}