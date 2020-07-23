/*
    gcw0-toolchain's libm doesn't export fminf/fmaxf.
    Based on the C and C++ headers, it's expected that the compiler would directly emit __builtin_{fminf,fmaxf} via stdlib.
    Since Rust's LLVM is trying to use libm for these symbols anyways, create our own lib
*/

float fminf(float left, float right) { return __builtin_fmin(left, right); }
float fmaxf(float left, float right) { return __builtin_fmax(left, right); }
