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

pub struct ServoStatus {
    position: u16,
    target: u16,
    speed: u16,
    accel: u8,
}

pub struct MaestroVariables {
    stack_pointer: u8,
    call_stack_pointers: u8,
    errors: u16,
    program_counter: u16,
    script_done: u8,
    performance_flags: u8,
}

pub struct MaestroMicroVariables {
    stack_pointer: u8,
    call_stack_pointers: u8,
    errors: u16,
    program_counter: u16,
    buffer: [i16; 3],
    stack: [i16; 32],
    call_stack: [u16; 10],
    script_done: u8,
    buffer2: u8,
}

pub struct MaestroMiniVariables {
    stack_pointer: u8,
    call_stack_pointers: u8,
    errors: u16,
    program_counter: u16,
    script_done: u8,
}

#[derive(Debug)]
pub enum UscSerialMode {
    SerialModeUsbChained = 1,
    SerialModeUartDetectBaudRate = 2,
    SerialModeUartFixedBaudRate = 3,
}

pub enum UscError {
    ErrorSerialSignal = 0,
    ErrorSerialOverrun = 1,
    ErrorSerialBufferFull = 2,
    ErrorSerialCrc = 3,
    ErrorSerialProtocol = 4,
    ErrorSerialTimeout = 5,
    ErrorScriptStack = 6,
    ErrorScriptCallStack = 7,
    ErrorScriptProgramCounter = 8,
}

pub enum PerformanceFlag {
    PErrorAdvancedUpdate = 0,
    PErrorBasicUpdate = 1,
    PErrorPeriod = 2,
}

// Eww just Eww
pub enum UscParameter {
    ParameterInitialized = 0,           // 1 byte - 0 or 0xFF
    ParameterServosAvailable = 1,       // 1 byte - 0-5
    ParameterServoPeriod = 2,           // 1 byte - ticks allocated to each servo/256
    ParameterSerialMode = 3, // 1 byte unsigned value.  Valid values are SERIAL_MODE_*.  Init variable.
    ParameterSerialFixedBaudRate = 4, // 2-byte unsigned value; 0 means autodetect.  Init parameter.
    ParameterSerialTimeout = 6, // 2-byte unsigned value
    ParameterSerialEnableCrc = 8, // 1 byte boolean value
    ParameterSerialNeverSuspend = 9, // 1 byte boolean value
    ParameterSerialDeviceNumber = 10, // 1 byte unsigned value, 0-127
    ParameterSerialBaudDetectType = 11, // 1 byte value

    ParameterIoMaskC = 84,     // 1 byte - pins used for I/O instead of servo
    ParameterOutputMaskC = 85, // 1 byte - outputs that are enabled

    ParameterChannelModes03 = 12,          // 1 byte - channel modes 0-3
    ParameterChannelModes47 = 13,          // 1 byte - channel modes 4-7
    ParameterChannelModes811 = 14,         // 1 byte - channel modes 8-11
    ParameterChannelModes1215 = 15,        // 1 byte - channel modes 12-15
    ParameterChannelModes1619 = 16,        // 1 byte - channel modes 16-19
    ParameterChannelModes2023 = 17,        // 1 byte - channel modes 20-23
    ParameterMiniMaestroServoPeriodL = 18, // servo period: 3-byte unsigned values, units of quarter microseconds
    ParameterMiniMaestroServoPeriodHu = 19,
    ParameterEnablePullups = 21,       // 1 byte: 0 or 1
    ParameterScriptCrc = 22, // 2 bytes - stores a checksum of the bytecode program, for comparison
    ParameterScriptDone = 24, // 1 byte - copied to scriptDone on startup
    ParameterSerialMiniSscOffset = 25, // 1 byte (0-254)
    ParameterServoMultiplier = 26, // 1 byte (0-255)

    // 9 * 24 = 216, so we can safely start at 30
    ParameterServo0Home = 30,    // 2 byte home position (0=off; 1=ignore)
    ParameterServo0Min = 32,     // 1 byte min allowed value (x2^6)
    ParameterServo0Max = 33,     // 1 byte max allowed value (x2^6)
    ParameterServo0Neutral = 34, // 2 byte neutral position
    ParameterServo0Range = 36,   // 1 byte range
    ParameterServo0Speed = 37,   // 1 byte (5 mantissa,3 exponent) us per 10ms
    ParameterServo0Acceleration = 38, // 1 byte (speed changes that much every 10ms)
    ParameterServo1Home = 39,    // 2 byte home position (0=off; 1=ignore)
    ParameterServo1Min = 41,     // 1 byte min allowed value (x2^6)
    ParameterServo1Max = 42,     // 1 byte max allowed value (x2^6)
    ParameterServo1Neutral = 43, // 2 byte neutral position
    ParameterServo1Range = 45,   // 1 byte range
    ParameterServo1Speed = 46,   // 1 byte (5 mantissa,3 exponent) us per 10ms
    ParameterServo1Acceleration = 47, // 1 byte (speed changes that much every 10ms)
    ParameterServo2Home = 48,    // 2 byte home position (0=off; 1=ignore)
    ParameterServo2Min = 50,     // 1 byte min allowed value (x2^6)
    ParameterServo2Max = 51,     // 1 byte max allowed value (x2^6)
    ParameterServo2Neutral = 52, // 2 byte neutral position
    ParameterServo2Range = 54,   // 1 byte range
    ParameterServo2Speed = 55,   // 1 byte (5 mantissa,3 exponent) us per 10ms
    ParameterServo2Acceleration = 56, // 1 byte (speed changes that much every 10ms)
    ParameterServo3Home = 57,    // 2 byte home position (0=off; 1=ignore)
    ParameterServo3Min = 59,     // 1 byte min allowed value (x2^6)
    ParameterServo3Max = 60,     // 1 byte max allowed value (x2^6)
    ParameterServo3Neutral = 61, // 2 byte neutral position
    ParameterServo3Range = 63,   // 1 byte range
    ParameterServo3Speed = 64,   // 1 byte (5 mantissa,3 exponent) us per 10ms
    ParameterServo3Acceleration = 65, // 1 byte (speed changes that much every 10ms)
    ParameterServo4Home = 66,    // 2 byte home position (0=off; 1=ignore)
    ParameterServo4Min = 68,     // 1 byte min allowed value (x2^6)
    ParameterServo4Max = 69,     // 1 byte max allowed value (x2^6)
    ParameterServo4Neutral = 70, // 2 byte neutral position
    ParameterServo4Range = 72,   // 1 byte range
    ParameterServo4Speed = 73,   // 1 byte (5 mantissa,3 exponent) us per 10ms
    ParameterServo4Acceleration = 74, // 1 byte (speed changes that much every 10ms)
    ParameterServo5Home = 75,    // 2 byte home position (0=off; 1=ignore)
    ParameterServo5Min = 77,     // 1 byte min allowed value (x2^6)
    ParameterServo5Max = 78,     // 1 byte max allowed value (x2^6)
    ParameterServo5Neutral = 79, // 2 byte neutral position
    ParameterServo5Range = 81,   // 1 byte range
    ParameterServo5Speed = 82,   // 1 byte (5 mantissa,3 exponent) us per 10ms
    ParameterServo5Acceleration = 83, // 1 byte (speed changes that much every 10ms)
}

// Why Just WHY
impl TryFrom<u8> for UscParameter {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(UscParameter::ParameterInitialized),
            1 => Ok(UscParameter::ParameterServosAvailable),
            2 => Ok(UscParameter::ParameterServoPeriod),
            3 => Ok(UscParameter::ParameterSerialMode),
            4 => Ok(UscParameter::ParameterSerialFixedBaudRate),
            6 => Ok(UscParameter::ParameterSerialTimeout),
            8 => Ok(UscParameter::ParameterSerialEnableCrc),
            9 => Ok(UscParameter::ParameterSerialNeverSuspend),
            10 => Ok(UscParameter::ParameterSerialDeviceNumber),
            11 => Ok(UscParameter::ParameterSerialBaudDetectType),
            12 => Ok(UscParameter::ParameterChannelModes03),
            13 => Ok(UscParameter::ParameterChannelModes47),
            14 => Ok(UscParameter::ParameterChannelModes811),
            15 => Ok(UscParameter::ParameterChannelModes1215),
            16 => Ok(UscParameter::ParameterIoMaskC),
            17 => Ok(UscParameter::ParameterOutputMaskC),
            18 => Ok(UscParameter::ParameterMiniMaestroServoPeriodL),
            19 => Ok(UscParameter::ParameterMiniMaestroServoPeriodHu),
            21 => Ok(UscParameter::ParameterEnablePullups),
            22 => Ok(UscParameter::ParameterScriptCrc),
            24 => Ok(UscParameter::ParameterScriptDone),
            25 => Ok(UscParameter::ParameterSerialMiniSscOffset),
            26 => Ok(UscParameter::ParameterServoMultiplier),
            30 => Ok(UscParameter::ParameterServo0Home),
            32 => Ok(UscParameter::ParameterServo0Min),
            33 => Ok(UscParameter::ParameterServo0Max),
            34 => Ok(UscParameter::ParameterServo0Neutral),
            36 => Ok(UscParameter::ParameterServo0Range),
            37 => Ok(UscParameter::ParameterServo0Speed),
            38 => Ok(UscParameter::ParameterServo0Acceleration),
            39 => Ok(UscParameter::ParameterServo1Home),
            41 => Ok(UscParameter::ParameterServo1Min),
            42 => Ok(UscParameter::ParameterServo1Max),
            43 => Ok(UscParameter::ParameterServo1Neutral),
            45 => Ok(UscParameter::ParameterServo1Range),
            46 => Ok(UscParameter::ParameterServo1Speed),
            47 => Ok(UscParameter::ParameterServo1Acceleration),
            48 => Ok(UscParameter::ParameterServo2Home),
            50 => Ok(UscParameter::ParameterServo2Min),
            51 => Ok(UscParameter::ParameterServo2Max),
            52 => Ok(UscParameter::ParameterServo2Neutral),
            54 => Ok(UscParameter::ParameterServo2Range),
            55 => Ok(UscParameter::ParameterServo2Speed),
            56 => Ok(UscParameter::ParameterServo2Acceleration),
            57 => Ok(UscParameter::ParameterServo3Home),
            59 => Ok(UscParameter::ParameterServo3Min),
            60 => Ok(UscParameter::ParameterServo3Max),
            61 => Ok(UscParameter::ParameterServo3Neutral),
            63 => Ok(UscParameter::ParameterServo3Range),
            64 => Ok(UscParameter::ParameterServo3Speed),
            65 => Ok(UscParameter::ParameterServo3Acceleration),
            66 => Ok(UscParameter::ParameterServo4Home),
            68 => Ok(UscParameter::ParameterServo4Min),
            69 => Ok(UscParameter::ParameterServo4Max),
            70 => Ok(UscParameter::ParameterServo4Neutral),
            72 => Ok(UscParameter::ParameterServo4Range),
            73 => Ok(UscParameter::ParameterServo4Speed),
            74 => Ok(UscParameter::ParameterServo4Acceleration),
            75 => Ok(UscParameter::ParameterServo5Home),
            77 => Ok(UscParameter::ParameterServo5Min),
            78 => Ok(UscParameter::ParameterServo5Max),
            79 => Ok(UscParameter::ParameterServo5Neutral),
            81 => Ok(UscParameter::ParameterServo5Range),
            82 => Ok(UscParameter::ParameterServo5Speed),
            83 => Ok(UscParameter::ParameterServo5Acceleration),
            _ => Err(()),
        };
    }
}
// Why Just WHY
impl TryInto<u8> for UscParameter {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        return Ok(match self {
            UscParameter::ParameterInitialized => 0,
            UscParameter::ParameterServosAvailable => 1,
            UscParameter::ParameterServoPeriod => 2,
            UscParameter::ParameterSerialMode => 3,
            UscParameter::ParameterSerialFixedBaudRate => 4,
            UscParameter::ParameterSerialTimeout => 6,
            UscParameter::ParameterSerialEnableCrc => 8,
            UscParameter::ParameterSerialNeverSuspend => 9,
            UscParameter::ParameterSerialDeviceNumber => 10,
            UscParameter::ParameterSerialBaudDetectType => 11,
            UscParameter::ParameterIoMaskC => 16,
            UscParameter::ParameterOutputMaskC => 17,
            UscParameter::ParameterChannelModes03 => 12,
            UscParameter::ParameterChannelModes47 => 13,
            UscParameter::ParameterChannelModes811 => 14,
            UscParameter::ParameterChannelModes1215 => 15,
            UscParameter::ParameterChannelModes1619 => 16,
            UscParameter::ParameterChannelModes2023 => 17,
            UscParameter::ParameterMiniMaestroServoPeriodL => 18,
            UscParameter::ParameterMiniMaestroServoPeriodHu => 19,
            UscParameter::ParameterEnablePullups => 21,
            UscParameter::ParameterScriptCrc => 22,
            UscParameter::ParameterScriptDone => 24,
            UscParameter::ParameterSerialMiniSscOffset => 25,
            UscParameter::ParameterServoMultiplier => 26,
            UscParameter::ParameterServo0Home => 30,
            UscParameter::ParameterServo0Min => 32,
            UscParameter::ParameterServo0Max => 33,
            UscParameter::ParameterServo0Neutral => 34,
            UscParameter::ParameterServo0Range => 36,
            UscParameter::ParameterServo0Speed => 37,
            UscParameter::ParameterServo0Acceleration => 38,
            UscParameter::ParameterServo1Home => 39,
            UscParameter::ParameterServo1Min => 41,
            UscParameter::ParameterServo1Max => 42,
            UscParameter::ParameterServo1Neutral => 43,
            UscParameter::ParameterServo1Range => 45,
            UscParameter::ParameterServo1Speed => 46,
            UscParameter::ParameterServo1Acceleration => 47,
            UscParameter::ParameterServo2Home => 48,
            UscParameter::ParameterServo2Min => 50,
            UscParameter::ParameterServo2Max => 51,
            UscParameter::ParameterServo2Neutral => 52,
            UscParameter::ParameterServo2Range => 54,
            UscParameter::ParameterServo2Speed => 55,
            UscParameter::ParameterServo2Acceleration => 56,
            UscParameter::ParameterServo3Home => 57,
            UscParameter::ParameterServo3Min => 59,
            UscParameter::ParameterServo3Max => 60,
            UscParameter::ParameterServo3Neutral => 61,
            UscParameter::ParameterServo3Range => 63,
            UscParameter::ParameterServo3Speed => 64,
            UscParameter::ParameterServo3Acceleration => 65,
            UscParameter::ParameterServo4Home => 66,
            UscParameter::ParameterServo4Min => 68,
            UscParameter::ParameterServo4Max => 69,
            UscParameter::ParameterServo4Neutral => 70,
            UscParameter::ParameterServo4Range => 72,
            UscParameter::ParameterServo4Speed => 73,
            UscParameter::ParameterServo4Acceleration => 74,
            UscParameter::ParameterServo5Home => 75,
            UscParameter::ParameterServo5Min => 77,
            UscParameter::ParameterServo5Max => 78,
            UscParameter::ParameterServo5Neutral => 79,
            UscParameter::ParameterServo5Range => 81,
            UscParameter::ParameterServo5Speed => 82,
            UscParameter::ParameterServo5Acceleration => 83,
        });
    }
}
