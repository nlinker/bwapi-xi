use crate::prelude::TilePosition;

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitType {
    Terran_Marine = 0,
    Terran_Ghost,
    Terran_Vulture,
    Terran_Goliath,
    Terran_Goliath_Turret,
    Terran_Siege_Tank_Tank_Mode,
    Terran_Siege_Tank_Tank_Mode_Turret,
    Terran_SCV,
    Terran_Wraith,
    Terran_Science_Vessel,
    Hero_Gui_Montag,
    Terran_Dropship,
    Terran_Battlecruiser,
    Terran_Vulture_Spider_Mine,
    Terran_Nuclear_Missile,
    Terran_Civilian,
    Hero_Sarah_Kerrigan,
    Hero_Alan_Schezar,
    Hero_Alan_Schezar_Turret,
    Hero_Jim_Raynor_Vulture,
    Hero_Jim_Raynor_Marine,
    Hero_Tom_Kazansky,
    Hero_Magellan,
    Hero_Edmund_Duke_Tank_Mode,
    Hero_Edmund_Duke_Tank_Mode_Turret,
    Hero_Edmund_Duke_Siege_Mode,
    Hero_Edmund_Duke_Siege_Mode_Turret,
    Hero_Arcturus_Mengsk,
    Hero_Hyperion,
    Hero_Norad_II,
    Terran_Siege_Tank_Siege_Mode,
    Terran_Siege_Tank_Siege_Mode_Turret,
    Terran_Firebat,
    Spell_Scanner_Sweep,
    Terran_Medic,
    Zerg_Larva,
    Zerg_Egg,
    Zerg_Zergling,
    Zerg_Hydralisk,
    Zerg_Ultralisk,
    Zerg_Broodling,
    Zerg_Drone,
    Zerg_Overlord,
    Zerg_Mutalisk,
    Zerg_Guardian,
    Zerg_Queen,
    Zerg_Defiler,
    Zerg_Scourge,
    Hero_Torrasque,
    Hero_Matriarch,
    Zerg_Infested_Terran,
    Hero_Infested_Kerrigan,
    Hero_Unclean_One,
    Hero_Hunter_Killer,
    Hero_Devouring_One,
    Hero_Kukulza_Mutalisk,
    Hero_Kukulza_Guardian,
    Hero_Yggdrasill,
    Terran_Valkyrie,
    Zerg_Cocoon,
    Protoss_Corsair,
    Protoss_Dark_Templar,
    Zerg_Devourer,
    Protoss_Dark_Archon,
    Protoss_Probe,
    Protoss_Zealot,
    Protoss_Dragoon,
    Protoss_High_Templar,
    Protoss_Archon,
    Protoss_Shuttle,
    Protoss_Scout,
    Protoss_Arbiter,
    Protoss_Carrier,
    Protoss_Interceptor,
    Hero_Dark_Templar,
    Hero_Zeratul,
    Hero_Tassadar_Zeratul_Archon,
    Hero_Fenix_Zealot,
    Hero_Fenix_Dragoon,
    Hero_Tassadar,
    Hero_Mojo,
    Hero_Warbringer,
    Hero_Gantrithor,
    Protoss_Reaver,
    Protoss_Observer,
    Protoss_Scarab,
    Hero_Danimoth,
    Hero_Aldaris,
    Hero_Artanis,
    Critter_Rhynadon,
    Critter_Bengalaas,
    Special_Cargo_Ship,
    Special_Mercenary_Gunship,
    Critter_Scantid,
    Critter_Kakaru,
    Critter_Ragnasaur,
    Critter_Ursadon,
    Zerg_Lurker_Egg,
    Hero_Raszagal,
    Hero_Samir_Duran,
    Hero_Alexei_Stukov,
    Special_Map_Revealer,
    Hero_Gerard_DuGalle,
    Zerg_Lurker,
    Hero_Infested_Duran,
    Spell_Disruption_Web,
    Terran_Command_Center,
    Terran_Comsat_Station,
    Terran_Nuclear_Silo,
    Terran_Supply_Depot,
    Terran_Refinery,
    Terran_Barracks,
    Terran_Academy,
    Terran_Factory,
    Terran_Starport,
    Terran_Control_Tower,
    Terran_Science_Facility,
    Terran_Covert_Ops,
    Terran_Physics_Lab,
    Unused_Terran1,
    Terran_Machine_Shop,
    Unused_Terran2,
    Terran_Engineering_Bay,
    Terran_Armory,
    Terran_Missile_Turret,
    Terran_Bunker,
    Special_Crashed_Norad_II,
    Special_Ion_Cannon,
    Powerup_Uraj_Crystal,
    Powerup_Khalis_Crystal,
    Zerg_Infested_Command_Center,
    Zerg_Hatchery,
    Zerg_Lair,
    Zerg_Hive,
    Zerg_Nydus_Canal,
    Zerg_Hydralisk_Den,
    Zerg_Defiler_Mound,
    Zerg_Greater_Spire,
    Zerg_Queens_Nest,
    Zerg_Evolution_Chamber,
    Zerg_Ultralisk_Cavern,
    Zerg_Spire,
    Zerg_Spawning_Pool,
    Zerg_Creep_Colony,
    Zerg_Spore_Colony,
    Unused_Zerg1,
    Zerg_Sunken_Colony,
    Special_Overmind_With_Shell,
    Special_Overmind,
    Zerg_Extractor,
    Special_Mature_Chrysalis,
    Special_Cerebrate,
    Special_Cerebrate_Daggoth,
    Unused_Zerg2,
    Protoss_Nexus,
    Protoss_Robotics_Facility,
    Protoss_Pylon,
    Protoss_Assimilator,
    Unused_Protoss1,
    Protoss_Observatory,
    Protoss_Gateway,
    Unused_Protoss2,
    Protoss_Photon_Cannon,
    Protoss_Citadel_of_Adun,
    Protoss_Cybernetics_Core,
    Protoss_Templar_Archives,
    Protoss_Forge,
    Protoss_Stargate,
    Special_Stasis_Cell_Prison,
    Protoss_Fleet_Beacon,
    Protoss_Arbiter_Tribunal,
    Protoss_Robotics_Support_Bay,
    Protoss_Shield_Battery,
    Special_Khaydarin_Crystal_Form,
    Special_Protoss_Temple,
    Special_XelNaga_Temple,
    Resource_Mineral_Field,
    Resource_Mineral_Field_Type_2,
    Resource_Mineral_Field_Type_3,
    Unused_Cave,
    Unused_Cave_In,
    Unused_Cantina,
    Unused_Mining_Platform,
    Unused_Independant_Command_Center,
    Special_Independant_Starport,
    Unused_Independant_Jump_Gate,
    Unused_Ruins,
    Unused_Khaydarin_Crystal_Formation,
    Resource_Vespene_Geyser,
    Special_Warp_Gate,
    Special_Psi_Disrupter,
    Unused_Zerg_Marker,
    Unused_Terran_Marker,
    Unused_Protoss_Marker,
    Special_Zerg_Beacon,
    Special_Terran_Beacon,
    Special_Protoss_Beacon,
    Special_Zerg_Flag_Beacon,
    Special_Terran_Flag_Beacon,
    Special_Protoss_Flag_Beacon,
    Special_Power_Generator,
    Special_Overmind_Cocoon,
    Spell_Dark_Swarm,
    Special_Floor_Missile_Trap,
    Special_Floor_Hatch,
    Special_Upper_Level_Door,
    Special_Right_Upper_Level_Door,
    Special_Pit_Door,
    Special_Right_Pit_Door,
    Special_Floor_Gun_Trap,
    Special_Wall_Missile_Trap,
    Special_Wall_Flame_Trap,
    Special_Right_Wall_Missile_Trap,
    Special_Right_Wall_Flame_Trap,
    Special_Start_Location,
    Powerup_Flag,
    Powerup_Young_Chrysalis,
    Powerup_Psi_Emitter,
    Powerup_Data_Disk,
    Powerup_Khaydarin_Crystal,
    Powerup_Mineral_Cluster_Type_1,
    Powerup_Mineral_Cluster_Type_2,
    Powerup_Protoss_Gas_Orb_Type_1,
    Powerup_Protoss_Gas_Orb_Type_2,
    Powerup_Zerg_Gas_Sac_Type_1,
    Powerup_Zerg_Gas_Sac_Type_2,
    Powerup_Terran_Gas_Tank_Type_1,
    Powerup_Terran_Gas_Tank_Type_2,

    None,
    AllUnits,
    Men,
    Buildings,
    Factories,
    Unknown,
    MAX,
}

// required for ffi layer
unsafe impl cxx::ExternType for UnitType {
    type Id = cxx::type_id!("BWAPI::UnitType");
    type Kind = cxx::kind::Trivial;
}

impl UnitType {
    pub fn get_race(&self) -> bool {
        todo!()
    }
    pub fn what_builds(&self) -> bool {
        todo!()
    }
    pub fn required_units(&self) -> bool {
        todo!()
    }
    pub fn required_tech(&self) -> bool {
        todo!()
    }
    pub fn cloaking_tech(&self) -> bool {
        todo!()
    }
    pub fn abilities(&self) -> bool {
        todo!()
    }
    pub fn upgrades(&self) -> bool {
        todo!()
    }
    pub fn armor_upgrade(&self) -> bool {
        todo!()
    }
    pub fn max_hit_points(&self) -> bool {
        todo!()
    }
    pub fn max_shields(&self) -> bool {
        todo!()
    }
    pub fn max_energy(&self) -> bool {
        todo!()
    }
    pub fn armor(&self) -> bool {
        todo!()
    }
    pub fn mineral_price(&self) -> bool {
        todo!()
    }
    pub fn gas_price(&self) -> bool {
        todo!()
    }
    pub fn build_time(&self) -> bool {
        todo!()
    }
    pub fn supply_required(&self) -> bool {
        todo!()
    }
    pub fn supply_provided(&self) -> bool {
        todo!()
    }
    pub fn space_required(&self) -> bool {
        todo!()
    }
    pub fn space_provided(&self) -> bool {
        todo!()
    }
    pub fn build_score(&self) -> bool {
        todo!()
    }
    pub fn destroy_score(&self) -> bool {
        todo!()
    }
    pub fn size(&self) -> bool {
        todo!()
    }
    pub fn tile_width(&self) -> bool {
        todo!()
    }
    pub fn tile_height(&self) -> bool {
        todo!()
    }
    pub fn tile_size(&self) -> TilePosition {
        todo!()
    }
    pub fn dimension_left(&self) -> bool {
        todo!()
    }
    pub fn dimension_up(&self) -> bool {
        todo!()
    }
    pub fn dimension_right(&self) -> bool {
        todo!()
    }
    pub fn dimension_down(&self) -> bool {
        todo!()
    }
    pub fn width(&self) -> bool {
        todo!()
    }
    pub fn height(&self) -> bool {
        todo!()
    }
    pub fn seek_range(&self) -> bool {
        todo!()
    }
    pub fn sight_range(&self) -> bool {
        todo!()
    }
    pub fn ground_weapon(&self) -> bool {
        todo!()
    }
    pub fn max_ground_hits(&self) -> bool {
        todo!()
    }
    pub fn air_weapon(&self) -> bool {
        todo!()
    }
    pub fn max_air_hits(&self) -> bool {
        todo!()
    }
    pub fn top_speed(&self) -> bool {
        todo!()
    }
    pub fn acceleration(&self) -> bool {
        todo!()
    }
    pub fn halt_distance(&self) -> bool {
        todo!()
    }
    pub fn turn_radius(&self) -> bool {
        todo!()
    }
    pub fn can_produce(&self) -> bool {
        todo!()
    }
    pub fn can_attack(&self) -> bool {
        todo!()
    }
    pub fn can_move(&self) -> bool {
        todo!()
    }
    pub fn is_flyer(&self) -> bool {
        todo!()
    }
    pub fn regenerates_hp(&self) -> bool {
        todo!()
    }
    pub fn is_spellcaster(&self) -> bool {
        todo!()
    }
    pub fn has_permanent_cloak(&self) -> bool {
        todo!()
    }
    pub fn is_invincible(&self) -> bool {
        todo!()
    }
    pub fn is_organic(&self) -> bool {
        todo!()
    }
    pub fn is_mechanical(&self) -> bool {
        todo!()
    }
    pub fn is_robotic(&self) -> bool {
        todo!()
    }
    pub fn is_detector(&self) -> bool {
        todo!()
    }
    pub fn is_resource_container(&self) -> bool {
        todo!()
    }
    pub fn is_resource_depot(&self) -> bool {
        todo!()
    }
    pub fn is_refinery(&self) -> bool {
        todo!()
    }
    pub fn is_worker(&self) -> bool {
        todo!()
    }
    pub fn requires_psi(&self) -> bool {
        todo!()
    }
    pub fn requires_creep(&self) -> bool {
        todo!()
    }
    pub fn is_two_units_in_one_egg(&self) -> bool {
        todo!()
    }
    pub fn is_burrowable(&self) -> bool {
        todo!()
    }
    pub fn is_cloakable(&self) -> bool {
        todo!()
    }
    pub fn is_building(&self) -> bool {
        todo!()
    }
    pub fn is_addon(&self) -> bool {
        todo!()
    }
    pub fn is_flying_building(&self) -> bool {
        todo!()
    }
    pub fn is_neutral(&self) -> bool {
        todo!()
    }
    pub fn is_hero(&self) -> bool {
        todo!()
    }
    pub fn is_powerup(&self) -> bool {
        todo!()
    }
    pub fn is_beacon(&self) -> bool {
        todo!()
    }
    pub fn is_flag_beacon(&self) -> bool {
        todo!()
    }
    pub fn is_special_building(&self) -> bool {
        todo!()
    }
    pub fn is_spell(&self) -> bool {
        todo!()
    }
    pub fn produces_creep(&self) -> bool {
        todo!()
    }
    pub fn produces_larva(&self) -> bool {
        todo!()
    }
    pub fn is_mineral_field(&self) -> bool {
        todo!()
    }
    pub fn is_critter(&self) -> bool {
        todo!()
    }
    pub fn can_build_addon(&self) -> bool {
        todo!()
    }
    pub fn builds_what(&self) -> bool {
        todo!()
    }
    pub fn researches_what(&self) -> bool {
        todo!()
    }
    pub fn upgrades_what(&self) -> bool {
        todo!()
    }
    pub fn is_successor_of(&self) -> bool {
        todo!()
    }
}
