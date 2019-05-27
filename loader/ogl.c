
#include "ogl.h"
#include <Windows.h>

typedef void (*PFN)(void);

// Either:
//      return a valid function pointer and do not modify pError,
//  OR
//      return NULL and increment *pError
// Crashes if pError is NULL.
static PFN load_fn(HMODULE hOpenGL, int* pError, const char* pName)
{
    PFN pfn = (PFN)wglGetProcAddress(pName);

    if (pfn == NULL) {
        // OpenGL 1.x/2.x functions may not load from wglGetProcAddress,
        // so load them through OpenGL32 directly.
        pfn = (PFN)GetProcAddress(hOpenGL, pName);
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
    HMODULE hOpenGL = LoadLibraryA("OpenGL32");
    if (hOpenGL == NULL) {
        return -2;
    }
    pFns->hOpenGL = (uint64_t)hOpenGL;
    int error = 0;

    // Shaders
    pFns->CreateShader  = (PFNGLCREATESHADERPROC)   load_fn(hOpenGL, &error, "glCreateShader");
    pFns->ShaderSource  = (PFNGLSHADERSOURCEPROC)   load_fn(hOpenGL, &error, "glShaderSource");
    pFns->CompileShader = (PFNGLCOMPILESHADERPROC)  load_fn(hOpenGL, &error, "glCompileShader");
    pFns->AttachShader  = (PFNGLATTACHSHADERPROC)   load_fn(hOpenGL, &error, "glAttachShader");

    // Shader Programs
    pFns->CreateProgram = (PFNGLCREATEPROGRAMPROC)  load_fn(hOpenGL, &error, "glCreateProgram");
    pFns->LinkProgram   = (PFNGLLINKPROGRAMPROC)    load_fn(hOpenGL, &error, "glLinkProgram");
    pFns->UseProgram    = (PFNGLUSEPROGRAMPROC)     load_fn(hOpenGL, &error, "glUseProgram");

    pFns->UniformMatrix4fv = (PFNGLUNIFORMMATRIX4FVPROC) load_fn(hOpenGL, &error, "glUniformMatrix4fv");
    pFns->Uniform1f        = (PFNGLUNIFORM1FPROC)        load_fn(hOpenGL, &error, "glUniform1f");

    // Buffers and Presenting
    pFns->ClearColor = (PFNGLCLEARCOLORPROC)        load_fn(hOpenGL, &error, "glClearColor");
    pFns->Clear      = (PFNGLCLEARPROC)             load_fn(hOpenGL, &error, "glClear");
    pFns->ClearDepth = (PFNGLCLEARDEPTHPROC)        load_fn(hOpenGL, &error, "glClearDepth");

    // Drawing Geometry
    pFns->DrawArrays = (PFNGLDRAWARRAYSPROC)        load_fn(hOpenGL, &error, "glDrawArrays");

    // Debug Only
    #if P11E_DEVBUILD
        pFns->GetShaderiv       = (PFNGLGETSHADERIVPROC)        load_fn(hOpenGL, &error, "glGetShaderiv");
        pFns->GetShaderInfoLog  = (PFNGLGETSHADERINFOLOGPROC)   load_fn(hOpenGL, &error, "glGetShaderInfoLog");
        pFns->GetProgramiv      = (PFNGLGETPROGRAMIVPROC)       load_fn(hOpenGL, &error, "glGetProgramiv");
        pFns->GetProgramInfoLog = (PFNGLGETPROGRAMINFOLOGPROC)  load_fn(hOpenGL, &error, "glGetProgramInfoLog");
        pFns->Viewport          = (PFNGLVIEWPORTPROC)           load_fn(hOpenGL, &error, "glViewport");
    #endif

    return error;
}
