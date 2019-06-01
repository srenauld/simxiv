extern crate simxiv_spelldata;
extern crate simxiv_prelude;

use std::path::PathBuf;
use simxiv_spelldata::{load_actions, RawAction, Range, CostType, KnownCost};

#[test]
fn it_works() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/action.csv");
    let data = load_actions(path).unwrap();
    let bootshine = data.get(&53).unwrap();
    assert_eq!(bootshine, &RawAction {
        id: 53,
        name: "Bootshine".to_string(),
        cost: 50,
        cost_type: CostType::Known(KnownCost::Tact),
        range: Range::Melee,
        can_target_self: false,
        can_target_friendly: false,
        can_target_hostile: true,
        cast: 0,
        recast: 25,
    });

    let veraero = data.get(&7507).unwrap();
    assert_eq!(veraero, &RawAction {
        id: 7507,
        name: "Veraero".to_string(),
        cost: 4,
        cost_type: CostType::Known(KnownCost::Mana),
        range: Range::Ranged(25),
        can_target_self: false,
        can_target_friendly: false,
        can_target_hostile: true,
        cast: 50,
        recast: 25,
    });
}
