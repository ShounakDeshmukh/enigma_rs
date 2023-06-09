#[derive(Debug,Clone,)]
pub struct Rotor {
    pub wiring: String,
    pub notchs: String,
    pub name: String,
}

impl Rotor {
    pub fn load_default_rotors() -> [Rotor; 8] {
        let ROTOR_I = Rotor {
            wiring: "EKMFLGDQVZNTOWYHXUSPAIBRCJ".into(),
            notchs: "R".into(),
            name: "I".into(),
        };

        let ROTOR_II = Rotor {
            wiring: "AJDKSIRUXBLHWTMCQGZNPYFVOE".into(),
            notchs: "F".into(),
            name: "II".into(),
        };
        let ROTOR_III = Rotor {
            wiring: "BDFHJLCPRTXVZNYEIWGAKMUSQO".into(),
            notchs: "W".into(),
            name: "III".into(),
        };
        let ROTOR_IV = Rotor {
            wiring: "ESOVPZJAYQUIRHXLNFTGKDCMWB".into(),
            notchs: "K".into(),
            name: "IV".into(),
        };
        let ROTOR_V = Rotor {
            wiring: "VZBRGITYUPSDNHLXAWMJQOFECK".into(),
            notchs: "A".into(),
            name: "V".into(),
        };

        let ROTOR_VI = Rotor {
            wiring: "JPGVOUMFYQBENHZRDKASXLICTW".into(),
            notchs: "AN".into(),
            name: "VI".into(),
        };
        let ROTOR_VII = Rotor {
            wiring: "NZJHGRCXMYSWBOUFAIVLPEKQDT".into(),
            notchs: "AN".into(),
            name: "VII".into(),
        };
        let ROTOR_VIII = Rotor {
            wiring: "FKQHTLXOCBJSPDZRAMEWNIUYGV".into(),
            notchs: "AN".into(),
            name: "VIII".into(),
        };
        [
            ROTOR_I, ROTOR_II, ROTOR_III, ROTOR_IV, ROTOR_V, ROTOR_VI, ROTOR_VII, ROTOR_VIII,
        ]
    }
}
