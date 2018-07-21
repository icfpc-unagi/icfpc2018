use *;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Command {
    Halt,
    Wait,
    SMove(P),
    LMove(P, P),
    FusionP(P),
    FusionS(P),
    Fission(P, usize),
    Fill(P),
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Halt => {
                "HALT".to_owned()
            },
            Command::Wait => {
                "WAIT".to_owned()
            },
            Command::SMove(d) => {
                format!("SMOVE {}", d.fmt_ld())
            },
            Command::LMove(d1, d2) => {
                format!("LMOVE {} {}", d1.fmt_ld(), d2.fmt_ld())
            },
            Command::FusionP(p) => {
                format!("FUSIONP {} {} {}", p.x, p.y, p.z)
            },
            Command::FusionS(p) => {
                format!("FUSIONS {} {} {}", p.x, p.y, p.z)
            },
            Command::Fission(p, m) => {
                format!("FISSION {} {} {} {}", p.x, p.y, p.z, m)
            },
            Command::Fill(p) => {
                format!("FILL {} {} {}", p.x, p.y, p.z)
            },
        }
    }
}