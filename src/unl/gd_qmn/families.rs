//! GD-QMN Code Families (F1-F6)
//!
//! Each family groups related cognitive operations.

/// F1: Transduction - Input processing codes
pub mod f1 {
    pub const PERCEIVE_RAW: u16 = 0x0100;
    pub const PERCEIVE_FRAMED: u16 = 0x0101;
    pub const TRANSDUCE_CARRIER: u16 = 0x0110;
    pub const TRANSDUCE_PATTERN: u16 = 0x0111;
    pub const TRANSDUCE_STRUCTURE: u16 = 0x0112;
}

/// F2: Composition - Assembly codes
pub mod f2 {
    pub const COMPOSE_SIGNAL: u16 = 0x0200;
    pub const COMPOSE_PATTERN: u16 = 0x0201;
    pub const COMPOSE_ACTION: u16 = 0x0202;
    pub const COMPOSE_DNA: u16 = 0x0210;
}

/// F3: Motors - Cognitive motor codes
pub mod f3 {
    pub const MOTOR_PRAXIS: u16 = 0x0300;
    pub const MOTOR_NASH: u16 = 0x0301;
    pub const MOTOR_CHAOS: u16 = 0x0302;
    pub const MOTOR_MERISTIC: u16 = 0x0303;
    pub const MOTOR_VETO: u16 = 0x030F;
}

/// F4: Emission - Output codes
pub mod f4 {
    pub const EMIT_DNA: u16 = 0x0400;
    pub const EMIT_REPORT: u16 = 0x0401;
    pub const EMIT_SIGNAL: u16 = 0x0402;
}

/// F5: Scale - Scaling/normalization codes
pub mod f5 {
    pub const SCALE_LINEAR: u16 = 0x0500;
    pub const SCALE_LOG: u16 = 0x0501;
    pub const SCALE_SIGMOID: u16 = 0x0502;
    pub const NORMALIZE: u16 = 0x0510;
}

/// F6: Operational - Transport/control codes
pub mod f6 {
    // Transport
    pub const BOF: u16 = 0x0001;
    pub const EOF: u16 = 0x0002;
    pub const BOFR: u16 = 0x0003;
    pub const EOFR: u16 = 0x0004;
    pub const VERSION: u16 = 0x0010;
    pub const CHECKSUM: u16 = 0x0011;
    pub const NOP: u16 = 0x0000;
    
    // Origin markers (AO-18: Autorreferência Cognitiva)
    // Used to mark cognitive state provenance
    pub const ORIGIN_EXTERNAL: u16 = 0x0020;   // State from perception
    pub const ORIGIN_INTERNAL: u16 = 0x0021;   // State from MCI/Meristic
    pub const ORIGIN_RECOMBINED: u16 = 0x0022; // State from cognitive recombination
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f6_transport_codes() {
        assert_eq!(f6::BOF, 0x0001);
        assert_eq!(f6::EOF, 0x0002);
    }

    #[test]
    fn test_f6_origin_codes() {
        // AO-18: Autorreferência Cognitiva
        assert_eq!(f6::ORIGIN_EXTERNAL, 0x0020);
        assert_eq!(f6::ORIGIN_INTERNAL, 0x0021);
        assert_eq!(f6::ORIGIN_RECOMBINED, 0x0022);
    }

    #[test]
    fn test_motor_codes_in_f3() {
        assert!(f3::MOTOR_PRAXIS >= 0x0300);
        assert!(f3::MOTOR_MERISTIC < 0x0400);
    }
}
