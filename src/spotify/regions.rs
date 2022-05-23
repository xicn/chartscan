#[derive(Debug, PartialEq)]
pub enum Regions {
    GLOBAL,
    US,
    GB,
    AU,
    CA,
    FR,
    NL,
    FI,
    NO,
    DK,
    SE,
    CH,
    IE,
    NZ,
    MY,
    SG,
    JP,
    IN,
    PH,
    ID,
    DE,
    IT,
    AT,
    BE,
    HU,
    PT,
    ES,
    BR,
    CO,
    MX,
    AR,
    PL,
    CL,
    TR,
    NOTVALID,
}

impl From<u8> for Regions {
    fn from(num: u8) -> Self {
        match num {
            0 => Regions::GLOBAL,
            1 => Regions::US,
            2 => Regions::GB,
            3 => Regions::AU,
            4 => Regions::CA,
            5 => Regions::FR,
            6 => Regions::NL,
            7 => Regions::FI,
            8 => Regions::NO,
            9 => Regions::DK,
            10 => Regions::SE,
            11 => Regions::CH,
            12 => Regions::IE,
            13 => Regions::NZ,
            14 => Regions::MY,
            15 => Regions::SG,
            16 => Regions::JP,
            17 => Regions::IN,
            18 => Regions::PH,
            19 => Regions::ID,
            20 => Regions::DE,
            21 => Regions::IT,
            22 => Regions::AT,
            23 => Regions::BE,
            24 => Regions::HU,
            25 => Regions::PT,
            26 => Regions::ES,
            27 => Regions::BR,
            28 => Regions::CO,
            29 => Regions::MX,
            30 => Regions::AR,
            31 => Regions::PL,
            32 => Regions::CL,
            33 => Regions::TR,
            _ => Regions::NOTVALID,
        }
    }
}

impl From<&str> for Regions {
    fn from(st: &str) -> Self {
        match st {
            "global" => Regions::GLOBAL,
            "us" => Regions::US,
            "gb" => Regions::GB,
            "au" => Regions::AU,
            "ca" => Regions::CA,
            "fr" => Regions::FR,
            "nl" => Regions::NL,
            "fi" => Regions::FI,
            "no" => Regions::NO,
            "dk" => Regions::DK,
            "se" => Regions::SE,
            "ch" => Regions::CH,
            "ie" => Regions::IE,
            "nz" => Regions::NZ,
            "my" => Regions::MY,
            "sg" => Regions::SG,
            "jp" => Regions::JP,
            "in" => Regions::IN,
            "ph" => Regions::PH,
            "id" => Regions::ID,
            "de" => Regions::DE,
            "it" => Regions::IT,
            "at" => Regions::AT,
            "be" => Regions::BE,
            "hu" => Regions::HU,
            "pt" => Regions::PT,
            "es" => Regions::ES,
            "br" => Regions::BR,
            "co" => Regions::CO,
            "mx" => Regions::MX,
            "ar" => Regions::AR,
            "pl" => Regions::PL,
            "cl" => Regions::CL,
            "tr" => Regions::TR,
            _ => Regions::NOTVALID,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn from_str_1() -> Result<(), Box<dyn Error>> {
        let st = "global";
        assert_eq!(Regions::GLOBAL, From::from(st));
        Ok(())
    }

    #[test]
    fn from_str_2() -> Result<(), Box<dyn Error>> {
        let st = "at";
        assert_eq!(Regions::AT, From::from(st));
        Ok(())
    }

    #[test]
    fn from_str_3() -> Result<(), Box<dyn Error>> {
        let st = "at ";
        assert_eq!(Regions::NOTVALID, From::from(st));
        Ok(())
    }

    #[test]
    fn from_u8_1() -> Result<(), Box<dyn Error>> {
        assert_eq!(Regions::GLOBAL, From::from(0));
        Ok(())
    }

    #[test]
    fn from_u8_2() -> Result<(), Box<dyn Error>> {
        assert_eq!(Regions::US, From::from(1));
        Ok(())
    }

    #[test]
    fn from_u8_3() -> Result<(), Box<dyn Error>> {
        assert_eq!(Regions::NOTVALID, From::from(40));
        Ok(())
    }
}
