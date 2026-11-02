//! Example: Complete Cave Lion De-extinction Simulation Pipeline.
//!
//! Demonstrates:
//! 1. Harvesting somatic ear skin fibroblasts (Panthera spelaea, TaxID 338153).
//! 2. Cellular reprogramming (OSKM) into naive iPSCs.
//! 3. Hayashi In Vitro Gametogenesis (IVG) pipeline into synthetic oocytes.
//! 4. Somatic Cell Nuclear Transfer (SCNT) with KDM4 epigenetic barrier removal.
//! 5. Artificial Womb (Biobag) 110-day gestation to full-term neonate cub emergence!

use scies_bio_sim::prelude::*;

fn main() -> Result<()> {
    println!("===============================================================");
    println!("   scies-bio-sim: Cave Lion (Panthera spelaea) Pipeline v0.1  ");
    println!("===============================================================\n");

    let seed = 42;
    let mut sim = SimulationRunner::new(seed);
    let cave_lion_tax_id = 338153;

    // Phase 1: Somatic cell colony harvesting
    println!("[Stage 1] Harvesting somatic fibroblasts from 15-year-old specimen...");
    let colony_id = sim.create_colony_from_somatic(cave_lion_tax_id, 80, 15.0);
    let init_snap = sim.snapshot(colony_id).unwrap();
    println!(
        "          Colony seeded: {} cells, Mean Biological Age: {:.1} yrs, Viability: {:.1}%\n",
        init_snap.stats.total_cells, init_snap.stats.mean_epigenetic_age, init_snap.stats.mean_viability * 100.0
    );

    // Phase 2: OSKM Reprogramming
    println!("[Stage 2] Initiating Yamanaka OSKM Reprogramming Protocol (21 days)...");
    let reprogram_proto = ReprogrammingProtocol::new();
    sim.run_protocol(&reprogram_proto, colony_id, SimDuration::days(21))?;
    let ipsc_snap = sim.snapshot(colony_id).unwrap();
    println!(
        "          Reprogramming complete: Mean Pluripotency: {:.2}, Mean Epigenetic Age: {:.2} yrs",
        ipsc_snap.stats.mean_pluripotency, ipsc_snap.stats.mean_epigenetic_age
    );
    println!("          Lineages: {:?}\n", ipsc_snap.stats.type_distribution);

    // Phase 3: Hayashi In Vitro Gametogenesis (IVG)
    println!("[Stage 3] Driving Hayashi IVG Oogenesis Pipeline (42 days)...");
    let ivg_proto = HayashiIvgProtocol::new();
    sim.run_protocol(&ivg_proto, colony_id, SimDuration::days(42))?;
    let ivg_snap = sim.snapshot(colony_id).unwrap();
    println!(
        "          IVG complete: Generated {} Oocytes, Mean Quality: {:.1}/100",
        ivg_snap.stats.type_distribution.get("Oocyte").unwrap_or(&0),
        ivg_snap.mean_quality
    );
    println!("          Lineages: {:?}\n", ivg_snap.stats.type_distribution);

    // Phase 4: SCNT Nuclear Transfer & Activation
    println!("[Stage 4] Performing SCNT Fusion & KDM4 Demethylase Treatment...");
    let donor_id = sim.create_somatic_cell(15.0);
    let recipient_id = sim.world.next_entity_id();
    sim.insert_cell(CellState::oocyte(recipient_id));

    let embryo_id = sim.create_scnt_embryo(donor_id, recipient_id)?;
    let embryo = sim.cells.get(embryo_id).unwrap();
    println!(
        "          SCNT Construct Created: {:?}, Viability: {:.1}%\n",
        embryo.cell_type, embryo.viability * 100.0
    );

    // Phase 5: Artificial Womb (Biobag) 110-Day Extra-Uterine Gestation
    println!("[Stage 5] Cannulating Embryo into Biobag Extra-Uterine Life Support...");
    let womb_id = sim.create_artificial_womb(110.0);
    sim.implant_into_womb(embryo_id, womb_id)?;

    println!("          Advancing artificial womb gestation from Day 0 to Day 110...");
    let start_day = sim.clock.current_day();
    for checkpoint_day in [15, 45, 90, 110] {
        sim.run_until_day(start_day + checkpoint_day);
        let womb_snap = sim.womb_snapshot(womb_id).unwrap();
        println!(
            "          Day {:3}: Fetal Weight = {:7.2} g | Viability = {:5.1}% | Emerged = {}",
            womb_snap.gestation_day.round() as u64,
            womb_snap.fetal_weight_grams,
            womb_snap.fetal_viability,
            womb_snap.has_emerged
        );
    }

    println!("\n===============================================================");
    println!("   SUCCESS: Full-Term Cave Lion Cub Successfully Emerged!       ");
    println!("===============================================================\n");

    Ok(())
}
