extern crate glium;
use std::collections::HashMap;
use std::cell::RefCell;
use std::ops::Deref;

pub struct ShaderBundle {
    vs: String,
    fs: String,
    gs: Option<String>,
    tc: Option<String>,
    te: Option<String>,
}

impl ShaderBundle {
    pub fn new(vs: &str, fs: &str, gs: Option<&str>, tc: Option<&str>, te: Option<&str>) -> Self {
        ShaderBundle {
            vs: vs.to_string(),
            fs: fs.to_string(),
            gs: gs.and_then(|s| Some(s.to_string())),
            tc: tc.and_then(|s| Some(s.to_string())),
            te: te.and_then(|s| Some(s.to_string())),
        }
    }
}

fn read_shader(path: &String) -> String {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let src_dir = Path::new("shaders");

    let complete_path = src_dir.join(path);
    let as_str = complete_path.to_str().unwrap();

    println!("Debug Loading shader: {}", as_str);
    let mut file = File::open(as_str).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return contents;
}

pub struct ProgramManager;


impl ProgramManager {

    pub fn new() -> Self {
        ProgramManager
    }

    pub fn create<'a, T: glium::backend::Facade>(&'a self, display: &T, shaders: &'a ShaderBundle) -> Result<glium::Program, glium::program::ProgramCreationError> {
        let vs = read_shader(&shaders.vs);
        let fs = read_shader(&shaders.fs);

        let gs_data = match shaders.gs {
            None => None,
            Some(ref as_string) => Some(read_shader(as_string)),
        };
        let tc_data = match shaders.tc {
            None => None,
            Some(ref as_string) => Some(read_shader(as_string)),
        };
        let te_data = match shaders.te {
            None => None,
            Some(ref as_string) => Some(read_shader(as_string)),
        };

        let gs: Option<&str> = match gs_data {
            None => None,
            Some(ref as_string) => Some(as_string.as_ref()),
        };

        let tc: Option<&str> = match tc_data {
            None => None,
            Some(ref as_string) => Some(as_string.as_ref()),
        };

        let te: Option<&str> = match te_data {
            None => None,
            Some(ref as_string) => Some(as_string.as_ref()),
        };

        return glium::Program::new(display,
            glium::program::SourceCode {
                vertex_shader: &vs,
                fragment_shader: &fs,
                geometry_shader: gs,
                tessellation_control_shader: tc,
                tessellation_evaluation_shader: te,
            });
    }

}

