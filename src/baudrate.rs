use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Baudrate {
    B300 = 300,
    B1200 = 1200,
    B2400 = 2400,
    B4800 = 4800,
    B9600 = 9600,
    B19200 = 19200,
    B38400 = 38400,
    B57600 = 57600,
    B74880 = 74880,
    B115200 = 115200,
    B230400 = 230400,
    B250000 = 250000,
    B500000 = 500000,
    B1000000 = 1000000,
    B2000000 = 2000000,
}

impl Default for Baudrate {
    fn default() -> Self {
        Baudrate::B115200
    }
}

impl From<u32> for Baudrate {
    fn from(int: u32) -> Self {
        match int {
            300 => Baudrate::B300,
            1200 => Baudrate::B1200,
            2400 => Baudrate::B2400,
            4800 => Baudrate::B4800,
            9600 => Baudrate::B9600,
            19200 => Baudrate::B19200,
            38400 => Baudrate::B38400,
            57600 => Baudrate::B57600,
            74880 => Baudrate::B74880,
            115200 => Baudrate::B115200,
            230400 => Baudrate::B230400,
            250000 => Baudrate::B250000,
            500000 => Baudrate::B500000,
            1000000 => Baudrate::B1000000,
            2000000 => Baudrate::B2000000,
            _ => panic!("Invalid baudrate given {}", int)
        }
    }
}

impl Display for Baudrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (*self) as u32
        )
    }
}

impl Baudrate {
    pub(crate) const ALL: [Baudrate; 15] = [
        Baudrate::B300,
        Baudrate::B1200,
        Baudrate::B2400,
        Baudrate::B4800,
        Baudrate::B9600,
        Baudrate::B19200,
        Baudrate::B38400,
        Baudrate::B57600,
        Baudrate::B74880,
        Baudrate::B115200,
        Baudrate::B230400,
        Baudrate::B250000,
        Baudrate::B500000,
        Baudrate::B1000000,
        Baudrate::B2000000
    ];

    pub fn _from_string(name: &str) -> Option<Baudrate> {
        println!("{}", name);
        match name {
            "300" => Some(Baudrate::B300),
            "1200" => Some(Baudrate::B1200),
            "2400" => Some(Baudrate::B2400),
            "4800" => Some(Baudrate::B4800),
            "9600" => Some(Baudrate::B9600),
            "19200" => Some(Baudrate::B19200),
            "38400" => Some(Baudrate::B38400),
            "57600" => Some(Baudrate::B57600),
            "74880" => Some(Baudrate::B74880),
            "115200" => Some(Baudrate::B115200),
            "230400" => Some(Baudrate::B230400),
            "250000" => Some(Baudrate::B250000),
            "500000" => Some(Baudrate::B500000),
            "1000000" => Some(Baudrate::B1000000),
            "2000000" => Some(Baudrate::B2000000),
            _ => None
        }
    }
}
