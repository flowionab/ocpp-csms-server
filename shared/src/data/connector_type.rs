use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ConnectorType {
    /// Combined Charging System 1 (captive cabled) a.k.a. Combo 1
    ConnectorCcs1,
    /// Combined Charging System 2 (captive cabled) a.k.a. Combo 2
    ConnectorCcs2,
    /// JARI G105-1993 (captive cabled) a.k.a. CHAdeMO
    ConnectorG105,
    /// Tesla Connector (captive cabled)
    ConnectorTesla,
    /// IEC62196-2 Type 1 connector (captive cabled) a.k.a. J1772
    ConnectorType1,
    /// IEC62196-2 Type 2 connector (captive cabled) a.k.a. Mennekes connector
    ConnectorType2,
    /// 16A 1 phase IEC60309 socket
    Socket3091p16a,
    /// 32A 1 phase IEC60309 socket
    Socket3091p32a,
    /// 16A 3 phase IEC60309 socket
    Socket3093p16a,
    /// 32A 3 phase IEC60309 socket
    Socket3093p32a,
    /// UK domestic socket a.k.a. 13Amp
    SocketBs1361,
    /// CEE 7/7 16A socket. May represent 7/4 & 7/5 a.k.a Schuko
    SocketCee77,
    /// IEC62196-2 Type 2 socket a.k.a. Mennekes connector
    SocketType2,
    /// IEC62196-2 Type 2 socket a.k.a. Scame
    SocketType3,
    /// Other single phase (domestic) sockets not mentioned above, rated at no more than 16A. CEE7/17, AS3112, NEMA 5-15, NEMA 5-20, JISC8303, TIS166, SI 32, CPCS-CCC, SEV1011, etc.
    Other1phMax16a,
    /// Other single phase sockets not mentioned above (over 16A)
    Other1phOver16a,
    /// Other 3 phase sockets not mentioned above. NEMA14-30, NEMA14-50.
    Other3ph,
    /// Pantograph connector
    Pantograph,
    /// Wireless inductively coupled connection (generic)
    WirelessInductive,
    /// Wireless resonant coupled connection (generic)
    WirelessResonant,
    /// Yet to be determined (e.g. before plugged in)
    Undetermined,
    /// Unknown; not determinable
    #[default]
    Unknown,
}
