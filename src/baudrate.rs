use core::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Baudrate {
    B300,
    B1200,
    B2400,
    B4800,
    B9600,
    B19200,
    B38400,
    B57600,
    B74880,
    B115200,
    B230400,
    B250000,
    B500000,
    B1000000,
    B2000000
}

impl Default for Baudrate{
    fn default() -> Self {
        Baudrate::B115200
    }
}

impl Display for Baudrate{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Baudrate::B300 => "300",
                Baudrate::B1200 => "1200",
                Baudrate::B2400 => "2400",
                Baudrate::B4800 => "4800",
                Baudrate::B9600 => "9600",
                Baudrate::B19200 => "19200",
                Baudrate::B38400 => "38400",
                Baudrate::B57600 => "57600",
                Baudrate::B74880 => "74880",
                Baudrate::B115200 => "115200",
                Baudrate::B230400 => "230400",
                Baudrate::B250000 => "250000",
                Baudrate::B500000 => "500000",
                Baudrate::B1000000 => "1000000",
                Baudrate::B2000000 => "2000000"
            }
        )
    }
}

impl Baudrate{
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
}