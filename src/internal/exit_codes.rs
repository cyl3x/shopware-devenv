pub enum AppExitCode {
    RunAsRoot = 1,
    InvalidArgs = 2,

    // Devenv
    DevenvStart = 10,
    DevenvOnce = 11,
    DevenvExec = 12,

    // Config
    ConfigWrite = 20,
    ConfigBak = 21,

    // Context
    InvalidContext = 30,
}
