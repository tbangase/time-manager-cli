use clap::ValueEnum;
use strum::Display;

#[derive(ValueEnum, Debug, Clone, Copy, Display)]
#[strum(serialize_all = "camelCase")]
pub enum Methods {
    Gm,
    Gn,
    #[strum(serialize = "rest")]
    Afk,
    #[strum(serialize = "rest")]
    Back,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_test() {
        assert_eq!(Methods::Gm.to_string(), "gm");
        assert_eq!(Methods::Gn.to_string(), "gn");
        assert_eq!(Methods::Afk.to_string(), "rest");
        assert_eq!(Methods::Back.to_string(), "rest");
    }
}
