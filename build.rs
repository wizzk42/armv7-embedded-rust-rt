use std::{
    env,
    error::Error,
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
    string::ToString,
};

use cc::Build;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Device<'a> {
    KNOWN(Description<'a>)
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Description<'a> {
    pub name: &'a str,
    pub abi: &'a str,
    pub fpu: bool,
    pub target: Target<'a>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Target<'a> {
    ARM { version: u8, class: &'a str }
}

static __SUPPORTED_DEVICES: [Device; 1] = [
    Device::KNOWN(Description {
        name: "lm3s6965",
        abi: "thumbv7m-none-eabi",
        fpu: false,
        target: Target::ARM {
            version: 7,
            class: "armv7",
        },
    })
];

impl FromStr for Device<'_> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            __SUPPORTED_DEVICES
                .iter()
                .find(|d: &&Device| {
                    match d {
                        Device::KNOWN(_desc) => _desc.name == s
                    }
                })
                .expect("device not found")
                .clone()
        )
    }
}

impl Display for Device<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Device::KNOWN(x) => write!(f, "{0}", x.name)
        }
    }
}

impl FromStr for Target<'_> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "armv6" => Ok(Target::ARM { version: 6, class: "armv6" }),
            "armv7" => Ok(Target::ARM { version: 7, class: "armv7" }),
            "armv8" => Ok(Target::ARM { version: 8, class: "armv8" }),
            _ => Ok(Target::ARM { version: 7, class: "armv7" }),
        }
    }
}

impl Display for Target<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Target::ARM { class, .. } => write!(f, "{0}", *class)
        }
    }
}

fn assemble<'a>(_device: &'a Device, _out: &PathBuf) -> Result<(), Box<dyn Error>> {
    fn _prepare<'b>(_desc: &'b Description, _out: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut f: File = File::create(_out.join("arm.s"))?;

        if let Ok(custom_files_iter) = fs::read_dir(
            format!("asm/{0}", _desc.name)
        ) {
            for file in custom_files_iter {
                let _ = f.write(fs::read(file?.path())?.as_ref())?;
            }
        }

        if let Ok(common_files_iter) = match _desc.target {
            Target::ARM { class, .. } => {
                fs::read_dir(
                    format!("asm/{0}", class)
                )
            }
        } {
            for file in common_files_iter {
                let _ = f.write(fs::read(file?.path())?.as_ref())?;
            }
        };

        Ok(())
    }

    fn _build(_out: &PathBuf) {
        Build::new().file(_out.join("arm.s")).compile("arm");
        println!("cargo:rerun-if-changed=asm/*/*.s");
    }

    match _device {
        Device::KNOWN(desc) => {
            _prepare(desc, _out)?;
        }
    }

    _build(_out);
    Ok(())
}

fn link<'a>(_device: &'a Device, _out: &PathBuf) -> Result<(), Box<dyn Error>> {
    fn _prepare<'b>(_desc: &'b Description, _out: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut f: File = File::create(
            _out.join("link.x")
        ).expect("unable to create link.x script");

        if let Ok(all_files_iter) = fs::read_dir(
            format!("devices/{0}", _desc.name.to_string())
        ) {
            for file in all_files_iter {
                let ref path = file?.path();
                let _ = f.write(
                    fs::read(path)
                        .expect("file is not available")
                        .as_ref()
                ).expect("could not be written");

                println!("cargo:rerun-if-changed={:?}", path);
            }
        }
        let _ = f.write(
            include_bytes!("devices/common/link.x.in")
        ).expect("linker script template");

        Ok(())
    }

    match _device {
        Device::KNOWN(_desc) => { _prepare(_desc, _out)? }
    }

    println!("cargo:rerun-if-changed=link.x.in");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out = PathBuf::from(
        env::var_os("OUT").unwrap_or_default()
    );

    let device = Device::from_str(
        env::var("DEVICE")
            .unwrap_or("lm3s6965".to_string())
            .to_lowercase()
            .as_str()
    ).expect("device not recognized");

    has_fpu(&device);

    eprintln!("out_dir: {0}", out.to_str().unwrap_or_default());
    eprintln!("device: {0}", device);

    println!("cargo:rustc-link-search={0}", out.display());
    println!("cargo:rerun-if-changed=build.rs");

    assemble(&device, &out)?;
    link(&device, &out)?;

    Ok(())
}

fn has_fpu(_device: &Device) {
    match _device {
        Device::KNOWN(desc) if desc.fpu => println!("cargo:rustc-cfg=has_fpu"),
        _ => {}
    }
}
