# GUIA DE EXECUÇÃO — VALIDAÇÃO CANÔNICA
## Para o Humano

---

## RESUMO

Este guia explica como executar a validação canônica do GDC v0.1.0-rc1 e enviar os resultados para Claude.

**Tempo estimado:** 30-60 minutos
**Requisitos:** Rust 1.75+, 4GB RAM, 1GB espaço em disco

---

## PASSO 1: PREPARAR AMBIENTE

```bash
# Extrair pacote
unzip digital-genome-community-v0.1.0-rc1.zip
cd digital-genome-community

# Verificar build
cargo build --release
```

---

## PASSO 2: EXECUTAR VALIDAÇÃO BÁSICA (SEM DATASETS)

```bash
cargo run --release --example canonical_validation
```

**Resultado esperado:** Todos os testes passam (✅ PASS)

---

## PASSO 3: COLETAR DATASETS REAIS

Crie a estrutura de diretórios:

```bash
mkdir -p validation/datasets/{A,B,C,D,E,F}
```

### Datasets OBRIGATÓRIOS (mínimo):

| ID | O que coletar | Como obter |
|----|---------------|------------|
| **A1** | Código Rust | `cp src/lib.rs validation/datasets/A/` |
| **B1** | Qualquer imagem PNG | Copie uma foto |
| **B2** | Qualquer imagem JPEG | Copie uma foto |
| **E1** | Arquivo vazio | `touch validation/datasets/E/empty.bin` |
| **E3** | 1MB de zeros | `dd if=/dev/zero of=validation/datasets/E/zeros.bin bs=1M count=1` |
| **E5** | 1MB aleatório | `dd if=/dev/urandom of=validation/datasets/E/random.bin bs=1M count=1` |
| **F1** | Cargo.toml | `cp Cargo.toml validation/datasets/F/` |

### Datasets RECOMENDADOS (rigor máximo):

| ID | O que coletar | Fonte |
|----|---------------|-------|
| **A3** | CSV financeiro | Yahoo Finance (baixe histórico de ações) |
| **B3** | PDF qualquer | Qualquer documento PDF |
| **B4** | Áudio WAV | Grave 10 segundos de áudio |
| **C1** | Genoma FASTA | NCBI GenBank |
| **D2** | Certificado SSL | `openssl s_client -connect google.com:443 </dev/null 2>/dev/null | openssl x509 > cert.pem` |

---

## PASSO 4: EXECUTAR VALIDAÇÃO COMPLETA

```bash
cargo run --release --example canonical_validation -- --datasets validation/datasets
```

---

## PASSO 5: ENVIAR RELATÓRIO PARA CLAUDE

Após a execução, copie TUDO que aparecer após:

```
═══════════════════════════════════════════════════════════════════════
JSON REPORT (copy this for Claude):
═══════════════════════════════════════════════════════════════════════
```

Cole na conversa com Claude usando este formato:

```
RELATÓRIO DE VALIDAÇÃO CANÔNICA

Ambiente:
- OS: [Windows/Linux/macOS + versão]
- Rust: [saída de rustc --version]
- CPU: [modelo]
- RAM: [quantidade]

Resultado:
[COLE O JSON AQUI]

Observações:
[qualquer erro ou aviso que apareceu]
```

---

## O QUE CADA BLOCO TESTA

### BLOCO 1: Invariantes Fundamentais
- Determinismo (mesma entrada → mesma saída)
- Limites de valores (entropy ∈ [0,1], mean ∈ [0,255])
- Thread-safety (100 threads simultâneas)

### BLOCO 2: Consistência Inter-Execução
- Replay determinístico
- IDs determinísticos
- Convergência de maturação

### BLOCO 3: Estabilidade Numérica
- Sem NaN
- Sem Infinity
- Sem overflow em dados grandes

### BLOCO 4: Limites e Recursos
- Budget enforcement
- Limite de recursão
- Memória limitada

### BLOCO 5: Emulação GDE↔GDC
- Serialização JSON
- Deserialização JSON
- Integridade de roundtrip

### BLOCO 6: Casos Patológicos
- Todos zeros
- Todos 0xFF
- Padrão alternado
- Byte único

### BLOCO 7: Datasets Reais
- Processa cada arquivo coletado
- Valida invariantes em dados reais

---

## CRITÉRIOS DE APROVAÇÃO

| Critério | Necessário |
|----------|------------|
| Todos os testes PASS | ✅ SIM |
| Zero falhas | ✅ SIM |
| Datasets processados | ✅ Mínimo 7 |
| JSON válido gerado | ✅ SIM |

Se QUALQUER teste falhar → Release BLOQUEADO.

---

## PROBLEMAS COMUNS

### "error: could not compile"
```bash
# Limpe o cache
cargo clean
cargo build --release
```

### "cannot find datasets directory"
```bash
# Verifique o caminho
ls -la validation/datasets/
```

### "file too large"
Arquivos > 100MB são automaticamente ignorados (com PASS).

### Teste falhou
Copie a mensagem de erro COMPLETA e envie para Claude.

---

## TEMPO ESPERADO

| Fase | Tempo |
|------|-------|
| Build release | 2-5 min |
| Validação básica | 10-30 seg |
| Coleta de datasets | 10-30 min |
| Validação completa | 1-5 min |
| Envio do relatório | 5 min |

---

**FIM DO GUIA**
