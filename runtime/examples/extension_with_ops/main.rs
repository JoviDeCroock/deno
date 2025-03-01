// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::path::Path;
use std::rc::Rc;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;

deno_core::extension!(hello_runtime, ops = [op_hello]);

#[op2(fast)]
fn op_hello(#[string] text: &str) {
  println!("Hello {}!", text);
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
  let js_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("examples/extension_with_ops/main.js");
  let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
  let mut worker = MainWorker::bootstrap_from_options(
    main_module.clone(),
    PermissionsContainer::allow_all(),
    WorkerOptions {
      module_loader: Rc::new(FsModuleLoader),
      extensions: vec![hello_runtime::init_ops()],
      ..Default::default()
    },
  );
  worker.execute_main_module(&main_module).await?;
  worker.run_event_loop(false).await?;
  Ok(())
}
