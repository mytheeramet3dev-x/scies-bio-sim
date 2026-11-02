//! Integration test: Yamanaka OSKM cellular reprogramming pipeline.

use scies_bio_sim::prelude::*;

#[test]
fn test_reprogramming_pipeline_end_to_end() {
    let seed = 2026;
    let mut sim = SimulationRunner::new(seed);

    // Colony of aged somatic fibroblasts (age 50)
    let colony_id = sim.create_colony_from_somatic(9606, 50, 50.0);
    let initial_snap = sim.snapshot(colony_id).unwrap();
    assert_eq!(initial_snap.stats.mean_epigenetic_age, 50.0);
    assert_eq!(initial_snap.stats.mean_pluripotency, 0.0);

    let protocol = ReprogrammingProtocol::new();

    // Run reprogramming protocol for 21 days
    sim.run_protocol(&protocol, colony_id, SimDuration::days(21))
        .expect("Reprogramming execution succeeded");

    let final_snap = sim.snapshot(colony_id).unwrap();

    println!("Reprogrammed cell distribution: {:?}", final_snap.stats.type_distribution);
    println!("Mean pluripotency: {:.2}", final_snap.stats.mean_pluripotency);
    println!("Mean epigenetic age: {:.2}", final_snap.stats.mean_epigenetic_age);

    // After 21 days of OSKM, pluripotency should be significantly elevated and epigenetic age rejuvenated
    assert!(final_snap.stats.mean_pluripotency > 0.40);
    assert!(final_snap.stats.mean_epigenetic_age < 30.0);
    assert!(final_snap.stats.type_distribution.contains_key("NaiveiPSC") || final_snap.stats.type_distribution.contains_key("PrimedPluripotent"));
}
