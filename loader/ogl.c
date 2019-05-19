
#include "ogl.h"
#include <Windows.h>

typedef void (*PFN)(void);

// Either:
//      return a valid function pointer and do not modify pError,
//  OR
//      return NULL and increment *pError
// Crashes if pError is NULL.
static PFN load_fn(const char* pName, HMODULE hOpenGL, int* pError)
{
    PFN pfn = wglGetProcAddress(pName);

    if (pfn == NULL) {
        // OpenGL 1.x/2.x functions may not load from wglGetProcAddress,
        // so load them through OpenGL32 directly.
        pfn = GetProcAddress(hOpenGL, pName);
    }

    if (pfn == NULL) {
        *pError += 1;
    }

    return pfn;
}

int ogl_load(GlFuncs* pFns)
{
    if (pFns == NULL) {
        return -1;
    }

    memset(pFns, 0, sizeof(*pFns));

    // This handle is meant to be leaked.
    // No `ogl_unload` is provided. Just exit the process.
    pFns->hOpenGL = LoadLibraryA("OpenGL32");
    if (pFns->hOpenGL == NULL) {
        return -2;
    }

    int error = 0;

    #define LOAD(NAME) load_fn((NAME), pFns->hOpenGL, &error);

    // Shaders
    pFns->CreateShader  = (PFNGLCREATESHADERPROC)   LOAD("glCreateShader");
    pFns->ShaderSource  = (PFNGLSHADERSOURCEPROC)   LOAD("glShaderSource");
    pFns->CompileShader = (PFNGLCOMPILESHADERPROC)  LOAD("glCompileShader");
    pFns->AttachShader  = (PFNGLATTACHSHADERPROC)   LOAD("glAttachShader");

    // Shader Programs
    pFns->CreateProgram = (PFNGLCREATEPROGRAMPROC)  LOAD("glCreateProgram");
    pFns->LinkProgram   = (PFNGLLINKPROGRAMPROC)    LOAD("glLinkProgram");
    pFns->UseProgram    = (PFNGLUSEPROGRAMPROC)     LOAD("glUseProgram");

    // Buffers and Presenting
    pFns->ClearColor    = (PFNGLCLEARCOLORPROC)     LOAD("glClearColor");
    pFns->Clear         = (PFNGLCLEARPROC)          LOAD("glClear");
    pFns->ClearDepth    = (PFNGLCLEARDEPTHPROC)     LOAD("glClearDepth");

    #undef LOAD

    return error;
}
