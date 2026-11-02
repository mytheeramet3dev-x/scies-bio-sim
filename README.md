# scies-bio-sim

> **Deterministic, Discrete Time-step Biological Process Simulator for Rust.**  
> ส่วนหนึ่งของระบบนิเวศ **scies-series** (`scies-bio-th`, `scies-bio-db`, `scies-bio-sim`)

[![Rust](https://img.shields.io/badge/rust-1.80%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## 🔬 ภาพรวม (Overview)

`scies-bio-sim` คือ Simulation Engine และ Protocol Runner สำหรับจำลองกระบวนการทางชีววิทยาระดับเซลล์และการพัฒนาของตัวอ่อนตามลำดับเวลา (Time-step Simulation) 

### ขีดความสามารถหลักใน v0.1:
1. **Deterministic by Default**: ควบคุมด้วย seed (`DeterministicRng`) ให้ผลลัพธ์การจำลองแบบ bit-for-bit identical 100% ทุกครั้งที่รัน
2. **In Vitro Gametogenesis (IVG) Pipeline**: จำลองกระบวนการ Hayashi 2016 (Somatic/Naive iPSC $\rightarrow$ EpiLC $\rightarrow$ PGCLC $\rightarrow$ Epigenetic Imprint Reset $\rightarrow$ Synthetic Oocyte)
3. **Somatic Cell Nuclear Transfer (SCNT)**: การย้ายฝากนิวเคลียส, การกระตุ้นด้วย SrCl2/Electrofusion และการฉีด KDM4 demethylase เพื่อปลดล็อก H3K9me3 reprogramming barrier
4. **Yamanaka OSKM Reprogramming**: การเหนี่ยวนำด้วย Oct4, Sox2, Klf4, c-Myc เพื่อย้อนวัย Epigenetic Age กลับสู่ Naive Ground State
5. **Artificial Womb (Biobag Extra-Uterine Life Support)**: จำลองการตั้งครรภ์ภายนอกมดลูกตาม gestational timeline (เช่น 110 วันสำหรับ Cave Lion หรือ 280 วันสำหรับมนุษย์) พร้อมมวลน้ำหนักทารกและการตรวจประเมิน viability

---

## 🏗️ โครงสร้างสถาปัตยกรรม (Module Architecture)

```
scies_bio_sim::
├── core
│   ├── clock          — SimulationClock, TimeUnit (Hour, Day, Cycle), SimDuration
│   ├── world          — World, EntityId, EntityStore (Generic component store)
│   ├── event          — Event, EventPayload, EventQueue (Event bus บันทึกประวัติ)
│   └── rng            — DeterministicRng (Seedable SplitMix64 PRNG)
├── state
│   ├── cell           — CellState, CellType, Viability, Pluripotency, Epigenetic Age
│   ├── colony         — Colony, PopulationStats (การคำนวณสถิติประชากรเซลล์)
│   ├── epigenetic     — EpigeneticState (Demethylation, H3K9me3 barrier, Horvath clock)
│   └── metrics        — QualityScore, StageProgress, WombState
├── protocol
│   ├── trait_def      — Protocol trait, Step, TimeCondition, Action
│   ├── ivg            — HayashiIvgProtocol (2016 murine/mammalian)
│   ├── scnt           — ScntProtocol (Nuclear transfer + KDM4 + Cleavage)
│   ├── reprogramming  — ReprogrammingProtocol (OSKM 21-day timeline)
│   └── womb           — ArtificialWombProtocol (Biobag Extra-uterine gestation)
├── engine
│   ├── runner         — SimulationRunner (Main orchestrator)
│   ├── scheduler      — StepScheduler (ประเมิน trigger ตามเวลา)
│   └── reporter       — ColonySnapshot, WombSnapshot, SimulationReport (JSON export)
└── prelude            — รวม re-exports ที่ใช้บ่อยทั้งหมด
```

---

## 🚀 ตัวอย่างการใช้งาน (Quick Start)

### 1. การรัน Hayashi IVG Protocol

```rust
use scies_bio_sim::prelude::*;

fn main() -> Result<()> {
    // 1. กำหนด Seed เพื่อผลลัพธ์ที่ทำซ้ำได้แน่นอน (Deterministic)
    let mut sim = SimulationRunner::new(42);

    // 2. สร้างกลุ่มเซลล์เริ่มต้น (100 somatic fibroblasts, TaxID 9606 = Homo sapiens)
    let colony = sim.create_colony_from_somatic(9606, 100, 30.0);

    // 3. รัน Hayashi IVG Protocol เป็นเวลา 42 วัน
    let protocol = HayashiIvgProtocol::new();
    sim.run_protocol(&protocol, colony, SimDuration::days(42))?;

    // 4. สรุปผลการทดลอง
    let snapshot = sim.snapshot(colony).unwrap();
    println!("Total viable cells: {}", snapshot.stats.viable_cells);
    println!("Oocytes produced: {:?}", snapshot.stats.type_distribution.get("Oocyte"));
    println!("Mean Quality Score: {:.1}/100", snapshot.mean_quality);

    Ok(())
}
```

### 2. การจำลอง Artificial Womb (Biobag 110 วัน สำหรับ Cave Lion)

```rust
use scies_bio_sim::prelude::*;

fn main() -> Result<()> {
    let mut sim = SimulationRunner::new(777);

    // สร้างเซลล์ต้นทางและไข่รับฝาก -> ทำ SCNT
    let donor_id = sim.create_somatic_cell(15.0);
    let oocyte_id = sim.world.next_entity_id();
    sim.insert_cell(CellState::oocyte(oocyte_id));

    let embryo_id = sim.create_scnt_embryo(donor_id, oocyte_id)?;

    // ติดตั้งเครื่อง Artificial Womb สำหรับ 110 วัน
    let womb_id = sim.create_artificial_womb(110.0);
    sim.implant_into_womb(embryo_id, womb_id)?;

    // รันจำลองจนถึงวันกำหนดคลอด Day 110
    sim.run_until_day(110);

    let snap = sim.womb_snapshot(womb_id).unwrap();
    println!("Gestation Day: {}", snap.gestation_day);
    println!("Fetal weight: {:.1} g", snap.fetal_weight_grams);
    println!("Full term emerged: {}", snap.has_emerged);

    Ok(())
}
```

---

## 🧪 การรันเทสต์และตัวอย่าง (Testing & Examples)

```bash
# รัน Unit Tests และ Integration Tests ทั้งหมด
cargo test

# รันตัวอย่าง Full De-extinction Pipeline ของ Cave Lion
cargo run --example cave_lion_pipeline
```

---

## 🔗 ความสัมพันธ์กับระบบนิเวศ `scies-series`

| Crate | บทบาทหน้าที่ |
|---|---|
| **`scies-bio-th`** | Data structures และสูตรการคำนวณทางชีววิทยาพื้นฐาน (Dna, Rna, Crispr, Ivg primitives) |
| **`scies-bio-db`** | ฐานข้อมูลจีโนม, ลำดับเบส, ยีน, และอนุกรมวิธาน (Taxonomy) |
| **`scies-bio-sim`** | Time-step Engine, Protocol Scheduler, และ Deterministic Population Simulation |

---

## 📄 License

MIT License. Developed as part of the `scies-series` project.
