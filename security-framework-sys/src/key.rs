use core_foundation_sys::base::CFTypeID;

extern {
    pub fn SecKeyGetTypeID() -> CFTypeID;
}
