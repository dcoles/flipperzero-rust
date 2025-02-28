# Debugging

## Using ST-Link as an In-Circuit Debugger

The easiest way to debug low-level crashes is to use a ST-Link compatible USB dongle that can be
[cheaply picked up online](https://www.adafruit.com/product/2548).

With this you can easily inspect the state of the Flipper Zero, including running applications.
The [Flipper Application SDK](https://github.com/flipperdevices/flipperzero-ufbt) provides
a full debugging toolchain which is by-far the easiest way to get started, as opposed
to configurating [OpenOCD](https://openocd.org/) yourself.

The pins should be connected to the following pins on the ST-Link compatible dongle:

- Flipper Zero `SWC` (Port 10) to `SWCLK`
- Flipper Zero `GND` (Port 11) to `GND`
- Flipper Zero `SIO` (Port 12) to `SWDIO`

> [!TIP]
> Make sure to set the Debug setting (Settings > System) to "ON" otherwise the connection will fail.

## Installing Flipper Application SDK

A pre-requisite is a recent [Python](https://www.python.org/) installation.

- Linux & macOS: `python3 -m pip install --upgrade fbt`
- Windows: `py -m pip install --upgrade ufbt`

## Updating the SDK

The toolchain used by the Flipper Application SDK needs to exactly match the version of the current
firmware being used.

```
ufbt update --branch 1.2.0
```

We provide an [`.env`](.env) file which will cause the `ufbt` tool link the selected SDK to the
`.ufbt` directory at the root of this repository.

## Debugging symbols

> [!NOTE]
> The Rust ARM toolchain does not provide support for [`split-debuginfo`](https://doc.rust-lang.org/rustc/codegen-options/index.html#split-debuginfo)
> at this time. For this reason the default Flipper Zero Rust configuration strips the resulting
> binary of all debugging symbols. See #218 for more information.

To build a split-debug style binary:

1. Edit `crates/.cargo/config.toml` to remove `-C debuginfo=0` and `--discard-all --strip-all` from `link-args`.
2. Edit `crates/rt/flipperzero-rt.ld` to remove the `(.debug*);` line from under the `/DISCARD/` section.
3. Build your binary with `cargo build`.
4. Move the debug sections of the binary into a separate file:
    ```
    cp my-app my-app.fap
    .ufbt/toolchain/current/arm-none-eabi/bin/objcopy --only-keep-debug my-app.fap my-app.debug
    .ufbt/toolchain/current/arm-none-eabi/bin/strip --strip-debug --strip-unneeded my-app.fap
    .ufbt/toolchain/current/arm-none-eabi/bin/objcopy --add-gnu-debuglink my-app.debug my-app.fap
    ```
5. Push the stripped binary to the Flipper Zero.

## Using Visual Studio Code for debugging

The [Coretex-Debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug) extension
for [Visual Studio Code](https://code.visualstudio.com/) works really nicely with the Flipper Zero.

This repository provides a "Attach FW (ST-Link)" debug launch action in `.vscode/launch.json`.

Under the "Run and Debug" sidebar (`Ctrl+Shift+D`), select "Attach FW (ST-Link)" from the drop-down
and then click the Green ▷ button to attach the debugger. If successful the LED on the ST-Link dongle
should now be blinking and the the Flipper Zero should appear frozen.

You should be able to see the full state of the Flipper Zero, including registers and all threads.

Click the |▷ "Continue" button (`F5`) that appears at the top of the window to resume execution.

It's highly recommended to also have the Debug Console (`Ctrl+Shift+Y`) open to see any messages
from the debugger.

> [!TIP]
> The debugger will also automatically pause execution upon starting any user app.

## Troubleshooting

### Debug symbol file not found

```
Failed to execute GDB command 'add-symbol-file -readnow crates/target/thumbv7em-none-eabihf/debug/examples/ 0x2000c5ec -s .rodata 0x2000d044': crates/target/thumbv7em-none-eabihf/debug/examples/: No such file or directory.
Failed to load debug info for AppState(name='Hello, Rust!', text_address=536921580, entry_address=536924021, other_sections={'.rodata': 536924228}, debug_link_elf='', debug_link_crc=0)
```

The Flipper SDK GDB extention uses the binary's "debuglink" value to locate the symbol file.
Unfortunately only the filename is kept, so you may need to change the value of `fap-set-debug-elf-root`
(by default this is `crates/target/thumbv7em-none-eabihf/debug/examples`).

See the value of `fap-set-debug-elf-root` for the "Attach FW (ST-Link)" in `.vscode/launch.json`.
