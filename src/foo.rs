impl std::str::FromStr for QClass {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "IN" => Ok(QClass::IN),
            "CS" => Ok(QClass::CS),
            "CH" => Ok(QClass::CH),
            "HS" => Ok(QClass::HS),
            "ANY" => Ok(QClass::ANY),
            _ => {
                if let Some(n) = s.strip_prefix("CLASS") {
                    if let Ok(x) = n.parse::<u16>() {
                        Ok(QClass::CLASS(x))
                    } else {
                        Err(format!("no variant corresponding to value '{}'", s))
                    }
                } else {
                    Err(format!("no variant corresponding to value '{}'", s))
                }
            }
            _ => Err(format!("no variant corresponding to value '{}'", s)),
        }
    }
}
