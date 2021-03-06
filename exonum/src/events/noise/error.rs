// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

use failure;
use snow::SnowError;

use std::io;

#[derive(Fail, Debug, Clone)]
pub enum NoiseError {
    #[fail(display = "Wrong handshake message length {}", _0)]
    WrongMessageLength(usize),

    #[fail(display = "{}", _0)]
    Other(String),
}

impl From<NoiseError> for io::Error {
    fn from(e: NoiseError) -> Self {
        let message = match e {
            NoiseError::Other(message) => message,
            _ => format!("{:?}", e),
        };

        io::Error::new(io::ErrorKind::Other, message)
    }
}

impl From<failure::Error> for NoiseError {
    fn from(e: failure::Error) -> Self {
        NoiseError::Other(format!("{:?}", e))
    }
}

impl From<SnowError> for NoiseError {
    fn from(e: SnowError) -> Self {
        NoiseError::Other(format!("{:?}", e))
    }
}
