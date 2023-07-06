use std::collections::HashMap;

use js_sys::Function;

use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Store<T> {
    engine: Engine,
    functions: FunctionsStore,
    _data: T,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        Self {
            engine: engine.clone(),
            functions: FunctionsStore::new(),
            _data: data,
        }
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub(crate) fn add_function(&mut self, function: Function) -> FuncId {
        self.functions.add_function(function)
    }

    pub(crate) fn get_function(&self, id: u32) -> Option<&Function> {
        self.functions.get_function(id)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FunctionsStore {
    functions: HashMap<FuncId, FunctionHandle>,
    count: FuncId,
    next_free_id: FuncId,
}

#[derive(Clone, Debug)]
pub(crate) enum FunctionHandle {
    Free(FuncId), // Next free "slot"
    Full(Function),
}

impl FunctionHandle {}

pub(crate) type FuncId = u32;

impl FunctionsStore {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            count: 0,
            next_free_id: 0,
        }
    }

    pub fn add_function(&mut self, function: Function) -> FuncId {
        if self.count as u32 == self.next_free_id {
            let id = self.next_free_id;
            self.functions.insert(id, FunctionHandle::Full(function));
            self.next_free_id = self.next_free_id.checked_add(1).unwrap();
            self.count = self.count.checked_add(1).unwrap();
            id
        } else {
            let id = self.next_free_id;
            self.next_free_id = match self.functions.get(&id).unwrap() {
                FunctionHandle::Free(id) => *id,
                _ => unreachable!(),
            };
            self.count = self.count.checked_add(1).unwrap();
            id
        }
    }

    pub fn get_function(&self, id: FuncId) -> Option<&Function> {
        match self.functions.get(&id)? {
            FunctionHandle::Full(function) => Some(function),
            FunctionHandle::Free(_) => None,
        }
    }

    pub fn remove_function(&mut self, id: FuncId) {
        match self.functions.get(&id).unwrap() {
            FunctionHandle::Full(_) => {} // OK
            FunctionHandle::Free(_) => panic!("Removing a free function"),
        };
        self.functions
            .insert(id, FunctionHandle::Free(self.next_free_id));
        self.next_free_id = id;
        self.count = self.count - 1;
    }
}