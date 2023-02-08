pub enum AppExitCode {
    RunAsRoot = 1,
    InvalidArgs = 2,
    BuildPlatformError = 3,

    // Devenv
    DevenvStart = 10,
    DevenvExec = 11,

    // Config
    ConfigWrite = 20,
    ConfigBak = 21,
}
