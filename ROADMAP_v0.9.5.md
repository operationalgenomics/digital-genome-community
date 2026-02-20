# ROADMAP v0.9.5 - SINAPSES E NEURÔNIOS EMERGENTES

**Versão:** 0.9.5 (Emergência Cognitiva)  
**Base:** v0.9.0 + Canon v5.1  
**Estratégia:** Implementação completa em sessão única

---

## ESCOPO COMPLETO

### Módulos a Implementar

#### 1. src/synapse/ (3 arquivos)
- [x] `connection.rs` - Sinapse: conexão persistente (COMPLETO)
- [ ] `strength.rs` - Força sináptica (fortalece/enfraquece)
- [ ] `pruning.rs` - Poda sináptica

#### 2. src/neuron/ (2 arquivos)
- [ ] `cluster.rs` - Neurônio emergente
- [ ] `activation.rs` - Padrão de ativação (AF-15)

#### 3. src/coordination/ (2 arquivos)
- [ ] `multi_field.rs` - R(Σ) com N manifestações
- [ ] `multi_gdc.rs` - Orquestração N GDCs

---

## TESTES OBRIGATÓRIOS

1. [x] `synapse_forms_between_cooperating_gdcs` - connection.rs
2. [ ] `synapse_strengthens_with_repeated_use` - strength.rs
3. [ ] `synapse_weakens_without_use` - strength.rs
4. [ ] `neuron_cluster_emerges_from_activation_pattern` - cluster.rs
5. [ ] `multi_gdc_field_integrates_n_manifestations` - multi_field.rs
6. [ ] `determinism_preserved_at_scale` - multi_gdc.rs

---

## STATUS ATUAL

**Completo:**
- synapse/connection.rs (270 linhas, 1 teste)

**Pendente:**
- synapse/strength.rs (~200 linhas)
- synapse/pruning.rs (~150 linhas)
- neuron/cluster.rs (~250 linhas)
- neuron/activation.rs (~200 linhas)
- coordination/multi_field.rs (~300 linhas)
- coordination/multi_gdc.rs (~400 linhas)

**Total estimado:** ~1.770 linhas

---

## PRÓXIMA SESSÃO

Iniciar com:
1. synapse/strength.rs (fortalecimento/enfraquecimento)
2. synapse/pruning.rs (poda)
3. neuron/* completo
4. coordination/* updates

**Referência Canon:**
- AF-11: Aprendizado Cognitivo Autônomo (lines 2344-2480)
- AF-15: Ressonância Estrutural
- LEI-AF-11-01 a LEI-AF-11-04

---

**Estimativa de conclusão:** 1 sessão adicional
