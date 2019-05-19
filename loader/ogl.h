
#include <stdint.h>

// Defined through the preprocessor so that bindgen doesn't seen the khronos_* types.

#define khronos_float_t     float

#define khronos_int8_t      int8_t
#define khronos_int16_t     int16_t
#define khronos_int32_t     int32_t
#define khronos_int64_t     int64_t

#define khronos_uint8_t     uint8_t
#define khronos_uint16_t    uint16_t
#define khronos_uint64_t    uint64_t

#define khronos_intptr_t    intptr_t
#define khronos_size_t      size_t
#define khronos_ssize_t     size_t

#include "glad_46.h"

typedef struct GlFuncs {
    /// OpenGL32 module - must outlive the pointers in this struct.
    uint64_t                hOpenGL;

    // Shaders
    PFNGLCREATESHADERPROC   pfn_glCreateShader;
    PFNGLSHADERSOURCEPROC   pfn_glShaderSource;
    PFNGLCOMPILESHADERPROC  pfn_glCompileShader;
    PFNGLATTACHSHADERPROC   pfn_glAttachShader;

    // Shader Programs
    PFNGLCREATEPROGRAMPROC  pfn_glCreateProgram;
    PFNGLLINKPROGRAMPROC    pfn_glLinkProgram;
    PFNGLUSEPROGRAMPROC     pfn_glUseProgram;

    // Buffers and Presenting
    PFNGLCLEARCOLORPROC     pfn_glClearColor;
    PFNGLCLEARPROC          pfn_glClear;
    PFNGLCLEARDEPTHPROC     pfn_glClearDepth;
} GlFuncs;

/// Loads required OpenGL functions into pFns.
///
/// Return value == 0  if all required functions are loaded.
/// Return value <= -1 if someone went wrong - e.g. invalid arguments.
/// Return value >=  1 if 1 or more functions failed to load.
///
/// Functions are marked as required in the `GlFuncs` struct definition.
int ogl_load(GlFuncs* pFns);
