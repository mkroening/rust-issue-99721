fn main() {
    extern "C" {
        fn staticlib();
    }

    unsafe {
        staticlib();
    }
}
