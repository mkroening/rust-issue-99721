mod note {
    #[used]
    #[link_section = ".note.my-note"]
    static MY_NOTE: u8 = 0;

    #[inline(always)]
    pub fn dummy() {}
}

#[no_mangle]
pub extern "C" fn staticlib() {
    note::dummy();
}
