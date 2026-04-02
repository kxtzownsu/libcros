# libcros
A Rust library that gives a high-level API for interacting with Chrome(OS) devices.

>[!IMPORTANT]
>Some parts of the library may be locked behind `features`, <br />
>please refer to the documentation to see which features you need to <br />
>enable to use some parts of the library. 

## Installation
```
cargo add libcros
```

## Usage
First, you need to install the package to your project. See [Installation](#Installation)

Then, look at some examples in [examples/](examples/). There are examples on how to use the following features:
- Logging
- Tlcl (TPM1.2 & TPM2.0)


## Credits
- [appleflyer](https://github.com/appleflyerv3) - intial Tlcl rust port. wouldn't have been able to start w/out them
- [Google](https://chromium.googlesource.com/chromiumos/platform/vboot_reference/+/e388d1f93c9573a79a04b633c3a0569ddbce6c94/firmware/lib/) - writing Tlcl.
- [zeglol](https://github.com/ZegLolTheThirtySixth) - writing the initial version of libargs in C. writing [AGENTS.md](/AGENTS.md)