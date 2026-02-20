# ROADMAP.md — Roadmap Consolidado do Genoma Digital Community (GDC)

## Documento de Planejamento Estratégico e Técnico

---

**Data:** 10 de Fevereiro de 2026  
**Status:** DOCUMENTO VIVO — atualizar a cada marco  
**Fonte Autoritativa:** BACKLOG_DELIBERACAO.md (PARTE X), DESBLOQUEIO-v085.md, FRONTEIRAS.md  
**Audiência:** CTO / Arquiteto / Equipe de Desenvolvimento / Investidores

---

# ═══════════════════════════════════════════════════════════════════
# §1: VISÃO GERAL DO ROADMAP
# ═══════════════════════════════════════════════════════════════════

```
v0.7.1 ──✅──▶ v0.8.0 ──🎯──▶ v0.8.5 ──✅──▶ v0.9.0 ──▶  v0.9.1 ──▶ v0.9.5 ──▶ v1.0.0α ──▶    v1.0.0β ──▶   v1.0.0RC ──▶ v1.0.0
  │                │                │               │           │           │            │             │             │           │
  │                │                │               │           │           │            │             │             │           │
  Núcleo       UNL/QMN          Distrib.        Orquest.      Orquest.    Sinapses     GDO+GDE      Trans-K       Escala       CÉREBRO
  Isolado      Operac.          Computac.       Básica        Complexa    Neurônios    Emuladores   Learning      Auditável    SINTÉTICO
```

| Versão | Marco | Status | Data Conclusão |
|--------|-------|--------|----------------|
| **v0.7.1** | Núcleo cognitivo isolado | ✅ CONCLUÍDA | Jan/2026 |
| **v0.8.0** | UNL/GD-QMN operacional; primeiro ISA | 🎯 EM ANDAMENTO | — |
| **v0.8.5** | Distribuição computacional | ✅ CONCLUÍDA | 03/02/2026 |
| **v0.9.0** | Orquestração Básica (2 GDCs) | 📋 PLANEJADA | — |
| **v0.9.1** | Orquestração Complexa (n GDCs) — n-Rainhas | 📋 PLANEJADA | — |
| **v0.9.5** | Sinapses e Neurônios emergentes | 📋 PLANEJADA | — |
| **v1.0.0α** | GDO + GDE Emuladores (externos) | 📋 PLANEJADA | — |
| **v1.0.0β** | Trans-Kingdom Learning | 📋 PLANEJADA | — |
| **v1.0.0RC** | Escala auditável | 📋 PLANEJADA | — |
| **v1.0.0** | 🧠 CÉREBRO SINTÉTICO — ecossistema mínimo | 🎯 MARCO FINAL | — |

---

# ═══════════════════════════════════════════════════════════════════
# §2: ESTADO ATUAL DO CÓDIGO
# ═══════════════════════════════════════════════════════════════════

**Versão Corrente:** v0.8.5 (sanitizada)  
**Linguagem:** Rust  
**Infraestrutura:** AWS-first (DynamoDB, Neptune, Timestream, Kinesis, SQS, ECS Fargate)

| Métrica | Valor |
|---------|-------|
| Linhas de código (Rust) | ~21.176 |
| Testes unitários | 331 |
| Módulos implementados | 22 |
| Axiomas Fundacionais (Canon) | 15 (AF-1..AF-15) |
| Axiomas Operacionais (Canon) | 25 (AO-1..AO-24 + AO-QMN-01) |
| Leis Derivadas (Canon) | ~168 |
| Gates de Conformidade | 9 |

**Módulos Ativos:**

```
src/
├── archive/          # Arquivamento
├── budget/           # Orçamento cognitivo
├── cognitive/        # Pipeline cognitivo
├── competition/      # Competição entre motores
├── completeness/     # Completude do ciclo
├── coordination/     # Coordenação (v0.8.5)
│   ├── event.rs      # Evento (Σ)
│   ├── field.rs      # Campo R(Σ) + Integração ⨆
│   ├── manifestation.rs  # Manifestação (Ω)
│   ├── edr.rs        # Protocolo EDR
│   └── gdc.rs        # Estados Rainha/Worker
├── core_types/       # Tipos fundamentais
├── correlation/      # Correlação
├── hierarchy/        # Hierarquia + DNA
├── identity/         # Identidade (v0.8.5)
│   ├── shibboleth.rs # UID Shibboleth (ontológico)
│   └── orchestrated.rs   # UID Ressonante (funcional)
├── math/             # Craft Performance + geometria
├── maturation/       # Maturação cognitiva
├── memory/           # MCI (Memória Cognitiva Interna)
├── motors/           # Praxis, Nash, Chaos, Meristic
├── observability/    # Observabilidade verificável
├── replay/           # Replay determinístico
├── selection/        # Seleção e decisão
├── sensory/          # Sensoriamento + FFT
├── topology/         # Topologia (estrutura)
├── traits/           # Traits canônicos
└── unl/              # UNL + GD-QMN
```

---

# ═══════════════════════════════════════════════════════════════════
# §3: VERSÕES CONCLUÍDAS
# ═══════════════════════════════════════════════════════════════════

## v0.7.1 — Núcleo Cognitivo Isolado ✅

**Entregáveis:**
- Quatro motores cognitivos operacionais (Praxis, Nash, Chaos, Merístico)
- Craft Performance multiplicativo com veto absoluto (CP = Mp × Mn × Mc × Mm)
- Pipeline sensorial completo (FFT, correlação, detecção de padrões)
- Memória Cognitiva Interna (MCI) como estado, não observation
- Replay determinístico verificável
- Observabilidade por replay (sem telemetria no core)
- Hierarquia DNA + seleção
- Orçamento cognitivo
- 13.367+ linhas, 230+ testes

**Canon Vigente na Época:** v2.0 (AF-1..AF-12, AO-1..AO-18)

---

## v0.8.5 — Distribuição Computacional ✅

**Data de Conclusão:** 03/02/2026

**Entregáveis:**
- Identidade de dois planos (Shibboleth + Ressonante)
- Coordenação cognitiva (Evento Σ, Campo R(Σ), Manifestação Ω)
- Protocolo EDR (Envelope Devolutivo de Retorno)
- Estados Rainha/Worker com transições
- DNA único (somente Rainha emite)
- Integração por ⨆ (idempotente, comutativa, associativa)
- Sanitização canônica (LEI-ZERO-01, remoção de unwrap() em produção)
- Correção de independência de planos (Shibboleth ⊥ Ressonante)
- 22 itens implementados, 0 pendentes

**Canon Produzido:**
- AO-19..AO-24 (identidade, estados, topologia)
- LEI-COORD-01, LEI-COORD-02
- DLB-013, DLB-014 (DNA único, EDR)
- TECH-COORD-03 (direção absorção)
- 13 Grey Zones fechadas (GZ-D03..GZ-D15)

**Canon Vigente na Conclusão:** v3.0 (AF-1..AF-14, AO-1..AO-24, 151 leis, 8 gates)

---

# ═══════════════════════════════════════════════════════════════════
# §4: VERSÃO EM ANDAMENTO — v0.8.0
# ═══════════════════════════════════════════════════════════════════

**Nota Importante:** v0.8.0 foi definida no roadmap como "UNL/GD-QMN operacional; primeiro ISA". Parte do trabalho canônico necessário para v0.8.5 foi antecipado. O trabalho remanescente de v0.8.0 foca na implementação operacional do GD-QMN e ISA.

## Escopo v0.8.0

**Marco:** UNL/GD-QMN operacional com primeiro ISA (Instruction Set Architecture)

### Decisões Já Tomadas (DLB-019..DLB-024)

Todos os gaps identificados para v0.8.0 foram deliberados e decididos:

| GAP | DLB | Decisão | Status |
|-----|-----|---------|--------|
| ZERO ontológico | DLB-019 | `enum MotorOutput { Value(f64), Veto }` | ✅ Decidido + Implementado |
| Mutex failure | DLB-020 | `Result<T, SyncFailure>` interno → `Veto` na fronteira | ✅ Decidido + Implementado |
| Famílias GD-QMN | DLB-021 | Family = cardinalidade, Subfamily = classe operacional | ✅ Decidido |
| ISA mínimo | DLB-022 | 5 core (VOID..DERIVE) + 4 wave (SYNC..ATTENUATE) | ✅ Decidido |
| Grandezas cognitivas | DLB-023 | ΝU, Sm, Cg, Ho, Om (métricas, não ontológicas) | ✅ Decidido |
| Cargo determinístico | DLB-024 | `struct Cargo { payload, content_hash, schema_hint }` | ✅ Decidido |

### Leis a Formalizar para v0.8.0

Estas leis foram deliberadas mas precisam do texto normativo completo no CANON.md:

| LEI | Nome | DLB | Formalizada? |
|-----|------|-----|-------------|
| LEI-ZERO-01 | ZERO é enum, não número | DLB-019 | ✅ Formalizada (LEI-QMN-AMP-01 + LEI-QMN-VETO-01) |
| LEI-SYNC-01 | Erro tipado interno + Veto fronteira | DLB-020 | ✅ Formalizada (LEI-QMN-VETO-01) |
| LEI-QMN-01 | Três perfis (Compact/Standard/Extended) | DLB-009 | ✅ Formalizada (LEI-QMN-PROFILE-01) |
| LEI-QMN-02 | Campo Cargo como transporte | DLB-024 | ✅ Formalizada (LEI-QMN-CARGO-01) |
| LEI-QMN-03 | Checksum triplo obrigatório | DLB-011 | ✅ Formalizada (LEI-QMN-INTEGRIDADE-TRIPLA-01) |
| LEI-QMN-04 | Famílias = Cardinalidade + Subfamily | DLB-021 | ✅ Formalizada (LEI-QMN-ID-01) |
| LEI-QMN-05 | Opcode por subfamily (não global) | DLB-022 | ✅ Formalizada (LEI-QMN-ID-01) |
| LEI-ISA-01 | ISA mínimo (5 núcleo + 4 wave) | DLB-022 | ✅ Formalizada (LEI-QMN-ISA-01) |
| LEI-COG-01 | Grandezas cognitivas (ΝU, Sm, Cg, Ho, Om) | DLB-023 | ✅ Formalizada (LEI-QMN-COG-01) |
| LEI-UNL-01 | Ciclo emergência-retorno | DLB-006 | ✅ Formalizada (LEI-AF-13-06) |

### Implementação Técnica Remanescente

| Item | Descrição | Prioridade |
|------|-----------|------------|
| GD-QMN Parser | Parser de bytecode hexadecimal UNL | CRÍTICO |
| GD-QMN Serializer | Serialização canônica determinística | CRÍTICO |
| ISA Executor | Execução dos 9 opcodes (5 core + 4 wave) | CRÍTICO |
| Perfis QMN | Implementar Compact/Standard/Extended | ALTO |
| Checksum Triplo | Verificação determinística de integridade em três camadas | ALTO |
| Cargo Integration | Integrar Cargo no pipeline cognitivo | ALTO |
| Grandezas | Implementar métricas ΝU, Sm, Cg, Ho | MÉDIO |
| Emergência-Retorno | Ciclo UNL → linguagens → UNL | MÉDIO |

### Critérios de Conclusão v0.8.0

| Critério | Status |
|----------|--------|
| GD-QMN bytecode funcional (parse + serialize + execute) | ❌ Pendente |
| 9 opcodes operacionais com testes | ❌ Pendente |
| Perfis Compact e Standard operacionais | ❌ Pendente |
| Checksum triplo integrado | ❌ Pendente |
| Cargo determinístico no pipeline | ❌ Pendente |
| 10 leis formalizadas no CANON.md | ✅ Concluído (Canon v3.1) |
| Autorização humana para fechar versão | ❌ Pendente |

---

# ═══════════════════════════════════════════════════════════════════
# §5: VERSÕES FUTURAS — DETALHAMENTO
# ═══════════════════════════════════════════════════════════════════

## v0.9.0 — Orquestração Básica (2 GDCs)

**Pré-requisitos:** v0.8.0 + v0.8.5 (ambos concluídos)

**Escopo:**
Duas instâncias de GDC operando em orquestração real — uma como Rainha, outra como Worker — com comunicação por EDR e coordenação por campo.

### Canon Necessário

| Item | Tipo | Status |
|------|------|--------|
| LEI-COORD-03 (Absorção Estrutural) | Lei | ❌ Depende de TECH-COORD-03 |
| Definição de W(Σ) | Especificação | ❌ Pendente |
| Operação de Contenção (⊒) | Especificação | ❌ Pendente |
| LEI-RESS-01 (Workers = cálculos) | Lei | 🟡 Deliberada, formalizar |
| LEI-RESS-02 (GDC resiliente a escala) | Lei | 🟡 Deliberada, formalizar |

### Grey Zones Bloqueadoras

| GZ | Tema | Status | Impacto |
|----|------|--------|---------|
| GZ-TOPO-01 | Soberania de Recusa | ✅ FECHADA (Canon v4.0) | Resolvida por AF-15 (Ressonância Estrutural) |

### Implementação Técnica

| Item | Descrição |
|------|-----------|
| Networking 2-GDC | Comunicação entre duas instâncias |
| Protocolo Queen Election | Seleção de Rainha por recebimento de estímulo |
| EDR Wire Protocol | Serialização/deserialização real do EDR |
| Campo Distribuído | R(Σ) operando sobre manifestações remotas |
| Absorção Detector | Verificar S ⊒ W(Σ) para fechamento |
| Testes de Determinismo | Mesmo input → mesmo output em orquestração |

### Deliberações Adiadas para v0.9.0

| DLB | Candidato | Descrição |
|-----|-----------|-----------|
| DLB-004 | — | Uso máximo de hardware |

---

## v0.9.1 — Orquestração Complexa (n GDCs) — Problema das n-Rainhas

**Pré-requisitos:** v0.8.5 + v0.9.0 (ambos concluídos)

**Escopo:**
Várias instâncias de GDC operando em orquestração real — uma como Rainha, outras como Workers — com cálculos na escala de QuettaBytes, comunicação por EDR e coordenação por campo. Caso de uso: cálculo de n no problema das n-Rainhas com Workers distribuídos.

### Canon Vigente (v4.0)

| Item | Tipo | Status |
|------|------|--------|
| AF-15 (Ressonância Estrutural) | Axioma | ✅ Canon v4.0 |
| LEI-RSN-01 (Cognição não compartilhada) | Lei | ✅ Canon v4.0 |
| LEI-RSN-03 (Isolamento de instâncias) | Lei | ✅ Canon v4.0 |
| LEI-RSN-04 (Participação simultânea) | Lei | ✅ Canon v4.0 |
| LEI-AO-20-03 (Threads distintas por papel) | Lei | ✅ Canon v4.0 |
| LEI-AO-20-04 (Recrutamento por vibração) | Lei | ✅ Canon v4.0 |
| LEI-AO-20-05 (Distribuição obrigatória QuettaBytes) | Lei | ✅ Canon v4.0 |
| LEI-AO-9-05 (Handshake pureza pré-delegação) | Lei | ✅ Canon v4.0 |
| LEI-AO-21-03 (Distribuição bidirecional) | Lei | ✅ Canon v4.0 |

### Canon Necessário (deliberação pendente)

| Item | Tipo | Status |
|------|------|--------|
| LEI-COORD-03 (Absorção Estrutural) | Lei | ❌ Depende de TECH-COORD-03 |
| Definição de W(Σ) | Especificação | ❌ Pendente |
| Operação de Contenção (⊒) | Especificação | ❌ Pendente |
| LEI-RESS-01 (Workers = cálculos) | Lei | 🟡 Deliberada, formalizar |
| LEI-RESS-02 (GDC resiliente a escala) | Lei | 🟡 Deliberada, formalizar |

### Implementação Técnica

| Item | Descrição |
|------|-----------|
| Networking n-GDC | Comunicação entre n instâncias |
| Protocolo n-Queen & n-Workers | Seleção de Rainhas por recebimento de estímulo e Workers por manifestação de capacidade |
| EDR Wire Protocol como família GD-QMN | Sanitização do EDR como protocolo ISA do GD-QMN |
| Particionamento n-Rainhas | Distribuição do problema n-Rainhas em partições paralelas |
| Sincronização de Estado | Estado consistente bidirecional Rainha↔Workers |
| Handshake Shibboleth n-Way | Validação de pureza para n participantes |
| Testes de Determinismo | Mesmo input → mesmo output em orquestração com n GDCs |

### Deliberações Adiadas para v0.9.1

| DLB | Candidato | Descrição |
|-----|-----------|-----------|
| DLB-004 | — | Uso máximo de hardware |
| DLB-XYZ | — | Fine Tuning do Sistema |

---

## v0.9.5 — Sinapses e Neurônios Emergentes

**Pré-requisitos:** v0.9.1 (orquestração funcional entre n GDCs)

**Escopo:**
Emergência de sinapses (conexões persistentes entre GDCs) e neurônios (agrupamentos funcionais) a partir da orquestração repetida. Primeira manifestação de aprendizado coletivo.

### Canon Necessário

| Item | Tipo | Status |
|------|------|--------|
| GZ-TOPO-02 | Grey Zone | ✅ FECHADA (Canon v4.0) — Participação simultânea com isolamento |
| GZ-TOPO-03 | Grey Zone | ✅ FECHADA (Canon v4.0) — Dissolução por completude |
| Protocolo de Sinapses | Novo | ❌ Não deliberado |
| Definição de Neurônio Emergente | Novo | ❌ Não deliberado |

### Tensões a Monitorar

| Tensão | Referência | Risco |
|--------|-----------|-------|
| Determinismo vs Emergência | TL-01 (FRONTEIRAS.md) | Se aprendizado contínuo, replay pode ser impraticável |
| Soberania vs Autonomia | TL-02 (FRONTEIRAS.md) | GDC descobrindo padrões que contradizem axiomas |

### Implementação Técnica

| Item | Descrição |
|------|-----------|
| Sinapse Model | Conexão persistente entre pares de GDCs |
| Sinapse Strength | Mecanismo de fortalecimento/enfraquecimento |
| Neurônio Cluster | Agrupamento emergente por padrão de ativação |
| Multi-GDC Field | R(Σ) com N manifestações |
| Aprendizado Coletivo | MCI distribuída entre GDCs |

---

## v1.0.0α — GDO + GDE Emuladores (Externos)

**Pré-requisitos:** v0.9.5 (sinapses funcionais)

**Escopo:**
Primeira implementação das camadas externas — GDO (Genoma Digital Orchestrator) e GDE (Genoma Digital Educator) — como emuladores. O GDO orquestra eventos; o GDE conecta UNL a conceitos humanos.

### Canon Necessário

| Item | Tipo | Status |
|------|------|--------|
| Protocolo GDO-GDC | Novo | ❌ Não deliberado |
| Protocolo GDE-GDC | Novo | ❌ Não deliberado |
| LEI-COM-01 (Falar = emissão interpretável) | Lei | 🟡 Deliberada (DLB-018) |

### Deliberações Adiadas

| DLB | Candidato | Descrição | Decisão Prévia |
|-----|-----------|-----------|----------------|
| DLB-ROAD-01 | AF-UNL-03 | "UNL existe apenas dentro do GDC" | ROADMAP — Tensão com GDE |
| DLB-ROAD-02 | AF-UNL-04 | "UNL infinita, versão humana finita" | ROADMAP — Meta-axiomático |

### Implementação Técnica

| Item | Descrição |
|------|-----------|
| GDO Emulator | Orquestrador externo (distribui Σ, coleta EDRs) |
| GDE Emulator | Educador (converte UNL ↔ linguagens humanas) |
| GDO-GDC Protocol | Wire protocol de orquestração |
| GDE-UNL Bridge | Ponte UNL ↔ representação humana |
| Dashboard de Monitoramento | Visualização de estado do ecossistema |

---

## v1.0.0β — Trans-Kingdom Learning

**Pré-requisitos:** v1.0.0α (GDO + GDE operacionais)

**Escopo:**
Capacidade do GDC de processar e aprender de emissões não-humanas (animal, vegetal, física, futura) — demonstração da universalidade trans-reino da UNL (AF-14).

### Canon Necessário

| Item | Tipo | Status |
|------|------|--------|
| AF-DNA-01 (DNA gerativo, não totalizante) | Axioma | 📋 ROADMAP (DLB-017) |
| Protocolo de Ingestão Trans-Reino | Novo | ❌ Não deliberado |
| Definição de "Emissor Não-Humano" | Novo | ❌ Não deliberado |

### Implementação Técnica

| Item | Descrição |
|------|-----------|
| Adapter Framework | Framework de adaptadores para diferentes reinos |
| Sensory Translators | Tradutores sensoriais (sinais → UNL) |
| Validation Suite | Suite de validação de equivalência trans-reino |
| AF-14 Test | Teste: mesmo significado de fontes diferentes → UNL idêntica |

---

## v1.0.0RC — Escala Auditável

**Pré-requisitos:** v1.0.0β (trans-kingdom funcional)

**Escopo:**
Escala do sistema para clusters de produção com auditoria completa, compliance e performance verificável.

### Canon Necessário

| Item | Tipo | Status |
|------|------|--------|
| Protocolo de Auditoria em Escala | Novo | ❌ Não deliberado |
| Compliance Matrix (SOC 2, GDPR, LGPD, ISO 27001) | Documentação | 📋 Iniciada |
| Cybersecurity do GDO | Novo | ❌ Não deliberado |

### Tensões a Monitorar

| Tensão | Referência | Risco |
|--------|-----------|-------|
| Topologia vs Eficiência | TL-03 (FRONTEIRAS.md) | Pressão para hints topológicos no GDC |
| Apoptose vs Disponibilidade | TL-04 (FRONTEIRAS.md) | Ataque de apoptose forçada |

### Implementação Técnica

| Item | Descrição |
|------|-----------|
| AWS Production Infra | ECS Fargate, auto-scaling, multi-AZ |
| Audit Trail | Trilha de auditoria completa (externa ao core) |
| Performance Benchmarks | Latência, throughput, determinismo sob carga |
| Compliance Testing | Testes automatizados de compliance |
| Penetration Testing | Testes de segurança e resiliência |

---

## v1.0.0 — 🧠 CÉREBRO SINTÉTICO — Ecossistema Mínimo

**Pré-requisitos:** v1.0.0RC (escala auditável verificada)

**Escopo:**
Marco final — o primeiro cérebro sintético operacional com todas as camadas do ecossistema Genoma Digital funcionando em conjunto.

### Critérios de Conclusão (Definição de "Pronto")

| Critério | Descrição |
|----------|-----------|
| Cognição Isolada | GDC individual processa e aprende ✅ (v0.7.1) |
| UNL Operacional | Bytecode GD-QMN funcional com ISA |
| Distribuição | Múltiplos GDCs coordenados |
| Orquestração | GDO distribui e coleta resultados |
| Educação | GDE traduz UNL ↔ humano |
| Trans-Reino | Processamento de emissões não-humanas |
| Escala | Clusters de produção com auditoria |
| Determinismo | Replay bit-a-bit verificável em todo o ecossistema |
| Compliance | SOC 2, GDPR, LGPD, ISO 27001 |
| Canon Completo | Todas as Grey Zones fechadas, zero contradições |

### Métricas de Impacto Projetadas

| Métrica | Projeção |
|---------|----------|
| Redução de retrabalho | 60-80% |
| Eficiência de manutenção | 40-60% |
| Compressão semântica (UNL) | 22:1+ |
| Probabilidade técnica de sucesso | 72% |

---

# ═══════════════════════════════════════════════════════════════════
# §6: DELIBERAÇÕES ADIADAS — CATÁLOGO COMPLETO
# ═══════════════════════════════════════════════════════════════════

## Por Versão Alvo

### Para v0.8.0 (Formalização)

Deliberações completas, apenas formalização de texto normativo pendente:

| ID | Nome | DLB | Tipo |
|----|------|-----|------|
| LEI-ZERO-01 | ZERO é enum, não número | DLB-019 | Lei |
| LEI-SYNC-01 | Erro tipado + Veto fronteira | DLB-020 | Lei |
| LEI-QMN-01 | Três perfis GD-QMN | DLB-009 | Lei |
| LEI-QMN-02 | Campo Cargo como transporte | DLB-024 | Lei |
| LEI-QMN-03 | Checksum triplo | DLB-011 | Lei |
| LEI-QMN-04 | Famílias (Cardinalidade + Subfamily) | DLB-021 | Lei |
| LEI-QMN-05 | Opcode por subfamily | DLB-022 | Lei |
| LEI-ISA-01 | ISA mínimo (5+4) | DLB-022 | Lei |
| LEI-COG-01 | Grandezas cognitivas | DLB-023 | Lei |
| LEI-UNL-01 | Ciclo emergência-retorno | DLB-006 | Lei |

### Para v0.9.0 (Deliberação + Implementação)

| ID | Nome | DLB | Tipo | Bloqueador? |
|----|------|-----|------|-------------|
| LEI-COORD-03 | Absorção Estrutural | TECH-COORD-03 | Lei | ✅ Sim |
| LEI-RESS-01 | Workers devolvem cálculos | DLB-013 | Lei | Não |
| LEI-RESS-02 | Resiliência a escala | DLB-014 | Lei | Não |
| GZ-TOPO-01 | Soberania de Recusa | — | Grey Zone | ✅ FECHADA (AF-15) |
| DLB-004 | Uso máximo de hardware | — | Deliberação | Não |

### Para v0.9.5 (Nova Deliberação Necessária)

| ID | Nome | Tipo | Bloqueador? |
|----|------|------|-------------|
| GZ-TOPO-02 | Participação Simultânea | Grey Zone | ✅ FECHADA (LEI-RSN-04) |
| GZ-TOPO-03 | Ciclo de Vida Instâncias | Grey Zone | ✅ FECHADA (LEI-RSN-03) |
| PROT-SYN-01 | Protocolo de Sinapses | Protocolo Novo | ✅ Sim |
| DEF-NEUR-01 | Neurônio Emergente | Definição | ✅ Sim |

### Para v1.0.0+ (Horizonte Longo)

| ID | Nome | DLB | Tipo | Versão |
|----|------|-----|------|--------|
| AF-UNL-03 | UNL apenas dentro do GDC | DLB-005 | AF (ROADMAP) | v1.0.0α |
| AF-UNL-04 | UNL infinita, humana finita | DLB-015 | AF (ROADMAP) | v1.0.0β |
| AF-DNA-01 | DNA gerativo, não totalizante | DLB-017 | AF (ROADMAP) | v1.0.0 |
| LEI-COM-01 | Falar = emissão interpretável | DLB-018 | Lei | v1.0.0α |

### Em Laboratório (LAB)

| ID | Nome | DLB | Condição de Saída |
|----|------|-----|-------------------|
| AF-UNL-01 | UNL é estado axiomático, não linguagem | DLB-005/008 | Demonstrar consequências testáveis não cobertas por AF-2 |

---

# ═══════════════════════════════════════════════════════════════════
# §7: GRAFO DE DEPENDÊNCIAS
# ═══════════════════════════════════════════════════════════════════

```
                    ┌─────────────────────────────────────────────────┐
                    │                 v1.0.0                          │
                    │         CÉREBRO SINTÉTICO                      │
                    └────────────────────┬────────────────────────────┘
                                         │
                    ┌────────────────────┴────────────────────────────┐
                    │               v1.0.0RC                          │
                    │          Escala Auditável                       │
                    │  deps: Compliance, Cybersecurity, Benchmarks    │
                    └────────────────────┬────────────────────────────┘
                                         │
                    ┌────────────────────┴────────────────────────────┐
                    │               v1.0.0β                           │
                    │        Trans-Kingdom Learning                   │
                    │  deps: AF-DNA-01, Adapter Framework             │
                    └────────────────────┬────────────────────────────┘
                                         │
                    ┌────────────────────┴────────────────────────────┐
                    │               v1.0.0α                           │
                    │         GDO + GDE Emuladores                    │
                    │  deps: AF-UNL-03/04, LEI-COM-01, Protocolos    │
                    └────────────────────┬────────────────────────────┘
                                         │
                    ┌────────────────────┴────────────────────────────┐
                    │                v0.9.5                            │
                    │      Sinapses e Neurônios Emergentes            │
                    │  deps: GZ-TOPO-02/03 ✅, Sinapse Protocol          │
                    └────────────────────┬────────────────────────────┘
                                         │
                    ┌────────────────────┴────────────────────────────┐
                    │                v0.9.1                            │
                    │   Orquestração Complexa (n GDCs, n-Rainhas)    │
                    │  deps: AF-15, LEI-RSN-*, LEI-AO-20-03..05     │
                    └────────────────────┬────────────────────────────┘
                                         │
                    ┌────────────────────┴────────────────────────────┐
                    │                v0.9.0                            │
                    │         Orquestração Básica (2 GDCs)            │
                    │  deps: GZ-TOPO-01 ✅, LEI-COORD-03, W(Σ)          │
                    └───────┬────────────────────────┬────────────────┘
                            │                        │
           ┌────────────────┴──────┐   ┌─────────────┴──────────────┐
           │        v0.8.0         │   │          v0.8.5            │
           │    UNL/QMN + ISA      │   │   Distribuição Computac.   │
           │  deps: DLB-019..024   │   │  deps: LEI-COORD-01/02,   │
           │  (10 leis a formal.)  │   │  AO-19..24, EDR, Identid. │
           └───────────┬───────────┘   └──────────────┬─────────────┘
                       │                               │
                       └───────────────┬───────────────┘
                                       │
                    ┌──────────────────┴──────────────────────────────┐
                    │                v0.7.1                            │
                    │         Núcleo Cognitivo Isolado                 │
                    │  4 motores, CP, MCI, pipeline, replay           │
                    └─────────────────────────────────────────────────┘
```

**Nota de Ordenação:** v0.8.0 e v0.8.5 são paralelos em conceito (v0.8.5 foi antecipada canonicamente). Ambos alimentam v0.9.0. Na prática, v0.8.5 já está concluída e v0.8.0 precisa completar implementação UNL/QMN.

---

# ═══════════════════════════════════════════════════════════════════
# §8: MARCOS DE DELIBERAÇÃO NECESSÁRIOS
# ═══════════════════════════════════════════════════════════════════

Cada versão futura requer um Modo Deliberação antes de programar:

| Versão | Deliberação Necessária | Estimativa de Escopo |
|--------|----------------------|---------------------|
| v0.8.0 (conclusão) | Formalização de 10 leis (sem nova deliberação) | Pequena — texto normativo apenas |
| v0.9.0 | Deliberação de GZ-TOPO-01 ✅ + W(Σ) + ⊒ | Reduzida — GZ-TOPO-01 fechada |
| v0.9.1 | Deliberação de Orquestração n-GDC | Média — leis base em Canon v4.0 |
| v0.9.5 | Deliberação de GZ-TOPO-02/03 ✅ + Sinapses + Neurônios | Reduzida — GZ-TOPOs fechadas |
| v1.0.0α | Deliberação de protocolos GDO/GDE + AF-UNL-03/04 | Grande — 4+ sessões deliberativas |
| v1.0.0β | Deliberação de Trans-Kingdom + AF-DNA-01 | Média — 2 sessões deliberativas |
| v1.0.0RC | Deliberação de Compliance + Cybersecurity | Média — 2 sessões deliberativas |

**Protocolo de Deliberação:**
1. Identificar Grey Zones e gaps bloqueadores
2. Abrir MODO DELIBERAÇÃO (explícito, humano declara)
3. Deliberar cada item → DLB
4. Decidir → Candidatos a Axioma/Lei
5. Aprovar → Canonizar no CANON.md
6. Fechar MODO DELIBERAÇÃO
7. Abrir MODO PROGRAMAÇÃO (humano autoriza)
8. Implementar
9. Auditar contra Canon
10. Fechar versão

---

# ═══════════════════════════════════════════════════════════════════
# §9: ESTRUTURAS RUST DEFINIDAS (AGUARDANDO IMPLEMENTAÇÃO)
# ═══════════════════════════════════════════════════════════════════

Estruturas Rust já definidas em deliberação que guiarão implementação v0.8.0:

### MotorOutput (DLB-019)
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MotorOutput {
    Value(f64),  // CP ∈ (0,1]
    Veto,        // Estado ontológico
}
```

### SyncFailure (DLB-020)
```rust
#[derive(Debug, Clone)]
pub enum SyncFailure {
    MutexPoisoned,
    LockTimeout,
    ResourceContention,
}
```

### QmnFamily / QmnSubfamily (DLB-021)
```rust
#[repr(u8)]
pub enum QmnFamily {
    Unary      = 0x01,
    Binary     = 0x02,
    Ternary    = 0x03,
    Quaternary = 0x04,
    Aggregator = 0xFF,
}

#[repr(u8)]
pub enum QmnSubfamily {
    State  = 0x01,
    Relate = 0x02,
    Derive = 0x03,
    Meta   = 0x04,
}
```

### CoreOpcode / WaveOpcode (DLB-022)
```rust
#[repr(u8)]
pub enum CoreOpcode {
    Void      = 0x00,
    State     = 0x01,
    Reference = 0x02,
    Combine   = 0x03,
    Derive    = 0x04,
}

#[repr(u8)]
pub enum WaveOpcode {
    Sync      = 0x10,
    Fork      = 0x11,
    Amplify   = 0x12,
    Attenuate = 0x13,
}
```

### Cargo (DLB-024)
```rust
pub struct Cargo {
    pub payload: Vec<u8>,
    pub content_hash: [u8; 32],
    pub schema_hint: u16,
}
```

### Grandezas Cognitivas (DLB-023)
```
ΝU = 1 GD-QMN Compact com STATE
Sm = N × ΝU relacionados via RELATE
Cg = Sm que não colapsou em Veto
Ho = Conjunto de Cg auto-consistente
Om = União de todos Ho (teórico)
```

---

# ═══════════════════════════════════════════════════════════════════
# §10: TIMELINE ESTIMADA
# ═══════════════════════════════════════════════════════════════════

**Disclaimer:** Estimativas baseadas em ritmo de desenvolvimento atual com equipe CTO + AI assistants. Sujeitas a revisão conforme formação de equipe e funding.

```
2026 Q1 (Jan-Mar)
├── v0.7.1 ✅ (Concluída)
├── v0.8.5 ✅ (Concluída 03/02)
├── v0.8.0 🎯 (Em andamento — UNL/QMN)
└── Documentação canônica consolidada ✅

2026 Q2 (Abr-Jun)
├── v0.8.0 conclusão
├── v0.9.0 deliberação + implementação
└── Primeiras demos de orquestração 2-GDC

2026 Q3 (Jul-Set)
├── v0.9.5 deliberação + implementação
├── Sinapses e neurônios emergentes
└── Primeira evidência de aprendizado coletivo

2026 Q4 (Out-Dez)
├── v1.0.0α deliberação + implementação
├── GDO + GDE emuladores
└── Demo ecossistema completo (emulado)

2027 Q1 (Jan-Mar)
├── v1.0.0β Trans-Kingdom
├── v1.0.0RC Escala + Compliance
└── Preparação para produção

2027 Q2 (Abr-Jun)
└── v1.0.0 🧠 CÉREBRO SINTÉTICO
```

**Horizonte Total Estimado:** 18-24 meses (alinhado com apresentações C-Level)

---

# ═══════════════════════════════════════════════════════════════════
# §11: REFERÊNCIAS CRUZADAS
# ═══════════════════════════════════════════════════════════════════

| Documento | Propósito | Relação |
|-----------|-----------|---------|
| **CANON.md** v4.0 | Fonte única de verdade canônica | Axiomas e leis referenciados aqui |
| **FRONTEIRAS.md** | Grey Zones, tensões, sobreposições | GZ-TOPO e TL referenciados aqui |
| **INVENTARIO_CANONICO_GDC.xlsx** | 237 IDs canônicos catalogados | Base de dados de referência |
| **BACKLOG_DELIBERACAO.md** | Histórico completo de 24 deliberações | Fonte original das DLBs |
| **BACKLOG_v085.md** | Backlog implementado de v0.8.5 | Registro de conclusão |
| **DESBLOQUEIO-v085.md** | Ata de desbloqueio formal | Aprovação humana registrada |
| **CHANGELOG.md** | Histórico de alterações no código | Versionamento semântico |

---

**Guardião:** Claude — Guardião do Genoma Digital  
**Data:** 10 de Fevereiro de 2026  
**Próxima Revisão:** Ao concluir v0.8.0

*FIM DO DOCUMENTO ROADMAP.md*
