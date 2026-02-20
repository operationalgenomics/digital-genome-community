//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GD-QMN ISA Executor (v0.8.0)
//! Author: Claude (sob supervisão de Carlos Eduardo Favini)
//! Date: 2026-02-15
//! Version: 0.8.0
//! Description: Executor da ISA GD-QMN com 9 opcodes (5 core + 4 wave).
//!              Motor Vibracional Executável (MVE).
//!
//! Canon References:
//!   - LEI-QMN-ISA-01: ISA Mínima v1.0.0
//!   - LEI-QMN-MODE-01: Natureza do Efeito
//!   - LEI-QMN-VETO-01: Veto Ontológico
//!   - AF-1: Não-Simulação Cognitiva
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2026-02-15 - Claude - Criação inicial (v0.8.0)
//! --------------------------

use super::instruction::{GdQmnInstruction, EffectMode};
use super::core::{Opcode, CoreOpcode, WaveOpcode, MotorOutput};
use std::collections::HashMap;

// =============================================================================
// CONTEXTO DE EXECUÇÃO
// =============================================================================

/// Contexto de execução do GD-QMN.
///
/// Mantém estado mínimo necessário para execução determinística.
/// Não é simulação cognitiva (AF-1) - é estado funcional.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Estados nomeados (para Reference)
    states: HashMap<String, Vec<u8>>,
    
    /// Resultado da última execução
    last_result: Option<Vec<u8>>,
    
    /// Pilha de execução (para Combine)
    stack: Vec<Vec<u8>>,
}

impl ExecutionContext {
    /// Cria novo contexto vazio.
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            last_result: None,
            stack: Vec::new(),
        }
    }
    
    /// Define estado nomeado.
    pub fn set_state(&mut self, name: String, data: Vec<u8>) {
        self.states.insert(name, data);
    }
    
    /// Obtém estado nomeado.
    pub fn get_state(&self, name: &str) -> Option<&Vec<u8>> {
        self.states.get(name)
    }
    
    /// Define último resultado.
    pub fn set_result(&mut self, data: Vec<u8>) {
        self.last_result = Some(data);
    }
    
    /// Obtém último resultado.
    pub fn get_result(&self) -> Option<&Vec<u8>> {
        self.last_result.as_ref()
    }
    
    /// Push na pilha.
    pub fn push_stack(&mut self, data: Vec<u8>) {
        self.stack.push(data);
    }
    
    /// Pop da pilha.
    pub fn pop_stack(&mut self) -> Option<Vec<u8>> {
        self.stack.pop()
    }
    
    /// Limpa pilha.
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// EXECUTOR ISA
// =============================================================================

/// Motor Vibracional Executável (MVE).
///
/// Executa instruções GD-QMN conforme LEI-QMN-ISA-01.
pub struct GdQmnExecutor {
    context: ExecutionContext,
}

impl GdQmnExecutor {
    /// Cria novo executor.
    pub fn new() -> Self {
        Self {
            context: ExecutionContext::new(),
        }
    }
    
    /// Executa instrução GD-QMN.
    ///
    /// Retorna MotorOutput:
    /// - Value(amplitude): execução bem-sucedida
    /// - Veto: erro ou rejeição ontológica
    pub fn execute(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        // Inferir Mode a partir do UID (LEI-QMN-MODE-01)
        let mode = Self::infer_mode(instruction);
        
        // Executar conforme opcode
        match instruction.envelope.opcode {
            Opcode::Core(core_op) => self.execute_core(core_op, instruction, mode),
            Opcode::Wave(wave_op) => self.execute_wave(wave_op, instruction, mode),
        }
    }
    
    /// Infere Mode a partir do UID (LEI-QMN-MODE-01).
    ///
    /// Mode não é transportado no bytecode - é inferido.
    fn infer_mode(instruction: &GdQmnInstruction) -> EffectMode {
        // Lógica de inferência baseada em UID e contexto
        match instruction.envelope.opcode {
            Opcode::Core(CoreOpcode::Void) => EffectMode::Inhibitory,
            Opcode::Core(CoreOpcode::State) => EffectMode::Structural,
            Opcode::Core(CoreOpcode::Reference) => EffectMode::Relational,
            Opcode::Core(CoreOpcode::Combine) => EffectMode::Structural,
            Opcode::Core(CoreOpcode::Derive) => EffectMode::Transformative,
            Opcode::Wave(WaveOpcode::Sync) => EffectMode::Structural,
            Opcode::Wave(WaveOpcode::Fork) => EffectMode::Structural,
            Opcode::Wave(WaveOpcode::Amplify) => EffectMode::Transformative,
            Opcode::Wave(WaveOpcode::Attenuate) => EffectMode::Transformative,
        }
    }
    
    /// Executa opcode CORE.
    fn execute_core(
        &mut self,
        opcode: CoreOpcode,
        instruction: &GdQmnInstruction,
        _mode: EffectMode, // Não usado ainda, reservado para v0.9.0+
    ) -> MotorOutput {
        match opcode {
            CoreOpcode::Void => self.execute_void(instruction),
            CoreOpcode::State => self.execute_state(instruction),
            CoreOpcode::Reference => self.execute_reference(instruction),
            CoreOpcode::Combine => self.execute_combine(instruction),
            CoreOpcode::Derive => self.execute_derive(instruction),
        }
    }
    
    /// Executa opcode WAVE.
    fn execute_wave(
        &mut self,
        opcode: WaveOpcode,
        instruction: &GdQmnInstruction,
        _mode: EffectMode, // Não usado ainda, reservado para v0.9.0+
    ) -> MotorOutput {
        match opcode {
            WaveOpcode::Sync => self.execute_sync(instruction),
            WaveOpcode::Fork => self.execute_fork(instruction),
            WaveOpcode::Amplify => self.execute_amplify(instruction),
            WaveOpcode::Attenuate => self.execute_attenuate(instruction),
        }
    }
    
    // =========================================================================
    // CORE OPCODES
    // =========================================================================
    
    /// VOID: Ausência ontológica.
    ///
    /// Não é NOOP - é vazio intencional.
    /// Retorna Veto (estado ontológico de não-manifestação).
    fn execute_void(&mut self, _instruction: &GdQmnInstruction) -> MotorOutput {
        MotorOutput::Veto
    }
    
    /// STATE: Declarar/emitir estado.
    ///
    /// Armazena payload como estado nomeado.
    /// Nome extraído dos primeiros 32 bytes do cargo (se houver).
    fn execute_state(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        let payload = &instruction.cargo.payload;
        
        if payload.is_empty() {
            return MotorOutput::Veto;
        }
        
        // Extrai nome (primeiros 32 bytes como hash, ou todo payload se < 32)
        let name = if payload.len() >= 32 {
            hex::encode(&payload[..32])
        } else {
            hex::encode(payload)
        };
        
        // Armazena estado
        self.context.set_state(name, payload.clone());
        self.context.set_result(payload.clone());
        
        // Retorna amplitude como sucesso
        MotorOutput::Value(instruction.envelope.amplitude)
    }
    
    /// REFERENCE: Apontar para estado existente.
    ///
    /// Busca estado pelo nome no cargo.
    /// Retorna Veto se estado não existe.
    fn execute_reference(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        let name = String::from_utf8_lossy(&instruction.cargo.payload).to_string();
        
        match self.context.get_state(&name) {
            Some(data) => {
                self.context.set_result(data.clone());
                MotorOutput::Value(instruction.envelope.amplitude)
            },
            None => MotorOutput::Veto,
        }
    }
    
    /// COMBINE: Composição estruturada.
    ///
    /// Agrega múltiplos estados da pilha.
    /// Concatena todos elementos da pilha.
    fn execute_combine(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        if self.context.stack.is_empty() {
            return MotorOutput::Veto;
        }
        
        let mut combined = Vec::new();
        while let Some(data) = self.context.pop_stack() {
            combined.extend_from_slice(&data);
        }
        
        self.context.set_result(combined);
        MotorOutput::Value(instruction.envelope.amplitude)
    }
    
    /// DERIVE: Derivar novo estado de inputs.
    ///
    /// Aplica transformação determinística sobre último resultado.
    /// Implementação básica: XOR com payload.
    fn execute_derive(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        let last = match self.context.get_result() {
            Some(data) => data,
            None => return MotorOutput::Veto,
        };
        
        let payload = &instruction.cargo.payload;
        
        // Transformação determinística: XOR byte-a-byte
        let mut derived = Vec::new();
        let max_len = last.len().max(payload.len());
        
        for i in 0..max_len {
            let a = last.get(i).copied().unwrap_or(0);
            let b = payload.get(i).copied().unwrap_or(0);
            derived.push(a ^ b);
        }
        
        self.context.set_result(derived);
        MotorOutput::Value(instruction.envelope.amplitude)
    }
    
    // =========================================================================
    // WAVE OPCODES
    // =========================================================================
    
    /// SYNC: Sincronização / phase-lock.
    ///
    /// Garante coerência temporal.
    /// Implementação básica: valida frequência.
    fn execute_sync(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        // Sincronização requer frequência
        match instruction.envelope.frequency {
            Some(_freq) => {
                // Sincronizado - preserva último resultado
                MotorOutput::Value(instruction.envelope.amplitude)
            },
            None => MotorOutput::Veto,
        }
    }
    
    /// FORK: Bifurcação controlada.
    ///
    /// Duplica último resultado para pilha.
    /// Permite processamento paralelo conceitual.
    fn execute_fork(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        let last = match self.context.get_result() {
            Some(data) => data.clone(), // Clone aqui para liberar borrow
            None => return MotorOutput::Veto,
        };
        
        // Agora podemos fazer borrow mutável
        self.context.push_stack(last.clone());
        self.context.push_stack(last);
        
        MotorOutput::Value(instruction.envelope.amplitude)
    }
    
    /// AMPLIFY: Modulação de amplitude (+).
    ///
    /// Amplifica amplitude do último resultado.
    /// Implementação: aumenta magnitude dos bytes.
    fn execute_amplify(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        let last = match self.context.get_result() {
            Some(data) => data,
            None => return MotorOutput::Veto,
        };
        
        let factor = instruction.envelope.amplitude;
        let mut amplified = Vec::new();
        
        for &byte in last {
            let value = (byte as f64 * factor).min(255.0) as u8;
            amplified.push(value);
        }
        
        self.context.set_result(amplified);
        MotorOutput::Value(instruction.envelope.amplitude)
    }
    
    /// ATTENUATE: Modulação de amplitude (-).
    ///
    /// Atenua amplitude do último resultado.
    /// Implementação: reduz magnitude dos bytes.
    fn execute_attenuate(&mut self, instruction: &GdQmnInstruction) -> MotorOutput {
        let last = match self.context.get_result() {
            Some(data) => data,
            None => return MotorOutput::Veto,
        };
        
        let factor = 1.0 - instruction.envelope.amplitude.min(1.0);
        let mut attenuated = Vec::new();
        
        for &byte in last {
            let value = (byte as f64 * factor) as u8;
            attenuated.push(value);
        }
        
        self.context.set_result(attenuated);
        MotorOutput::Value(instruction.envelope.amplitude)
    }
    
    /// Acessa contexto (para testes).
    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }
    
    /// Acessa contexto mutável (para testes).
    pub fn context_mut(&mut self) -> &mut ExecutionContext {
        &mut self.context
    }
}

impl Default for GdQmnExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::instruction::WaveEnvelope;
    use super::super::core::Cargo;
    
    #[test]
    fn test_execute_void() {
        let mut executor = GdQmnExecutor::new();
        
        let envelope = WaveEnvelope::new(
            super::super::core::QmnFamily::Unary,
            super::super::core::QmnSubfamily::State,
            Opcode::Core(CoreOpcode::Void),
            super::super::instruction::Profile::Standard,
            1.0,
        ).unwrap();
        
        let instruction = GdQmnInstruction::new(envelope, Cargo::empty());
        let result = executor.execute(&instruction);
        
        // VOID sempre retorna Veto
        assert!(result.is_veto());
    }
    
    #[test]
    fn test_execute_state() {
        let mut executor = GdQmnExecutor::new();
        
        let envelope = WaveEnvelope::new(
            super::super::core::QmnFamily::Unary,
            super::super::core::QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            super::super::instruction::Profile::Standard,
            1.0,
        ).unwrap();
        
        let payload = vec![1, 2, 3, 4];
        let instruction = GdQmnInstruction::new(envelope, Cargo::new(payload.clone(), 0x0001));
        
        let result = executor.execute(&instruction);
        
        // Deve retornar amplitude
        assert!(result.is_value());
        assert_eq!(result.as_f64(), 1.0);
        
        // Deve ter armazenado resultado
        assert_eq!(executor.context().get_result().unwrap(), &payload);
    }
    
    #[test]
    fn test_execute_reference() {
        let mut executor = GdQmnExecutor::new();
        
        // Primeiro, criar estado
        let state_name = "test_state".to_string();
        let state_data = vec![1, 2, 3, 4];
        executor.context_mut().set_state(state_name.clone(), state_data.clone());
        
        // Agora referenciar
        let envelope = WaveEnvelope::new(
            super::super::core::QmnFamily::Unary,
            super::super::core::QmnSubfamily::Relate,
            Opcode::Core(CoreOpcode::Reference),
            super::super::instruction::Profile::Standard,
            0.5,
        ).unwrap();
        
        let instruction = GdQmnInstruction::new(
            envelope, 
            Cargo::new(state_name.as_bytes().to_vec(), 0x0001)
        );
        
        let result = executor.execute(&instruction);
        
        // Deve retornar amplitude
        assert!(result.is_value());
        
        // Deve ter recuperado estado
        assert_eq!(executor.context().get_result().unwrap(), &state_data);
    }
    
    #[test]
    fn test_execute_derive() {
        let mut executor = GdQmnExecutor::new();
        
        // Setup: criar estado inicial
        executor.context_mut().set_result(vec![0xFF, 0x00, 0xAA]);
        
        let envelope = WaveEnvelope::new(
            super::super::core::QmnFamily::Binary,
            super::super::core::QmnSubfamily::Derive,
            Opcode::Core(CoreOpcode::Derive),
            super::super::instruction::Profile::Standard,
            1.0,
        ).unwrap();
        
        let transform = vec![0x0F, 0x0F, 0x0F];
        let instruction = GdQmnInstruction::new(envelope, Cargo::new(transform, 0x0001));
        
        let result = executor.execute(&instruction);
        
        // Deve retornar amplitude
        assert!(result.is_value());
        
        // Deve ter derivado: [0xFF XOR 0x0F, 0x00 XOR 0x0F, 0xAA XOR 0x0F]
        let expected = vec![0xF0, 0x0F, 0xA5];
        assert_eq!(executor.context().get_result().unwrap(), &expected);
    }
    
    #[test]
    fn test_execute_amplify() {
        let mut executor = GdQmnExecutor::new();
        
        // Setup
        executor.context_mut().set_result(vec![100, 50, 200]);
        
        let envelope = WaveEnvelope::new(
            super::super::core::QmnFamily::Unary,
            super::super::core::QmnSubfamily::State,
            Opcode::Wave(WaveOpcode::Amplify),
            super::super::instruction::Profile::Standard,
            1.5, // 150% amplitude
        ).unwrap();
        
        let instruction = GdQmnInstruction::new(envelope, Cargo::empty());
        let result = executor.execute(&instruction);
        
        // Deve retornar amplitude
        assert!(result.is_value());
        
        // Valores devem ser amplificados (mas <= 255)
        let amplified = executor.context().get_result().unwrap();
        assert_eq!(amplified[0], 150); // 100 * 1.5
        assert_eq!(amplified[1], 75);  // 50 * 1.5
        assert_eq!(amplified[2], 255); // 200 * 1.5 = 300, clamped to 255
    }
}
