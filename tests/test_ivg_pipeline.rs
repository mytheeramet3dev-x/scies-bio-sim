//! Integration test: End-to-end Hayashi IVG protocol execution and reproducibility.

use scies_bio_sim::prelude::*;

#[test]
fn test_hayashi_ivg_pipeline_end_to_end() {
    let seed = 1337;
    let mut sim = SimulationRunner::new(seed);

    // Seed 100 human somatic/iPSC cells (TaxID 9606)
    let colony_id = sim.create_colony_from_ipsc(9606, 100);
    let initial_snap = sim.snapshot(colony_id).expect("Colony exists");
    assert_eq!(initial_snap.stats.total_cells, 100);
    assert_eq!(initial_snap.stats.viable_cells, 100);

    let protocol = HayashiIvgProtocol::new();

    // Run Hayashi protocol for 42 days (1008 hours)
    sim.run_protocol(&protocol, colony_id, SimDuration::days(42))
        .expect("Protocol execution succeeded");

    let final_snap = sim.snapshot(colony_id).expect("Snapshot exists");

    // After 42 days of IVG, cells should have differentiated into Oocyte and germline types
    println!("Final stats: {:?}", final_snap.stats.type_distribution);
    println!("Mean quality score: {:.2}", final_snap.mean_quality);
    println!("Viability ratio: {:.2}%", final_snap.success_rate * 100.0);

    assert!(final_snap.stats.viable_cells > 0);
    assert!(final_snap.stats.type_distribution.contains_key("Oocyte"));
    let oocyte_count = final_snap.stats.type_distribution.get("Oocyte").copied().unwrap_or(0);
    assert!(oocyte_count > 0, "Expected at least one mature synthetic oocyte to be generated");

    // Report generation
    let report = sim.generate_report(Some(colony_id), None);
    let json = report.to_json().expect("JSON serialization should succeed");
    assert!(json.contains("Hayashi-PGCLC") || json.contains("iPSC-Colony"));
}

#[test]
fn test_ivg_strict_seed_reproducibility() {
    let seed = 99999;

    let run_once = |seed: u64| {
        let mut sim = SimulationRunner::new(seed);
        let colony_id = sim.create_colony_from_somatic(9606, 60, 28.0);
        let proto = HayashiIvgProtocol::new();
        sim.run_protocol(&proto, colony_id, SimDuration::days(36)).unwrap();
        sim.snapshot(colony_id).unwrap()
    };

    let snap_a = run_once(seed);
    let snap_b = run_once(seed);

    assert_eq!(snap_a.stats.total_cells, snap_b.stats.total_cells);
    assert_eq!(snap_a.stats.viable_cells, snap_b.stats.viable_cells);
    assert_eq!(snap_a.stats.type_distribution, snap_b.stats.type_distribution);
    assert_eq!(snap_a.stats.mean_viability, snap_b.stats.mean_viability);
    assert_eq!(snap_a.stats.mean_pluripotency, snap_b.stats.mean_pluripotency);
    assert_eq!(snap_a.stats.mean_epigenetic_age, snap_b.stats.mean_epigenetic_age);
}
