pub enum UscRequest {
    RequestGetParameter = 0x81,
    RequestSetParameter = 0x82,
    RequestGetVariables = 0x83,
    RequestSetServoVariable = 0x84,
    RequestSetTarget = 0x85,
    RequestClearErrors = 0x86,

    // These four requests are only valid on *Mini* Maestros.
    RequestGetServoSettings = 0x87,
    RequestGetStack = 0x88,
    RequestGetCallStack = 0x89,
    RequestSetPwm = 0x8A,

    RequestReinitialize = 0x90,
    RequestEraseScript = 0xA0,
    RequestWriteScript = 0xA1,
    RequestSetScriptDone = 0xA2, // wValue is 0 for go, 1 for stop, 2 for single-step
    RequestRestartScriptAtSubroutine = 0xA3,
    RequestRestartScriptAtSubroutineWithParameter = 0xA4,
    RequestRestartScript = 0xA5,
    RequestStartBootloader = 0xFF,
}

pub enum UscCommand {
    CommandSetTarget = 0x84,                              // 3 data bytes
    CommandSetSpeed = 0x87,                               // 3 data bytes
    CommandSetAcceleration = 0x89,                        // 3 data bytes
    CommandGetPosition = 0x90,                            // 0 data
    CommandGetMovingState = 0x93,                         // 0 data
    CommandGetErrors = 0xA1,                              // 0 data
    CommandGoHome = 0xA2,                                 // 0 data
    CommandStopScript = 0xA4,                             // 0 data
    CommandRestartScriptAtSubroutine = 0xA7,              // 1 data bytes
    CommandRestartScriptAtSubroutineWithParameter = 0xA8, // 3 data bytes
    CommandGetScriptStatus = 0xAE,                        // 0 data
    CommandMiniSsc = 0xFF,                                // (2 data bytes)
}
