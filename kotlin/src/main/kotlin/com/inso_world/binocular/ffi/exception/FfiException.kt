package com.inso_world.binocular.ffi.exception

class BinocularFfiException: Exception {
    constructor(message: String) : super(message)
    constructor(cause: Throwable) : super(cause)
    constructor(message: String, cause: Throwable) : super(message, cause)
}