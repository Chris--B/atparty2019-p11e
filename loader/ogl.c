
#include "ogl.h"
#include <Windows.h>

typedef void (*PFN)(void);
// Either:
//      return a valid function pointer and do not modify pError,
//  OR
//      return NULL and increment *pError
// Crashes if pError is NULL.
static PFN load_fn(const char* pName, int* pError)
{
    PFN pfn = wglGetProcAddress(pName);
    if (pfn == NULL) {
        *pError += 1;
        return NULL;
    } else {
        return pfn;
    }
}

int ogl_load(GlFuncs* pFns)
{
    if (pFns == NULL) {
        return -1;
    }
    memset(pFns, 0, sizeof(*pFns));

    int error = 0;

    pFns->pfn_glCreateProgram = (PFNGLCREATEPROGRAMPROC) load_fn("glCreateProgram", &error);
    pFns->pfn_glCreateShader  = (PFNGLCREATESHADERPROC)  load_fn("glCreateShader",  &error);
    pFns->pfn_glShaderSource  = (PFNGLSHADERSOURCEPROC)  load_fn("glShaderSource",  &error);
    pFns->pfn_glCompileShader = (PFNGLCOMPILESHADERPROC) load_fn("glCompileShader", &error);
    pFns->pfn_glAttachShader  = (PFNGLATTACHSHADERPROC)  load_fn("glAttachShader",  &error);
    pFns->pfn_glLinkProgram   = (PFNGLLINKPROGRAMPROC)   load_fn("glLinkProgram",   &error);
    pFns->pfn_glUseProgram    = (PFNGLUSEPROGRAMPROC)    load_fn("glUseProgram",    &error);

    return error;
}
