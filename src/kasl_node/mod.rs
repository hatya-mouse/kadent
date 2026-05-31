mod error;

pub use error::KaslNodeError;

use kasl::{
    core::{KaslCompiler, ast_nodes::scope_manager::IOBlueprint, run_buffer},
    cranelift_backend::CraneliftBackend,
    ir::Optimizer,
};
use kreniq_engine::{
    data_types::{AudioContext, TypeInfo},
    graph::error::NodeError,
    node::Node,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct KaslNode {
    backend: Option<CraneliftBackend>,
    blueprint: Option<IOBlueprint>,
    search_paths: Vec<String>,
    /// Relative path to the KASL source file within the project directory.
    file_path: Option<String>,
    project_dir: Option<PathBuf>,

    input_types: Vec<TypeInfo>,
    output_types: Vec<TypeInfo>,

    /// Cached buffer size to calculate the size of input and output for single buffer.
    buffer_size: usize,

    states: Vec<*mut ()>,
    program: Option<*const u8>,
    is_first_process: bool,
}

impl KaslNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_search_paths(&mut self, paths: Vec<String>) {
        self.search_paths = paths;
    }

    pub fn set_file_path(&mut self, path: String) {
        self.file_path = Some(path);
    }

    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    pub fn set_project_dir(&mut self, dir: PathBuf) {
        self.project_dir = Some(dir);
    }

    pub fn compile(&mut self) -> Result<(), KaslNodeError> {
        // Clean up the old states
        self.deallocate_states();
        // Drop the old backend and the program
        self.blueprint.take();
        self.backend.take();
        self.program.take();

        // Create a compiler
        let mut compiler = KaslCompiler::default();
        // Add the search paths to the compiler
        compiler.set_search_paths(self.search_paths.iter().map(PathBuf::from).collect());

        // Read source code from disk at compile time so changes are always picked up
        let (Some(project_dir), Some(file_path)) = (&self.project_dir, &self.file_path) else {
            // If the file path is not set, return
            return Ok(());
        };

        let code = match std::fs::read_to_string(project_dir.join(file_path)) {
            Ok(code) => code,
            Err(err) => return Err(KaslNodeError::FileRead(err)),
        };

        // Parse, build and compile the source codes
        compiler
            .parse(&code)
            .map_err(|e| KaslNodeError::Compile(vec![*e]))?;
        let (blueprint, _) = compiler.build().map_err(KaslNodeError::Compile)?;
        let func = compiler
            .lower_buffer(&blueprint)
            .map_err(|e| KaslNodeError::Compile(vec![e]))?;

        // Optimize the compiled function
        let mut optimizer = Optimizer::default();
        let func = optimizer.optimize(func);

        // DEBUG IR DUMPING
        // let mut f = File::create("ir_dump").unwrap();
        // write!(
        //     f,
        //     "BEFORE OPTIMIZATION -->\n{}\n\nAFTER OPTIMIZATION -->\n{}",
        //     before_func, func
        // )
        // .unwrap();

        // Compile the program to executable binary
        let mut backend = CraneliftBackend::default();
        self.program = Some(backend.compile(func).map_err(KaslNodeError::Backend)?);

        self.allocate_states(&blueprint);

        // Set the blueprint
        self.blueprint = Some(blueprint);
        // Move the compiler to KaslNode to preserve the compiled program until next compile
        self.backend = Some(backend);
        // Update the types
        self.update_type_infos();

        Ok(())
    }

    fn update_type_infos(&mut self) {
        // Create TypeInfo for input types and output types
        self.input_types = self
            .blueprint
            .as_ref()
            .map(|blueprint| {
                blueprint
                    .get_inputs()
                    .iter()
                    .map(|item| {
                        TypeInfo::new(
                            (item.actual_size as usize) * self.buffer_size,
                            item.align as usize,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.output_types = self
            .blueprint
            .as_ref()
            .map(|blueprint| {
                blueprint
                    .get_outputs()
                    .iter()
                    .map(|item| {
                        TypeInfo::new(
                            (item.actual_size as usize) * self.buffer_size,
                            item.align as usize,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    fn allocate_states(&mut self, blueprint: &IOBlueprint) {
        // Allocate the state memory based of the blueprint
        for state_item in blueprint.get_states() {
            let layout = std::alloc::Layout::from_size_align(
                state_item.actual_size as usize,
                state_item.align as usize,
            )
            .unwrap();
            let ptr = unsafe { std::alloc::alloc_zeroed(layout) as *mut () };
            self.states.push(ptr);
        }
    }

    fn deallocate_states(&mut self) {
        // De-allocate the allocated states
        for (ptr, state_item) in self
            .states
            .iter()
            .zip(self.blueprint.iter().flat_map(|b| b.get_states()))
        {
            let layout = std::alloc::Layout::from_size_align(
                state_item.actual_size as usize,
                state_item.align as usize,
            )
            .unwrap();
            unsafe { std::alloc::dealloc(*ptr as *mut u8, layout) };
        }
        self.states.clear();
    }
}

impl Node for KaslNode {
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_input_names(&self) -> Vec<String> {
        self.blueprint
            .as_ref()
            .map(|blueprint| {
                blueprint
                    .get_inputs()
                    .iter()
                    .map(|i| i.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn get_output_names(&self) -> Vec<String> {
        self.blueprint
            .as_ref()
            .map(|blueprint| {
                blueprint
                    .get_outputs()
                    .iter()
                    .map(|i| i.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn get_input_len(&self) -> usize {
        self.blueprint
            .as_ref()
            .map(|blueprint| blueprint.get_inputs().len())
            .unwrap_or_default()
    }

    fn get_output_len(&self) -> usize {
        self.blueprint
            .as_ref()
            .map(|blueprint| blueprint.get_outputs().len())
            .unwrap_or_default()
    }

    fn get_input_type(&self, index: usize) -> Option<&TypeInfo> {
        self.input_types.get(index)
    }

    fn get_output_type(&self, index: usize) -> Option<&TypeInfo> {
        self.output_types.get(index)
    }

    fn update(&mut self, audio_ctx: &AudioContext) {
        self.buffer_size = audio_ctx.buffer_size;
        self.update_type_infos();
    }

    fn prepare(&mut self) -> Result<(), Box<dyn NodeError>> {
        self.is_first_process = true;

        let result = self.compile();
        match &result {
            Ok(_) => (),
            Err(records) => eprintln!("KaslNode::prepare: compile FAILED: {:?}", records),
        }
        result.map_err(|error| -> Box<dyn NodeError> { Box::new(error) })
    }

    fn process(&mut self, inputs: &[*const u8], outputs: &[*mut u8], audio_ctx: &AudioContext) {
        let inputs: Vec<*const ()> = inputs.iter().map(|p| *p as *const ()).collect();
        let outputs: Vec<*mut ()> = outputs.iter().map(|p| *p as *mut ()).collect();

        // Return if the program pointer is null
        if self.program.is_none_or(|program| program.is_null()) {
            return;
        }

        unsafe {
            // Execute the JIT-compiled program thus this has to be unsafe.
            run_buffer(
                self.program.unwrap(),
                &inputs,
                &outputs,
                &self.states,
                if self.is_first_process { 1 } else { 0 },
                audio_ctx.buffer_size as i32,
            );
        }

        self.is_first_process = false;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Clone for KaslNode {
    fn clone(&self) -> Self {
        Self {
            backend: None,
            blueprint: None,
            search_paths: self.search_paths.clone(),
            file_path: self.file_path.clone(),
            project_dir: self.project_dir.clone(),
            input_types: self.input_types.clone(),
            output_types: self.output_types.clone(),
            buffer_size: self.buffer_size,
            states: Vec::new(),
            program: None,
            is_first_process: false,
        }
    }
}

impl Drop for KaslNode {
    fn drop(&mut self) {
        // De-allocate the allocated states
        self.deallocate_states();
    }
}

unsafe impl Send for KaslNode {}
