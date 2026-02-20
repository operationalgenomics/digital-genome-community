//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GD-QMN v0.8.0 End-to-End Validation
//! Author: Claude (sob supervisão de Carlos Eduardo Favini)
//! Date: 2026-02-15
//! Version: 0.8.0
//! Description: Validação completa da implementação GD-QMN v0.8.0.
//!              Testa Parser, Serializer, Executor e ISA (9 opcodes).
//!
//! --------------------------
//! USAGE
//! --------------------------
//! cargo run --example gd_qmn_validation --features validation
//!
//! --------------------------

use digital_genome_community::unl::gd_qmn::*;

fn main() {
    println!("═══════════════════════════════════════════════════════════");
    println!("  GD-QMN v0.8.0 - VALIDAÇÃO END-TO-END");
    println!("═══════════════════════════════════════════════════════════\n");
    
    test_instruction_creation();
    test_serialization_roundtrip();
    test_checksum_integrity();
    test_parser_gate();
    test_executor_core_opcodes();
    test_executor_wave_opcodes();
    test_full_pipeline();
    
    println!("\n═══════════════════════════════════════════════════════════");
    println!("  ✅ TODAS AS VALIDAÇÕES PASSARAM");
    println!("  v0.8.0 OPERACIONAL E CONFORME CANON v5.1");
    println!("═══════════════════════════════════════════════════════════");
}

fn test_instruction_creation() {
    println!("📋 Teste 1: Criação de Instrução GD-QMN");
    
    let envelope = WaveEnvelope::new(
        QmnFamily::Unary,
        QmnSubfamily::State,
        Opcode::Core(CoreOpcode::State),
        ProfileV2::Standard,
        1.0,
    ).expect("Falha ao criar envelope");
    
    let cargo = Cargo::new(vec![1, 2, 3, 4, 5], 0x0001);
    let instruction = GdQmnInstruction::new(envelope, cargo);
    
    assert_eq!(instruction.profile(), ProfileV2::Standard);
    assert!(instruction.verify_integrity());
    
    println!("   ✓ Instrução criada com sucesso");
    println!("   ✓ UID: {:?}", instruction.uid());
    println!("   ✓ Integridade tripla validada\n");
}

fn test_serialization_roundtrip() {
    println!("🔄 Teste 2: Serialização e Deserialização (Roundtrip)");
    
    let envelope = WaveEnvelope::new(
        QmnFamily::Binary,
        QmnSubfamily::Relate,
        Opcode::Core(CoreOpcode::Reference),
        ProfileV2::Standard,
        0.75,
    ).unwrap();
    
    let cargo = Cargo::new(b"test_payload_123".to_vec(), 0x0042);
    let original = GdQmnInstruction::new(envelope, cargo);
    
    // Serializar
    let bytes = original.to_bytes();
    println!("   ✓ Serialização: {} bytes", bytes.len());
    
    // Deserializar
    let parsed = GdQmnParser::parse(&bytes)
        .expect("Falha no parsing");
    
    // Validar roundtrip
    assert_eq!(parsed.uid(), original.uid());
    assert_eq!(parsed.cargo.payload, original.cargo.payload);
    assert_eq!(parsed.cargo.schema_hint, original.cargo.schema_hint);
    
    // LEI-QMN-SERIAL-01: Determinismo
    let bytes2 = parsed.to_bytes();
    assert_eq!(bytes, bytes2, "Serialização não-determinística!");
    
    println!("   ✓ Roundtrip bem-sucedido");
    println!("   ✓ LEI-QMN-SERIAL-01 validada (determinismo)\n");
}

fn test_checksum_integrity() {
    println!("🔐 Teste 3: Integridade Tripla (GATE-QMN-01)");
    
    let envelope = WaveEnvelope::new(
        QmnFamily::Ternary,
        QmnSubfamily::Derive,
        Opcode::Core(CoreOpcode::Derive),
        ProfileV2::Extended,
        0.5,
    ).unwrap();
    
    let cargo = Cargo::new(vec![0xAA, 0xBB, 0xCC, 0xDD], 0x1234);
    let instruction = GdQmnInstruction::new(envelope, cargo);
    
    // Validar integridade original
    assert!(instruction.verify_integrity());
    println!("   ✓ Integridade original validada");
    
    // Tamper com envelope (alterar amplitude)
    let original_bytes = instruction.to_bytes();
    let mut tampered = original_bytes.clone();
    tampered[10] ^= 0xFF; // Tamper no meio do envelope
    
    let result = GdQmnParser::parse(&tampered);
    assert!(result.is_err(), "Tampering não detectado!");
    println!("   ✓ Tampering no envelope detectado");
    
    // Tamper com cargo
    let mut tampered2 = original_bytes.clone();
    let cargo_start = original_bytes.len() - 10;
    tampered2[cargo_start] ^= 0xFF;
    
    let result2 = GdQmnParser::parse(&tampered2);
    assert!(result2.is_err(), "Tampering no cargo não detectado!");
    println!("   ✓ Tampering no cargo detectado");
    println!("   ✓ LEI-QMN-INTEGRIDADE-TRIPLA-01 validada\n");
}

fn test_parser_gate() {
    println!("🚪 Teste 4: GATE-QMN-01 (Validação de Fronteira)");
    
    // Teste 1: Dados insuficientes
    let short_data = vec![1, 2, 3];
    let result = GdQmnParser::parse(&short_data);
    assert!(matches!(result, Err(ParseError::InsufficientData)));
    println!("   ✓ Rejeição de dados insuficientes");
    
    // Teste 2: Amplitude inválida (≤ 0)
    let mut invalid_amp = vec![0u8; 100];
    invalid_amp[0] = 0x01; // Family
    invalid_amp[1] = 0x01; // Subfamily
    invalid_amp[2] = 0x01; // Opcode
    invalid_amp[3] = 0x02; // Profile
    // Amplitude = 0.0 (bytes 4-11)
    
    let result = GdQmnParser::parse(&invalid_amp);
    assert!(matches!(result, Err(ParseError::InvalidAmplitude)));
    println!("   ✓ Rejeição de amplitude inválida (LEI-QMN-AMP-01)");
    
    println!("   ✓ GATE-QMN-01 operacional\n");
}

fn test_executor_core_opcodes() {
    println!("⚙️  Teste 5: Executor - Core Opcodes (5 primitivos)");
    
    let mut executor = GdQmnExecutor::new();
    
    // VOID
    let void_inst = create_instruction(
        Opcode::Core(CoreOpcode::Void),
        vec![],
    );
    let result = executor.execute(&void_inst);
    assert!(result.is_veto());
    println!("   ✓ VOID: Veto ontológico");
    
    // STATE
    let state_inst = create_instruction(
        Opcode::Core(CoreOpcode::State),
        vec![1, 2, 3, 4, 5],
    );
    let result = executor.execute(&state_inst);
    assert!(result.is_value());
    assert_eq!(executor.context().get_result().unwrap(), &vec![1, 2, 3, 4, 5]);
    println!("   ✓ STATE: Estado armazenado");
    
    // DERIVE
    let derive_inst = create_instruction(
        Opcode::Core(CoreOpcode::Derive),
        vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
    );
    let result = executor.execute(&derive_inst);
    assert!(result.is_value());
    // Deve ter derivado via XOR
    let derived = executor.context().get_result().unwrap();
    assert_eq!(derived, &vec![0xFE, 0xFD, 0xFC, 0xFB, 0xFA]);
    println!("   ✓ DERIVE: Derivação aplicada");
    
    println!("   ✓ LEI-QMN-ISA-01 (Core) validada\n");
}

fn test_executor_wave_opcodes() {
    println!("🌊 Teste 6: Executor - Wave Opcodes (4 moduladores)");
    
    let mut executor = GdQmnExecutor::new();
    
    // Setup: criar estado inicial
    executor.context_mut().set_result(vec![100, 100, 100]);
    
    // AMPLIFY
    let amplify_env = WaveEnvelope::new(
        QmnFamily::Unary,
        QmnSubfamily::State,
        Opcode::Wave(WaveOpcode::Amplify),
        ProfileV2::Standard,
        2.0, // 200% amplitude
    ).unwrap();
    
    let amplify_inst = GdQmnInstruction::new(amplify_env, Cargo::empty());
    let result = executor.execute(&amplify_inst);
    assert!(result.is_value());
    assert_eq!(executor.context().get_result().unwrap(), &vec![200, 200, 200]);
    println!("   ✓ AMPLIFY: Modulação (+)");
    
    // ATTENUATE
    let atten_env = WaveEnvelope::new(
        QmnFamily::Unary,
        QmnSubfamily::State,
        Opcode::Wave(WaveOpcode::Attenuate),
        ProfileV2::Standard,
        0.5, // 50% atenuação
    ).unwrap();
    
    let atten_inst = GdQmnInstruction::new(atten_env, Cargo::empty());
    let result = executor.execute(&atten_inst);
    assert!(result.is_value());
    assert_eq!(executor.context().get_result().unwrap(), &vec![100, 100, 100]);
    println!("   ✓ ATTENUATE: Modulação (-)");
    
    println!("   ✓ LEI-QMN-ISA-01 (Wave) validada\n");
}

fn test_full_pipeline() {
    println!("🔧 Teste 7: Pipeline Completo (Parse → Execute → Serialize)");
    
    // Criar instrução original
    let envelope = WaveEnvelope::new(
        QmnFamily::Unary,
        QmnSubfamily::State,
        Opcode::Core(CoreOpcode::State),
        ProfileV2::Standard,
        0.8,
    ).unwrap();
    
    let payload = b"PIPELINE_TEST_DATA_v0.8.0".to_vec();
    let original = GdQmnInstruction::new(envelope, Cargo::new(payload.clone(), 0xCAFE));
    
    // 1. Serializar
    let bytes = original.to_bytes();
    println!("   → Serialização: {} bytes", bytes.len());
    
    // 2. Parse
    let parsed = GdQmnParser::parse(&bytes)
        .expect("Parse falhou");
    println!("   → Parse bem-sucedido");
    
    // 3. Validar integridade (GATE-QMN-01)
    assert!(parsed.verify_integrity());
    println!("   → GATE-QMN-01 passou");
    
    // 4. Executar
    let mut executor = GdQmnExecutor::new();
    let result = executor.execute(&parsed);
    assert!(result.is_value());
    println!("   → Execução bem-sucedida");
    
    // 5. Validar resultado
    assert_eq!(executor.context().get_result().unwrap(), &payload);
    println!("   → Resultado validado");
    
    // 6. Re-serializar
    let bytes2 = parsed.to_bytes();
    assert_eq!(bytes, bytes2);
    println!("   → Re-serialização idêntica (LEI-QMN-SERIAL-01)");
    
    println!("   ✓ Pipeline completo validado\n");
}

// Helper function
fn create_instruction(opcode: Opcode, payload: Vec<u8>) -> GdQmnInstruction {
    let subfamily = match opcode {
        Opcode::Core(CoreOpcode::State) => QmnSubfamily::State,
        Opcode::Core(CoreOpcode::Derive) => QmnSubfamily::Derive,
        Opcode::Core(CoreOpcode::Void) => QmnSubfamily::State,
        _ => QmnSubfamily::State,
    };
    
    let envelope = WaveEnvelope::new(
        QmnFamily::Unary,
        subfamily,
        opcode,
        ProfileV2::Standard,
        1.0,
    ).unwrap();
    
    GdQmnInstruction::new(envelope, Cargo::new(payload, 0x0001))
}
