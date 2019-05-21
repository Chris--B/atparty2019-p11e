// Copied over manually because I want the values in hex.
// BUFFER_BIT
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;

// Shaders
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;

// Shader Info Status
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_LINK_STATUS: u32 = 0x8B82;

// Primitive Topologies
pub const _GL_POINTS: u32 = 0x0000;
pub const _GL_LINES: u32 = 0x0001;
pub const _GL_LINE_LOOP: u32 = 0x0002;
pub const _GL_LINE_STRIP: u32 = 0x0003;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const _GL_TRIANGLE_STRIP: u32 = 0x0005;
pub const _GL_TRIANGLE_FAN: u32 = 0x0006;

// automatically generated by rust-bindgen

pub type GLenum = libc::c_uint;
pub type GLbitfield = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = f32;
pub type GLdouble = f64;
pub type GLchar = libc::c_char;
pub type PFNGLCLEARPROC = unsafe extern "C" fn(mask: GLbitfield);
pub type PFNGLCLEARCOLORPROC = unsafe extern "C" fn(
    red: GLfloat,
    green: GLfloat,
    blue: GLfloat,
    alpha: GLfloat,
);
pub type PFNGLCLEARDEPTHPROC = unsafe extern "C" fn(depth: GLdouble);
pub type PFNGLVIEWPORTPROC = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLDRAWARRAYSPROC =
    unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei);
pub type PFNGLATTACHSHADERPROC =
    unsafe extern "C" fn(program: GLuint, shader: GLuint);
pub type PFNGLCOMPILESHADERPROC = unsafe extern "C" fn(shader: GLuint);
pub type PFNGLCREATEPROGRAMPROC = unsafe extern "C" fn() -> GLuint;
pub type PFNGLCREATESHADERPROC = unsafe extern "C" fn(type_: GLenum) -> GLuint;
pub type PFNGLGETPROGRAMIVPROC =
    unsafe extern "C" fn(program: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETPROGRAMINFOLOGPROC = unsafe extern "C" fn(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
);
pub type PFNGLGETSHADERIVPROC =
    unsafe extern "C" fn(shader: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSHADERINFOLOGPROC = unsafe extern "C" fn(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
);
pub type PFNGLLINKPROGRAMPROC = unsafe extern "C" fn(program: GLuint);
pub type PFNGLSHADERSOURCEPROC = unsafe extern "C" fn(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
);
pub type PFNGLUSEPROGRAMPROC = unsafe extern "C" fn(program: GLuint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlFuncs {
    /// OpenGL32 module - must outlive the pointers in this struct.
    pub hOpenGL: u64,
    pub CreateShader: PFNGLCREATESHADERPROC,
    pub ShaderSource: PFNGLSHADERSOURCEPROC,
    pub CompileShader: PFNGLCOMPILESHADERPROC,
    pub AttachShader: PFNGLATTACHSHADERPROC,
    pub CreateProgram: PFNGLCREATEPROGRAMPROC,
    pub LinkProgram: PFNGLLINKPROGRAMPROC,
    pub UseProgram: PFNGLUSEPROGRAMPROC,
    pub ClearColor: PFNGLCLEARCOLORPROC,
    pub Clear: PFNGLCLEARPROC,
    pub ClearDepth: PFNGLCLEARDEPTHPROC,
    pub DrawArrays: PFNGLDRAWARRAYSPROC,
    pub GetShaderiv: PFNGLGETSHADERIVPROC,
    pub GetShaderInfoLog: PFNGLGETSHADERINFOLOGPROC,
    pub GetProgramiv: PFNGLGETPROGRAMIVPROC,
    pub GetProgramInfoLog: PFNGLGETPROGRAMINFOLOGPROC,
    pub Viewport: PFNGLVIEWPORTPROC,
}
extern "C" {
    /// Loads required OpenGL functions into pFns.
    ///
    /// Return value == 0  if all required functions are loaded.
    /// Return value <= -1 if someone went wrong - e.g. invalid arguments.
    /// Return value >=  1 if 1 or more functions failed to load.
    ///
    /// Functions are marked as required in the `GlFuncs` struct definition.
    pub fn ogl_load(pFns: *mut GlFuncs) -> libc::c_int;
}
