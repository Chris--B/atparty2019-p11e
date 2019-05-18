
#include <stdint.h>

typedef float    khronos_float_t;

typedef int8_t   khronos_int8_t;
typedef int16_t  khronos_int16_t;
typedef int32_t  khronos_int32_t;
typedef int64_t  khronos_int64_t;

typedef uint8_t  khronos_uint8_t;
typedef uint16_t khronos_uint16_t;
typedef uint64_t khronos_uint64_t;

typedef intptr_t khronos_intptr_t;
typedef size_t   khronos_size_t;
typedef size_t   khronos_ssize_t;

#include "glad_46.h"

typedef struct GlFuncs {
    PFNGLCREATEPROGRAMPROC   pfn_glCreateProgram;
    PFNGLCREATESHADERPROC    pfn_glCreateShader;
    PFNGLSHADERSOURCEPROC    pfn_glShaderSource;
    PFNGLCOMPILESHADERPROC   pfn_glCompileShader;
    PFNGLATTACHSHADERPROC    pfn_glAttachShader;
    PFNGLLINKPROGRAMPROC     pfn_glLinkProgram;
    PFNGLUSEPROGRAMPROC      pfn_glUseProgram;
} GlFuncs;

/// Loads required OpenGL functions into pFns.
///
/// Return value == 0  if all required functions are loaded.
/// Return value <= -1 if someone went wrong - e.g. invalid arguments.
/// Return value >=  1 if 1 or more functions failed to load.
///
/// Functions are marked as required in the `GlFuncs` struct definition.
int ogl_load(GlFuncs* pFns);
