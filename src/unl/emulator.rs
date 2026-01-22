//! GDO Emulator - Observer that decides framing
//!
//! The GDO (Genoma Digital Observador) is responsible for:
//! - Deciding when an action starts/ends (BOF/EOF)
//! - Fragmenting large observations (BOFR/EOFR)
//! - Providing motor context (payoff matrices, conditions, etc.)
//! - Sending properly-framed data to GDC
//!
//! The GDC is stateless and processes whatever arrives between delimiters.
//! **Framing decisions and motor context belong to GDO, not GDC.**

use crate::cognitive::{
    TransportCode, ObservationReport, MotorSignatures,
    CognitiveCycle, CycleOutput, MotorContext,
};
use crate::sensory::SensorySignals;
use sha2::{Sha256, Digest};

/// Default frame size: 64KB (GDO decides this, not GDC)
pub const DEFAULT_FRAME_SIZE: usize = 64 * 1024;

/// GDO Emulator - Makes framing decisions for testing.
pub struct GdoEmulator {
    cycle_counter: u64,
    frame_size: usize,
    cognitive: CognitiveCycle,
    motor_context: MotorContext,
}

/// A complete observation with multiple frames.
#[derive(Debug, Clone)]
pub struct Observation {
    pub observation_id: [u8; 16],
    pub frames: Vec<PerceptualFrame>,
    pub total_bytes: usize,
}

/// A single framed perception (between BOF/EOF or BOFR/EOFR).
#[derive(Debug, Clone)]
pub struct PerceptualFrame {
    pub frame_id: [u8; 16],
    pub sequence: u32,
    pub is_fragment: bool,
    pub bof: TransportCode,
    pub payload: Vec<u8>,
    pub eof: TransportCode,
    pub checksum: [u8; 32],
}

/// Aggregated result from GDO processing.
#[derive(Debug, Clone)]
pub struct GdoResult {
    pub observation_id: [u8; 16],
    pub frames_processed: usize,
    pub total_bytes: usize,
    pub aggregated_signals: SensorySignals,
    pub reports: Vec<ObservationReport>,
    /// Aggregated motor scores (average across frames)
    pub motor_scores: AggregatedMotorScores,
    /// Average Craft Performance
    pub avg_craft_performance: f64,
    /// Combined DNA fingerprint
    pub combined_dna: [u8; 32],
}

/// Aggregated motor scores across frames.
#[derive(Debug, Clone, Copy, Default)]
pub struct AggregatedMotorScores {
    pub praxis: f64,
    pub nash: f64,
    pub chaos: f64,
    pub meristic: f64,
}

impl GdoEmulator {
    pub fn new() -> Self {
        Self { 
            cycle_counter: 0, 
            frame_size: DEFAULT_FRAME_SIZE,
            cognitive: CognitiveCycle::new(),
            motor_context: MotorContext::default(),
        }
    }

    pub fn with_frame_size(size: usize) -> Self {
        let mut emu = Self::new();
        emu.frame_size = size.max(64); // Minimum 64 bytes
        emu
    }

    /// Set motor context (GDO decides this based on domain knowledge)
    pub fn with_motor_context(mut self, ctx: MotorContext) -> Self {
        self.motor_context = ctx;
        self
    }

    /// GDO decides how to frame the input data.
    pub fn frame_observation(&mut self, data: &[u8]) -> Observation {
        self.cycle_counter += 1;
        
        let mut obs_id = [0u8; 16];
        obs_id[8..].copy_from_slice(&self.cycle_counter.to_le_bytes());
        
        let mut frames = Vec::new();
        
        if data.len() <= self.frame_size {
            frames.push(self.create_frame(data, 0, false));
        } else {
            let chunks: Vec<&[u8]> = data.chunks(self.frame_size).collect();
            for (i, chunk) in chunks.iter().enumerate() {
                frames.push(self.create_frame(chunk, i as u32, true));
            }
        }

        Observation {
            observation_id: obs_id,
            frames,
            total_bytes: data.len(),
        }
    }

    fn create_frame(&mut self, data: &[u8], seq: u32, is_fragment: bool) -> PerceptualFrame {
        self.cycle_counter += 1;
        
        let mut frame_id = [0u8; 16];
        frame_id[0..4].copy_from_slice(&seq.to_le_bytes());
        frame_id[8..].copy_from_slice(&self.cycle_counter.to_le_bytes());

        let (bof, eof) = if is_fragment {
            (TransportCode::BOFR, TransportCode::EOFR)
        } else {
            (TransportCode::BOF, TransportCode::EOF)
        };

        PerceptualFrame {
            frame_id,
            sequence: seq,
            is_fragment,
            bof,
            payload: data.to_vec(),
            eof,
            checksum: Self::hash(data),
        }
    }

    /// Process observation through GDC, aggregating results.
    pub fn observe(&mut self, data: &[u8]) -> GdoResult {
        let observation = self.frame_observation(data);
        self.process_observation(&observation)
    }

    /// Process a stream (file) through GDC without loading all in memory.
    pub fn observe_stream<R: std::io::Read>(&mut self, mut reader: R) -> std::io::Result<GdoResult> {
        self.cycle_counter += 1;
        
        let mut obs_id = [0u8; 16];
        obs_id[8..].copy_from_slice(&self.cycle_counter.to_le_bytes());

        let mut reports = Vec::new();
        let mut sum_entropy = 0.0f64;
        let mut m2 = 0.0f64;
        let mut prev_mean = 0.0f64;
        let mut count = 0usize;
        let mut total_bytes = 0usize;
        let mut seq = 0u32;

        // Motor score accumulators
        let mut sum_praxis = 0.0f64;
        let mut sum_nash = 0.0f64;
        let mut sum_chaos = 0.0f64;
        let mut sum_meristic = 0.0f64;
        let mut sum_cp = 0.0f64;
        let mut dna_hasher = Sha256::new();

        let mut chunk = vec![0u8; self.frame_size];
        
        loop {
            let bytes_read = reader.read(&mut chunk)?;
            if bytes_read == 0 {
                break;
            }

            let frame = self.create_frame(&chunk[..bytes_read], seq, true);
            seq += 1;
            total_bytes += bytes_read;

            // GDC processes complete cognitive cycle
            let cycle_output = self.cognitive.process(&frame.payload, &self.motor_context);
            
            count += 1;
            
            // Aggregate sensory signals (Welford's)
            sum_entropy += cycle_output.perception.signals.entropy;
            let delta = cycle_output.perception.signals.mean - prev_mean;
            prev_mean += delta / (count as f64);
            m2 += delta * (cycle_output.perception.signals.mean - prev_mean);

            // Aggregate motor scores
            sum_praxis += cycle_output.motor_scores.praxis;
            sum_nash += cycle_output.motor_scores.nash;
            sum_chaos += cycle_output.motor_scores.chaos;
            sum_meristic += cycle_output.motor_scores.meristic;
            sum_cp += cycle_output.cp_value;

            // Combine DNA
            dna_hasher.update(cycle_output.dna_fingerprint);

            reports.push(ObservationReport {
                cycle_id: frame.frame_id,
                frame_fingerprint: frame.checksum,
                protocol_markers: vec![frame.bof, frame.eof],
                motor_signatures: MotorSignatures {
                    praxis: Self::hash(&cycle_output.motor_scores.praxis.to_le_bytes()),
                    nash: Self::hash(&cycle_output.motor_scores.nash.to_le_bytes()),
                    chaos: Self::hash(&cycle_output.motor_scores.chaos.to_le_bytes()),
                    meristic: Self::hash(&cycle_output.motor_scores.meristic.to_le_bytes()),
                },
                dna_fingerprint: cycle_output.dna_fingerprint,
            });
        }

        let mut signals = SensorySignals::empty();
        let mut motor_scores = AggregatedMotorScores::default();
        let mut avg_cp = 0.0;
        
        if count > 0 {
            signals.entropy = (sum_entropy / count as f64).clamp(0.0, 1.0);
            signals.mean = prev_mean;
            signals.std_dev = if count > 1 { (m2 / (count - 1) as f64).sqrt() } else { 0.0 };

            motor_scores.praxis = sum_praxis / count as f64;
            motor_scores.nash = sum_nash / count as f64;
            motor_scores.chaos = sum_chaos / count as f64;
            motor_scores.meristic = sum_meristic / count as f64;
            avg_cp = sum_cp / count as f64;
        }

        Ok(GdoResult {
            observation_id: obs_id,
            frames_processed: count,
            total_bytes,
            aggregated_signals: signals,
            reports,
            motor_scores,
            avg_craft_performance: avg_cp,
            combined_dna: dna_hasher.finalize().into(),
        })
    }

    fn process_observation(&mut self, observation: &Observation) -> GdoResult {
        let mut reports = Vec::new();
        let mut sum_entropy = 0.0f64;
        let mut m2 = 0.0f64;
        let mut prev_mean = 0.0f64;
        let mut count = 0usize;

        let mut sum_praxis = 0.0f64;
        let mut sum_nash = 0.0f64;
        let mut sum_chaos = 0.0f64;
        let mut sum_meristic = 0.0f64;
        let mut sum_cp = 0.0f64;
        let mut dna_hasher = Sha256::new();

        for frame in &observation.frames {
            let cycle_output = self.cognitive.process(&frame.payload, &self.motor_context);
            
            count += 1;
            
            sum_entropy += cycle_output.perception.signals.entropy;
            let delta = cycle_output.perception.signals.mean - prev_mean;
            prev_mean += delta / (count as f64);
            m2 += delta * (cycle_output.perception.signals.mean - prev_mean);

            sum_praxis += cycle_output.motor_scores.praxis;
            sum_nash += cycle_output.motor_scores.nash;
            sum_chaos += cycle_output.motor_scores.chaos;
            sum_meristic += cycle_output.motor_scores.meristic;
            sum_cp += cycle_output.cp_value;

            dna_hasher.update(cycle_output.dna_fingerprint);

            reports.push(ObservationReport {
                cycle_id: frame.frame_id,
                frame_fingerprint: frame.checksum,
                protocol_markers: vec![frame.bof, frame.eof],
                motor_signatures: MotorSignatures {
                    praxis: Self::hash(&cycle_output.motor_scores.praxis.to_le_bytes()),
                    nash: Self::hash(&cycle_output.motor_scores.nash.to_le_bytes()),
                    chaos: Self::hash(&cycle_output.motor_scores.chaos.to_le_bytes()),
                    meristic: Self::hash(&cycle_output.motor_scores.meristic.to_le_bytes()),
                },
                dna_fingerprint: cycle_output.dna_fingerprint,
            });
        }

        let mut signals = SensorySignals::empty();
        let mut motor_scores = AggregatedMotorScores::default();
        let mut avg_cp = 0.0;
        
        if count > 0 {
            signals.entropy = (sum_entropy / count as f64).clamp(0.0, 1.0);
            signals.mean = prev_mean;
            signals.std_dev = if count > 1 { (m2 / (count - 1) as f64).sqrt() } else { 0.0 };

            motor_scores.praxis = sum_praxis / count as f64;
            motor_scores.nash = sum_nash / count as f64;
            motor_scores.chaos = sum_chaos / count as f64;
            motor_scores.meristic = sum_meristic / count as f64;
            avg_cp = sum_cp / count as f64;
        }

        GdoResult {
            observation_id: observation.observation_id,
            frames_processed: observation.frames.len(),
            total_bytes: observation.total_bytes,
            aggregated_signals: signals,
            reports,
            motor_scores,
            avg_craft_performance: avg_cp,
            combined_dna: dna_hasher.finalize().into(),
        }
    }

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

impl Default for GdoEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl PerceptualFrame {
    pub fn is_valid(&self) -> bool {
        (self.bof == TransportCode::BOF && self.eof == TransportCode::EOF) ||
        (self.bof == TransportCode::BOFR && self.eof == TransportCode::EOFR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_observation_single_frame() {
        let mut gdo = GdoEmulator::new();
        let obs = gdo.frame_observation(&[1, 2, 3]);
        assert_eq!(obs.frames.len(), 1);
        assert!(!obs.frames[0].is_fragment);
    }

    #[test]
    fn test_large_observation_multiple_frames() {
        let mut gdo = GdoEmulator::with_frame_size(100);
        let data = vec![0u8; 350];
        let obs = gdo.frame_observation(&data);
        assert_eq!(obs.frames.len(), 4);
        assert!(obs.frames[0].is_fragment);
    }

    #[test]
    fn test_observe_aggregates_results() {
        let mut gdo = GdoEmulator::with_frame_size(100);
        let data = vec![128u8; 500];
        let result = gdo.observe(&data);
        assert_eq!(result.frames_processed, 5);
        assert_eq!(result.total_bytes, 500);
        assert_eq!(result.reports.len(), 5);
        // Verify motor scores are calculated
        assert!(result.motor_scores.praxis >= 0.0);
        assert!(result.avg_craft_performance >= 0.0);
    }

    #[test]
    fn test_dna_generated() {
        let mut gdo = GdoEmulator::new();
        let result = gdo.observe(&[1, 2, 3, 4, 5]);
        assert_ne!(result.combined_dna, [0u8; 32]);
        assert!(!result.reports.is_empty());
        assert_ne!(result.reports[0].dna_fingerprint, [0u8; 32]);
    }

    #[test]
    fn test_determinism() {
        let mut gdo = GdoEmulator::with_frame_size(100);
        let data: Vec<u8> = (0..500).map(|i| (i % 256) as u8).collect();
        let r1 = gdo.observe(&data);
        let r2 = gdo.observe(&data);
        assert_eq!(r1.aggregated_signals.entropy, r2.aggregated_signals.entropy);
        assert_eq!(r1.combined_dna, r2.combined_dna);
    }
}


