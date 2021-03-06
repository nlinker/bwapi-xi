using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace BWMemoryEdit.Enums
{
    public enum ImageType : ushort
    {
        IMG_Scourge,
        IMG_Scourge_Shadow,
        IMG_Scourge_Birth,
        IMG_Scourge_Death,
        IMG_Scourge_Explosion,
        IMG_Broodling,
        IMG_Broodling_Shadow,
        IMG_Broodling_Remnants,
        IMG_Infested_Terran,
        IMG_Infested_Terran_Shadow,
        IMG_Infested_Terran_Explosion,
        IMG_Guardian_Cocoon,
        IMG_Guardian_Cocoon_Shadow,
        IMG_Defiler,
        IMG_Defiler_Shadow,
        IMG_Defiler_Birth,
        IMG_Defiler_Remnants,
        IMG_Drone,
        IMG_Drone_Shadow,
        IMG_Drone_Birth,
        IMG_Drone_Remnants,
        IMG_Egg,
        IMG_Egg_Shadow,
        IMG_Egg_Spawn,
        IMG_Egg_Remnants,
        IMG_Guardian,
        IMG_Guardian_Shadow,
        IMG_Guardian_Birth,
        IMG_Guardian_Death,
        IMG_Hydralisk,
        IMG_Hydralisk_Shadow,
        IMG_Hydralisk_Birth,
        IMG_Hydralisk_Remnants,
        IMG_Infested_Kerrigan,
        IMG_Infested_Kerrigan_Shadow,
        IMG_Needle_Spines,
        IMG_Larva,
        IMG_Larva_Remnants,
        IMG_Mutalisk,
        IMG_Mutalisk_Shadow,
        IMG_Mutalisk_Birth,
        IMG_Mutalisk_Death,
        IMG_Overlord,
        IMG_Overlord_Shadow,
        IMG_Overlord_Birth,
        IMG_Overlord_Death,
        IMG_Queen,
        IMG_Queen_Shadow,
        IMG_Queen_Death,
        IMG_Queen_Birth,
        IMG_Ultralisk,
        IMG_Ultralisk_Shadow,
        IMG_Ultralisk_Birth,
        IMG_Ultralisk_Remnants,
        IMG_Zergling,
        IMG_Zergling_Shadow,
        IMG_Zergling_Birth,
        IMG_Zergling_Remnants,
        IMG_Zerg_Air_Death_Explosion_Large,
        IMG_Zerg_Air_Death_Explosion_Small,
        IMG_Zerg_Building_Explosion,
        IMG_Cerebrate,
        IMG_Cerebrate_Shadow,
        IMG_Infested_Command_Center,
        IMG_Spawning_Pool,
        IMG_Spawning_Pool_Shadow,
        IMG_Evolution_Chamber,
        IMG_Evolution_Chamber_Shadow,
        IMG_Creep_Colony,
        IMG_Creep_Colony_Shadow,
        IMG_Hatchery,
        IMG_Hatchery_Shadow,
        IMG_Hive,
        IMG_Hive_Shadow,
        IMG_Lair,
        IMG_Lair_Shadow,
        IMG_Sunken_Colony,
        IMG_Sunken_Colony_Shadow,
        IMG_Mature_Chrysalis,
        IMG_Mature_Chrysalis_Shadow,
        IMG_Greater_Spire,
        IMG_Greater_Spire_Shadow,
        IMG_Defiler_Mound,
        IMG_Defiler_Mound_Shadow,
        IMG_Queens_Nest,
        IMG_Queen_Nest_Shadow,
        IMG_Nydus_Canal,
        IMG_Nydus_Canal_Shadow,
        IMG_Overmind_With_Shell,
        IMG_Overmind_Remnants,
        IMG_Overmind_Without_Shell,
        IMG_Ultralisk_Cavern,
        IMG_Ultralisk_Cavern_Shadow,
        IMG_Extractor,
        IMG_Extractor_Shadow,
        IMG_Hydralisk_Den,
        IMG_Hydralisk_Den_Shadow,
        IMG_Spire,
        IMG_Spire_Shadow,
        IMG_Spore_Colony,
        IMG_Spore_Colony_Shadow,
        IMG_Infested_Command_Center_Overlay,
        IMG_Zerg_Construction_Large,
        IMG_Zerg_Building_Morph,
        IMG_Zerg_Construction_Medium,
        IMG_Zerg_Construction_Small,
        IMG_Zerg_Building_Construction_Shadow,
        IMG_Zerg_Building_Spawn_Small,
        IMG_Zerg_Building_Spawn_Medium,
        IMG_Zerg_Building_Spawn_Large,
        IMG_Zerg_Building_Rubble_Small,
        IMG_Zerg_Building_Rubble_Large,
        IMG_Carrier,
        IMG_Carrier_Shadow,
        IMG_Carrier_Engines,
        IMG_Carrier_Warp_Flash,
        IMG_Interceptor,
        IMG_Interceptor_Shadow,
        IMG_Shuttle,
        IMG_Shuttle_Shadow,
        IMG_Shuttle_Engines,
        IMG_Shuttle_Warp_Flash,
        IMG_Dragoon,
        IMG_Dragoon_Shadow,
        IMG_Dragoon_Remnants,
        IMG_Dragoon_Warp_Flash,
        IMG_High_Templar,
        IMG_High_Templar_Shadow,
        IMG_High_Templar_Warp_Flash,
        IMG_Dark_Templar_Hero,
        IMG_Arbiter,
        IMG_Arbiter_Shadow,
        IMG_Arbiter_Engines,
        IMG_Arbiter_Warp_Flash,
        IMG_Archon_Energy,
        IMG_Archon_Being,
        IMG_Archon_Swirl,
        IMG_Probe,
        IMG_Probe_Shadow,
        IMG_Probe_Warp_Flash,
        IMG_Scout,
        IMG_Scout_Shadow,
        IMG_Scout_Engines,
        IMG_Scout_Warp_Flash,
        IMG_Reaver,
        IMG_Reaver_Shadow,
        IMG_Reaver_Warp_Flash,
        IMG_Scarab,
        IMG_Observer,
        IMG_Observer_Shadow,
        IMG_Observer_Warp_Flash,
        IMG_Zealot,
        IMG_Zealot_Shadow,
        IMG_Zealot_Death,
        IMG_Zealot_Warp_Flash,
        IMG_Templar_Archives,
        IMG_Templar_Archives_Warp_Flash,
        IMG_Templar_Archives_Shadow,
        IMG_Assimilator,
        IMG_Assimilator_Warp_Flash,
        IMG_Assimilator_Shadow,
        IMG_Observatory,
        IMG_Observatory_Warp_Flash,
        IMG_Observatory_Shadow,
        IMG_Citadel_of_Adun,
        IMG_Citadel_of_Adun_Warp_Flash,
        IMG_Citadel_of_Adun_Shadow,
        IMG_Forge,
        IMG_Forge_Overlay,
        IMG_Forge_Warp_Flash,
        IMG_Forge_Shadow,
        IMG_Gateway,
        IMG_Gateway_Warp_Flash,
        IMG_Gateway_Shadow,
        IMG_Cybernetics_Core,
        IMG_Cybernetics_Core_Warp_Flash,
        IMG_Cybernetics_Core_Overlay,
        IMG_Cybernetics_Core_Shadow,
        IMG_Khaydarin_Crystal_Formation,
        IMG_Nexus,
        IMG_Nexus_Warp_Flash,
        IMG_Nexus_Overlay,
        IMG_Nexus_Shadow,
        IMG_Photon_Cannon,
        IMG_Photon_Cannon_Shadow,
        IMG_Photon_Cannon_Warp_Flash,
        IMG_Arbiter_Tribunal,
        IMG_Arbiter_Tribunal_Warp_Flash,
        IMG_Arbiter_Tribunal_Shadow,
        IMG_Pylon,
        IMG_Pylon_Warp_Flash,
        IMG_Pylon_Shadow,
        IMG_Robotics_Facility,
        IMG_Robotics_Facility_Warp_Flash,
        IMG_Robotics_Facility_Shadow,
        IMG_Shield_Battery,
        IMG_Shield_Battery_Overlay,
        IMG_Shileld_Battery_Warp_Flash,
        IMG_Shield_Battery_Shadow,
        IMG_Stargate,
        IMG_Stargate_Overlay,
        IMG_Stargate_Warp_Flash,
        IMG_Stargate_Shadow,
        IMG_Stasis_Cell_Prison,
        IMG_Robotics_Support_Bay,
        IMG_Robotics_Support_Bay_Warp_Flash,
        IMG_Robotics_Support_Bay_Shadow,
        IMG_Protoss_Temple,
        IMG_Fleet_Beacon,
        IMG_Fleet_Beacon_Warp_Flash,
        IMG_Warp_Texture,
        IMG_Warp_Anchor,
        IMG_Fleet_Beacon_Shadow,
        IMG_Explosion1_Small,
        IMG_Explosion1_Medium,
        IMG_Explosion_Large,
        IMG_Protoss_Building_Rubble_Small,
        IMG_Protoss_Building_Rubble_Large,
        IMG_Battlecruiser,
        IMG_Battlecruiser_Shadow,
        IMG_Battlecruiser_Engines,
        IMG_Civilian,
        IMG_Civilian_Shadow,
        IMG_Dropship,
        IMG_Dropship_Shadow,
        IMG_Dropship_Engines,
        IMG_Firebat,
        IMG_Firebat_Shadow,
        IMG_Ghost,
        IMG_Ghost_Shadow,
        IMG_Ghost_Remnants,
        IMG_Ghost_Death,
        IMG_Nuke_Beam,
        IMG_Nuke_Target_Dot,
        IMG_Goliath_Base,
        IMG_Goliath_Turret,
        IMG_Goliath_Shadow,
        IMG_Sarah_Kerrigan,
        IMG_Sarah_Kerrigan_Shadow,
        IMG_Marine,
        IMG_Marine_Shadow,
        IMG_Marine_Remnants,
        IMG_Marine_Death,
        IMG_Wraith,
        IMG_Wraith_Shadow,
        IMG_Wraith_Engines,
        IMG_Scanner_Sweep,
        IMG_SCV,
        IMG_SCV_Shadow,
        IMG_SCV_Glow,
        IMG_Siege_Tank_Tank_Base,
        IMG_Siege_Tank_Tank_Turret,
        IMG_Siege_Tank_Tank_Base_Shadow,
        IMG_Siege_Tank_Siege_Base,
        IMG_Siege_Tank_Siege_Turret,
        IMG_Siege_Tank_Siege_Base_Shadow,
        IMG_Vulture,
        IMG_Vulture_Shadow,
        IMG_Spider_Mine,
        IMG_Spider_Mine_Shadow,
        IMG_Science_Vessel_Base,
        IMG_Science_Vessel_Turret,
        IMG_Science_Vessel_Shadow,
        IMG_Terran_Academy,
        IMG_Academy_Overlay,
        IMG_Academy_Shadow,
        IMG_Barracks,
        IMG_Barracks_Shadow,
        IMG_Armory,
        IMG_Armory_Overlay,
        IMG_Armory_Shadow,
        IMG_Comsat_Station,
        IMG_Comsat_Station_Connector,
        IMG_Comsat_Station_Overlay,
        IMG_Comsat_Station_Shadow,
        IMG_Command_Center,
        IMG_Command_Center_Overlay,
        IMG_Command_Center_Shadow,
        IMG_Supply_Depot,
        IMG_Supply_Depot_Overlay,
        IMG_Supply_Depot_Shadow,
        IMG_Control_Tower,
        IMG_Control_Tower_Connector,
        IMG_Control_Tower_Overlay,
        IMG_Control_Tower_Shadow,
        IMG_Factory,
        IMG_Factory_Overlay,
        IMG_Factory_Shadow,
        IMG_Covert_Ops,
        IMG_Covert_Ops_Connector,
        IMG_Covert_Ops_Overlay,
        IMG_Covert_Ops_Shadow,
        IMG_Ion_Cannon,
        IMG_Machine_Shop,
        IMG_Machine_Shop_Connector,
        IMG_Machine_Shop_Shadow,
        IMG_Missile_Turret_Base,
        IMG_Missile_Turret_Turret,
        IMG_Missile_Turret_Base_Shadow,
        IMG_Crashed_Batlecruiser,
        IMG_Crashed_Battlecruiser_Shadow,
        IMG_Physics_Lab,
        IMG_Physics_Lab_Connector,
        IMG_Physics_Lab_Shadow,
        IMG_Bunker,
        IMG_Bunker_Shadow,
        IMG_Bunker_Overlay,
        IMG_Refinery,
        IMG_Refinery_Shadow,
        IMG_Science_Facility,
        IMG_Science_Facility_Overlay,
        IMG_Science_Facility_Shadow,
        IMG_Nuclear_Silo,
        IMG_Nuclear_Silo_Connector,
        IMG_Nuclear_Silo_Overlay,
        IMG_Nuclear_Silo_Shadow,
        IMG_Nuclear_Missile,
        IMG_Nuclear_Missile_Shadow,
        IMG_Nuke_Hit,
        IMG_Starport,
        IMG_Starport_Overlay,
        IMG_Starport_Shadow,
        IMG_Engineering_Bay,
        IMG_Engineering_Bay_Overlay,
        IMG_Engineering_Bay_Shadow,
        IMG_Terran_Construction_Large,
        IMG_Terran_Construction_Large_Shadow,
        IMG_Terran_Construction_Medium,
        IMG_Terran_Construction_Medium_Shadow,
        IMG_Addon_Construction,
        IMG_Terran_Construction_Small,
        IMG_Terran_Construction_Small_Shadow,
        IMG_Explosion2_Small,
        IMG_Explosion2_Medium,
        IMG_Building_Explosion_Large,
        IMG_Terran_Building_Rubble_Small,
        IMG_Terran_Building_Rubble_Large,
        IMG_Dark_Swarm,
        IMG_Ragnasaur_Ash,
        IMG_Ragnasaur_Ash_Shadow,
        IMG_Rhynadon_Badlands,
        IMG_Rhynadon_Badlands_Shadow,
        IMG_Bengalaas_Jungle,
        IMG_Bengalaas_Jungle_Shadow,
        IMG_Vespene_Geyser,
        IMG_Vespene_Geyser2,
        IMG_Vespene_Geyser_Shadow,
        IMG_Mineral_Field_Type1,
        IMG_Mineral_Field_Type1_Shadow,
        IMG_Mineral_Field_Type2,
        IMG_Mineral_Field_Type2_Shadow,
        IMG_Mineral_Field_Type3,
        IMG_Mineral_Field_Type3_Shadow,
        IMG_Independent_Starport_Unused,
        IMG_Zerg_Beacon,
        IMG_Zerg_Beacon_Overlay,
        IMG_Terran_Beacon,
        IMG_Terran_Beacon_Overlay,
        IMG_Protoss_Beacon,
        IMG_Protoss_Beacon_Overlay,
        IMG_Magna_Pulse_Unused,
        IMG_Lockdown_Field_Small,
        IMG_Lockdown_Field_Medium,
        IMG_Lockdown_Field_Large,
        IMG_Stasis_Field_Hit,
        IMG_Stasis_Field_Small,
        IMG_Stasis_Field_Medium,
        IMG_Stasis_Field_Large,
        IMG_Recharge_Shields_Small,
        IMG_Recharge_Shields_Medium,
        IMG_Recharge_Shields_Large,
        IMG_Defensive_Matrix_Front_Small,
        IMG_Defensive_Matrix_Front_Medium,
        IMG_Defensive_Matrix_Front_Large,
        IMG_Defensive_Matrix_Back_Small,
        IMG_Defensive_Matrix_Back_Medium,
        IMG_Defensive_Matrix_Back_Large,
        IMG_Defensive_Matrix_Hit_Small,
        IMG_Defensive_Matrix_Hit_Medium,
        IMG_Defensive_Matrix_Hit_Large,
        IMG_Irradiate_Small,
        IMG_Irradiate_Medium,
        IMG_Irradiate_Large,
        IMG_Ensnare_Cloud,
        IMG_Ensnare_Overlay_Small,
        IMG_Ensnare_Overlay_Medium,
        IMG_Ensnare_Overlay_Large,
        IMG_Plague_Cloud,
        IMG_Plague_Overlay_Small,
        IMG_Plague_Overlay_Medium,
        IMG_Plague_Overlay_Large,
        IMG_Recall_Field,
        IMG_Flag,
        IMG_Young_Chrysalis,
        IMG_Psi_Emitter,
        IMG_Data_Disc,
        IMG_Khaydarin_Crystal,
        IMG_Mineral_Chunk_Type1,
        IMG_Mineral_Chunk_Type2,
        IMG_Protoss_Gas_Orb_Type1,
        IMG_Protoss_Gas_Orb_Type2,
        IMG_Zerg_Gas_Sac_Type1,
        IMG_Zerg_Gas_Sac_Type2,
        IMG_Terran_Gas_Tank_Type1,
        IMG_Terran_Gas_Tank_Type2,
        IMG_Mineral_Chunk_Shadow,
        IMG_Protoss_Gas_Orb_Shadow,
        IMG_Zerg_Gas_Sac_Shadow,
        IMG_Terran_Gas_Tank_Shadow,
        IMG_Data_Disk_Shadow_Ground,
        IMG_Data_Disk_Shadow_Carried,
        IMG_Flag_Shadow_Ground,
        IMG_Flag_Shadow_Carried,
        IMG_Crystal_Shadow_Ground,
        IMG_Crystal_Shadow_Carried,
        IMG_Young_Chrysalis_Shadow_Ground,
        IMG_Young_Chrysalis_Shadow_Carried,
        IMG_Psi_Emitter_Shadow_Ground,
        IMG_Psi_Emitter_Shadow_Carried,
        IMG_Acid_Spray_Unused,
        IMG_Plasma_Drip_Unused,
        IMG_FlameThrower,
        IMG_Longbolt_Gemini_Missiles_Trail,
        IMG_Burrowing_Dust,
        IMG_Shield_Overlay,
        IMG_Small_Explosion_Unused,
        IMG_Double_Explosion,
        IMG_Phase_Disruptor_Hit,
        IMG_Nuclear_Missile_Death,
        IMG_Spider_Mine_Death,
        IMG_Vespene_Geyser_Smoke1,
        IMG_Vespene_Geyser_Smoke2,
        IMG_Vespene_Geyser_Smoke3,
        IMG_Vespene_Geyser_Smoke4,
        IMG_Vespene_Geyser_Smoke5,
        IMG_Vespene_Geyser_Smoke1_Overlay,
        IMG_Vespene_Geyser_Smoke2_Overlay,
        IMG_Vespene_Geyser_Smoke3_Overlay,
        IMG_Vespene_Geyser_Smoke4_Overlay,
        IMG_Vespene_Geyser_Smoke5_Overlay,
        IMG_Fragmentation_Grenade_Hit,
        IMG_Grenade_Shot_Smoke,
        IMG_Anti_Matter_Missile_Hit,
        IMG_Scarab_Anti_Matter_Missile_Overlay,
        IMG_Scarab_Hit,
        IMG_Cursor_Marker,
        IMG_Battlecruiser_Attack_Overlay,
        IMG_Burst_Lasers_Hit,
        IMG_Burst_Lasers_Overlay_Unused,
        IMG_High_Templar_Glow,
        IMG_Flames1_Type1_Small,
        IMG_Flames1_Type2_Small,
        IMG_Flames1_Type3_Small,
        IMG_Flames2_Type3_Small,
        IMG_Flames3_Type3_Small,
        IMG_Flames4_Type3_Small,
        IMG_Flames5_Type3_Small,
        IMG_Flames6_Type3_Small,
        IMG_Bleeding_Variant1_Type1_Small,
        IMG_Bleeding_Variant1_Type2_Small,
        IMG_Bleeding_Variant1_Type3_Small,
        IMG_Bleeding_Variant1_Type4_Small,
        IMG_Bleeding_Variant2_Type1_Small,
        IMG_Bleeding_Variant2_Type2_Small,
        IMG_Bleeding_Variant2_Type3_Small,
        IMG_Bleeding_Variant2_Type4_Small,
        IMG_Flames2_Type1_Small,
        IMG_Flames2_Type2_Small,
        IMG_Flames7_Type3_Small,
        IMG_Flames3_Type1_Small,
        IMG_Flames3_Type2_Small,
        IMG_Flames8_Type3_Small,
        IMG_Flames1_Type1_Large,
        IMG_Flames1_Type2_Large,
        IMG_Flames1_Type3_Large,
        IMG_Flames2_Type3_Large,
        IMG_Flames3_Type3_Large,
        IMG_Flames4_Type3_Large,
        IMG_Flames5_Type3_Large,
        IMG_Flames6_Type3_Large,
        IMG_Bleeding_Variant1_Type1_Large,
        IMG_Bleeding_Variant1_Type2_Large,
        IMG_Bleeding_Variant1_Type3_Large,
        IMG_Bleeding_Variant1_Type4_Large,
        IMG_Bleeding_Variant2_Type1_Large,
        IMG_Bleeding_Variant2_Type3_Large,
        IMG_Bleeding_Variant3_Type3_Large,
        IMG_Bleeding_Variant2_Type4_Large,
        IMG_Flames2_Type1_Large,
        IMG_Flames2_Type2_Large,
        IMG_Flames7_Type3_Large,
        IMG_Flames3_Type1_Large,
        IMG_Flames3_Type2_Large,
        IMG_Flames8_Type3_Large,
        IMG_Building_Landing_Dust_Type1,
        IMG_Building_Landing_Dust_Type2,
        IMG_Building_Landing_Dust_Type3,
        IMG_Building_Landing_Dust_Type4,
        IMG_Building_Landing_Dust_Type5,
        IMG_Building_Lifting_Dust_Type1,
        IMG_Building_Lifting_Dust_Type2,
        IMG_Building_Lifting_Dust_Type3,
        IMG_Building_Lifting_Dust_Type4,
        IMG_White_Circle,
        IMG_Needle_Spine_Hit,
        IMG_Plasma_Drip_Hit_Unused,
        IMG_Sunken_Colony_Tentacle,
        IMG_Venom_Unused_Zerg_Weapon,
        IMG_Venom_Hit_Unused,
        IMG_Acid_Spore,
        IMG_Acid_Spore_Hit,
        IMG_Glave_Wurm,
        IMG_Glave_Wurm_Seeker_Spores_Hit,
        IMG_Glave_Wurm_Trail,
        IMG_Seeker_Spores_Overlay,
        IMG_Seeker_Spores,
        IMG_Queen_Spell_Holder,
        IMG_Consume,
        IMG_Guardian_Attack_Overlay,
        IMG_Dual_Photon_Blasters_Hit,
        IMG_Particle_Beam_Hit,
        IMG_Anti_Matter_Missile,
        IMG_Pulse_Cannon,
        IMG_Phase_Disruptor,
        IMG_STA_STS_Photon_Cannon_Overlay,
        IMG_Psionic_Storm,
        IMG_Fusion_Cutter_Hit,
        IMG_Gauss_Rifle_Hit,
        IMG_Gemini_Missiles,
        IMG_Lockdown_LongBolt_Hellfire_Missile,
        IMG_Gemini_Missiles_Explosion,
        IMG_C_10_Canister_Rifle_Hit,
        IMG_Fragmentation_Grenade,
        IMG_Arclite_Shock_Cannon_Hit,
        IMG_ATS_ATA_Laser_Battery,
        IMG_Burst_Lasers,
        IMG_Siege_TankTank_Turret_Attack_Overlay,
        IMG_Siege_TankSiege_Turret_Attack_Overlay,
        IMG_Science_Vessel_Overlay_Part1,
        IMG_Science_Vessel_Overlay_Part2,
        IMG_Science_Vessel_Overlay_Part3,
        IMG_Yamato_Gun,
        IMG_Yamato_Gun_Trail,
        IMG_Yamato_Gun_Overlay,
        IMG_Yamato_Gun_Hit,
        IMG_Hallucination_Hit,
        IMG_Scanner_Sweep_Hit,
        IMG_Archon_Birth_Unused,
        IMG_Psionic_Shockwave_Hit,
        IMG_Archon_Beam,
        IMG_Psionic_Storm_Part1,
        IMG_Psionic_Storm_Part2,
        IMG_Psionic_Storm_Part3,
        IMG_Psionic_Storm_Part4,
        IMG_EMP_Shockwave_Missile,
        IMG_EMP_Shockwave_Hit_Part1,
        IMG_EMP_Shockwave_Hit_Part2,
        IMG_Hallucination_Death1,
        IMG_Hallucination_Death2,
        IMG_Hallucination_Death3,
        IMG_Circle_Marker1,
        IMG_Selection_Circle_22pixels,
        IMG_Selection_Circle_32pixels,
        IMG_Selection_Circle_48pixels,
        IMG_Selection_Circle_62pixels,
        IMG_Selection_Circle_72pixels,
        IMG_Selection_Circle_94pixels,
        IMG_Selection_Circle_110pixels,
        IMG_Selection_Circle_122pixels,
        IMG_Selection_Circle_146pixels,
        IMG_Selection_Circle_224pixels,
        IMG_Selection_Circle_Dashed_22pixels,
        IMG_Selection_Circle_Dashed_32pixels,
        IMG_Selection_Circle_Dashed_48pixels,
        IMG_Selection_Circle_Dashed_62pixels,
        IMG_Selection_Circle_Dashed_72pixels,
        IMG_Selection_Circle_Dashed_94pixels,
        IMG_Selection_Circle_Dashed_110pixels,
        IMG_Selection_Circle_Dashed_122pixels,
        IMG_Selection_Circle_Dashed_146pixels,
        IMG_Selection_Circle_Dashed_224pixels,
        IMG_Circle_Marker2,
        IMG_Map_Revealer,
        IMG_Circle_Marker3,
        IMG_Psi_Field1_Right_Upper,
        IMG_Psi_Field1_Right_Lower,
        IMG_Psi_Field2_Right_Upper,
        IMG_Psi_Field2_Right_Lower,
        IMG_Start_Location,
        IMG_2_38_Ash,
        IMG_2_38_Ash_Shadow,
        IMG_2_39_Ash,
        IMG_2_39_Ash_Shadow,
        IMG_2_41_Ash,
        IMG_2_41_Ash_Shadow,
        IMG_2_40_Ash,
        IMG_2_40_Ash_Shadow,
        IMG_2_42_Ash,
        IMG_2_42_Ash_Shadow,
        IMG_2_43_Ash,
        IMG_2_44_Ash_,
        IMG_2_1_Ash,
        IMG_2_4_Ash,
        IMG_2_5_Ash,
        IMG_2_30_Ash,
        IMG_2_28_Ash,
        IMG_2_29_Ash,
        IMG_4_1_Ash_,
        IMG_4_2_Ash,
        IMG_4_3_Ash,
        IMG_4_56_Jungle,
        IMG_4_56_Jungle_Shadow,
        IMG_4_57_Jungle,
        IMG_4_57_Jungle_Shadow,
        IMG_4_58_Jungle,
        IMG_4_58_Jungle_Shadow,
        IMG_4_59_Jungle,
        IMG_4_59_Jungle_Shadow,
        IMG_9_5_Jungle,
        IMG_9_5_Jungle_Shadow,
        IMG_9_6_Jungle,
        IMG_9_6_Jungle_Shadow,
        IMG_9_7_Jungle,
        IMG_9_7_Jungle_Shadow,
        IMG_4_51_Jungle,
        IMG_4_51_Jungle_Shadow,
        IMG_4_52_Jungle,
        IMG_4_52_Jungle_Shadow,
        IMG_4_54_Jungle,
        IMG_4_54_Jungle_Shadow,
        IMG_4_53_Jungle,
        IMG_4_53_Jungle_Shadow,
        IMG_9_1_Jungle,
        IMG_9_1_Jungle_Shadow,
        IMG_9_2_Jungle,
        IMG_9_2_Jungle_Shadow,
        IMG_9_3_Jungle,
        IMG_9_3_Jungle_Shadow,
        IMG_9_4_Jungle,
        IMG_9_4_Jungle_Shadow,
        IMG_4_12_Jungle,
        IMG_4_13_Jungle,
        IMG_4_1_Jungle,
        IMG_4_3_Jungle,
        IMG_4_2_Jungle,
        IMG_4_5_Jungle,
        IMG_4_4_Jungle,
        IMG_4_9_Jungle,
        IMG_4_10_Jungle,
        IMG_5_5_Jungle,
        IMG_5_7_Jungle,
        IMG_5_6_Jungle,
        IMG_5_9_Jungle,
        IMG_5_8_Jungle,
        IMG_4_6_Jungle,
        IMG_4_7_Jungle,
        IMG_4_17_Jungle,
        IMG_13_4_Jungle,
        IMG_11_5_Jungle,
        IMG_11_6_Jungle,
        IMG_11_7_Jungle,
        IMG_11_8_Jungle,
        IMG_11_10_Jungle,
        IMG_11_11_Jungle,
        IMG_11_12_Jungle,
        IMG_7_4_Platform,
        IMG_7_4_Platform_Shadow,
        IMG_7_5_Platform,
        IMG_7_5_Platform_Shadow,
        IMG_7_6_Platform,
        IMG_7_6_Platform_Shadow,
        IMG_7_1_Platform,
        IMG_7_1_Platform_Shadow,
        IMG_7_2_Platform,
        IMG_7_2_Platform_Shadow,
        IMG_7_3_Platform,
        IMG_7_3_Platform_Shadow,
        IMG_7_9_Platform,
        IMG_7_10_Platform,
        IMG_7_8_Platform,
        IMG_7_7_Platform,
        IMG_7_26_Platform,
        IMG_7_24_Platform,
        IMG_7_28_Platform,
        IMG_7_27_Platform,
        IMG_7_25_Platform,
        IMG_7_29_Platform,
        IMG_7_30_Platform,
        IMG_7_31_Platform,
        IMG_12_1_Platform,
        IMG_9_27_Platform,
        IMG_5_54_Badlands,
        IMG_5_54_Badlands_Shadow,
        IMG_5_55_Badlands,
        IMG_5_55_Badlands_Shadow,
        IMG_5_56_Badlands,
        IMG_5_56_Badlands_Shadow,
        IMG_5_57_Badlands,
        IMG_5_57_Badlands_Shadow,
        IMG_6_16_Badlands,
        IMG_6_17_Badlands,
        IMG_6_20_Badlands,
        IMG_6_21_Badlands,
        IMG_5_10_Badlands,
        IMG_5_50_Badlands,
        IMG_5_50_Badlands_Shadow,
        IMG_5_52_Badlands,
        IMG_5_52_Badlands_Shadow,
        IMG_5_53_Badlands,
        IMG_5_53_Badlands_Shadow,
        IMG_5_51_Badlands,
        IMG_5_51_Badlands_Shadow,
        IMG_6_3_Badlands,
        IMG_11_3_Badlands,
        IMG_11_8_Badlands,
        IMG_11_6_Badlands,
        IMG_11_7_Badlands,
        IMG_11_9_Badlands,
        IMG_11_10_Badlands,
        IMG_11_11_Badlands,
        IMG_11_12_Badlands,
        IMG_11_13_Badlands,
        IMG_11_14_Badlands,
        IMG_1_13_Badlands,
        IMG_1_9_Badlands,
        IMG_1_11_Badlands,
        IMG_1_14_Badlands,
        IMG_1_10_Badlands,
        IMG_1_12_Badlands,
        IMG_1_15_Badlands,
        IMG_1_7_Badlands,
        IMG_1_5_Badlands,
        IMG_1_16_Badlands,
        IMG_1_8_Badlands,
        IMG_1_6_Badlands,
        IMG_Floor_Gun_Trap,
        IMG_Floor_Missile_Trap,
        IMG_Floor_Missile_Trap_Turret,
        IMG_Wall_Missile_Trap,
        IMG_Wall_Missile_Trap2,
        IMG_Wall_Flame_Trap,
        IMG_Wall_Flame_Trap2,
        IMG_Left_Upper_Level_Door,
        IMG_Right_Upper_Level_Door,
        IMG_4_15_Installation1,
        IMG_4_15_Installation2,
        IMG_3_9_Installation,
        IMG_3_10_Installation,
        IMG_3_11_Installation,
        IMG_3_12_Installation,
        IMG_Substructure_Left_Door,
        IMG_Substructure_Right_Door,
        IMG_3_1_Installation,
        IMG_3_2_Installation,
        IMG_Substructure_Opening_Hole,
        IMG_7_21_Twilight,
        IMG_Unknown_Twilight,
        IMG_7_13_Twilight,
        IMG_7_14_Twilight,
        IMG_7_16_Twilight,
        IMG_7_15_Twilight,
        IMG_7_19_Twilight,
        IMG_7_20_Twilight,
        IMG_7_17_Twilight,
        IMG_6_1_Twilight,
        IMG_6_2_Twilight,
        IMG_6_3_Twilight,
        IMG_6_4_Twilight,
        IMG_6_5_Twilight,
        IMG_8_3_Twilight1,
        IMG_8_3_Twilight2,
        IMG_9_29_Ice,
        IMG_9_29_Ice_Shadow,
        IMG_9_28_Ice,
        IMG_9_28_Ice_Shadow,
        IMG_12_38_Ice_,
        IMG_12_38_Ice_Shadow,
        IMG_12_37_Ice,
        IMG_12_37_Ice_Shadow,
        IMG_12_33_Ice1,
        IMG_12_33_Ice1_Shadow,
        IMG_9_21_Ice,
        IMG_9_21_Ice_Shadow,
        IMG_9_15_Ice,
        IMG_9_15_Ice_Shadow,
        IMG_9_16_Ice,
        IMG_9_16_Ice_Shadow,
        IMG_Unknown787,
        IMG_Unknown788,
        IMG_12_9_Ice1,
        IMG_12_10_Ice1,
        IMG_9_24_Ice,
        IMG_9_24_Ice_Shadow,
        IMG_9_23_Ice,
        IMG_9_23_Ice_Shadow,
        IMG_Unknown795,
        IMG_Unknown_Ice_Shadow,
        IMG_12_7_Ice,
        IMG_12_7_Ice_Shadow,
        IMG_12_8_Ice,
        IMG_12_8_Ice_Shadow,
        IMG_12_9_Ice2,
        IMG_12_9_Ice2_Shadow,
        IMG_12_10_Ice2,
        IMG_12_10_Ice2_Shadow,
        IMG_12_40_Ice,
        IMG_12_40_Ice_Shadow,
        IMG_12_41_Ice,
        IMG_12_41_Ice_Shadow,
        IMG_12_42_Ice,
        IMG_12_42_Ice_Shadow,
        IMG_12_5_Ice,
        IMG_12_5_Ice_Shadow,
        IMG_12_6_Ice,
        IMG_12_6_Ice_Shadow,
        IMG_12_36_Ice,
        IMG_12_36_Ice_Shadow,
        IMG_12_32_Ice,
        IMG_12_32_Ice_Shadow,
        IMG_12_33_Ice2,
        IMG_12_33_Ice2_Shadow,
        IMG_12_34_Ice,
        IMG_12_34_Ice_Shadow,
        IMG_12_24_Ice1,
        IMG_12_24_Ice1_Shadow,
        IMG_12_25_Ice1,
        IMG_12_25_Ice1_Shadow,
        IMG_12_30_Ice1,
        IMG_12_30_Ice1_Shadow,
        IMG_12_31_Ice,
        IMG_12_31_Ice_Shadow,
        IMG_12_20_Ice,
        IMG_12_30_Ice2,
        IMG_12_30_Ice2_Shadow,
        IMG_9_22_Ice,
        IMG_9_22_Ice_Shadow,
        IMG_12_24_Ice2,
        IMG_12_24_Ice2_Shadow,
        IMG_12_25_Ice2,
        IMG_12_25_Ice2_Shadow,
        IMG_Unknown840,
        IMG_4_1_Ice,
        IMG_6_1_Ice,
        IMG_5_6_Ice_,
        IMG_5_6_Ice_Shadow,
        IMG_5_7_Ice_,
        IMG_5_7_Ice_Shadow,
        IMG_5_8_Ice_,
        IMG_5_8_Ice_Shadow,
        IMG_5_9_Ice,
        IMG_5_9_Ice_Shadow,
        IMG_10_10_Desert1,
        IMG_10_12_Desert1,
        IMG_10_12_Desert1_Shadow,
        IMG_10_8_Desert1,
        IMG_10_8_Desert1_Shadow,
        IMG_10_9_Desert1,
        IMG_10_9_Desert1_Shadow,
        IMG_6_10_Desert,
        IMG_6_10_Desert_Shadow,
        IMG_6_13_Desert1,
        IMG_6_13_Desert1_Shadow,
        IMG_Unknown_Desert,
        IMG_Unknown_Desert_Shadow,
        IMG_10_12_Desert2,
        IMG_10_12_Desert2_Shadow,
        IMG_10_9_Desert2,
        IMG_10_9_Desert2_Shadow,
        IMG_10_10_Desert2,
        IMG_10_10_Desert2_Shadow,
        IMG_10_11_Desert,
        IMG_10_11_Desert_Shadow,
        IMG_10_14_Desert,
        IMG_10_14_Desert_Shadow,
        IMG_10_41_Desert,
        IMG_10_41_Desert_Shadow,
        IMG_10_39_Desert,
        IMG_1_39_Desert_Shadow,
        IMG_10_8_Desert2,
        IMG_10_8_Desert2_Shadow,
        IMG_10_6_Desert,
        IMG_10_7_Desert,
        IMG_10_7_Desert_Shadow,
        IMG_4_6_Desert,
        IMG_4_6_Desert_Shadow,
        IMG_4_11_Desert,
        IMG_4_11_Desert_Shadow,
        IMG_4_10_Desert,
        IMG_4_10_Desert_Shadow,
        IMG_4_9_Desert,
        IMG_4_7_Desert,
        IMG_4_7_Desert_Shadow,
        IMG_4_12_Desert,
        IMG_4_12_Desert_Shadow,
        IMG_4_8_Desert,
        IMG_4_13_Desert,
        IMG_4_13_Desert_Shadow,
        IMG_4_17_Desert,
        IMG_4_15_Desert1,
        IMG_4_15_Desert1_Shadow,
        IMG_10_23_Desert,
        IMG_10_23_Desert_Shadow,
        IMG_10_5_Desert,
        IMG_10_5_Desert_Shadow,
        IMG_6_13_Desert2,
        IMG_6_13_Desert2_Shadow,
        IMG_6_20_Desert,
        IMG_4_15_Desert2,
        IMG_4_15_Desert2_Shadow,
        IMG_8_23_Desert,
        IMG_8_23_Desert_Shadow,
        IMG_12_1_Desert_Overlay,
        IMG_11_3_Desert,
        IMG_Unknown913,
        IMG_Lurker_Egg,
        IMG_Devourer,
        IMG_Devourer_Shadow,
        IMG_Devourer_Birth,
        IMG_Devourer_Death,
        IMG_Lurker_Birth,
        IMG_Lurker_Remnants,
        IMG_Lurker,
        IMG_Lurker_Shadow,
        IMG_Overmind_Cocoon,
        IMG_Overmind_Cocoon_Shadow,
        IMG_Dark_Archon_Energy,
        IMG_Dark_Archon_Being,
        IMG_Dark_Archon_Swirl,
        IMG_Dark_Archon_Death,
        IMG_Corsair,
        IMG_Corsair_Shadow,
        IMG_Corsair_Engines,
        IMG_Neutron_Flare_Overlay_Unused,
        IMG_Dark_Templar_Unit,
        IMG_Warp_Gate,
        IMG_Warp_Gate_Shadow,
        IMG_Warp_Gate_Overlay,
        IMG_XelNaga_Temple,
        IMG_XelNaga_Temple_Shadow,
        IMG_Valkyrie,
        IMG_Valkyrie_Shadow,
        IMG_Valkyrie_Engines,
        IMG_Valkyrie_Engines2_Unused,
        IMG_Valkyrie_Afterburners_Unused,
        IMG_Medic,
        IMG_Medic_Shadow,
        IMG_Medic_Remnants,
        IMG_Psi_Disrupter,
        IMG_Psi_Disrupter_Shadow,
        IMG_Power_Generator,
        IMG_Power_Generator_Shadow,
        IMG_Disruption_Web,
        IMG_Scantid_Desert,
        IMG_Scantid_Desert_Shadow,
        IMG_Kakaru_Twilight,
        IMG_Kakaru_Twilight_Shadow,
        IMG_Ursadon_Ice,
        IMG_Ursadon_Ice_Shadow,
        IMG_Uraj,
        IMG_Khalis,
        IMG_Halo_Rockets_Trail,
        IMG_Subterranean_Spines,
        IMG_Corrosive_Acid_Shot,
        IMG_Corrosive_Acid_Hit,
        IMG_Neutron_Flare,
        IMG_Halo_Rockets,
        IMG_Optical_Flare_Grenade,
        IMG_Restoration_Hit_Small,
        IMG_Restoration_Hit_Medium,
        IMG_Restoration_Hit_Large,
        IMG_Unused_Heal_Small,
        IMG_Unused_Heal_Medium,
        IMG_Unused_Heal_Large,
        IMG_Mind_Control_Hit_Small,
        IMG_Mind_Control_Hit_Medium,
        IMG_Mind_Control_Hit_Large,
        IMG_Optical_Flare_Hit_Small,
        IMG_Optical_Flare_Hit_Medium,
        IMG_Optical_Flare_Hit_Large,
        IMG_Feedback_Small,
        IMG_Feedback_Medium,
        IMG_Feedback_Hit_Large,
        IMG_Maelstorm_Overlay_Small,
        IMG_Maelstorm_Overlay_Medium,
        IMG_Maelstorm_Overlay_Large,
        IMG_Subterranean_Spines_Hit,
        IMG_Acid_Spores_1_Overlay_Small,
        IMG_Acid_Spores_2_3_Overlay_Small,
        IMG_Acid_Spores_4_5_Overlay_Small,
        IMG_Acid_Spores_6_9_Overlay_Small,
        IMG_Acid_Spores_1_Overlay_Medium,
        IMG_Acid_Spores_2_3_Overlay_Medium,
        IMG_Acid_Spores_4_5_Overlay_Medium,
        IMG_Acid_Spores_6_9_Overlay_Medium,
        IMG_Acid_Spores_1_Overlay_Large,
        IMG_Acid_Spores_2_3_Overlay_Large,
        IMG_Acid_Spores_4_5_Overlay_Large,
        IMG_Acid_Spores_6_9_Overlay_Large,
        IMG_Maelstorm_Hit,
        IMG_None
    }
}
