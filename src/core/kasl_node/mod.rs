mod error;
mod syntax;

pub(crate) use error::KaslNodeError;
pub(crate) use syntax::kasl_syntax_set;

use crate::core::audio_engine::{
    data_types::{PlaybackContext, TypeInfo},
    graph::error::NodeError,
    node::Node,
    timing::TempoMap,
};
use kasl::{
    core::{KaslCompiler, ast_nodes::scope_manager::IOBlueprint, run_buffer},
    cranelift_backend::CraneliftBackend,
    ir::Optimizer,
};
use std::path::PathBuf;

#[derive(Default)]
pub(crate) struct KaslNode {
    backend: Option<CraneliftBackend>,
    blueprint: Option<IOBlueprint>,
    search_paths: Vec<String>,
    /// Relative path to the KASL source file within the project directory.
    file_path: Option<String>,
    project_dir: Option<PathBuf>,

    input_types: Vec<TypeInfo>,
    output_types: Vec<TypeInfo>,

    states: Vec<Vec<u8>>,
    program: Option<*const u8>,
    is_first_process: bool,

    /// Source code of the last successful compile, used to skip recompiling.
    last_source: Option<String>,
    /// Playback context of the last successful compile.
    last_playback_key: Option<(usize, u64, usize)>,
}

impl KaslNode {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with_path(path: String) -> Self {
        KaslNode {
            file_path: Some(path),
            ..Default::default()
        }
    }

    pub(crate) fn set_search_paths(&mut self, paths: Vec<String>) {
        self.search_paths = paths;
    }

    pub(crate) fn set_file_path(&mut self, path: String) {
        self.file_path = Some(path);
    }

    pub(crate) fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    pub(crate) fn set_project_dir(&mut self, dir: PathBuf) {
        self.project_dir = Some(dir);
    }

    pub(crate) fn compile(
        &mut self,
        playback_ctx: &PlaybackContext,
    ) -> Result<bool, KaslNodeError> {
        // Read source code from disk at compile time
        // If the file path is not set, there is nothing to compile
        let (Some(project_dir), Some(file_path)) = (&self.project_dir, &self.file_path) else {
            return Ok(false);
        };

        let code = match std::fs::read_to_string(project_dir.join(file_path)) {
            Ok(code) => code,
            Err(err) => return Err(KaslNodeError::FileRead(err)),
        };

        let playback_key = (
            playback_ctx.channels,
            playback_ctx.sample_rate,
            playback_ctx.buffer_size,
        );

        // Skip recompiling if the source code and playback context are unchanged
        if self.program.is_some()
            && self.last_source.as_deref() == Some(code.as_str())
            && self.last_playback_key == Some(playback_key)
        {
            return Ok(false);
        }

        let mut search_paths = self
            .search_paths
            .clone()
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<_>>();
        if let Some(local_path) = PathBuf::from(file_path)
            .parent()
            .and_then(|path| path.canonicalize().ok())
        {
            search_paths.push(local_path.to_path_buf());
        }

        // Clean up the old states
        self.deallocate_states();
        // Drop the old backend and the program
        self.blueprint.take();
        self.backend.take();
        self.program.take();

        // Create a compiler
        let mut compiler = KaslCompiler::default();

        // Add the search paths to the compiler
        compiler.set_search_paths(search_paths);
        let audio_module = format!(
            include_str!("../../../kasl_module/audio.kasl"),
            playback_ctx.channels, playback_ctx.sample_rate, playback_ctx.buffer_size,
        );
        compiler.add_virtual_file(PathBuf::from("audio"), audio_module);

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
        self.update_type_info();

        self.last_source = Some(code);
        self.last_playback_key = Some(playback_key);

        Ok(true)
    }

    fn allocate_states(&mut self, blueprint: &IOBlueprint) {
        // Allocate the state memory based on the blueprint
        for state_item in blueprint.get_states() {
            let buffer = vec![0u8; state_item.actual_size as usize];
            self.states.push(buffer);
        }
    }

    fn deallocate_states(&mut self) {
        // De-allocate the allocated states
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

    fn update_type_info(&mut self) {
        // Create TypeInfo for input types and output types
        self.input_types = self
            .blueprint
            .as_ref()
            .map(|blueprint| {
                blueprint
                    .get_inputs()
                    .iter()
                    .map(|item| TypeInfo::new(item.actual_size as usize, item.align as usize))
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
                    .map(|item| TypeInfo::new(item.actual_size as usize, item.align as usize))
                    .collect()
            })
            .unwrap_or_default();
    }

    fn prepare(
        &mut self,
        _tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), Box<dyn NodeError>> {
        let result = self.compile(playback_ctx);
        match &result {
            // If the compilation is successful, set is_first_process to true if recompiled, otherwise false
            // which prevents the program from resetting the states on the first process after recompilation
            Ok(recompiled) => self.is_first_process = *recompiled,
            Err(records) => eprintln!("KaslNode::prepare: compile FAILED: {:?}", records),
        }
        result
            .map(|_| ())
            .map_err(|error| -> Box<dyn NodeError> { Box::new(error) })
    }

    fn process(
        &mut self,
        inputs: &[&[u8]],
        outputs: &mut [&mut [u8]],
        _playhead: usize,
        playback_ctx: &PlaybackContext,
    ) {
        let inputs: Vec<*const ()> = inputs.iter().map(|p| p.as_ptr() as *const ()).collect();
        let outputs: Vec<*mut ()> = outputs
            .iter_mut()
            .map(|p| p.as_mut_ptr() as *mut ())
            .collect();

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
                &self
                    .states
                    .iter_mut()
                    .map(|buffer| buffer.as_mut_ptr() as *mut ())
                    .collect::<Vec<_>>(),
                if self.is_first_process { 1 } else { 0 },
                playback_ctx.buffer_size as i32,
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
            states: Vec::new(),
            program: None,
            is_first_process: false,
            last_source: None,
            last_playback_key: None,
        }
    }
}

unsafe impl Send for KaslNode {}
