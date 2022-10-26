$version: "1.0"

namespace com.example

use aws.protocols#restJson1

/// Service definition.
@title("A Service")
@restJson1
service Service {
    version: "2022-10-21",
    operations: [
        Operation,
    ]
}

/// Input definition.
@readonly
@http(uri: "/operation", method: "GET")
operation Operation {
    input: OperationInput,
    output: OperationOutput,
    errors: [OperationException],
}

/// Error definition..
@input
structure OperationInput {}

/// Output definition..
@output
structure OperationOutput {}

/// Error definition..
@error("server")
@httpError(500)
structure OperationException {
    @required
    message: String,
}
