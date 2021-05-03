#![feature(never_type)]
#[allow(non_snake_case)]
pub mod bw;
pub mod prelude;
mod sys;

use crate::prelude::{AIModule, BoxedAIModule, Event, Game, GAME};
use cxx::CxxString;
use once_cell::sync::OnceCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::pin::Pin;

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    use std::process;
    process::abort();
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn _Unwind_RaiseException() -> ! {
    use std::process;
    process::abort();
}

#[no_mangle]
pub unsafe extern "C" fn gameInit(game: *const std::ffi::c_void) {
    println!("gameInit called: game = {:?}", game);
    *GAME.lock().unwrap() = Game { raw: game as *mut ffi::Game };
}

/// `FromRaw` is a trait for entities that
/// are typically created outside of Rust code.
pub trait FromRaw<T> {
    unsafe fn from_raw(raw: *const T) -> Self;
}

#[allow(non_snake_case)]
#[cxx::bridge]
pub mod ffi {

    // https://github.com/dtolnay/cxx/issues/855
    #[derive(Debug, Copy, Clone)]
    struct PositionSyn {
        x: i32,
        y: i32,
    }

    // https://github.com/dtolnay/cxx/issues/855
    #[derive(Debug, Copy, Clone)]
    struct TilePositionSyn {
        x: i32,
        y: i32,
    }

    // #[namespace = "BWAPI"]
    // #[cxx_name = "Unit"]
    // struct UnitSyn {
    //     raw: * mut UnitInterface,
    // }
    extern "C++" {
        unsafe fn setRallyPoint(self: &Unitset, target: *mut UnitInterface) -> bool;
    }

    // #[namespace = "BWAPI"]
    // struct Player {
    //     raw: * mut PlayerInterface,
    // }

    #[namespace = "BWAPI"]
    unsafe extern "C++" {
        include!("library/openbw/include/BWAPI.h");
        pub fn BWAPI_getRevision() -> i32;
        pub fn BWAPI_isDebug() -> bool;

        pub type BulletInterface;
        pub type ForceInterface;
        pub type Game;
        pub type PlayerInterface;
        pub type RegionInterface;
        pub type TournamentModule;
        pub type UnitInterface;
        pub type Unit;

        pub type DamageType;
        pub type Error;
        pub type ExplosionType;
        pub type UnitSizeType;

        type Event;
        type UnitFilter;

        pub type Bulletset;
        pub type Forceset;
        pub type Playerset;
        pub type Regionset;
        pub type Unitset;

        #[namespace = "BWAPI::CoordinateType"]
        #[cxx_name = "Enum"]
        type CoordinateType = crate::bw::coordinate_type::CoordinateType;

        #[namespace = "BWAPI::Text::Size"]
        #[cxx_name = "Enum"]
        type TextSize = crate::bw::color::TextSize;

        #[namespace = "BWAPI::Text"]
        #[cxx_name = "Enum"]
        type TextColor = crate::bw::color::TextColor;

        type BulletType = crate::bw::bullet_type::BulletType;
        type Color = crate::bw::color::Color;
        type GameType = crate::bw::game_type::GameType;
        type Key = crate::bw::input::KeyButton;
        type MouseButton = crate::bw::input::MouseButton;
        type PlayerType = crate::bw::player_type::PlayerType;
        type Position = crate::bw::position::Position;
        type Race = crate::bw::race::Race;
        type TechType = crate::bw::tech_type::TechType;
        type TilePosition = crate::bw::position::TilePosition;
        type UnitCommand = crate::bw::unit_command::UnitCommand;
        type UnitType = crate::bw::unit_type::UnitType;
        type UpgradeType = crate::bw::upgrade_type::UpgradeType;
        type WalkPosition = crate::bw::position::WalkPosition;
        type WeaponType = crate::bw::weapon_type::WeaponType;
        type UnitCommandType = crate::bw::unit_command::UnitCommandType;
        type Order = crate::bw::order;

        // type Error = BWAPI_Errors_Enum_Enum;
        // type Flag = BWAPI_Flag_Enum;
        // type Order = BWAPI_Orders_Enum_Enum;
        // type DamageType = BWAPI_DamageTypes_Enum_Enum;
        // type UnitSizeType = BWAPI_UnitSizeTypes_Enum_Enum;
        // type UpgradeType = crate::BWAPI_UpgradeTypes_Enum_Enum;
        // type ExplosionType = BWAPI_ExplosionTypes_Enum_Enum;

        // #[namespace = ""]
        // pub type PositionList;
        // #[namespace = ""]
        // pub type TilePositionList;
        #[namespace = ""]
        pub type EventList;
    }

    unsafe extern "C++" {
        // unfortunately we have to create our type: https://github.com/dtolnay/cxx/issues/796
        pub type c_void;

        pub type BulletsetIterator;
        pub fn next(self: Pin<&mut BulletsetIterator>) -> *const BulletInterface;
        pub fn sizeHint(self: &BulletsetIterator) -> usize;
        pub fn createBulletsetIterator(set: &Bulletset) -> UniquePtr<BulletsetIterator>;

        pub type ForcesetIterator;
        pub fn next(self: Pin<&mut ForcesetIterator>) -> *const ForceInterface;
        pub fn sizeHint(self: &ForcesetIterator) -> usize;
        pub fn createForcesetIterator(set: &Forceset) -> UniquePtr<ForcesetIterator>;

        pub type PlayersetIterator;
        pub fn next(self: Pin<&mut PlayersetIterator>) -> *const PlayerInterface;
        pub fn sizeHint(self: &PlayersetIterator) -> usize;
        pub fn createPlayersetIterator(set: &Playerset) -> UniquePtr<PlayersetIterator>;

        pub type UnitsetIterator;
        pub fn next(self: Pin<&mut UnitsetIterator>) -> *const UnitInterface;
        pub fn sizeHint(self: &UnitsetIterator) -> usize;
        pub fn createRegionsetIterator(set: &Regionset) -> UniquePtr<RegionsetIterator>;

        pub type RegionsetIterator;
        pub fn next(self: Pin<&mut RegionsetIterator>) -> *const RegionInterface;
        pub fn sizeHint(self: &RegionsetIterator) -> usize;
        pub fn createUnitsetIterator(set: &Unitset) -> UniquePtr<UnitsetIterator>;

        // helpers so far, unit api is coming
        pub unsafe fn Unit_getId(unit: *const UnitInterface) -> i32;
        pub unsafe fn Unit_getType(unit: *const UnitInterface) -> UnitType;
        pub unsafe fn Unit_getPosition(unit: *const UnitInterface) -> Position;

    }

    // region BWAPI::BulletInterface
    unsafe extern "C++" {
        fn exists(self: &BulletInterface) -> bool;
        fn getAngle(self: &BulletInterface) -> f64;
        fn getID(self: &BulletInterface) -> i32;
        fn getPlayer(self: &BulletInterface) -> *mut PlayerInterface;
        fn getPosition(self: &BulletInterface) -> Position;
        fn getRemoveTimer(self: &BulletInterface) -> i32;
        fn getSource(self: &BulletInterface) -> *mut UnitInterface;
        fn getTarget(self: &BulletInterface) -> *mut UnitInterface;
        fn getTargetPosition(self: &BulletInterface) -> Position;
        fn getType(self: &BulletInterface) -> BulletType;
        fn getVelocityX(self: &BulletInterface) -> f64;
        fn getVelocityY(self: &BulletInterface) -> f64;
        unsafe fn isVisible(self: &BulletInterface, player: *mut PlayerInterface) -> bool;
    }
    // endregion

    // region BWAPI::ForceInterface
    unsafe extern "C++" {
        pub fn getID(self: &ForceInterface) -> i32;
        pub fn _force_getName(force: &ForceInterface) -> UniquePtr<CxxString>;
        pub fn _force_getPlayers(force: &ForceInterface) -> UniquePtr<Playerset>;
    }
    // endregion

    // region BWAPI::Unitset
    unsafe extern "C++" {
        fn _unitset_getClosestUnit(set: &Unitset, pred: fn(Unit) -> bool, radius: i32) -> *const UnitInterface;
        fn _unitset_getInterceptors(set: &Unitset) -> UniquePtr<Unitset>;
        fn _unitset_getLarva(set: &Unitset) -> UniquePtr<Unitset>;
        fn _unitset_getLoadedUnits(set: &Unitset) -> UniquePtr<Unitset>;
        fn getPosition(self: &Unitset) -> Position;
        fn _unitset_getUnitsInRadius(set: &Unitset, radius: i32, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>;
        unsafe fn setClientInfo(self: &Unitset, clientInfo: *mut c_void, index: i32);
        #[cxx_name = "setClientInfo"]
        fn setClientInfo1(self: &Unitset, client_info: i32, index: i32);
        fn issueCommand(self: &Unitset, command: UnitCommand) -> bool;
        unsafe fn attack(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        #[cxx_name = "attack"]
        fn attack1(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn build(self: &Unitset, utype: UnitType, target: TilePosition) -> bool;
        fn buildAddon(self: &Unitset, utype: UnitType) -> bool;
        fn train(self: &Unitset, utype: UnitType) -> bool;
        fn morph(self: &Unitset, utype: UnitType) -> bool;
        // unsafe fn setRallyPoint(self: &Unitset, target: Unit) -> bool;
        #[cxx_name = "setRallyPoint"]
        fn setRallyPoint1(self: &Unitset, target: Position) -> bool;
        fn _unitset_move(set: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn patrol(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn holdPosition(self: &Unitset, shiftQueueCommand: bool) -> bool;
        fn stop(self: &Unitset, shiftQueueCommand: bool) -> bool;
        unsafe fn follow(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        unsafe fn gather(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn returnCargo(self: &Unitset, shiftQueueCommand: bool) -> bool;
        unsafe fn repair(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn burrow(self: &Unitset) -> bool;
        fn unburrow(self: &Unitset) -> bool;
        fn cloak(self: &Unitset) -> bool;
        fn decloak(self: &Unitset) -> bool;
        fn siege(self: &Unitset) -> bool;
        fn unsiege(self: &Unitset) -> bool;
        fn lift(self: &Unitset) -> bool;
        unsafe fn load(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn unloadAll(self: &Unitset, shiftQueueCommand: bool) -> bool;
        #[cxx_name = "unloadAll"]
        fn unloadAll1(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        unsafe fn rightClick(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        #[cxx_name = "rightClick"]
        fn rightClick1(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn haltConstruction(self: &Unitset) -> bool;
        fn cancelConstruction(self: &Unitset) -> bool;
        fn cancelAddon(self: &Unitset) -> bool;
        fn cancelTrain(self: &Unitset, slot: i32) -> bool;
        fn cancelMorph(self: &Unitset) -> bool;
        fn cancelResearch(self: &Unitset) -> bool;
        fn cancelUpgrade(self: &Unitset) -> bool;
        unsafe fn useTech(self: &Unitset, tech: TechType, target: *mut UnitInterface) -> bool;
        #[cxx_name = "useTech"]
        fn useTech1(self: &Unitset, tech: TechType, target: Position) -> bool;
    }
    // endregion

    // region BWAPI::Game
    unsafe extern "C++" {
        #[namespace = "BWAPI"]
        #[cxx_name = "Position"]
        type PositionSyn;
        #[namespace = "BWAPI"]
        #[cxx_name = "TilePosition"]
        type TilePositionSyn;

        fn _game_debug(game: &Game) -> UniquePtr<Playerset>;
        fn _game_debug_fun(game: &Game, fun: fn(Unit) -> bool);
        // fn getLastError(self: &Game) -> Error;

        fn allies(self: Pin<&mut Game>) -> Pin<&mut Playerset>;
        unsafe fn canBuildHere(self: Pin<&mut Game>, position: TilePosition, uType: UnitType, builder: *mut UnitInterface, checkExplored: bool) -> bool;
        unsafe fn canMake(self: &Game, utype: UnitType, builder: *mut UnitInterface) -> bool;
        unsafe fn canResearch(self: Pin<&mut Game>, ttype: TechType, unit: *mut UnitInterface, checkCanIssueCommandType: bool) -> bool;
        unsafe fn canUpgrade(self: Pin<&mut Game>, utype: UpgradeType, unit: *mut UnitInterface, checkCanIssueCommandType: bool) -> bool;
        fn countdownTimer(self: &Game) -> i32;
        fn elapsedTime(self: &Game) -> i32;
        fn enableFlag(self: Pin<&mut Game>, flag: i32);
        fn enemies(self: Pin<&mut Game>) -> Pin<&mut Playerset>;
        unsafe fn enemy(self: &Game) -> *mut PlayerInterface;
        fn getAllRegions(self: &Game) -> &Regionset;
        fn getAllUnits(self: &Game) -> &Unitset;
        fn getAPM(self: &Game, includeSelects: bool) -> i32;
        fn getAverageFPS(self: &Game) -> f64;
        fn _game_getBestUnit(game: &Game, best: fn(Unit, Unit) -> Unit, pred: fn(Unit) -> bool, center: Position, radius: i32) -> *mut UnitInterface;
        fn getBuildLocation(self: &Game, unitType: UnitType, desiredPosition: TilePosition, maxRange: i32, creep: bool) -> TilePosition;
        fn getBullets(self: &Game) -> &Bulletset;
        fn getClientVersion(self: &Game) -> i32;
        fn _game_getClosestUnit(game: &Game, center: Position, pred: fn(Unit) -> bool, radius: i32) -> *mut UnitInterface;
        fn _game_getClosestUnitInRectangle(game: &Game, center: Position, pred: fn(Unit) -> bool, left: i32, top: i32, right: i32, bottom: i32) -> *mut UnitInterface;
        unsafe fn getDamageFrom(self: &Game, fromType: UnitType, toType: UnitType, fromPlayer: *mut PlayerInterface, toPlayer: *mut PlayerInterface) -> i32;
        unsafe fn getDamageTo(self: &Game, toType: UnitType, fromType: UnitType, toPlayer: *mut PlayerInterface, fromPlayer: *mut PlayerInterface) -> i32;
        fn _game_getEvents(game: &Game) -> &EventList;
        fn getForce(self: &Game, forceId: i32) -> *mut ForceInterface;
        fn getForces(self: &Game) -> &Forceset;
        fn getFPS(self: &Game) -> i32;
        fn getFrameCount(self: &Game) -> i32;
        fn getGameType(self: &Game) -> GameType;
        fn getGeysers(self: &Game) -> &Unitset;
        fn getGroundHeight(self: &Game, position: TilePosition) -> i32;
        fn getInstanceNumber(self: &Game) -> i32;
        fn getKeyState(self: &Game, key: Key) -> bool;
        fn getLastEventTime(self: &Game) -> i32;
        fn getLatency(self: &Game) -> i32;
        fn getLatencyFrames(self: &Game) -> i32;
        fn getLatencyTime(self: &Game) -> i32;
        fn getMinerals(self: &Game) -> &Unitset;
        fn getMousePosition(self: &Game) -> Position;
        fn getMouseState(self: &Game, button: MouseButton) -> bool;
        fn getNeutralUnits(self: &Game) -> &Unitset;
        fn _game_getNukeDots(game: &Game) -> Vec<PositionSyn>;
        fn getPlayer(self: &Game, playerId: i32) -> *mut PlayerInterface;
        fn getPlayers(self: &Game) -> &Playerset;
        fn getRandomSeed(self: &Game) -> u32;
        fn getRegion(self: &Game, regionID: i32) -> *mut RegionInterface;
        fn getRegionAt(self: &Game, position: Position) -> *mut RegionInterface;
        fn getRemainingLatencyFrames(self: &Game) -> i32;
        fn getRemainingLatencyTime(self: &Game) -> i32;
        fn getReplayFrameCount(self: &Game) -> i32;
        fn getRevision(self: &Game) -> i32;
        fn getScreenPosition(self: &Game) -> Position;
        fn getSelectedUnits(self: &Game) -> &Unitset;
        fn _game_getStartLocations(game: &Game) -> Vec<TilePositionSyn>;
        fn getStaticGeysers(self: &Game) -> &Unitset;
        fn getStaticMinerals(self: &Game) -> &Unitset;
        fn getStaticNeutralUnits(self: &Game) -> &Unitset;
        fn getUnit(self: &Game, unitID: i32) -> *mut UnitInterface;
        fn _game_getUnitsInRadius(game: &Game, position: Position, radius: i32, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>;
        fn _game_getUnitsInRectangle(game: &Game, topLeft: Position, bottomRight: Position, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>;
        fn _game_getUnitsOnTile(game: &Game, tile: TilePosition, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>;
        fn hasCreep(self: &Game, position: TilePosition) -> bool;
        fn hasPath(self: &Game, source: Position, destination: Position) -> bool;
        fn hasPower(self: &Game, position: TilePosition, unitType: UnitType) -> bool;
        fn hasPowerPrecise(self: &Game, position: Position, unitType: UnitType) -> bool;
        fn indexToUnit(self: &Game, unitIndex: i32) -> *mut UnitInterface;
        fn isBattleNet(self: &Game) -> bool;
        fn isBuildable(self: &Game, position: TilePosition, includeBuildings: bool) -> bool;
        fn isDebug(self: &Game) -> bool;
        fn isExplored(self: &Game, position: TilePosition) -> bool;
        fn isFlagEnabled(self: &Game, flag: i32) -> bool;
        fn isGUIEnabled(self: &Game) -> bool;
        fn isInGame(self: &Game) -> bool;
        fn isLatComEnabled(self: &Game) -> bool;
        fn isMultiplayer(self: &Game) -> bool;
        fn isPaused(self: &Game) -> bool;
        fn isReplay(self: &Game) -> bool;
        fn issueCommand(self: Pin<&mut Game>, units: &Unitset, command: UnitCommand) -> bool;
        fn isVisible(self: &Game, position: TilePosition) -> bool;
        fn isWalkable(self: &Game, position: WalkPosition) -> bool;
        fn leaveGame(self: Pin<&mut Game>);
        fn _game_mapFileName(game: &Game) -> UniquePtr<CxxString>;
        fn _game_mapHash(game: &Game) -> UniquePtr<CxxString>;
        fn mapHeight(self: &Game) -> i32;
        fn _game_mapName(game: &Game) -> UniquePtr<CxxString>;
        fn _game_mapPathName(game: &Game) -> UniquePtr<CxxString>;
        fn mapWidth(self: &Game) -> i32;
        fn neutral(self: &Game) -> *mut PlayerInterface;
        fn observers(self: Pin<&mut Game>) -> Pin<&mut Playerset>;
        fn pauseGame(self: Pin<&mut Game>);
        fn pingMinimap(self: Pin<&mut Game>, p: Position);
        fn _game_printf(game: Pin<&mut Game>, text: &str);
        fn restartGame(self: Pin<&mut Game>);
        fn resumeGame(self: Pin<&mut Game>);
        fn _game_self(game: &Game) -> *mut PlayerInterface;
        fn _game_sendText(game: Pin<&mut Game>, text: &str);
        fn _game_sendTextEx(game: Pin<&mut Game>, toAllies: bool, text: &str);
        unsafe fn setAlliance(self: Pin<&mut Game>, player: *mut PlayerInterface, allied: bool, alliedVictory: bool) -> bool;
        fn setCommandOptimizationLevel(self: Pin<&mut Game>, level: i32);
        fn setFrameSkip(self: Pin<&mut Game>, frameSkip: i32);
        fn setGUI(self: Pin<&mut Game>, enabled: bool);
        fn setLatCom(self: Pin<&mut Game>, isEnabled: bool);
        fn setLocalSpeed(self: Pin<&mut Game>, speed: i32);
        fn _game_setMap(game: Pin<&mut Game>, text: &str) -> bool;
        fn setRevealAll(self: Pin<&mut Game>, reveal: bool) -> bool;
        fn setScreenPosition(self: Pin<&mut Game>, p: Position);
        unsafe fn setVision(self: Pin<&mut Game>, player: *mut PlayerInterface, enabled: bool) -> bool;
        fn setTextSize(self: Pin<&mut Game>, size: TextSize);
        fn _game_drawText(game: Pin<&mut Game>, ctype: CoordinateType, x: i32, y: i32, text: &str);
        fn drawBox(self: Pin<&mut Game>, ctype: CoordinateType, left: i32, top: i32, right: i32, bottom: i32, color: Color, isSolid: bool);
        fn drawTriangle(self: Pin<&mut Game>, ctype: CoordinateType, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, color: Color, isSolid: bool);
        fn drawCircle(self: Pin<&mut Game>, ctype: CoordinateType, x: i32, y: i32, radius: i32, color: Color, isSolid: bool);
        fn drawEllipse(self: Pin<&mut Game>, ctype: CoordinateType, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, isSolid: bool);
        fn drawDot(self: Pin<&mut Game>, ctype: CoordinateType, x: i32, y: i32, color: Color);
        fn drawLine(self: Pin<&mut Game>, ctype: CoordinateType, x1: i32, y1: i32, x2: i32, y2: i32, color: Color);
    }
    // endregion

    // region BWAPI::PlayerInterface
    unsafe extern "C++" {
        // type Player;
        fn allUnitCount(self: &PlayerInterface, unit: UnitType) -> i32;
        fn armor(self: &PlayerInterface, unit: UnitType) -> i32;
        fn completedUnitCount(self: &PlayerInterface, unit: UnitType) -> i32;
        fn damage(self: &PlayerInterface, wpn: WeaponType) -> i32;
        fn deadUnitCount(self: &PlayerInterface, unit: UnitType) -> i32;
        fn gas(self: &PlayerInterface) -> i32;
        fn gatheredGas(self: &PlayerInterface) -> i32;
        fn gatheredMinerals(self: &PlayerInterface) -> i32;
        fn getBuildingScore(self: &PlayerInterface) -> i32;
        fn getColor(self: &PlayerInterface) -> Color;
        fn getCustomScore(self: &PlayerInterface) -> i32;
        fn getForce(self: &PlayerInterface) -> *mut ForceInterface;
        fn getID(self: &PlayerInterface) -> i32;
        fn getKillScore(self: &PlayerInterface) -> i32;
        fn getMaxUpgradeLevel(self: &PlayerInterface, upgrade: UpgradeType) -> i32;
        fn _player_getName(player: &PlayerInterface) -> UniquePtr<CxxString>;
        fn getRace(self: &PlayerInterface) -> Race;
        fn getRazingScore(self: &PlayerInterface) -> i32;
        fn getStartLocation(self: &PlayerInterface) -> TilePosition;
        fn _player_getTextColor(player: &PlayerInterface) -> TextColor;
        fn getType(self: &PlayerInterface) -> PlayerType;
        fn getUnits(self: &PlayerInterface) -> &Unitset;
        fn getUnitScore(self: &PlayerInterface) -> i32;
        fn getUpgradeLevel(self: &PlayerInterface, upgrade: UpgradeType) -> i32;
        fn hasResearched(self: &PlayerInterface, tech: TechType) -> bool;
        fn hasUnitTypeRequirement(self: &PlayerInterface, unit: UnitType, amount: i32) -> bool;
        fn incompleteUnitCount(self: &PlayerInterface, unit: UnitType) -> i32;
        unsafe fn isAlly(self: &PlayerInterface, player: *mut PlayerInterface) -> bool;
        fn isDefeated(self: &PlayerInterface) -> bool;
        unsafe fn isEnemy(self: &PlayerInterface, player: *mut PlayerInterface) -> bool;
        fn isNeutral(self: &PlayerInterface) -> bool;
        fn isObserver(self: &PlayerInterface) -> bool;
        fn isResearchAvailable(self: &PlayerInterface, tech: TechType) -> bool;
        fn isResearching(self: &PlayerInterface, tech: TechType) -> bool;
        fn isUnitAvailable(self: &PlayerInterface, unit: UnitType) -> bool;
        fn isUpgrading(self: &PlayerInterface, upgrade: UpgradeType) -> bool;
        fn isVictorious(self: &PlayerInterface) -> bool;
        fn killedUnitCount(self: &PlayerInterface, unit: UnitType) -> i32;
        fn leftGame(self: &PlayerInterface) -> bool;
        fn maxEnergy(self: &PlayerInterface, unit: UnitType) -> i32;
        fn minerals(self: &PlayerInterface) -> i32;
        fn refundedGas(self: &PlayerInterface) -> i32;
        fn refundedMinerals(self: &PlayerInterface) -> i32;
        fn repairedGas(self: &PlayerInterface) -> i32;
        fn repairedMinerals(self: &PlayerInterface) -> i32;
        fn sightRange(self: &PlayerInterface, unit: UnitType) -> i32;
        fn spentGas(self: &PlayerInterface) -> i32;
        fn spentMinerals(self: &PlayerInterface) -> i32;
        fn supplyTotal(self: &PlayerInterface, race: Race) -> i32;
        fn supplyUsed(self: &PlayerInterface, race: Race) -> i32;
        fn topSpeed(self: &PlayerInterface, unit: UnitType) -> f64;
        fn visibleUnitCount(self: &PlayerInterface, unit: UnitType) -> i32;
        fn weaponDamageCooldown(self: &PlayerInterface, unit: UnitType) -> i32;
        fn weaponMaxRange(self: &PlayerInterface, weapon: WeaponType) -> i32;
    }
    // endregion

    // region BWAPI::RegionInterface
    unsafe extern "C++" {
        fn getBoundsBottom(self: &RegionInterface) -> i32;
        fn getBoundsLeft(self: &RegionInterface) -> i32;
        fn getBoundsRight(self: &RegionInterface) -> i32;
        fn getBoundsTop(self: &RegionInterface) -> i32;
        fn getCenter(self: &RegionInterface) -> Position;
        fn getClosestAccessibleRegion(self: &RegionInterface) -> *mut RegionInterface;
        fn getClosestInaccessibleRegion(self: &RegionInterface) -> *mut RegionInterface;
        fn getDefensePriority(self: &RegionInterface) -> i32;
        unsafe fn getDistance(self: &RegionInterface, other: *mut RegionInterface) -> i32;
        fn getID(self: &RegionInterface) -> i32;
        fn getNeighbors(self: &RegionInterface) -> &Regionset;
        fn getRegionGroupID(self: &RegionInterface) -> i32;
        fn _region_getUnits(region: &RegionInterface, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>;
        fn isAccessible(self: &RegionInterface) -> bool;
        fn isHigherGround(self: &RegionInterface) -> bool;
    }
    // endregion

    // region BWAPI::UnitInterface
    unsafe extern "C++" {
        fn exists(self: &UnitInterface) -> bool;
        fn getAcidSporeCount(self: &UnitInterface) -> i32;
        fn getAddon(self: &UnitInterface) -> *mut UnitInterface;
        fn getAirWeaponCooldown(self: &UnitInterface) -> i32;
        fn getAngle(self: &UnitInterface) -> f64;
        fn getBottom(self: &UnitInterface) -> i32;
        fn getBuildType(self: &UnitInterface) -> UnitType;
        fn getBuildUnit(self: &UnitInterface) -> *mut UnitInterface;
        fn getCarrier(self: &UnitInterface) -> *mut UnitInterface;
        fn _unit_getClosestUnit(unit: &UnitInterface, pred: fn(Unit) -> bool, radius: i32) -> *mut UnitInterface;
        fn getDefenseMatrixPoints(self: &UnitInterface) -> i32;
        fn getDefenseMatrixTimer(self: &UnitInterface) -> i32;
        fn getDistanceP(self: &UnitInterface, target: Position) -> i32;
        unsafe fn getDistanceU(self: &UnitInterface, target: *mut UnitInterface) -> i32;
        fn getEnergy(self: &UnitInterface) -> i32;
        fn getEnsnareTimer(self: &UnitInterface) -> i32;
        fn getGroundWeaponCooldown(self: &UnitInterface) -> i32;
        fn getHatchery(self: &UnitInterface) -> *mut UnitInterface;
        fn getHitPoints(self: &UnitInterface) -> i32;
        fn getID(self: &UnitInterface) -> i32;
        fn getInitialHitPoints(self: &UnitInterface) -> i32;
        fn getInitialPosition(self: &UnitInterface) -> Position;
        fn getInitialResources(self: &UnitInterface) -> i32;
        fn getInitialTilePosition(self: &UnitInterface) -> TilePosition;
        fn getInitialType(self: &UnitInterface) -> UnitType;
        fn getInterceptorCount(self: &UnitInterface) -> i32;
        fn getInterceptors(self: &UnitInterface) -> UniquePtr<Unitset>;
        fn getIrradiateTimer(self: &UnitInterface) -> i32;
        fn getKillCount(self: &UnitInterface) -> i32;
        fn getLarva(self: &UnitInterface) -> UniquePtr<Unitset>;
        fn getLastAttackingPlayer(self: &UnitInterface) -> *mut PlayerInterface;
        fn getLastCommand(self: &UnitInterface) -> UnitCommand;
        fn getLastCommandFrame(self: &UnitInterface) -> i32;
        fn getLeft(self: &UnitInterface) -> i32;
        fn getLoadedUnits(self: &UnitInterface) -> UniquePtr<Unitset>;
        fn getLockdownTimer(self: &UnitInterface) -> i32;
        fn getMaelstromTimer(self: &UnitInterface) -> i32;
        fn getNydusExit(self: &UnitInterface) -> *mut UnitInterface;
        fn getOrder(self: &UnitInterface) -> Order;
        fn getOrderTarget(self: &UnitInterface) -> *mut UnitInterface;
        fn getOrderTargetPosition(self: &UnitInterface) -> Position;
        fn getOrderTimer(self: &UnitInterface) -> i32;
        fn getPlagueTimer(self: &UnitInterface) -> i32;
        fn getPlayer(self: &UnitInterface) -> *mut PlayerInterface;
        fn getPosition(self: &UnitInterface) -> Position;
        fn getPowerUp(self: &UnitInterface) -> *mut UnitInterface;
        fn getRallyPosition(self: &UnitInterface) -> Position;
        fn getRallyUnit(self: &UnitInterface) -> *mut UnitInterface;
        fn getRegion(self: &UnitInterface) -> *mut RegionInterface;
        fn getRemainingBuildTime(self: &UnitInterface) -> i32;
        fn getRemainingResearchTime(self: &UnitInterface) -> i32;
        fn getRemainingTrainTime(self: &UnitInterface) -> i32;
        fn getRemainingUpgradeTime(self: &UnitInterface) -> i32;
        fn getRemoveTimer(self: &UnitInterface) -> i32;
        fn getReplayID(self: &UnitInterface) -> i32;
        fn getResourceGroup(self: &UnitInterface) -> i32;
        fn getResources(self: &UnitInterface) -> i32;
        fn getRight(self: &UnitInterface) -> i32;
        fn getScarabCount(self: &UnitInterface) -> i32;
        fn getSecondaryOrder(self: &UnitInterface) -> Order;
        fn getShields(self: &UnitInterface) -> i32;
        fn getSpaceRemaining(self: &UnitInterface) -> i32;
        fn getSpellCooldown(self: &UnitInterface) -> i32;
        fn getSpiderMineCount(self: &UnitInterface) -> i32;
        fn getStasisTimer(self: &UnitInterface) -> i32;
        fn getStimTimer(self: &UnitInterface) -> i32;
        fn getTarget(self: &UnitInterface) -> *mut UnitInterface;
        fn getTargetPosition(self: &UnitInterface) -> Position;
        fn getTech(self: &UnitInterface) -> TechType;
        fn getTilePosition(self: &UnitInterface) -> TilePosition;
        fn getTop(self: &UnitInterface) -> i32;
        fn getTrainingQueue(self: &UnitInterface) -> Vec<UnitType>;
        fn getTransport(self: &UnitInterface) -> *mut UnitInterface;
        fn getType(self: &UnitInterface) -> UnitType;
        fn getUnitsInRadius(self: &UnitInterface, radius: i32, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>;
        fn getUnitsInWeaponRange(self: &UnitInterface, weapon: WeaponType, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>;
        fn getUpgrade(self: &UnitInterface) -> UpgradeType;
        fn getVelocityX(self: &UnitInterface) -> f64;
        fn getVelocityY(self: &UnitInterface) -> f64;
        fn hasNuke(self: &UnitInterface) -> bool;
        fn hasPathP(self: &UnitInterface, target: Position) -> bool;
        unsafe fn hasPathU(self: &UnitInterface, target: *mut UnitInterface) -> bool;
        fn isAccelerating(self: &UnitInterface) -> bool;
        fn isAttackFrame(self: &UnitInterface) -> bool;
        fn isAttacking(self: &UnitInterface) -> bool;
        fn isBeingConstructed(self: &UnitInterface) -> bool;
        fn isBeingGathered(self: &UnitInterface) -> bool;
        fn isBeingHealed(self: &UnitInterface) -> bool;
        fn isBlind(self: &UnitInterface) -> bool;
        fn isBraking(self: &UnitInterface) -> bool;
        fn isBurrowed(self: &UnitInterface) -> bool;
        fn isCarryingGas(self: &UnitInterface) -> bool;
        fn isCarryingMinerals(self: &UnitInterface) -> bool;
        fn isCloaked(self: &UnitInterface) -> bool;
        fn isCompleted(self: &UnitInterface) -> bool;
        fn isConstructing(self: &UnitInterface) -> bool;
        fn isDefenseMatrixed(self: &UnitInterface) -> bool;
        fn isDetected(self: &UnitInterface) -> bool;
        fn isEnsnared(self: &UnitInterface) -> bool;
        fn isFlying(self: &UnitInterface) -> bool;
        fn isFollowing(self: &UnitInterface) -> bool;
        fn isGatheringGas(self: &UnitInterface) -> bool;
        fn isGatheringMinerals(self: &UnitInterface) -> bool;
        fn isHallucination(self: &UnitInterface) -> bool;
        fn isHoldingPosition(self: &UnitInterface) -> bool;
        fn isIdle(self: &UnitInterface) -> bool;
        fn isInterruptible(self: &UnitInterface) -> bool;
        fn isInvincible(self: &UnitInterface) -> bool;
        unsafe fn isInWeaponRange(self: &UnitInterface, target: *mut UnitInterface) -> bool;
        fn isIrradiated(self: &UnitInterface) -> bool;
        fn isLifted(self: &UnitInterface) -> bool;
        fn isLoaded(self: &UnitInterface) -> bool;
        fn isLockedDown(self: &UnitInterface) -> bool;
        fn isMaelstrommed(self: &UnitInterface) -> bool;
        fn isMorphing(self: &UnitInterface) -> bool;
        fn isMoving(self: &UnitInterface) -> bool;
        fn isParasited(self: &UnitInterface) -> bool;
        fn isPatrolling(self: &UnitInterface) -> bool;
        fn isPlagued(self: &UnitInterface) -> bool;
        fn isPowered(self: &UnitInterface) -> bool;
        fn isRepairing(self: &UnitInterface) -> bool;
        fn isResearching(self: &UnitInterface) -> bool;
        fn isSelected(self: &UnitInterface) -> bool;
        fn isSieged(self: &UnitInterface) -> bool;
        fn isStartingAttack(self: &UnitInterface) -> bool;
        fn isStasised(self: &UnitInterface) -> bool;
        fn isStimmed(self: &UnitInterface) -> bool;
        fn isStuck(self: &UnitInterface) -> bool;
        fn isTargetable(self: &UnitInterface) -> bool;
        fn isTraining(self: &UnitInterface) -> bool;
        fn isUnderAttack(self: &UnitInterface) -> bool;
        fn isUnderDarkSwarm(self: &UnitInterface) -> bool;
        fn isUnderDisruptionWeb(self: &UnitInterface) -> bool;
        fn isUnderStorm(self: &UnitInterface) -> bool;
        fn isUpgrading(self: &UnitInterface) -> bool;
        unsafe fn isVisible(self: &UnitInterface, player: *mut PlayerInterface) -> bool;
        fn issueCommand(self: &UnitInterface, command: UnitCommand, shiftQueueCommand: bool) -> bool;
        fn attackP(self: &UnitInterface, target: Position, shiftQueueCommand: bool) -> bool;
        unsafe fn attackU(self: &UnitInterface, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn build(self: &UnitInterface, uType: UnitType, target: TilePosition) -> bool;
        fn buildAddon(self: &UnitInterface, uType: UnitType) -> bool;
        fn train(self: &UnitInterface, uType: UnitType) -> bool;
        fn morph(self: &UnitInterface, uType: UnitType) -> bool;
        fn research(self: &UnitInterface, tech: TechType) -> bool;
        fn upgrade(self: &UnitInterface, upgrade: UpgradeType) -> bool;
        fn setRallyPoint(self: &UnitInterface, target: Position) -> bool;
        fn move_(self: &UnitInterface, target: Position, shiftQueueCommand: bool) -> bool;
        fn patrol(self: &UnitInterface, target: Position, shiftQueueCommand: bool) -> bool;
        fn holdPosition(self: &UnitInterface, shiftQueueCommand: bool) -> bool;
        fn stop(self: &UnitInterface, shiftQueueCommand: bool) -> bool;
        unsafe fn follow(self: &UnitInterface, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        unsafe fn gather(self: &UnitInterface, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn returnCargo(self: &UnitInterface, shiftQueueCommand: bool) -> bool;
        unsafe fn repair(self: &UnitInterface, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn burrow(self: &UnitInterface) -> bool;
        fn unburrow(self: &UnitInterface) -> bool;
        fn cloak(self: &UnitInterface) -> bool;
        fn decloak(self: &UnitInterface) -> bool;
        fn siege(self: &UnitInterface) -> bool;
        fn unsiege(self: &UnitInterface) -> bool;
        fn lift(self: &UnitInterface) -> bool;
        fn land(self: &UnitInterface, target: TilePosition) -> bool;
        unsafe fn load(self: &UnitInterface, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        unsafe fn unload(self: &UnitInterface, target: *mut UnitInterface) -> bool;
        fn unloadAll(self: &UnitInterface, shiftQueueCommand: bool) -> bool;
        fn unloadAllP(self: &UnitInterface, target: Position, shiftQueueCommand: bool) -> bool;
        fn rightClickP(self: &UnitInterface, target: Position, shiftQueueCommand: bool) -> bool;
        unsafe fn rightClickU(self: &UnitInterface, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn haltConstruction(self: &UnitInterface) -> bool;
        fn cancelConstruction(self: &UnitInterface) -> bool;
        fn cancelAddon(self: &UnitInterface) -> bool;
        fn cancelTrain(self: &UnitInterface, slot: i32) -> bool;
        fn cancelMorph(self: &UnitInterface) -> bool;
        fn cancelResearch(self: &UnitInterface) -> bool;
        fn cancelUpgrade(self: &UnitInterface) -> bool;
        fn useTechP(self: &UnitInterface, tech: TechType, target: Position) -> bool;
        unsafe fn useTechU(self: &UnitInterface, tech: TechType, target: *mut UnitInterface) -> bool;
        fn placeCOP(self: &UnitInterface, target: TilePosition) -> bool;
        fn canIssueCommand(self: &UnitInterface, command: UnitCommand, checkCanUseTechPositionOnPositions: bool, checkCanUseTechUnitOnUnits: bool, checkCanBuildUnitType: bool, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canIssueCommandGrouped(self: &UnitInterface, command: UnitCommand, checkCanUseTechPositionOnPositions: bool, checkCanUseTechUnitOnUnits: bool, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canCommand(self: &UnitInterface) -> bool;
        fn canCommandGrouped(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canIssueCommandType(self: &UnitInterface, ctype: UnitCommandType, checkCommandibility: bool) -> bool;
        fn canIssueCommandTypeGrouped(self: &UnitInterface, ctype: UnitCommandType, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        unsafe fn canTargetUnit(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCommandibility: bool) -> bool;
        fn canAttack(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canAttackP(self: &UnitInterface, target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        unsafe fn canAttackU(self: &UnitInterface, target: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canAttackGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canAttackGroupedP(self: &UnitInterface, target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        unsafe fn canAttackGroupedU(self: &UnitInterface, target: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canAttackMove(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canAttackMoveGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canAttackUnit(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canAttackUnitU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canAttackUnitGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        unsafe fn canAttackUnitGroupedU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canBuild(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canBuildT(self: &UnitInterface, uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canBuildU(self: &UnitInterface, uType: UnitType, tilePos: TilePosition, checkTargetUnitType: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canBuildAddon(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canBuildAddonT(self: &UnitInterface, uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canTrain(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canTrainT(self: &UnitInterface, uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canMorph(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canMorphT(self: &UnitInterface, uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canResearch(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canResearchT(self: &UnitInterface, tType: TechType, checkCanIssueCommandType: bool) -> bool;
        fn canUpgrade(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUpgradeT(self: &UnitInterface, tType: UpgradeType, checkCanIssueCommandType: bool) -> bool;
        fn canSetRallyPoint(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canSetRallyPointP(self: &UnitInterface, target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        unsafe fn canSetRallyPointU(self: &UnitInterface, target: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canSetRallyPosition(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canSetRallyUnit(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canSetRallyUnitU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canMove(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canMoveGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canPatrol(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canPatrolGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canFollow(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canFollowU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canGather(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canGatherU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canReturnCargo(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canHoldPosition(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canStop(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canRepair(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canRepairU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canBurrow(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUnburrow(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCloak(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canDecloak(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canSiege(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUnsiege(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canLift(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canLand(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canLandP(self: &UnitInterface, target: TilePosition, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canLoad(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canLoadU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUnloadWithOrWithoutTarget(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUnloadAtPosition(self: &UnitInterface, targDropPos: Position, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUnload(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canUnloadU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkPosition: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUnloadAll(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUnloadAllPosition(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUnloadAllPositionP(self: &UnitInterface, targDropPos: Position, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canRightClick(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canRightClickP(self: &UnitInterface, target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        unsafe fn canRightClickU(self: &UnitInterface, target: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canRightClickGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canRightClickGroupedP(self: &UnitInterface, target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        unsafe fn canRightClickGroupedU(self: &UnitInterface, target: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canRightClickPosition(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canRightClickPositionGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canRightClickUnit(self: &UnitInterface, checkCommandibility: bool) -> bool;
        unsafe fn canRightClickUnitU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canRightClickUnitGrouped(self: &UnitInterface, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        unsafe fn canRightClickUnitGroupedU(self: &UnitInterface, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
        fn canHaltConstruction(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCancelConstruction(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCancelAddon(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCancelTrain(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCancelTrainSlot(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCancelTrainSlotI(self: &UnitInterface, slot: i32, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canCancelMorph(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCancelResearch(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canCancelUpgrade(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUseTechWithOrWithoutTarget(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canUseTechWithOrWithoutTargetT(self: &UnitInterface, tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUseTechP(self: &UnitInterface, tech: TechType, target: Position, checkCanTargetUnit: bool, checkTargetsType: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        unsafe fn canUseTechU(self: &UnitInterface, tech: TechType, target: *mut UnitInterface, checkCanTargetUnit: bool, checkTargetsType: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUseTechWithoutTarget(self: &UnitInterface, tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUseTechUnit(self: &UnitInterface, tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        unsafe fn canUseTechUnitT(self: &UnitInterface, tech: TechType, targetUnit: *mut UnitInterface, checkCanTargetUnit: bool, checkTargetsUnits: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUseTechPosition(self: &UnitInterface, tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canUseTechPositionP(self: &UnitInterface, tech: TechType, target: Position, checkTargetsPositions: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
        fn canPlaceCOP(self: &UnitInterface, checkCommandibility: bool) -> bool;
        fn canPlaceCOPP(self: &UnitInterface, target: TilePosition, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    }
    // endregion

    extern "Rust" {
        type BoxedAIModule<'a>;
    }

    unsafe extern "C++" {
        pub type AIModuleWrapper;
        #[rust_name = "create_ai_module_wrapper"]
        fn createAIModuleWrapper(user_ai: Box<BoxedAIModule>) -> UniquePtr<AIModuleWrapper>;
        #[rust_name = "get_box"]
        fn getBox(self: Pin<&mut AIModuleWrapper>) -> &mut BoxedAIModule;
    }

    extern "Rust" {
        include!("library/src/aim.h");
        // the hack is to create AimBox to create AIModuleWrapper on the C++ side
        unsafe fn hack<'a>() -> &'static BoxedAIModule<'a>;

        unsafe fn on_start(wrapper: Pin<&mut AIModuleWrapper>);
        unsafe fn on_end(wrapper: Pin<&mut AIModuleWrapper>, is_winner: bool);
        unsafe fn on_frame(wrapper: Pin<&mut AIModuleWrapper>);
        unsafe fn on_send_text(wrapper: Pin<&mut AIModuleWrapper>, text: &CxxString);
        unsafe fn on_receive_text(wrapper: Pin<&mut AIModuleWrapper>, player: *const PlayerInterface, text: &CxxString);
        unsafe fn on_player_left(wrapper: Pin<&mut AIModuleWrapper>, player: *const PlayerInterface);
        unsafe fn on_nuke_detect(wrapper: Pin<&mut AIModuleWrapper>, target: Position);
        unsafe fn on_unit_discover(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_evade(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_show(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_hide(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_create(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_destroy(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_morph(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_renegade(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_save_game(wrapper: Pin<&mut AIModuleWrapper>, game_name: &CxxString);
        unsafe fn on_unit_complete(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
    }
} // pub mod ffi

//region ----------- Shims to the bw::ai_module::AIModule trait ------------
fn on_start(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    wrapper.get_box().on_event(Event::OnStart());
}
fn on_end(wrapper: Pin<&mut ffi::AIModuleWrapper>, is_winner: bool) {
    wrapper.get_box().on_event(Event::OnEnd(is_winner));
}
fn on_frame(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    wrapper.get_box().on_event(Event::OnFrame());
}
fn on_send_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, text: &CxxString) {
    wrapper.get_box().on_event(Event::OnSendText(text.to_string()));
}
fn on_receive_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface, text: &CxxString) {
    let player = unsafe { crate::bw::player::Player::from_raw(player) };
    wrapper.get_box().on_event(Event::OnReceiveText(player, text.to_string()));
}
fn on_player_left(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface) {
    let player = unsafe { crate::bw::player::Player::from_raw(player) };
    wrapper.get_box().on_event(Event::OnPlayerLeft(player));
}
fn on_nuke_detect(wrapper: Pin<&mut ffi::AIModuleWrapper>, target: ffi::Position) {
    let target = crate::bw::position::Position { x: target.x, y: target.y };
    wrapper.get_box().on_event(Event::OnNukeDetect(target));
}
fn on_unit_discover(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitDiscover(unit));
}
fn on_unit_evade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitEvade(unit));
}
fn on_unit_show(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitShow(unit));
}
fn on_unit_hide(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitHide(unit));
}
fn on_unit_create(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitCreate(unit));
}
fn on_unit_destroy(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitDestroy(unit));
}
fn on_unit_morph(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitMorph(unit));
}
fn on_unit_renegade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitRenegade(unit));
}
fn on_save_game(wrapper: Pin<&mut ffi::AIModuleWrapper>, game_name: &CxxString) {
    wrapper.get_box().on_event(Event::OnSaveGame(game_name.to_string()));
}
fn on_unit_complete(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitComplete(unit));
}
//------------------- endregion -------------------

impl Debug for ffi::AIModuleWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ffi::AIModuleWrapper({:p})", self)
    }
}

// used for testing
pub static HACK_BOX: OnceCell<BoxedAIModule> = OnceCell::new();
fn hack() -> &'static BoxedAIModule<'static> {
    &HACK_BOX.get().unwrap()
}

#[cxx::bridge]
pub mod ffi_test {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        fn cpp_test() -> i32;
    }
}
