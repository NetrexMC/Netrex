#[allow(unused_macros)]
#[macro_export]
macro_rules! ansii_format {
	($color: expr, $terminator: expr, $t: expr) => {
		format!("{}{}{}", $color, $t, $terminator)
	};
}

#[macro_export]
macro_rules! ansii_black {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;30m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_dark_blue {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;34m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_dark_green {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;32m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_dark_aqua {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;36m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_dark_red {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;31m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_dark_purple {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;35m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_gold {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;33m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_gray {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[0;37m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_dark_gray {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;30m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_blue {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;34m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_green {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;32m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_aqua {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;36m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_red {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;31m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_light_purple {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;35m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_yellow {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;33m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_white {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[1;37m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_minecoin_gold {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;221;5m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_quartz {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;227;209m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_iron {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;206;202;202m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_netherite {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;68;58;59m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_redstone {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;151;22;7m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_copper {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;180;104;77m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_gold {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;222;177;45m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_emerald {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;17;160;54m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_diamond {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;44;186;168m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_lapis {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;33;73;123m", "\x1b[0m", $($arg)*)
    };
}

#[macro_export]
macro_rules! ansii_material_amethyst {
    ($($arg:tt)*) => {
        crate::ansii_format!("\x1b[38;2;154;92;198m", "\x1b[0m", $($arg)*)
    };
}