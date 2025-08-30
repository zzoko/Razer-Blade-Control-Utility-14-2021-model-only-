use crate::feature;

// model_number_prefix shall conform to https://mysupport.razer.com/app/answers/detail/a_id/5481
#[derive(Debug, Clone)]
pub struct Descriptor {
    pub model_number_prefix: &'static str,
    pub name: &'static str,
    pub pid: u16,
    pub features: &'static [&'static str],
}

pub const SUPPORTED: &[Descriptor] = &[
    Descriptor {
        model_number_prefix: "RZ09-0370C", //model numbe MUSt match the laptop, see link above
        name: "Razer Blade 14” (2021) Black", //muodel name 
        pid: 0x0270, //run ".\razer-cli2.exe enumerate" to get the PID
        //note: enabling additional features that arent supported by the hardware will not work, see razer synapse to get an idea of what feature your model has
        features: &[
          //  "battery-care", //not working on '21 models
            "fan",
            "kbd-backlight",
            "lid-logo",
            "lights-always-on",
            "perf",
        ],

    // add more models down here

    },
];

const _VALIDATE_FEATURES: () = {
    crate::const_for! { device in SUPPORTED => {
        feature::validate_features(device.features);
    }}
};
