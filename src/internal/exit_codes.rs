pub enum AppExitCode {
    RunAsRoot = 1,
    InvalidArgs = 2,
    AppDirsCreation = 3,

    Runtime = 9,

    // Devenv
    DevenvStart = 10,
    DevenvStop = 11,
    DevenvOnce = 12,
    DevenvExec = 13,

    // Config
    ConfigWrite = 20,
    ConfigBak = 21,

    // Context
    InvalidContext = 30,
}
