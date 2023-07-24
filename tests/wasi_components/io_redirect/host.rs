use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasm_bridge::wasi::preview2::*;

use std::sync::{Arc, Mutex};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "io-redirect",
    async: true,
    with: {
       "wasi:cli-base/stdin": wasi::cli_base::stdin,
       "wasi:cli-base/stdout": wasi::cli_base::stdout,
       "wasi:cli-base/stderr": wasi::cli_base::stderr,
    }
});

struct State {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub async fn run_test(component_bytes: &[u8]) -> Result<()> {
    no_config(component_bytes).await?;
    inherit(component_bytes).await?;
    capture(component_bytes).await?;

    Ok(())
}

async fn no_config(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::command::add_to_linker(&mut linker)?;

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await?;

    let result = instance.call_readln_from_stdin(&mut store).await?;
    assert_eq!(result, None);

    instance.call_writeln_to_stdout(&mut store, "NO_PRINT").await?;
    instance.call_writeln_to_stderr(&mut store, "NO_PRINT").await?;

    Ok(())
}

async fn inherit(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::command::add_to_linker(&mut linker)?;

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await?;

    // Cannot really read a line in js when inheriting
    // let result = instance.call_readln_from_stdin(&mut store).await?;
    // assert_eq!(result, None);

    instance.call_writeln_to_stdout(&mut store, "PRINT_OUT_1").await?;
    instance.call_writeln_to_stderr(&mut store, "PRINT_ERR_1").await?;

    Ok(())
}

struct OutStream(Arc<Mutex<Vec<u8>>>, usize);

#[wasm_bridge::async_trait]
impl OutputStream for OutStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn writable(&self) -> Result<()> {
        Ok(())
    }

    async fn write(&mut self, buf: &[u8]) -> Result<u64> {
        let amount = buf.len().min(self.1);
        self.0.try_lock().unwrap().extend(&buf[..amount]);
        Ok(amount as u64)
    }
}

async fn capture(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let out_bytes = Arc::new(Mutex::new(Vec::<u8>::new()));
    let out_stream = OutStream(out_bytes.clone(), 3);

    let err_bytes = Arc::new(Mutex::new(Vec::<u8>::new()));
    let err_stream = OutStream(err_bytes.clone(), 3);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().set_stdout(out_stream).set_stderr(err_stream).build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::command::add_to_linker(&mut linker)?;

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await?;

    instance.call_writeln_to_stdout(&mut store, "PRINT_OUT_2").await?;
    instance.call_writeln_to_stdout(&mut store, "NO_PRINT").await?;

    instance.call_writeln_to_stderr(&mut store, "PRINT_ERR_2").await?;
    instance.call_writeln_to_stdout(&mut store, "NO_PRINT").await?;

    let text = String::from_utf8(out_bytes.try_lock().unwrap().clone())?;
    assert!(text.contains("PRINT_OUT_2"), "stdout is captured");

    let text = String::from_utf8(err_bytes.try_lock().unwrap().clone())?;
    assert!(text.contains("PRINT_ERR_2"), "stderr is captured");

    Ok(())
}
