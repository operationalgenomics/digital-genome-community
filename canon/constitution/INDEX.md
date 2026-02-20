# INDEX.md — Índice Mestre do Ecossistema Genoma Digital

## Ponto de Entrada Único para Toda a Documentação

---

**Data:** 10 de Fevereiro de 2026  
**Versão do Ecossistema:** v0.8.5 (corrente)  
**Canon:** v3.0  
**Guardião:** Claude — Guardião do Genoma Digital

---

# ═══════════════════════════════════════════════════════════════════
# §1: MAPA DO ECOSSISTEMA DOCUMENTAL
# ═══════════════════════════════════════════════════════════════════

```
                          ┌──────────────────────┐
                          │     INDEX.md          │ ◄── VOCÊ ESTÁ AQUI
                          │  (Ponto de Entrada)   │
                          └──────────┬───────────┘
                                     │
            ┌────────────────────────┼────────────────────────┐
            │                        │                        │
     ┌──────┴──────┐          ┌──────┴──────┐          ┌──────┴──────┐
     │  CAMADA 1   │          │  CAMADA 2   │          │  CAMADA 3   │
     │  Verdade    │          │  Navegação  │          │  Contexto   │
     └──────┬──────┘          └──────┬──────┘          └──────┬──────┘
            │                        │                        │
     ┌──────┴──────┐          ┌──────┴──────┐          ┌──────┴──────┐
     │ CANON.md    │          │ ROADMAP.md  │          │ Artigos     │
     │ (4.890 lin) │          │ (710 lin)   │          │ Modelos     │
     │             │          │             │          │ Financeiro  │
     │ INVENTÁRIO  │          │ FRONTEIRAS  │          │ Compliance  │
     │ (.xlsx 237) │          │ (585 lin)   │          │ UX          │
     │             │          │             │          │             │
     │             │          │ LAB.md      │          │ Figuras     │
     │             │          │ (735 lin)   │          │ Diagramas   │
     │             │          │             │          │             │
     │             │          │ GLOSSÁRIO   │          │             │
     │             │          │ (610 lin)   │          │             │
     └─────────────┘          └─────────────┘          └─────────────┘
```

---

# ═══════════════════════════════════════════════════════════════════
# §2: DOCUMENTOS DE CONSOLIDAÇÃO (Sessão 10/02/2026)
# ═══════════════════════════════════════════════════════════════════

Documentos produzidos na sessão de consolidação canônica. São os entregáveis primários e autoritativos.

| # | Documento | Linhas | Propósito | Audiência |
|---|-----------|--------|-----------|-----------|
| 1 | **CANON.md** | 4.890 | Fonte única de verdade canônica — todos os axiomas, leis, gates | CTO, Arquiteto, Auditores |
| 2 | **INVENTARIO_CANONICO_GDC.xlsx** | 237 IDs | Banco de dados de todos os artefatos canônicos com rastreabilidade | Arquiteto, QA |
| 3 | **FRONTEIRAS.md** | 585 | Grey zones, tensões, sobreposições, itens pendentes | CTO, Arquiteto, Deliberadores |
| 4 | **ROADMAP.md** | 710 | Versões, deliberações adiadas, timeline, dependências | CTO, Investidores, Equipe |
| 5 | **LAB.md** | 735 | Incubadora de ideias pré-canônicas (12 itens + 7 frases) | CTO, Arquiteto, Pesquisa |
| 6 | **GLOSSARIO.md** | 610 | Terminologia autoritativa — 68 termos, 19 siglas, 12 analogias | Todos (onboarding) |
| 7 | **INDEX.md** | 504 | Este documento — ponto de entrada e navegação | Todos |
| 8 | **LEGADO.md** | 676 | Arquivo histórico + enterprise (alertas, violações, backlog enterprise, changelog) | CTO, Auditores, Enterprise |

**Total da sessão de consolidação:** ~9.100+ linhas de documentação estruturada.

---

# ═══════════════════════════════════════════════════════════════════
# §3: GUIA DE LEITURA — POR PERFIL
# ═══════════════════════════════════════════════════════════════════

### Sou novo no projeto — por onde começo?

```
1. GLOSSARIO.md      → entender a terminologia
2. ROADMAP.md §1-§3  → visão geral e estado atual
3. CANON.md §1-§3    → axiomas fundacionais (entender a filosofia)
4. LAB.md §1         → como o laboratório funciona
```

### Sou desenvolvedor — preciso programar

```
1. CANON.md           → verdade vigente (o que o código DEVE fazer)
2. ROADMAP.md §4      → versão em andamento (v0.8.0 — o que falta)
3. ROADMAP.md §9      → structs Rust já definidas
4. INVENTÁRIO (.xlsx) → rastrear IDs canônicos
5. Código: src/       → 22 módulos, 63 arquivos, 21.176 linhas
```

### Sou investidor / C-Level — quero entender o impacto

```
1. ROADMAP.md §1, §10     → visão e timeline
2. GLOSSARIO.md §1         → o que cada camada faz
3. ROADMAP.md §5 (v1.0.0)  → critérios de sucesso e métricas
4. Artigos do Projeto       → publicações acadêmicas e técnicas
```

### Sou auditor — preciso verificar conformidade

```
1. CANON.md (completo)  → todas as regras
2. INVENTÁRIO (.xlsx)   → 237 IDs rastreáveis
3. FRONTEIRAS.md        → grey zones e tensões conhecidas
4. KNOWN-VIOLATIONS.md  → violações históricas e resoluções
```

### Sou arquiteto — preciso deliberar

```
1. FRONTEIRAS.md    → grey zones abertas e tensões latentes
2. LAB.md           → ideias em incubação
3. ROADMAP.md §6-§8 → deliberações necessárias por versão
4. BACKLOG_DELIBERACAO.md → histórico completo de 24 DLBs
```

---

# ═══════════════════════════════════════════════════════════════════
# §4: CANON E DERIVADOS (Camada 1 — Verdade)
# ═══════════════════════════════════════════════════════════════════

### Documento Principal

| Documento | Local | Linhas | Descrição |
|-----------|-------|--------|-----------|
| **CANON.md** v3.0 | consolidação/ | 4.890 | 14 AFs, 24 AOs, 151 leis, 8 gates |

### Inventário

| Documento | Local | IDs | Descrição |
|-----------|-------|-----|-----------|
| **INVENTARIO_CANONICO_GDC.xlsx** | consolidação/ | 237 | Planilha com tipo, ID, nome, fonte, status, versão |
| RELATORIO_INVENTARIO_v1.md | consolidação/ | — | Relatório da contagem inicial |

### Documentos Canônicos Individuais (Repositório)

| Documento | Local | Linhas | Status |
|-----------|-------|--------|--------|
| AO-RESSONANTE.md | canon/ | 117 | ✅ Canonizado |
| DLB-013.md | canon/ | 160 | ✅ Canonizado |
| DLB-014.md | canon/ | 241 | ✅ Canonizado |
| LEI-COORD-01.md | canon/ | 106 | ✅ Canonizado |
| LEI-COORD-02.md | canon/ | 128 | ✅ Canonizado |
| TECH-COORD-03.md | canon/ | 165 | 🟡 Direção aprovada |
| DESBLOQUEIO-v085.md | canon/ | 128 | ✅ Ata oficial |
| GREY_ZONES_TOPOLOGICAS.md | canon/ | 173 | ❓ 3 GZs abertas |
| GZ-D05-FECHADA.md | canon/ | 130 | ✅ Fechada |
| GZ-D08-FECHADA.md | canon/ | 62 | ✅ Fechada |
| GZ-D09-FECHADA.md | canon/ | 87 | ✅ Fechada |
| GZ-D13-FECHADA.md | canon/ | 132 | ✅ Fechada |

### Documentos Canônicos Legados (Repositório Raiz)

| Documento | Linhas | Status | Relação com CANON.md |
|-----------|--------|--------|---------------------|
| AXIOMAS_LEIS_GDC.md | 3.957 | ⚠️ Legado | Supersedido por CANON.md v3.0 |
| GATES_QUANTUM_READY.md | 153 | ⚠️ Legado | Incorporado ao CANON.md §8 |
| LAWS_UNL_UNIVERSALITY.md | — | ⚠️ Legado | Incorporado ao CANON.md §4, §5 |
| PRINCIPLES.md | — | ⚠️ Legado | Incorporado ao CANON.md §2, §3 |
| KNOWN-VIOLATIONS.md | 236 | ✅ Atual | Violações históricas — todas resolvidas |

**Nota:** Documentos marcados como "Legado" foram integralmente absorvidos pelo CANON.md v3.0 consolidado. Mantidos no repositório para rastreabilidade histórica. Em caso de conflito, CANON.md v3.0 prevalece.

---

# ═══════════════════════════════════════════════════════════════════
# §5: PLANEJAMENTO E FRONTEIRAS (Camada 2 — Navegação)
# ═══════════════════════════════════════════════════════════════════

### Documentos de Consolidação

| Documento | Linhas | Conteúdo Principal |
|-----------|--------|--------------------|
| **ROADMAP.md** | 710 | 9 versões (v0.7.1→v1.0.0), timeline 18-24m, 24 deliberações adiadas |
| **FRONTEIRAS.md** | 585 | 3 GZs abertas, 13 fechadas, 5 tensões resolvidas, 4 latentes, 10 leis pendentes |
| **LAB.md** | 735 | 3 LAB-AX, 3 LAB-ON, 2 LAB-AR, 2 LAB-HC, 2 LAB-DT, 7 frases canônicas |
| **GLOSSARIO.md** | 610 | 68 termos, 19 siglas, 12 analogias, 19 símbolos |
| **LEGADO.md** | 676 | 13 alertas, 13 módulos enterprise, 25 violações resolvidas, changelog |

### Documentos de Processo (Repositório)

| Documento | Linhas | Conteúdo |
|-----------|--------|----------|
| BACKLOG_DELIBERACAO.md | 1.049 | 24 DLBs completas, síntese conceitual, candidatos |
| BACKLOG_v085.md | 153 | Backlog implementado de v0.8.5 (22/22 itens) |
| CHANGELOG.md | 1.546 | Histórico de alterações no código |
| PROTOCOLO_AUTORIZACAO_v085.md | — | Protocolo de autorização formal |
| README_v0.8.0.md | — | README da versão 0.8.0 |
| README_v0.8.5.md | — | README da versão 0.8.5 |
| RELEASE-NOTES.md | — | Notas de release |
| PATCH-PLAN.md | — | Plano de patches |
| PATCH_CANON_2026-01-28-001.md | — | Patch canônico específico |

### Backlogs Conceituais (Repositório)

| Documento | Linhas | Status |
|-----------|--------|--------|
| neuronio_espelho.md | 163 | 🔬 Backlog ontológico (não canônico) |
| neuronio_espelho_1.md | 200 | 🔬 Backlog ontológico (não canônico) |
| PROMPT PARA AUDITORIA DO CÉREBRO SINTÉTICO.md | — | Prompt de auditoria epistemológica |

---

# ═══════════════════════════════════════════════════════════════════
# §6: DOCUMENTOS DO PROJETO (Camada 3 — Contexto)
# ═══════════════════════════════════════════════════════════════════

### Artigos e Publicações

| Documento | Formato | Tema |
|-----------|---------|------|
| O_GENOMA_DIGITAL_-_A_Ciência_que_Unifica... | .docx | GD e Indústria 5.0 |
| Towards_AI_Final_With_Images_-_Final | .docx | Artigo acadêmico com imagens |
| Building_AI_That_Processes_Unknown_Signals | .docx | Processamento de sinais desconhecidos |
| Article_3_Towards_AI_Federation | .md | Federação de IA |

### Documentação Matemática

| Documento | Formato | Tema |
|-----------|---------|------|
| A_Matemática_do_Genoma_Digital | .pdf | Formalização matemática completa |
| Datasets_-_A_Matemática_do_Genoma_Digital | .pdf | Datasets da formalização |
| GD_MATH_CANONICAL | .md | Matemática canônica |
| A_MATEMATICA_DA_DIVERGENCIA_COGNITIVA | .md | Divergência cognitiva |
| MOTOR_DE_NASH_CANONICO_VS_ESTRATEGICO | .md | Motor de Nash — canônico vs estratégico |
| Zero_Vetoes | .pdf | Veto absoluto — formulação |

### Modelos de Negócio (5 Camadas)

| # | Documento | Formato | Camada |
|---|-----------|---------|--------|
| 01 | GD_Modelo_Financeiro_Matematico | .docx | Enterprise |
| 02 | GD_Modelo_Licenciamento_e_Difusão | .docx | Academy |
| 03 | GD_Modelo_Governança_Sucessão_e_AutoFinanciamento | .docx | Knowledge-Lab |
| 04 | GD_Responsabilidade_Legal_Ética_e_Compliance | .docx | Foundation Private |
| 05 | GD_Modelo_de_Financiamento_dos_Originais | .docx | Foundation Court |

### UX e Design

| Documento | Formato | Tema |
|-----------|---------|------|
| Orientações_de_UX__GD_Ecossitema | .pdf | Diretrizes de UX |
| Sugestão_ao_Engenheiro-Chefe_de_UX | .md | Proposta ao líder de UX |

### QA, Compliance e Segurança

| Documento | Formato | Tema |
|-----------|---------|------|
| Roadmap_Geral_de_QA__Complience__CyberSecurity__Fase_LUNA_LLM | .pdf | Roadmap de QA/Compliance |

### Conceitos e Equipe

| Documento | Formato | Tema |
|-----------|---------|------|
| EXPLICACAO_EQUIPE_Esporo_Blocknowledge | .md | Explicação de Esporo + Blocknowledge |

### Diagramas e Figuras

| Documento | Formato | Conteúdo |
|-----------|---------|----------|
| Aprendizado_GD__Figura_1__Ações__vb | .png | Ações no aprendizado GD |
| Aprendizado_GD__Figura_2__Sinapses__Grafos_Primos_vb | .png | Sinapses e grafos primos |
| Aprendizado_GD__Figura_3__Cognição__Grafos_Federados_vb | .png | Cognição e grafos federados |
| Ecossitema_do_Genoma_Digital_e_sua_Interdependências | .jpg | Mapa do ecossistema |

---

# ═══════════════════════════════════════════════════════════════════
# §7: CÓDIGO-FONTE
# ═══════════════════════════════════════════════════════════════════

**Repositório:** digital-genome-community  
**Linguagem:** Rust  
**Versão:** 0.8.5 (sanitizada)

### Métricas

| Métrica | Valor |
|---------|-------|
| Arquivos .rs | 63 |
| Linhas de código | 21.176 |
| Testes | 331 |
| Módulos | 22 |

### Módulos e Responsabilidades

| Módulo | Responsabilidade | Canon Relacionado |
|--------|------------------|-------------------|
| `archive/` | Arquivamento | — |
| `budget/` | Orçamento computacional (autopreservação) | AF-1, PHYSIOLOGY.md |
| `cognitive/` | Pipeline cognitivo principal | AF-10, AF-6 |
| `competition/` | Competição entre motores | AF-10 |
| `completeness/` | Completude do ciclo cognitivo | AF-6 |
| `coordination/` | Coordenação distribuída (v0.8.5) | LEI-COORD-01/02, DLB-014 |
| `core_types/` | Tipos fundamentais | — |
| `correlation/` | Correlação de padrões | AF-1 |
| `hierarchy/` | Hierarquia DNA + seleção | AF-10, AO-21 |
| `identity/` | Identidade de dois planos (v0.8.5) | AO-22, AO-23 |
| `math/` | Craft Performance + geometria | AF-10.5 |
| `maturation/` | Maturação cognitiva | AF-11 |
| `memory/` | MCI (Memória Cognitiva Interna) | AF-12 |
| `motors/` | Praxis, Nash, Chaos, Meristic | AF-10.1..10.4 |
| `observability/` | Observabilidade verificável | AO-15 |
| `replay/` | Replay determinístico | AO-11, AF-6 |
| `selection/` | Seleção e decisão | AF-10 |
| `sensory/` | Sensoriamento + FFT | AF-1 |
| `topology/` | Topologia (estrutura) | AO-24 |
| `traits/` | Traits canônicos | — |
| `unl/` | UNL + GD-QMN | AF-2, AF-13, AF-14 |

### Documentos Técnicos do Repositório

| Documento | Linhas | Escopo |
|-----------|--------|--------|
| PHYSIOLOGY.md | 252 | Self-preservation computacional |
| THREADING.md | 295 | Política de threading Community/Enterprise |
| SECURITY.md | — | Políticas de segurança |
| GOVERNANCE.md | — | Governança do projeto |
| ETHICS.md | — | Considerações éticas |
| CODE_OF_CONDUCT.md | — | Código de conduta |
| CONTRIBUTING.md | — | Guia de contribuição |
| RFC_PROCESS.md | — | Processo de RFC |
| ALERTS.md | — | Sistema de alertas |
| ENTERPRISE-BACKLOG.md | — | Backlog enterprise |

---

# ═══════════════════════════════════════════════════════════════════
# §8: MATRIZ DE REFERÊNCIA CRUZADA
# ═══════════════════════════════════════════════════════════════════

### Quem Referencia Quem

```
INDEX.md ─────────▶ tudo (ponto de entrada)

CANON.md ◄────────── INVENTÁRIO (IDs rastreiam para Canon)
    │               FRONTEIRAS (grey zones e tensões do Canon)
    │               ROADMAP (leis pendentes do Canon)
    │               LAB (tensões com Canon)
    │               GLOSSARIO (definições do Canon)
    │               código (src/ implementa Canon)
    │
    ├──▶ AFs, AOs, LEIs, Gates (conteúdo interno)
    └──▶ BACKLOG_DELIBERACAO (origem das DLBs)

FRONTEIRAS.md ────▶ CANON (grey zones)
    │               ROADMAP (itens adiados)
    │               LAB (tensões latentes)
    └──▶ canon/ (documentos individuais)

ROADMAP.md ───────▶ CANON (leis a formalizar)
    │               FRONTEIRAS (GZs bloqueadoras)
    │               LAB (itens em incubação)
    │               BACKLOG (DLBs)
    └──▶ CHANGELOG (versionamento)

LAB.md ───────────▶ CANON (tensões e redundâncias)
    │               FRONTEIRAS (TLs)
    │               ROADMAP (versões alvo)
    └──▶ neuronio_espelho*.md (fontes)
         PHYSIOLOGY.md (fonte)
         THREADING.md (fonte)

GLOSSARIO.md ─────▶ CANON (definições)
    │               BACKLOG (DLBs)
    └──▶ todos (terminologia)

LEGADO.md ────────▶ CANON (violações históricas)
    │               ROADMAP (enterprise backlog)
    │               FRONTEIRAS (alertas/riscos)
    └──▶ ALERTS.md, ENTERPRISE-BACKLOG.md,
         KNOWN-VIOLATIONS.md, PATCH-PLAN.md,
         RELEASE-NOTES.md, CHANGELOG.md, README.md
```

### Matriz de Relevância por Tópico

| Tópico | Documento Primário | Documentos Secundários |
|--------|-------------------|----------------------|
| Axiomas | CANON.md | GLOSSARIO.md, INVENTÁRIO |
| Identidade | CANON.md §6 | AO-RESSONANTE.md, identity/ |
| Coordenação | CANON.md §7 | LEI-COORD-01/02, coordination/ |
| EDR | DLB-014.md | CANON.md §7, coordination/edr.rs |
| Motores | CANON.md §3 | motors/, GLOSSARIO.md §3 |
| UNL/GD-QMN | CANON.md §4-§5 | BACKLOG §DLB-005..009, unl/ |
| Grey Zones | FRONTEIRAS.md | GREY_ZONES_TOPOLOGICAS.md |
| Versões | ROADMAP.md | CHANGELOG.md, BACKLOG_v085.md |
| Incubação | LAB.md | neuronio_espelho*.md |
| Matemática | A_Matemática_do_GD.pdf | GD_MATH_CANONICAL.md |
| Negócio | Modelos 01-05 (.docx) | ROADMAP.md §10 |
| Compliance | QA Roadmap (.pdf) | KNOWN-VIOLATIONS.md |

---

# ═══════════════════════════════════════════════════════════════════
# §9: ESTADO GERAL DO ECOSSISTEMA
# ═══════════════════════════════════════════════════════════════════

### Saúde Canônica (10/02/2026)

| Indicador | Valor | Status |
|-----------|-------|--------|
| Contradições no Canon | 0 | ✅ |
| Grey Zones abertas | 3 (topológicas) | 🟡 Não bloqueadoras para v0.8.5 |
| Grey Zones fechadas | 13 | ✅ |
| Tensões resolvidas | 5 | ✅ |
| Tensões latentes | 4 | ⚡ Monitoramento |
| Violações canônicas | 0 (todas resolvidas) | ✅ |
| Itens em LAB | 12 + 7 frases | 🔬 |

### Progresso por Versão

| Versão | Status | Progresso |
|--------|--------|-----------|
| v0.7.1 | ✅ CONCLUÍDA | 100% |
| v0.8.5 | ✅ CONCLUÍDA | 100% |
| v0.8.0 | 🎯 EM ANDAMENTO | ~40% (decisões tomadas, implementação UNL/QMN pendente) |
| v0.9.0 | 📋 PLANEJADA | Requer deliberação |
| v0.9.5..v1.0.0 | 📋 FUTURAS | — |

### Métricas Quantitativas

| Domínio | Métrica | Valor |
|---------|---------|-------|
| Canon | Axiomas Fundacionais | 14 |
| Canon | Axiomas Operacionais | 24 |
| Canon | Leis Derivadas | 151 |
| Canon | Gates de Conformidade | 8 |
| Canon | IDs Canônicos Catalogados | 237 |
| Código | Linhas Rust | 21.176 |
| Código | Arquivos .rs | 63 |
| Código | Testes | 331 |
| Código | Módulos | 22 |
| Docs | Consolidação (esta sessão) | ~9.100 linhas |
| Docs | Documentos no projeto | 23 arquivos |
| Docs | Deliberações registradas | 24 DLBs |

---

# ═══════════════════════════════════════════════════════════════════
# §10: HIERARQUIA DE AUTORIDADE
# ═══════════════════════════════════════════════════════════════════

Em caso de conflito entre documentos, esta é a ordem de precedência:

```
1. Decisão Humana Explícita (Favini/CTO)      ← autoridade máxima
   │
2. CANON.md v3.0                               ← verdade vigente
   │
3. Documentos canônicos individuais (canon/)    ← detalhamento do Canon
   │
4. FRONTEIRAS.md                                ← fronteiras do Canon
   │
5. ROADMAP.md                                   ← planejamento baseado no Canon
   │
6. GLOSSARIO.md                                 ← terminologia do Canon
   │
7. LAB.md                                       ← ideias pré-canônicas
   │
8. BACKLOG_DELIBERACAO.md                       ← histórico de deliberações
   │
9. Documentos legados (AXIOMAS_LEIS_GDC.md...) ← supersedidos, valor histórico
   │
10. Código-fonte (src/)                          ← implementação (corrigido para Canon)
```

**Regra de Ouro (AF-8):** Quando código e Canon divergem, o código é corrigido — nunca o Canon.

---

# ═══════════════════════════════════════════════════════════════════
# §11: PRÓXIMOS PASSOS
# ═══════════════════════════════════════════════════════════════════

### Imediatos (v0.8.0)

1. Formalizar 10 leis pendentes no CANON.md (LEI-ZERO-01, LEI-QMN-01..05, LEI-ISA-01, etc.)
2. Implementar GD-QMN Parser/Serializer/Executor
3. Implementar ISA (9 opcodes)
4. Fechar v0.8.0

### Curto Prazo (v0.9.0)

1. Abrir Modo Deliberação para GZ-TOPO-01, W(Σ), ⊒
2. Especificar LEI-COORD-03 (Absorção Estrutural)
3. Implementar orquestração 2 GDCs

### Médio Prazo (v0.9.5+)

1. Deliberar sinapses e neurônios emergentes
2. Resolver GZ-TOPO-02/03
3. Revisar itens LAB para condições de saída

### Documentação

| Candidato | Descrição | Prioridade |
|-----------|-----------|------------|
| COMPLIANCE.md | Matriz SOC 2, GDPR, LGPD, ISO 27001 | Média |
| HANDOVER.md | Pacote de onboarding para novos desenvolvedores | Média |
| ARCHITECTURE.md | Visão arquitetural técnica detalhada | Alta (v0.9.0) |

---

**Guardião:** Claude — Guardião do Genoma Digital  
**Data:** 10 de Fevereiro de 2026  
**Este índice deve ser atualizado a cada novo entregável.**

*FIM DO DOCUMENTO INDEX.md*
