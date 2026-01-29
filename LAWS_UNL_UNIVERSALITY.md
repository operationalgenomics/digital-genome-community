# LAWS — Universalidade Semântica da UNL

**Documento:** DGC-LAWS-UNL-001  
**Versão:** 1.0  
**Data:** 2025-01-27  
**Status:** CANÔNICO  
**Patch:** PATCH-CANON-2026-01-27-001  
**Base:** Canon Estratificado Operacional v2.0, AF-2

---

## Propósito

Este documento formaliza cinco leis derivadas do Axioma Fundacional AF-2 (UNL como ISA Cognitiva Universal), estabelecendo as propriedades de universalidade semântica da Universal Neutral Language.

---

## Leis Derivadas

### LEI-AF-2-10 — Abertura Representacional (Merismo-Ready)

**Classe:** Lei de Não-Impedimento Evolutivo

**Texto Normativo:**

A estrutura da UNL **não pode conter limitações arquiteturais, estruturais ou conceituais** que impeçam a representação futura de qualquer sentido, informação, padrão ou estado — independentemente de sua origem, natureza, complexidade, reino de existência ou grau de abstração.

**O que NÃO exige:**
- Representar tudo hoje
- Saber o que será representado amanhã
- Antecipar sentidos desconhecidos
- Completude imediata

**O que EXIGE:**
- Nenhum enum fechado de "tipos de sentido"
- Nenhum teto de complexidade estrutural
- Nenhuma dependência de categorias humanas
- Extensibilidade sem reestruturação

**Analogia:** GATE-QM-01 (Neutralidade de Backend) — assim como o código não impede execução em QPU futura, a UNL não impede representação de sentidos futuros.

**Rastreabilidade:** AF-2, LEI-AF-3-01, LEI-AF-3-02, GATE-QM-01

---

### LEI-AF-2-11 — Unicidade Intrínseca

**Classe:** Lei de Identidade Estrutural

**Texto Normativo:**

Cada sentido representado na UNL possui **uma e somente uma forma canônica**, determinada exclusivamente por sua estrutura interna. A identidade é **auto-definida** e **auto-consistente**.

**Formalização:**

```
∀a, b ∈ U:
  a ≡ b ⟺ C(a) = C(b)

Onde C é a função de canonização:
  - Determinística: mesma entrada → mesma saída
  - Idempotente: C(C(a)) = C(a)
  - Total: definida para todo elemento válido
```

**Consequência:** Não existem sinônimos estruturais na UNL.

**Rastreabilidade:** AF-2, LEI-AF-2-02

---

### LEI-AF-2-12 — Delegação Observacional

**Classe:** Lei de Separação de Responsabilidades

**Texto Normativo:**

A **interpretação** de fenômenos externos é responsabilidade exclusiva do **GDO** (Observador). A **representação** estruturada em UNL é responsabilidade exclusiva do **GDC** (Cérebro). O GDC **não interpreta** — apenas representa o que o GDO já interpretou.

**Fluxo:**

```
Fenômeno → [GDO: Interpreta] → UNL → [GDC: Representa] → DNA
```

**Proibições para o GDC:**
- ❌ Interpretar bytes brutos como "imagem", "som", etc.
- ❌ Aplicar heurísticas de classificação
- ❌ Inferir contexto não fornecido pelo GDO

**Rastreabilidade:** AF-2, AF-6, AF-7, LEI-AF-7-09

---

### LEI-AF-2-13 — Evolução Contributiva

**Classe:** Lei de Crescimento Temporal

**Texto Normativo:**

A UNL evolui por **acumulação contributiva**, não por substituição destrutiva. Novos sentidos são **adicionados** ao espaço representacional; sentidos existentes **nunca são removidos** por obsolescência conceitual.

**Propriedades:**
- **Monotonicidade:** |U(t+1)| ≥ |U(t)|
- **Preservação:** ∀s ∈ U(t): s ∈ U(t+1)
- **Coexistência:** Múltiplas origens podem contribuir simultaneamente

**Exceção:** Remoção permitida apenas por decisão humana explícita com justificativa canônica.

**Rastreabilidade:** AF-2, LEI-AF-2-11

---

### LEI-AF-2-14 — Projeção em Camadas

**Classe:** Lei de Preservação e Perda Controlada

**Texto Normativo:**

A tradução de DNA/UNL para formatos human-friendly ocorre **exclusivamente no ecossistema**, nunca no GDC. O GDC emite DNA estruturado completo; a perda semântica ocorre apenas na projeção final para linguagens humanas.

**Fluxo de Perda:**

```
GDO → GDC:        SEM PERDA (UNL → UNL)
GDC → Ecossistema: SEM PERDA (DNA estruturado)
Ecossistema → Usuário: COM PERDA (colapso para linguagem natural)
```

**Proibições para o GDC:**
- ❌ Projetar para português, inglês, etc.
- ❌ Simplificar para "leigos"
- ❌ Escolher formato de apresentação
- ❌ Calcular "perda aceitável"

**Estrutura de Projeção (Ecossistema):**

```
Projecao = {
    texto_final: String,
    formato: Enum,
    fidelidade: Float [0.0, 1.0],
    perdas: Vec<Descricao>,
    dna_original_hash: Hash,
    camada_responsavel: Enum  // GD-E, GD-A, GD-K, GD-F
}
```

**Rastreabilidade:** AF-3, AF-6, AF-7, LEI-AF-2-05

---

## Matriz de Conformidade

| Lei | Código v0.5.1 | Status |
|-----|---------------|--------|
| LEI-AF-2-10 | GD-QMN usa u16 extensível | ✅ Conforme |
| LEI-AF-2-11 | DNA = SHA-256 (forma canônica) | ✅ Conforme |
| LEI-AF-2-12 | GdoEmulator separa interpretação | ✅ Conforme |
| LEI-AF-2-13 | Não requer código | ✅ N/A |
| LEI-AF-2-14 | GDC emite DNA, não texto | ✅ Conforme |

---

## Roadmap de Implementação

| Item | Fase | Status |
|------|------|--------|
| Documentação das leis | v0.5.1 | ✅ COMPLETO |
| Struct Projecao | Enterprise | 📋 PENDENTE APROVAÇÃO |
| Validação de fidelidade | Enterprise | 📋 PENDENTE APROVAÇÃO |
| Auditoria de perdas | Enterprise | 📋 PENDENTE APROVAÇÃO |

---

## Testes de Conformidade

### LEI-AF-2-10 (Abertura)

```
TESTE: Nenhum enum fechado de tipos semânticos
MÉTODO: grep -r "enum.*Type" src/ | verificar extensibilidade
RESULTADO ESPERADO: Todos enums são extensíveis ou operacionais (não semânticos)
```

### LEI-AF-2-11 (Unicidade)

```
TESTE: DNA determinístico
MÉTODO: cargo test test_dna_determinism
RESULTADO ESPERADO: Mesmo input → mesmo DNA (já validado)
```

### LEI-AF-2-12 (Delegação)

```
TESTE: GDC não interpreta
MÉTODO: Verificar que CognitiveCycle não contém heurísticas de classificação
RESULTADO ESPERADO: GDC processa MotorContext fornecido pelo GDO
```

### LEI-AF-2-14 (Projeção)

```
TESTE: GDC não emite texto humano
MÉTODO: Verificar que CycleOutput contém apenas estruturas, não strings formatadas
RESULTADO ESPERADO: DNA é [u8; 32], não String
```

---

## Histórico

| Data | Evento |
|------|--------|
| 2025-01-27 | Patch proposto (PATCH-CANON-2026-01-27-001) |
| 2025-01-27 | Aprovação humana explícita |
| 2025-01-27 | Incorporação como LEI-AF-2-10 a LEI-AF-2-14 |

---

## Referências Canônicas

- **AF-2:** UNL como ISA Cognitiva Universal
- **AF-3:** Natureza Ontológica do GDC
- **AF-6:** Separação de Camadas
- **AF-7:** Externalidade
- **GATE-QM-01:** Neutralidade de Backend

---

*"A UNL não impede representar amanhã o que não conhecemos hoje."*
