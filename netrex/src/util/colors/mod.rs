pub mod console;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	DarkGray,
	Gray,
	White,
	DarkPurple,
	LightPurple,
	Blue,
	DarkBlue,
	DarkAqua,
	Aqua,
	Green,
	DarkGreen,
	Yellow,
	Gold,
	Red,
	DarkRed,
	MinecoinGold,
	MaterialQuartz,
	MaterialIron,
	MaterialNetherite,
	MaterialRedstone,
	MaterialCopper,
	MaterialGold,
	MaterialEmerald,
	MaterialDiamond,
	MaterialLapis,
	MaterialAmethyst
}

impl Color {
	pub fn to_ansi(&self) -> &str {
		match *self {
            Color::Black => "\\e[0;30m",
            Color::DarkBlue => "\\e[0;34m",
            Color::DarkGreen => "\\e[0;32m",
            Color::DarkAqua => "\\e[0;36m",
            Color::DarkRed => "\\e[0;31m",
            Color::DarkPurple => "\\e[0;35m",
            Color::Gold => "\\e[0;33m",
            Color::Gray => "\\e[0;37m",
            Color::DarkGray => "\\e[0;90m",
            Color::Blue => "\\e[0;94m",
            Color::Green => "\\e[0;92m",
            Color::Aqua => "\\e[0;96m",
            Color::Red => "\\e[0;91m",
            Color::LightPurple => "\\e[0;95m",
            Color::Yellow => "\\e[0;93m",
            Color::White => "\\e[0;97m",
            Color::MinecoinGold => "\033[38;2;221;5m",
            Color::MaterialQuartz => "\033[38;2;227;209m",
            Color::MaterialIron => "\033[38;2;206;202;202m",
            Color::MaterialNetherite => "\033[38;2;68;58;59m",
            Color::MaterialRedstone => "\033[38;2;151;22;7m",
            Color::MaterialCopper => "\033[38;2;180;104;77m",
            Color::MaterialGold => "\033[38;2;222;177;45m",
            Color::MaterialEmerald => "\033[38;2;17;160;54m",
            Color::MaterialDiamond => "\033[38;2;44;186;168m",
            Color::MaterialLapis => "\033[38;2;33;73;123m",
            Color::MaterialAmethyst => "\033[38;2;154;92;198m"
		}
	}
}

