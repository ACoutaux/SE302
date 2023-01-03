pub mod net;

///Enumeration which represents possible issues for a welcome line fetch
pub enum IdentificationResult {
    WelcomeLine(String),
    NoWelcomeLine,
    ConnexionRefused,
}
