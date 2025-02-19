// Copyright 2023 Greptime Team
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

use std::any::Any;

use common_macro::stack_trace_debug;
use common_telemetry::common_error::ext::ErrorExt;
use common_telemetry::common_error::status_code::StatusCode;
use datatypes::data_type::ConcreteDataType;
use serde::{Deserialize, Serialize};
use snafu::{Location, Snafu};

/// EvalError is about errors happen on columnar evaluation
///
/// TODO(discord9): add detailed location of column/operator(instead of code) to errors tp help identify related column
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[stack_trace_debug]
pub enum EvalError {
    #[snafu(display("Division by zero"))]
    DivisionByZero { location: Location },

    #[snafu(display("Type mismatch: expected {expected}, actual {actual}"))]
    TypeMismatch {
        expected: ConcreteDataType,
        actual: ConcreteDataType,
        location: Location,
    },

    /// can't nest datatypes error because EvalError need to be store in map and serialization
    #[snafu(display("Fail to unpack from value to given type: {msg}"))]
    TryFromValue { msg: String, location: Location },

    #[snafu(display("Fail to cast value of type {from} to given type {to}"))]
    CastValue {
        from: ConcreteDataType,
        to: ConcreteDataType,
        source: datatypes::Error,
        location: Location,
    },

    #[snafu(display("Invalid argument: {reason}"))]
    InvalidArgument { reason: String, location: Location },

    #[snafu(display("Internal error: {reason}"))]
    Internal { reason: String, location: Location },

    #[snafu(display("Optimize error: {reason}"))]
    Optimize { reason: String, location: Location },

    #[snafu(display("Unsupported temporal filter: {reason}"))]
    UnsupportedTemporalFilter { reason: String, location: Location },
}
