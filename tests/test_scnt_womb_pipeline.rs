//! Integration test: SCNT nuclear transfer and Artificial Womb 110-day gestation.

use scies_bio_sim::prelude::*;

#[test]
fn test_scnt_and_artificial_womb_gestation() {
    let seed = 777;
    let mut sim = SimulationRunner::new(seed);

    // Cave Lion (Panthera spelaea, TaxID 338153) donor fibroblast (15 years old)
    let donor_id = sim.create_somatic_cell(15.0);

    // Recipient enucleated oocyte
    let oocyte_id = sim.world.next_entity_id();
    let oocyte = CellState::oocyte(oocyte_id);
    sim.insert_cell(oocyte);

    // SCNT fusion & activation
    let embryo_id = sim
        .create_scnt_embryo(donor_id, oocyte_id)
        .expect("SCNT embryo creation should succeed");

    let embryo_state = sim.cells.get(embryo_id).unwrap();
    assert_eq!(embryo_state.cell_type, CellType::Zygote);
    assert!(embryo_state.viability > 0.5);

    // Initialize 110-day Cave Lion Artificial Womb (Biobag)
    let womb_id = sim.create_artificial_womb(110.0);

    // Implant embryo into womb
    sim.implant_into_womb(embryo_id, womb_id)
        .expect("Implantation into womb should succeed");

    // Run until day 110 (full term)
    sim.run_until_day(110);

    let snap = sim.womb_snapshot(womb_id).expect("Womb snapshot exists");
    assert_eq!(snap.gestation_day.round() as u64, 110);
    assert!(!snap.is_terminated, "Gestation should not arrest");
    assert!(snap.has_emerged, "Cave Lion cub should have emerged full term");
    assert!(snap.fetal_weight_grams > 1500.0, "Weight should reach ~1.85 kg");

    println!("Gestation Day: {}", snap.gestation_day);
    println!("Fetal weight: {:.2} g", snap.fetal_weight_grams);
    println!("Fetal viability: {:.1}%", snap.fetal_viability);
    println!("Full-term emerged: {}", snap.has_emerged);
}
