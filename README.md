
Setting up:
Install the rust build system cargo using rustup. Make sure to install the nightly version as well, as we need features only available from there.
Install OpenOCD. Version 0.10 is not modern enough. It lacks features we need. You must build and install the nightly version from source.
    The feature we especially need is the support for xds110 devices. Make sure this is stated as going to be built when you configure the project.
Install gcc-multiarch.
Optionally install vscode and the following plugins:
    Cortex-Debug - makes for very easy debugging from within vscode.
    crates - Helps manage rust dependencies.
    Rust(rls) - Official Rust support

Development via command line:
    You must open two terminals in the root directory of this project folder. In one terminal you will run OpenOCD. Just type "openocd" and it should start with the correct configuration. This correct configuration was not guessed by OpenOCD, but was provided by openocd.cfg.

    In the other terminal you must first build the project. Typing "cargo build" should do it. It will automatically downloaded the needed dependencies. You need to use the nightly version of Rust to do this. Because of the rust-toolchain file, cargo will automatically know to use this tool chain, but may error if you have not installed the nightly version of Rust. Cargo already knows its build target because of the config file ./cargo/config .

    Once built you can run gdb as "gdb-multiarch -x openocd.gdb". This will cause gdb to launch and connect to OpenOCD as its debugging target, load the program onto the device, and be ready to run.

Development via VSCode:
    If you have installed all of the required plugins, to build and run the application is as simple as opening the debug menu and pressing the run button. It is already configured for you. All of the needed configuration was provided by .vscode/launch.json and .vscode/tasks.json
