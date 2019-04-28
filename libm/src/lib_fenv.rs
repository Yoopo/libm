use libc::{c_float, c_int, c_long, c_longlong, c_ulong};

/* Dummy functions for archs lacking fenv implementation */
// FIXME: implement function

const FE_ALL_EXCEPT: c_int = 0;
const FE_TONEAREST: c_int = 0;

type fexcept_t = c_ulong;

#[repr(C)]
pub struct fenv_t {
    __cw: c_ulong,
}

// todo: how to expose from rust may need fenv.h
//#define FE_DFL_ENV  ((const fenv_t *) -1)

#[no_mangle]
pub extern "C" fn feclearexcept(_mask: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn feraiseexcept(_mask: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fetestexcept(_mask: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fegetround() -> c_int {
    FE_TONEAREST
}

#[no_mangle]
extern "C" fn __fesetround(_r: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fegetenv(envp: *const fenv_t) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fesetenv(envp: *const fenv_t) -> c_int {
    0
}


// common fenv function may need to be moved in impl ?
// TODO: move it to env ?
/* __fesetround wrapper for arch independent argument check */
#[no_mangle]
pub extern "C" fn fesetround( r: c_int)  -> c_int {__fesetround(r)}
/*
pub fn fesetround( r: c_int)  -> c_int {
	if (r != FE_TONEAREST
#ifdef FE_DOWNWARD
		&& r != FE_DOWNWARD
#endif
#ifdef FE_UPWARD
		&& r != FE_UPWARD
#endif
#ifdef FE_TOWARDZERO
		&& r != FE_TOWARDZERO
#endif
	)
		return -1;
	return __fesetround(r);
}
*/
