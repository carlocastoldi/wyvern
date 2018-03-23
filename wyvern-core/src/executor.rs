// Copyright 2018 | Dario Ostuni <dario.ostuni@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::hash::Hash;
use program::{Program, TokenType, TokenValue};

pub trait Executor<'a> {
    type Config: Default + Clone;
    type Error: ToString;
    type Executable: Executable<'a>;
    type Resource: Resource;
    fn new(config: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn compile(&self, program: Program) -> Result<Self::Executable, Self::Error>;
    fn new_resource(&self) -> Result<Self::Resource, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IO {
    Input,
    Output,
}

pub trait Executable<'a> {
    type Resource: Resource;
    type Error: ToString;
    type Report: ToString;
    fn bind<S: ToString>(&'a mut self, name: S, kind: IO, res: &'a Self::Resource);
    fn unbind<S: ToString>(&self, name: S, kind: IO);
}

pub trait Resource: Eq + Hash {
    fn clear(&mut self);
    fn token_type(&self) -> TokenType;
    fn set_data(&mut self, value: TokenValue);
    fn get_data(&self) -> TokenValue;
}
