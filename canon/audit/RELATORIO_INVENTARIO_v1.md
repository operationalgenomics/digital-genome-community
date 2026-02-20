# RELATÓRIO — Inventário Canônico GDC
## Entregável 1 do Contrato Canon-Writer-v1.0
**Data:** 2026-02-10  
**Status:** AGUARDANDO REVISÃO HUMANA

---

## 1. NÚMEROS GERAIS

| Métrica | Valor |
|---------|-------|
| IDs canônicos únicos | **237** |
| Axiomas Fundacionais (AF-) | 13 |
| Axiomas Operacionais (AO-) | 25 |
| Leis (LEI-) | 154 |
| Deliberações (DLB-) | 25 |
| Grey Zones (GZ-) | 20 |
| Gates (GATE-) | 0 (registrados em doc sem ID formal) |

## 2. COBERTURA CÓDIGO ↔ CANON

| Tipo | Com código | Total | Cobertura |
|------|-----------|-------|-----------|
| AF- | 3 (AF-10, AF-11, AF-12) | 13 | 23% |
| AO- | 3 (AO-18, AO-RESSONANTE, AO-SHIBBOLETH) | 25 | 12% |
| LEI- | 17 | 154 | 11% |
| DLB- | 9 | 25 | 36% |
| GZ- | 1 (GZ-D13) | 20 | 5% |
| **TOTAL** | **33** | **237** | **14%** |

**Nota:** Cobertura baixa em código NÃO é necessariamente um problema. Muitos axiomas e leis são puramente normativos e governam decisões de design, não implementação direta. Os que TÊM código são os que governam o pipeline cognitivo (AF-10/11/12), identidade (AO-SHIBBOLETH/RESSONANTE), coordenação (LEI-COORD), e GD-QMN (DLB-019 a DLB-024).

## 3. FRAGMENTAÇÃO DETECTADA

### Fontes Primárias (CONFLITO POTENCIAL)
O Canon normativo está disperso em **pelo menos 8 fontes sobrepostas**:

1. **AXIOMAS_LEIS_GDC.md** (3.958 linhas) — Canon v2.0 "completo", mas NÃO integra:
   - AF-13 e leis do PATCH_CANON_2026-01-28-001.md
   - AO-SHIBBOLETH, AO-RESSONANTE, AO-TOPOLOGIA do /canon/
   - LEI-ZERO-01, LEI-SYNC-01, LEI-QMN-*, LEI-ISA-01 do BACKLOG

2. **BACKLOG_DELIBERACAO.md** (~1.000 linhas) — Contém DLBs 001-025 com:
   - Candidatos a axiomas NUNCA formalmente promovidos
   - Leis operacionais (LEI-ZERO-01 etc.) aprovadas mas fora do Canon principal
   - Estruturas GD-QMN definidas aqui, não no Canon

3. **/canon/ (11 arquivos)** — Documentos canônicos dedicados que:
   - Definem AO-RESSONANTE, DLB-013, DLB-014, LEI-COORD-01/02
   - Têm seu próprio INDEX.md v0.8.5-sanitized
   - NÃO estão refletidos no AXIOMAS_LEIS_GDC.md principal

4. **PATCH_CANON_2026-01-28-001.md** — AF-13 + 5 LEIs UNL + 5 GATE → NÃO integrado

5. **GATES_QUANTUM_READY.md** — 5 Gates → NÃO integrado ao Canon principal

6. **LAWS_UNL_UNIVERSALITY.md** — 5 LEIs → Sobrepõem com PATCH

### Fragmentação em Código
172 entradas canônicas dispersas em 63 arquivos .rs. Os arquivos com maior densidade canônica:
- `src/lib.rs` — 58 refs (índice + testes canônicos)
- `src/memory/mci.rs` — 39 refs
- `src/unl/gd_qmn/core.rs` — 37 refs
- `src/coordination/gdc.rs` — 32 refs
- `src/cognitive/cycle.rs` — 32 refs

## 4. CONFLITOS IDENTIFICADOS

### 4.1 Candidatos Não Promovidos (DECISÃO REQUERIDA)
O BACKLOG_DELIBERACAO.md lista candidatos a axioma que foram deliberados e aprovados mas **nunca formalmente incorporados ao AXIOMAS_LEIS_GDC.md**:
- AF-UNL-01 a AF-UNL-04 (UNL como estado, trans-reino, interna, infinita)
- AF-DNA-01 (DNA gerativo)
- AO-RESS-01 a AO-RESS-03 (isomorfismo, estados, emissão)

**Questão para o Humano:** Estes candidatos devem ser promovidos a axiomas formais no CANON.md consolidado, ou permanecem como deliberações?

### 4.2 Sobreposição PATCH × LAWS_UNL
O PATCH_CANON_2026-01-28-001.md e LAWS_UNL_UNIVERSALITY.md definem as mesmas 5 leis (LEI-AF-2-10 a LEI-AF-2-14). São consistentes entre si, mas existem em dois lugares.

### 4.3 Grey Zones
- **3 ABERTAS:** GZ-TOPO-01, GZ-TOPO-02, GZ-TOPO-03 (topológicas)
- **4 FECHADAS mas não arquivadas:** GZ-D05-FECHADA, GZ-D08-FECHADA, GZ-D09-FECHADA, GZ-D13-FECHADA
- **TECH-COORD-03:** Direção técnica pendente da especificação GD-QMN

## 5. PRÓXIMOS PASSOS (AGUARDANDO APROVAÇÃO)

Para prosseguir com os Entregáveis 2-7, preciso de decisões sobre:

1. **Promoção de candidatos:** Os AF-UNL-*, AF-DNA-01, AO-RESS-* devem ser promovidos?
2. **Numeração:** Os AO-SHIBBOLETH, AO-RESSONANTE, AO-TOPOLOGIA recebem números (AO-19, AO-20, AO-21)?
3. **BACKLOG:** O documento todo vai para ROADMAP.md, ou apenas os candidatos não promovidos vão para LAB.md?
4. **Gates:** Os 5 gates do GATES_QUANTUM_READY.md recebem IDs formais (GATE-1 a GATE-5)?
5. **Prioridade:** Qual entregável deve ser produzido a seguir?

---
*Documento gerado como parte do contrato Canon-Writer-v1.0. Nenhuma decisão normativa foi tomada.*
