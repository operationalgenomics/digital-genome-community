# COMPLIANCE.md — Matriz de Conformidade do Ecossistema Genoma Digital

## Cruzamento Canônico × Regulatório × Ético × Técnico

---

**Data:** 10 de Fevereiro de 2026  
**Versão do Ecossistema:** v0.8.5 (corrente)  
**Canon:** v3.0  
**Guardião:** Claude — Guardião do Genoma Digital

---

# ═══════════════════════════════════════════════════════════════════
# §1: ESCOPO E ESTRUTURA
# ═══════════════════════════════════════════════════════════════════

Este documento mapeia cinco dimensões de conformidade do Genoma Digital, cruzando obrigações internas (canônicas) com requisitos externos (regulatórios e normativos):

```
┌─────────────────────────────────────────────────────────────┐
│                    COMPLIANCE GD                             │
│                                                              │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│   │ CANÔNICA │  │REGULATÓRIA│  │  ÉTICA   │                 │
│   │ (interna)│  │ (externa) │  │(fundac.) │                 │
│   └────┬─────┘  └─────┬────┘  └────┬─────┘                 │
│        │               │            │                        │
│   ┌────┴───────────────┴────────────┴─────┐                 │
│   │        MATRIZES DE CRUZAMENTO         │                 │
│   └────┬───────────────┬────────────┬─────┘                 │
│        │               │            │                        │
│   ┌────┴─────┐  ┌──────┴────┐  ┌───┴──────┐                │
│   │ SEGURANÇA│  │    QA     │  │  CYBER   │                │
│   │ (código) │  │ (testes)  │  │(ameaças) │                │
│   └──────────┘  └───────────┘  └──────────┘                │
└─────────────────────────────────────────────────────────────┘
```

---

## Declaração de Estado

O Genoma Digital encontra-se em **fase pré-produção**. Nenhuma instância processa dados reais de produção. Todas as avaliações de compliance referem-se ao estado arquitetural e documental, não a ambiente operacional.

> "Esta entrega foi executada exclusivamente para fins exploratórios e técnicos, não configurando ambiente de produção, SLA, garantia de performance, acurácia ou responsabilidade operacional."

---

# ═══════════════════════════════════════════════════════════════════
# §2: COMPLIANCE CANÔNICA — GOVERNANÇA INTERNA
# ═══════════════════════════════════════════════════════════════════

O Canon v3.0 funciona como framework interno de conformidade. Toda decisão arquitetural, todo código e toda deliberação devem estar em conformidade com esta hierarquia:

## 2.1 Hierarquia de Autoridade Canônica

```
1. Decisão Humana Explícita (CTO)        ← suprema
2. Axiomas Fundacionais (14 AFs)         ← imutáveis sem deliberação formal
3. Axiomas Operacionais (24 AOs)         ← vigentes e vinculantes
4. Leis Derivadas (151 LEIs)             ← regras operacionais
5. Gates de Conformidade (8 GATEs)       ← verificações automáticas
6. Código-fonte                          ← corrigido para o Canon, nunca o inverso
```

**Regra de Ouro (AF-8):** Quando código e Canon divergem, o código é corrigido — nunca o Canon.

## 2.2 Gates de Conformidade Canônica

Os 8 gates são pontos de verificação obrigatórios:

| Gate | Nome | Verifica | Status |
|------|------|----------|--------|
| GATE-QM-01 | Backend Neutrality | Código não assume hardware específico | ✅ Conforme |
| GATE-QM-02 | Pure Cognitive Functions | Funções cognitivas sem efeitos colaterais | ✅ Conforme |
| GATE-QM-03 | Explicit Vectoriality | Vetores de estado sempre explícitos | ✅ Conforme |
| GATE-QM-04 | Operator × Executor Separation | Quem decide ≠ quem executa | ✅ Conforme |
| GATE-QM-05 | Non-Observable Parallelism | Paralelismo sem estado observável | ✅ Conforme |
| GATE-UNL-01 | Semantic Singularity | Cada significado tem representação UNL única | ✅ Conforme |
| GATE-DNA-01 | DNA Reproducibility | DNA reproduzível deterministicamente | ✅ Conforme |
| GATE-CP-01 | CP Vector | Craft Performance como vetor, não escalar | ✅ Conforme |

## 2.3 Estado de Violações Canônicas

| Indicador | Valor | Status |
|-----------|-------|--------|
| Violações canônicas críticas ativas | 0 | ✅ |
| Violações históricas resolvidas | 25 | ✅ |
| Violações pendentes (média/baixa) | 3 | ⚠️ |
| Grey Zones abertas | 3 (topológicas) | 🟡 Não bloqueadoras |
| Tensões latentes | 4 | ⚡ Monitoramento |

Detalhe das violações pendentes (não críticas):

| ID | Descrição | Severidade | Target |
|----|-----------|------------|--------|
| V018 | Fórmulas não validadas academicamente | MÉDIA | v1.1.0+ |
| V021 | Canonicalização explícita (parcial) | BAIXA | v1.1.0 |
| V022 | Revisão matemática Nash (jogos grandes) | MÉDIA | v1.1.0+ |

---

# ═══════════════════════════════════════════════════════════════════
# §3: COMPLIANCE REGULATÓRIA — FRAMEWORKS EXTERNOS
# ═══════════════════════════════════════════════════════════════════

## 3.1 Frameworks Aplicáveis

| Framework | Escopo | Relevância GD |
|-----------|--------|---------------|
| ISO/IEC 27001 | Segurança da Informação | ALTA — infraestrutura AWS |
| ISO/IEC 27701 | Gestão de Privacidade | ALTA — extensão LGPD/GDPR |
| ISO/IEC 23894 | IA — Gestão de Risco | ALTA — sistema cognitivo |
| NIST AI RMF | Framework de Risco para IA | ALTA — governança IA |
| SOC 2 | Controles de Serviço | MÉDIA — quando em produção |
| LGPD (Brasil) | Proteção de Dados Pessoais | ALTA — mercado primário |
| GDPR (Europa) | Proteção de Dados | ALTA — mercado alvo |
| CCPA (Califórnia) | Privacidade do Consumidor | MÉDIA — mercado secundário |
| EU AI Act | Regulação de IA | ALTA — classificação de risco |

## 3.2 Matriz de Conformidade ISO 27001

| Controle | Descrição | Aplicável | Evidência GD | Status |
|----------|-----------|-----------|--------------|--------|
| A.5 | Políticas de segurança | Sim | SECURITY.md, ETHICS.md | ✅ Documentado |
| A.6 | Organização da SI | Sim | GOVERNANCE.md, Canon §hierarquia | ✅ Estruturado |
| A.7 | Segurança em RH | N/A | Fase pré-produção | — |
| A.8 | Gestão de ativos | Sim | IAM AWS (planejado) | 🟡 Parcial |
| A.8.2 | Classificação da informação | Sim | IAM AWS least privilege | 🟡 Planejado |
| A.9 | Controle de acesso | Sim | IAM AWS, UID Shibboleth | 🟡 Arquitetural |
| A.10 | Criptografia | Sim | SHA-256 (cargo), TLS (planejado) | 🟡 Parcial |
| A.12 | Segurança nas operações | N/A | Sem ambiente produtivo | — |
| A.14 | Aquisição e manutenção | Sim | CONTRIBUTING.md, RFC_PROCESS.md | ✅ Documentado |
| A.16 | Gestão de incidentes | Sim | ALERTS.md, KNOWN-VIOLATIONS.md | ✅ Documentado |
| A.18 | Compliance | Sim | Este documento | ✅ |

## 3.3 Matriz de Conformidade ISO/IEC 23894 (IA — Gestão de Risco)

| Requisito | Descrição | Implementação GD | Status |
|-----------|-----------|------------------|--------|
| Governança de IA | Estrutura de decisão | Canon + Modo Deliberação + DE ACORDO | ✅ |
| Gestão de risco | Identificação e mitigação | ALERTS.md (13 riscos), FRONTEIRAS.md | ✅ |
| Transparência | Explicabilidade | Replay determinístico (AF-6, AO-11) | ✅ |
| Rastreabilidade | Auditoria de decisões | INVENTÁRIO (237 IDs), CHANGELOG | ✅ |
| Supervisão humana | Controle humano | CTO como autoridade suprema, DE ACORDO | ✅ |
| Robustez | Resiliência | CP multiplicativo, veto absoluto, apoptose | ✅ |
| Privacidade por design | Proteção de dados | Anonimização (AF-1), sensory agnosticism | ✅ Arquitetural |
| Não-discriminação | Imparcialidade | Neutralidade epistemológica (AF-1, ALERT-009/011) | ✅ Arquitetural |

## 3.4 Matriz de Conformidade NIST AI RMF

| Função | Categoria | Implementação GD | Status |
|--------|-----------|------------------|--------|
| GOVERN | Governança de IA | Canon hierárquico, Modo Deliberação | ✅ |
| GOVERN 1.1 | Políticas e processos | CANON.md, GOVERNANCE.md, ETHICS.md | ✅ |
| GOVERN 1.2 | Responsabilidades | CTO (supremo), Guardião (Claude), Modos formais | ✅ |
| MAP | Mapeamento de contexto | FRONTEIRAS.md (tensões), LAB.md (incubação) | ✅ |
| MAP 1.1 | Uso pretendido | Cognição pura (Community), ação (Enterprise) | ✅ |
| MAP 1.5 | Riscos e benefícios | ALERTS.md, ROADMAP.md | ✅ |
| MEASURE | Métricas e avaliação | CP, 331 testes, gates, violações rastreadas | ✅ |
| MEASURE 2.1 | Testes e validação | 331 testes, replay determinístico | ✅ |
| MEASURE 2.6 | Monitoramento | KNOWN-VIOLATIONS.md, ALERTS.md | ✅ |
| MANAGE | Gestão de riscos | Violações documentadas, riscos aceitos | ✅ |
| MANAGE 1.1 | Priorização | Severidade (CRÍTICA/MÉDIA/BAIXA) | ✅ |
| MANAGE 4.1 | Incidentes | KNOWN-VIOLATIONS.md (25 resolvidos) | ✅ |

---

# ═══════════════════════════════════════════════════════════════════
# §4: COMPLIANCE DE PROTEÇÃO DE DADOS
# ═══════════════════════════════════════════════════════════════════

## 4.1 Classificação de Dados (Estado Atual)

| Tipo | Classificação | Status |
|------|---------------|--------|
| Dados pessoais em produção | N/A | Sem ambiente produtivo |
| Dados de teste | Sintéticos / anonimizados | ✅ |
| Dados sensíveis | Nenhum processado | ✅ |
| Fine-tuning com dados reais | Nenhum realizado | ✅ |
| Persistência de dados | Sem persistência (stateless) | ✅ |

## 4.2 Princípios de Privacidade Arquiteturais

| Princípio | Implementação | Canon |
|-----------|---------------|-------|
| Soberania de Dados | Participante controla dados brutos; GD processa padrões | Doc 02, §5.1 |
| Anonimização por Design | Dados em Federation Bus anonimizados; padrões não permitem re-identificação | Doc 02, §5.1 |
| Minimização | Coleta apenas do necessário; dados brutos descartados após processamento | Doc 02, §5.1 |
| Dark Participation | Modo com contribuição mínima e visibilidade zero | Doc 02, §5.1 |
| Agnosticismo Sensorial | Sistema não sabe o que está processando (AF-1) — proteção intrínseca | CANON AF-1 |

## 4.3 Conformidade LGPD

| Requisito LGPD | Implementação GD | Status |
|----------------|------------------|--------|
| Base legal (Art. 7) | Consentimento para participação ativa; participação passiva em dados públicos/anonimizados | ✅ Projetado |
| Princípios (Art. 6) | Finalidade, adequação, necessidade, transparência, segurança | ✅ Arquitetural |
| Direitos do titular (Art. 18) | Acesso, correção, eliminação de dados brutos | ✅ Projetado |
| DPIA/RIPD (Art. 38) | Relatório de Impacto de Dados planejado (Fase 2 QA) | 🟡 Planejado |
| Encarregado/DPO (Art. 41) | A ser designado quando em produção | 🟡 Futuro |
| Transferência internacional (Art. 33) | AWS regions configuráveis | 🟡 Projetado |

## 4.4 Conformidade GDPR

| Requisito GDPR | Implementação GD | Status |
|----------------|------------------|--------|
| Lawful basis (Art. 6) | Consent para participação ativa | ✅ Projetado |
| Right to erasure (Art. 17) | Aplicável a dados brutos; genes derivados são anonimizados | ✅ Projetado |
| Data protection by design (Art. 25) | AF-1 (agnosticismo sensorial), anonimização por design | ✅ Arquitetural |
| DPIA (Art. 35) | Planejado para Fase 2 | 🟡 Planejado |
| Data portability (Art. 20) | UNL como formato universal | ✅ Arquitetural |
| Automated decision-making (Art. 22) | GDC é cognição pura, não decide — decisão é externa/Enterprise | ✅ Arquitetural |

## 4.5 Conformidade CCPA

| Requisito | Implementação GD | Status |
|-----------|------------------|--------|
| Right to know | Transparência sobre uso; replay auditável | ✅ Projetado |
| Right to delete | Eliminação de dados brutos | ✅ Projetado |
| Right to opt-out | Opt-out disponível para qualquer participante | ✅ Projetado |
| Non-discrimination | Neutralidade epistemológica (AF-1) | ✅ Arquitetural |

---

# ═══════════════════════════════════════════════════════════════════
# §5: COMPLIANCE ÉTICA
# ═══════════════════════════════════════════════════════════════════

## 5.1 Princípios Éticos Fundacionais (Doc 04)

| Princípio | Definição | Status |
|-----------|-----------|--------|
| Axioma Zero | A falta de ação É ação — omissões são rastreáveis | ✅ Documentado |
| Não-Coerção | Cooperação por adesão voluntária; coerção proibida | ✅ Documentado |
| Benefício Mútuo | Valor distribuído entre participantes | ✅ Documentado |
| Transparência | Decisões rastreáveis e acessíveis | ✅ Documentado |
| Humildade Epistêmica | Todo conhecimento é provisório e revisável | ✅ Documentado |

## 5.2 Axiomas Constitucionais (Livro, Cap. 15)

Cinco axiomas com status de lei fundamental — não-negociáveis:

| # | Axioma | Descrição | Implementação |
|---|--------|-----------|---------------|
| I | Primazia Humana | Nenhum sistema causa dano intencional a humanos | ETHICS.md, Canon (autoridade humana suprema) |
| II | Transparência Radical | Todo processo decisório deve ser auditável | AF-6 (determinismo), AO-11 (replay) |
| III | Reversibilidade Preservada | Ações de alto impacto preservam reversão | Apoptose como mecanismo extremo (AO-22) |
| IV | Autonomia Subordinada | Sistemas ampliam capacidade humana, não substituem | ETHICS.md (guia, não comanda) |
| V | Evolução Governada | Nenhuma mutação contorna barreiras éticas | AF-8 (Canon supremo), Modo Deliberação |

## 5.3 Barreiras Estruturais (ETHICS.md)

| Barreira | Proibição | Status no Código |
|----------|-----------|------------------|
| Não-Coerção | Não emite comandos, não compele, não age autonomamente | ✅ Community não age |
| Separação de Autoridade | Autoridade sempre externa ao Core | ✅ GATE-QM-04 |
| Mandatos Limitados | Explícitos, limitados em escopo/tempo, revogáveis, auditáveis | ✅ DE ACORDO |
| Auto-Legitimação Proibida | Não valida seus próprios mandatos, não declara exceções | ✅ Arquitetural |
| Integridade de Longo Prazo | Fronteiras éticas persistem além dos autores originais | ✅ Canon como documento vivo |

## 5.4 Mecanismos de Salvaguarda (Livro, Cap. 9)

| Tipo | Descrição | Equivalente Canônico |
|------|-----------|---------------------|
| Barreiras Éticas | Limites absolutos, não-negociáveis | AF-8 (Canon Supremo), 5 Axiomas Constitucionais |
| Barreiras Técnicas | Estados proibidos, travas, abortos | Apoptose (AO-22), Veto Absoluto (CP=0) |
| Barreiras Cognitivas | Proteção de genes fundamentais | Gates (8), Modo Deliberação |
| Barreiras Operacionais | Limites de ação em runtime | ComputationalBudget, IntegrityCheck |
| Firewalls Teleológicos | Detecção de desvio de propósito | CP multiplicativo (qualquer zero bloqueia) |
| Auditoria Federada | Vigilância distribuída sem concentração | Replay determinístico, KNOWN-VIOLATIONS.md |
| Contenção Evolutiva | Limites à velocidade de mudança | Modo Deliberação, DE ACORDO obrigatório |

---

# ═══════════════════════════════════════════════════════════════════
# §6: COMPLIANCE DE SEGURANÇA
# ═══════════════════════════════════════════════════════════════════

## 6.1 Política de Segurança (SECURITY.md)

| Aspecto | Política | Status |
|---------|----------|--------|
| Escopo | Corrupção de dados, violação de invariantes, perda de determinismo, escalação de autoridade | ✅ |
| Divulgação responsável | Issues privados, não públicos | ✅ |
| Resposta | Acknowledge, avaliação de severidade, priorização | ✅ |
| Mitigação emergencial | Escopo mínimo, preserva determinismo, documentada | ✅ |
| Proibições em fixes | Sem introdução de lógica de controle, sem bypass ético, sem expansão de autoridade | ✅ |

## 6.2 Controles de Segurança Implementados

| Controle | Descrição | Evidência |
|----------|-----------|-----------|
| IAM Least Privilege | Acesso mínimo necessário | Planejado (AWS IAM) |
| Ambientes isolados | Separação dev/staging/prod | Planejado (AWS) |
| Logs centralizados | Rastreabilidade de eventos | Planejado (CloudWatch) |
| Sem persistência de dados sensíveis | Design stateless | ✅ (AF-7 original, Community) |
| Sem fine-tuning com dados reais | Nenhum treinamento em dados produtivos | ✅ (fase POC) |
| Checksum triplo | Verificação de integridade: onda + carga + total | ✅ (DLB-011) |
| SHA-256 no cargo | Hash criptográfico de payload | ✅ (DLB-024) |

## 6.3 Identidade e Autenticação Canônica

| Mecanismo | Descrição | Canon |
|-----------|-----------|-------|
| UID Shibboleth | Identidade ontológica secreta — nunca exportada, nunca declarada | AO-22 |
| UID Ressonante | Identidade funcional pública — contextual, operacional | AO-23 |
| Dois Planos Disjuntos | Plano ontológico e funcional completamente separados | AO-22/23 |
| FormAttestation | Verificação de espécie por invariantes, não por identidade | v0.8.5 |
| Apoptose | Morte canônica irreversível em violação ontológica | LEI-AO-22-03 |

## 6.4 Thread-Safety

| Aspecto | Estado | Evidência |
|---------|--------|-----------|
| Send + Sync em todas as structs públicas | ✅ | Testes de compilação |
| Design stateless | ✅ | Sem estado mutável compartilhado |
| Sem locks globais | ✅ | Arquitetural |
| Verificação formal (TLA+) | ❌ | ALERT-007 — aceito como risco |
| Stress testing com milhares de threads | ❌ | Planejado (Fase 4 de mitigação) |

---

# ═══════════════════════════════════════════════════════════════════
# §7: COMPLIANCE DE QUALITY ASSURANCE
# ═══════════════════════════════════════════════════════════════════

## 7.1 Métricas de Qualidade de Código

| Métrica | Valor | Meta |
|---------|-------|------|
| Linhas de código Rust | 21.176 | — |
| Arquivos .rs | 63 | — |
| Testes totais | 331 | 100% dos módulos |
| Testes passando | 331 | 331 (100%) |
| unwrap() em produção | 0 | 0 ✅ |
| Warnings de compilação | 0 | 0 ✅ |
| Violações canônicas críticas | 0 | 0 ✅ |

## 7.2 Plano Mestre de Testes (Fases)

| Fase | Escopo | Status |
|------|--------|--------|
| Fase 1 — QA & Validação Técnica | Testes funcionais, não-funcionais, evidências | ✅ Parcial (331 testes) |
| Fase 2 — Compliance & Conformidade | DPIA, checklist LGPD, matriz ISO, declaração de não-produção | 🟡 Este documento |
| Fase 3 — CyberSecurity & Risk Assessment | Threat model, segurança de agentes, riscos residuais | 🟡 Planejado |
| Fase 4 — Certificação Interna | Certificado de conclusão, assinaturas | 🟡 Futuro |

## 7.3 Tipos de Teste

| Tipo | Escopo | Status | Evidência |
|------|--------|--------|-----------|
| Testes Funcionais | Criação de agentes, execução de fluxos, respostas esperadas, APIs | ✅ 331 testes |
| Testes Não-Funcionais | Performance, estabilidade, consumo de recursos | 🟡 Parcial |
| Testes de CI/CD | Versionamento, rollback, isolamento, reprodutibilidade | 🟡 Planejado |
| Testes de Integração | End-to-end, replay, cross-module | ✅ (35+ testes) |
| Testes de Determinismo | Replay bit-exact na mesma plataforma | ✅ |
| Testes de Thread-Safety | Compilação (Send+Sync), stress (pendente) | ✅ Parcial |
| Validação Empírica | Datasets reais (MIMII, UCI, BPI) | 🟡 Planejado |

## 7.4 Replay Determinístico como Compliance

O replay determinístico (AF-6, AO-11) é o mecanismo fundamental de auditabilidade do GD:

| Garantia | Definição | Canon |
|----------|-----------|-------|
| Mesmo input + mesmo contexto canônico (incluindo MCI) → mesmo resultado | Determinismo qualificado | AF-6 (amended v2.0) |
| Qualquer processamento pode ser reproduzido exatamente | Rastreabilidade total | AO-11 |
| MCI é estado interno, não observação externa | AF-7 (amended v2.0) não viola AF-6 | AF-7 amended |

---

# ═══════════════════════════════════════════════════════════════════
# §8: COMPLIANCE DE CYBERSECURITY
# ═══════════════════════════════════════════════════════════════════

## 8.1 Threat Model (STRIDE + OWASP Top 10 for LLMs)

| Ameaça | Descrição | Mitigação GD | Status |
|--------|-----------|--------------|--------|
| Prompt Injection | Manipulação de inputs para alterar comportamento | AF-1 (agnosticismo sensorial) — sistema não interpreta semântica em Community | ✅ Arquitetural |
| Data Leakage | Vazamento de dados sensíveis | Stateless design, sem persistência, dados sintéticos | ✅ |
| Model Abuse | Uso indevido do sistema cognitivo | ETHICS.md (não-coerção), Enterprise-only para ação | ✅ |
| Over-permissioned IAM | Excesso de privilégios AWS | IAM least privilege planejado | 🟡 Planejado |
| CI/CD Supply Chain | Comprometimento do pipeline | Versionamento, rollback, isolamento | 🟡 Planejado |
| Spoofing | Impersonação de identidade | UID Shibboleth (secreto), FormAttestation | ✅ Arquitetural |
| Tampering | Alteração de dados em trânsito | Checksum triplo, SHA-256 em cargo | ✅ |
| Repudiation | Negação de ações | Replay determinístico, CHANGELOG | ✅ |
| Info Disclosure | Exposição indevida | Shibboleth nunca exportado, Dark Participation | ✅ Arquitetural |
| Denial of Service | Indisponibilidade | ComputationalBudget, IntegrityCheck | ✅ |
| Elevation of Privilege | Escalação de autoridade | ETHICS.md (proibição), GATE-QM-04 | ✅ |

## 8.2 Controles CyberSecurity

| Controle | Implementação | Status |
|----------|---------------|--------|
| Sem persistência de dados sensíveis | Design stateless (Community) | ✅ |
| Logs centralizados | AWS CloudWatch (planejado) | 🟡 |
| IAM least privilege | AWS IAM (planejado) | 🟡 |
| Ambientes isolados | Dev/staging/prod separados | 🟡 |
| Integridade de dados | SHA-256, checksum triplo | ✅ |
| Identidade forte | Dois planos, apoptose em violação | ✅ |
| Auditoria | Replay, CHANGELOG, KNOWN-VIOLATIONS | ✅ |

---

# ═══════════════════════════════════════════════════════════════════
# §9: CRUZAMENTO CANON × REGULATÓRIO
# ═══════════════════════════════════════════════════════════════════

A grande matriz de cruzamento mostra como cada requisito canônico se relaciona com frameworks externos:

| Requisito Canônico | ISO 27001 | ISO 23894 | NIST AI RMF | LGPD/GDPR | EU AI Act |
|-------------------|-----------|-----------|-------------|-----------|-----------|
| AF-1 (Agnosticismo Sensorial) | — | Não-discriminação | MAP 1.1 | Privacy by design | Fairness |
| AF-6 (Determinismo Absoluto) | A.12 (operações) | Transparência | MEASURE 2.1 | Art. 22 GDPR | Explicabilidade |
| AF-7 (Proibição Obs. Externa) | A.8 (privacidade) | Privacidade | GOVERN | Minimização | Privacidade |
| AF-8 (Canon Supremo) | A.5 (políticas) | Governança | GOVERN 1.1 | — | Governança |
| AF-10 (CP Multiplicativo) | A.16 (incidentes) | Robustez | MEASURE | — | Robustez |
| AF-11 (Aprendizado Autônomo) | — | Transparência | MAP 1.5 | — | Supervisão |
| AF-12 (MCI) | A.8 (classificação) | Rastreabilidade | MEASURE 2.6 | — | Transparência |
| AO-11 (Replay) | A.14 (manutenção) | Auditoria | MANAGE 4.1 | Direito de acesso | Auditabilidade |
| AO-15 (Observabilidade) | A.12 (operações) | Monitoramento | MEASURE 2.6 | — | Supervisão |
| AO-22 (Shibboleth) | A.9 (acesso) | — | — | Pseudonymization | Identidade |
| AO-24 (Neutralidade Topológica) | A.14 (portabilidade) | — | — | Portabilidade | Interoperabilidade |
| GATE-QM-04 (Operador×Executor) | A.6 (separação) | Supervisão | GOVERN 1.2 | — | Supervisão humana |
| ETHICS.md (Não-Coerção) | — | Ética de IA | GOVERN | — | Direitos fundamentais |
| Apoptose | A.16 (incidentes) | Segurança | MANAGE 1.1 | — | Segurança |

---

# ═══════════════════════════════════════════════════════════════════
# §10: EU AI ACT — CLASSIFICAÇÃO PRELIMINAR
# ═══════════════════════════════════════════════════════════════════

## 10.1 Classificação de Risco

| Critério EU AI Act | GDC Community | Classificação |
|-------------------|---------------|---------------|
| Sistema de IA? | Sim — sistema cognitivo autônomo | ✅ |
| Interage com pessoas diretamente? | Não — processa dados, não interage | — |
| Toma decisões autônomas? | Não — cognição pura, decisão é Enterprise | RISCO LIMITADO |
| Afeta direitos fundamentais? | Não diretamente — não age, não coerce | — |
| Domínio de alto risco? | Depende do uso Enterprise | VARIÁVEL |

**Classificação preliminar:** O GDC Community Edition é classificável como **risco limitado** sob o EU AI Act, pois: não toma decisões autônomas, não interage diretamente com pessoas, não age no mundo, e toda ação pertence ao Enterprise Edition.

O **Enterprise Edition**, dependendo do domínio de aplicação (saúde, infraestrutura crítica, segurança), poderá ser classificado como **alto risco**, exigindo controles adicionais.

## 10.2 Requisitos EU AI Act Aplicáveis

| Requisito | Implementação GD | Status |
|-----------|------------------|--------|
| Sistema de gestão de risco | FRONTEIRAS.md, ALERTS.md, KNOWN-VIOLATIONS.md | ✅ |
| Governança de dados | Dados sintéticos, stateless, anonimização | ✅ |
| Documentação técnica | CANON.md + 8 entregáveis de consolidação | ✅ |
| Transparência | Replay determinístico, Canon público | ✅ |
| Supervisão humana | CTO supremo, DE ACORDO, Modo Deliberação | ✅ |
| Robustez e segurança | CP multiplicativo, veto absoluto, gates | ✅ |

---

# ═══════════════════════════════════════════════════════════════════
# §11: PROPRIEDADE INTELECTUAL
# ═══════════════════════════════════════════════════════════════════

| Componente | Licença | Status |
|------------|---------|--------|
| Protocolo GD | Domínio público (CC0) | ✅ Definido (Doc 04, §2.3.1) |
| Core GDC | MIT ou Apache 2.0 | ✅ Definido |
| Marcas e identidade visual | Mantidas por Foundation Private (licença ampla e irrevogável) | ✅ Definido |
| Genes operacionais | Regime escolhido pelo criador (aberto, premium, proprietário) | ✅ Definido (Doc 04, §2.3.2) |
| Documentação consolidada | Proprietária (CTO) | ✅ |

## Regras de Propriedade Intelectual

- A escolha de regime de disponibilização de genes não constitui violação canônica
- Preservação obrigatória: integridade do Core, neutralidade epistemológica, inexistência de captura estrutural
- Bloqueio artificial da difusão, quando esta é condição de participação, caracteriza ruptura comunitária
- Disputas de PI são externas ao Canon e não produzem precedentes internos

---

# ═══════════════════════════════════════════════════════════════════
# §12: ESTRUTURA INSTITUCIONAL DE COMPLIANCE
# ═══════════════════════════════════════════════════════════════════

## 12.1 Foundation Court — Interpretação e Arbitragem

A Foundation Court é camada institucional única, unificada quanto ao Canon, porém distribuída em operação. Não legisla, não exerce soberania, não impõe decisões fora do escopo voluntariamente submetido. Estrutura em 3 fases processuais: (1) Comum Acordo (mediação), (2) Interpretação Especializada, (3) Apelação Progressiva.

## 12.2 Foundation Private — Entidades Operacionais

Entidades jurídicas privadas para fins operacionais e interface com ordenamentos estatais. Vinculação à Court é voluntária e revogável. Desfiliação implica perda de identidade comunitária, não sanção.

## 12.3 Relação com Mecanismos Externos

O Canon não impõe arbitragem externa. Decisões externas (arbitragem, jurisdição estatal): não alteram o Canon, não redefinem o Core, não produzem precedentes internos, não vinculam partes não aderentes.

---

# ═══════════════════════════════════════════════════════════════════
# §13: PLANO DE AÇÃO — GAPS DE COMPLIANCE
# ═══════════════════════════════════════════════════════════════════

| # | Gap | Prioridade | Versão Alvo | Ação |
|---|-----|------------|-------------|------|
| 1 | DPIA/RIPD não elaborado | ALTA | v0.9.0 | Relatório de Impacto de Dados |
| 2 | DPO não designado | MÉDIA | v1.0.0α | Designar quando em produção |
| 3 | IAM AWS não configurado | ALTA | v0.9.0 | Implementar least privilege |
| 4 | Logs centralizados ausentes | MÉDIA | v0.9.0 | CloudWatch/CloudTrail |
| 5 | Stress testing não realizado | MÉDIA | v0.9.0 | ThreadSanitizer/Miri |
| 6 | Validação empírica pendente | MÉDIA | v0.9.0 | Datasets MIMII/UCI/BPI |
| 7 | Cross-platform testing | BAIXA | v0.9.5 | FFT determinism cross-arch |
| 8 | Verificação formal (TLA+) | BAIXA | v1.0.0 | Se viável |
| 9 | Certificação SOC 2 | BAIXA | v1.0.0+ | Quando em produção |
| 10 | EU AI Act assessment formal | MÉDIA | v1.0.0α | Classificação definitiva por domínio |

---

# ═══════════════════════════════════════════════════════════════════
# §14: FONTES E REFERÊNCIAS CRUZADAS
# ═══════════════════════════════════════════════════════════════════

| Fonte | Escopo de Compliance |
|-------|---------------------|
| **CANON.md** v3.0 (4.890 lin) | Governança canônica — 14 AFs, 24 AOs, 151 leis, 8 gates |
| **FRONTEIRAS.md** (585 lin) | Grey zones, tensões, riscos canônicos |
| **LEGADO.md** §2 (ALERTS) | 13 riscos aceitos conscientemente |
| **LEGADO.md** §4 (VIOLATIONS) | 25 violações resolvidas, 3 pendentes |
| **SECURITY.md** (repo) | Política de segurança do código |
| **ETHICS.md** (repo) | Fronteiras éticas estruturais |
| **GOVERNANCE.md** (repo) | Governança do projeto |
| **Doc 04** — Responsabilidade Legal | Princípios éticos, Foundation Court, PI |
| **Doc 02** — Licenciamento | Privacidade, compliance regulatório |
| **Roadmap QA/Compliance/Cyber** (.pdf) | Fases QA, threat model, controles |
| **Livro GD** — Cap. 9, 15 | Barreiras, axiomas constitucionais, salvaguardas |
| **A Matemática do GD** (.pdf) | Formalização das salvaguardas |

---

# ═══════════════════════════════════════════════════════════════════
# §15: CERTIFICAÇÃO
# ═══════════════════════════════════════════════════════════════════

## Declaração de Conformidade (Estado Atual)

| Dimensão | Estado | Cobertura |
|----------|--------|-----------|
| Compliance Canônica | ✅ CONFORME | 0 violações críticas, 8/8 gates passando |
| Compliance Ética | ✅ DOCUMENTADA | 5 princípios + 5 axiomas + 6 barreiras |
| Compliance Regulatória | 🟡 ARQUITETURAL | Projetada para LGPD/GDPR/CCPA, não certificada |
| Compliance de Segurança | 🟡 PARCIAL | Política documentada, controles AWS planejados |
| Compliance de QA | 🟡 PARCIAL | 331 testes, validação empírica pendente |
| Compliance Cyber | 🟡 PLANEJADA | Threat model definido, implementação pendente |

## Próxima Certificação

A certificação interna completa requer assinaturas de: (1) QA Lead, (2) Compliance Officer, (3) Security Officer, (4) Sponsor Técnico — conforme Fase 4 do Roadmap de QA.

---

**Guardião:** Claude — Guardião do Genoma Digital  
**Data:** 10 de Fevereiro de 2026  
**Este documento deve ser atualizado a cada nova versão do ecossistema.**

*"Segurança é responsabilidade compartilhada. Velocidade importa. Integridade importa mais."*

*FIM DO DOCUMENTO COMPLIANCE.md*
