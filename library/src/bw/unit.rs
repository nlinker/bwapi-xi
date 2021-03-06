use crate::bw::can_do::{
    CommandFlags, CommandGroupedFlags, Common3pFlags, Common3uFlags, Common4igFlags, Common4piFlags, Common4tiFlags,
    Common4uiFlags,
};
use crate::bw::order::Order;
use crate::bw::player::Player;
use crate::bw::position::{Position, TilePosition};
use crate::bw::region::Region;
use crate::bw::tech_type::TechType;
use crate::bw::unit_command::{UnitCommand, UnitCommandType};
use crate::bw::unit_type::UnitType;
use crate::bw::unitset::Unitset;
use crate::bw::upgrade_type::UpgradeType;
use crate::bw::weapon_type::WeaponType;
use crate::bw::{with_unit_filter, Handle};
use crate::{ffi, FromRaw};
use std::pin::Pin;
use std::ptr::{null_mut, NonNull};

#[derive(Debug, Clone)]
pub struct Unit {
    pub(crate) raw: NonNull<ffi::UnitInterface>,
}

impl FromRaw<ffi::UnitInterface> for Unit {
    unsafe fn from_raw(raw: *mut ffi::UnitInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

impl Unit {
    pub fn exists(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.exists()
    }
    pub fn get_acid_spore_count(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getAcidSporeCount()
    }
    pub fn get_addon(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getAddon())
    }
    pub fn get_air_weapon_cooldown(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getAirWeaponCooldown()
    }
    pub fn get_angle(&self) -> f64 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getAngle()
    }
    pub fn get_bottom(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getBottom()
    }
    pub fn get_build_type(&self) -> UnitType {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getBuildType()
    }
    pub fn get_build_unit(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getBuildUnit())
    }
    pub fn get_carrier(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getCarrier())
    }
    pub fn get_closest_unit(&self, unit_fn: impl Fn(Unit) -> bool + 'static, radius: i32) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let u = with_unit_filter(unit_fn, |uf| ffi::_unit_getClosestUnit(x, uf, radius));
        Unit::option(u)
    }
    pub fn get_defense_matrix_points(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getDefenseMatrixPoints()
    }
    pub fn get_defense_matrix_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getDefenseMatrixTimer()
    }
    pub fn get_distance_p(&self, target: Position) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getDistanceP(target)
    }
    pub fn get_distance_u(&self, target: &Unit) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        unsafe { x.getDistanceU(target.raw.as_ptr()) }
    }
    pub fn get_energy(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getEnergy()
    }
    pub fn get_ensnare_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getEnsnareTimer()
    }
    pub fn get_ground_weapon_cooldown(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getGroundWeaponCooldown()
    }
    pub fn get_hatchery(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getHatchery())
    }
    pub fn get_hit_points(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getHitPoints()
    }
    pub fn get_id(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getID()
    }
    pub fn get_initial_hit_points(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getInitialHitPoints()
    }
    pub fn get_initial_position(&self) -> Position {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getInitialPosition()
    }
    pub fn get_initial_resources(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getInitialResources()
    }
    pub fn get_initial_tile_position(&self) -> TilePosition {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getInitialTilePosition()
    }
    pub fn get_initial_type(&self) -> UnitType {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getInitialType()
    }
    pub fn get_interceptor_count(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getInterceptorCount()
    }
    pub fn get_interceptors(&self) -> Unitset {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let set = ffi::_unit_getInterceptors(x);
        Unitset {
            raw: Handle::Owned(set),
        }
    }
    pub fn get_irradiate_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getIrradiateTimer()
    }
    pub fn get_kill_count(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getKillCount()
    }
    pub fn get_larva(&self) -> Unitset {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let set = ffi::_unit_getLarva(x);
        Unitset {
            raw: Handle::Owned(set),
        }
    }
    pub fn get_last_attacking_player(&self) -> Option<Player> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Player::option(x.getLastAttackingPlayer())
    }
    pub fn get_last_command(&self) -> UnitCommand {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getLastCommand()
    }
    pub fn get_last_command_frame(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getLastCommandFrame()
    }
    pub fn get_left(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getLeft()
    }
    pub fn get_loaded_units(&self) -> Unitset {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let set = ffi::_unit_getLoadedUnits(x);
        Unitset {
            raw: Handle::Owned(set),
        }
    }
    pub fn get_lockdown_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getLockdownTimer()
    }
    pub fn get_maelstrom_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getMaelstromTimer()
    }
    pub fn get_nydus_exit(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getNydusExit())
    }
    pub fn get_order(&self) -> Order {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getOrder()
    }
    pub fn get_order_target(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getOrderTarget())
    }
    pub fn get_order_target_position(&self) -> Position {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getOrderTargetPosition()
    }
    pub fn get_order_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getOrderTimer()
    }
    pub fn get_plague_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getPlagueTimer()
    }
    pub fn get_player(&self) -> Player {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        unsafe { Player::from_raw(x.getPlayer()) }
    }
    pub fn get_position(&self) -> Position {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getPosition()
    }
    pub fn get_power_up(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getPowerUp())
    }
    pub fn get_rally_position(&self) -> Option<Position> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let p = x.getRallyPosition();
        // Position None{32000 / POSITION_SCALE, 32032 / POSITION_SCALE}
        let none = Position { x: 32000, y: 32032 };
        if p == none {
            None
        } else {
            Some(p)
        }
    }
    pub fn get_rally_unit(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getRallyUnit())
    }
    pub fn get_region(&self) -> Option<Region> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Region::option(x.getRegion())
    }
    pub fn get_remaining_build_time(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getRemainingBuildTime()
    }
    pub fn get_remaining_research_time(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getRemainingResearchTime()
    }
    pub fn get_remaining_train_time(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getRemainingTrainTime()
    }
    pub fn get_remaining_upgrade_time(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getRemainingUpgradeTime()
    }
    pub fn get_remove_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getRemoveTimer()
    }
    pub fn get_replay_id(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getReplayID()
    }
    pub fn get_resource_group(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getResourceGroup()
    }
    pub fn get_resources(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getResources()
    }
    pub fn get_right(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getRight()
    }
    pub fn get_scarab_count(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getScarabCount()
    }
    pub fn get_secondary_order(&self) -> Order {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getSecondaryOrder()
    }
    pub fn get_shields(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getShields()
    }
    pub fn get_space_remaining(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getSpaceRemaining()
    }
    pub fn get_spell_cooldown(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getSpellCooldown()
    }
    pub fn get_spider_mine_count(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getSpiderMineCount()
    }
    pub fn get_stasis_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getStasisTimer()
    }
    pub fn get_stim_timer(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getStimTimer()
    }
    pub fn get_target(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getTarget())
    }
    pub fn get_target_position(&self) -> Position {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getTargetPosition()
    }
    pub fn get_tech(&self) -> TechType {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getTech()
    }
    pub fn get_tile_position(&self) -> TilePosition {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getTilePosition()
    }
    pub fn get_top(&self) -> i32 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getTop()
    }
    pub fn get_training_queue(&self) -> Vec<UnitType> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        ffi::_unit_getTrainingQueue(x)
    }
    pub fn get_transport(&self) -> Option<Unit> {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        Unit::option(x.getTransport())
    }
    pub fn get_type(&self) -> UnitType {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getType()
    }
    pub fn get_units_in_radius(&self, radius: i32, unit_fn: impl Fn(Unit) -> bool + 'static) -> Unitset {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let set = with_unit_filter(unit_fn, |uf| ffi::_unit_getUnitsInRadius(x, radius, uf));
        Unitset {
            raw: Handle::Owned(set),
        }
    }
    pub fn get_units_in_weapon_range(&self, weapon: WeaponType, unit_fn: impl Fn(Unit) -> bool + 'static) -> Unitset {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let set = with_unit_filter(unit_fn, |uf| ffi::_unit_getUnitsInWeaponRange(x, weapon, uf));
        Unitset {
            raw: Handle::Owned(set),
        }
    }
    pub fn get_upgrade(&self) -> UpgradeType {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getUpgrade()
    }
    pub fn get_velocity_x(&self) -> f64 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getVelocityX()
    }
    pub fn get_velocity_y(&self) -> f64 {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.getVelocityY()
    }
    pub fn has_nuke(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.hasNuke()
    }
    pub fn has_path_p(&self, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.hasPathP(target)
    }
    pub fn has_path_u(&self, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        unsafe { x.hasPathU(target.raw.as_ptr()) }
    }
    pub fn is_accelerating(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isAccelerating()
    }
    pub fn is_attack_frame(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isAttackFrame()
    }
    pub fn is_attacking(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isAttacking()
    }
    pub fn is_being_constructed(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isBeingConstructed()
    }
    pub fn is_being_gathered(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isBeingGathered()
    }
    pub fn is_being_healed(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isBeingHealed()
    }
    pub fn is_blind(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isBlind()
    }
    pub fn is_braking(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isBraking()
    }
    pub fn is_burrowed(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isBurrowed()
    }
    pub fn is_carrying_gas(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isCarryingGas()
    }
    pub fn is_carrying_minerals(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isCarryingMinerals()
    }
    pub fn is_cloaked(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isCloaked()
    }
    pub fn is_completed(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isCompleted()
    }
    pub fn is_constructing(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isConstructing()
    }
    pub fn is_defense_matrixed(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isDefenseMatrixed()
    }
    pub fn is_detected(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isDetected()
    }
    pub fn is_ensnared(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isEnsnared()
    }
    pub fn is_flying(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isFlying()
    }
    pub fn is_following(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isFollowing()
    }
    pub fn is_gathering_gas(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isGatheringGas()
    }
    pub fn is_gathering_minerals(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isGatheringMinerals()
    }
    pub fn is_hallucination(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isHallucination()
    }
    pub fn is_holding_position(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isHoldingPosition()
    }
    pub fn is_idle(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isIdle()
    }
    pub fn is_interruptible(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isInterruptible()
    }
    pub fn is_invincible(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isInvincible()
    }
    pub fn is_in_weapon_range(&self, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        unsafe { x.isInWeaponRange(target.raw.as_ptr()) }
    }
    pub fn is_irradiated(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isIrradiated()
    }
    pub fn is_lifted(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isLifted()
    }
    pub fn is_loaded(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isLoaded()
    }
    pub fn is_locked_down(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isLockedDown()
    }
    pub fn is_maelstrommed(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isMaelstrommed()
    }
    pub fn is_morphing(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isMorphing()
    }
    pub fn is_moving(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isMoving()
    }
    pub fn is_parasited(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isParasited()
    }
    pub fn is_patrolling(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isPatrolling()
    }
    pub fn is_plagued(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isPlagued()
    }
    pub fn is_powered(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isPowered()
    }
    pub fn is_repairing(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isRepairing()
    }
    pub fn is_researching(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isResearching()
    }
    pub fn is_selected(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isSelected()
    }
    pub fn is_sieged(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isSieged()
    }
    pub fn is_starting_attack(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isStartingAttack()
    }
    pub fn is_stasised(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isStasised()
    }
    pub fn is_stimmed(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isStimmed()
    }
    pub fn is_stuck(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isStuck()
    }
    pub fn is_targetable(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isTargetable()
    }
    pub fn is_training(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isTraining()
    }
    pub fn is_under_attack(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isUnderAttack()
    }
    pub fn is_under_dark_swarm(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isUnderDarkSwarm()
    }
    pub fn is_under_disruption_web(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isUnderDisruptionWeb()
    }
    pub fn is_under_storm(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isUnderStorm()
    }
    pub fn is_upgrading(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.isUpgrading()
    }
    pub fn is_visible(&self) -> bool {
        self.is_visible_(None)
    }
    pub fn is_visible_(&self, player: Option<&Player>) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        if let Some(player) = player {
            unsafe { x.isVisible(player.raw.as_ptr()) }
        } else {
            unsafe { x.isVisible(null_mut()) }
        }
    }

    pub fn issue_command(&self, command: UnitCommand) -> bool {
        let x: Pin<&mut ffi::UnitInterface> = unsafe { Pin::new_unchecked(&mut *self.raw.as_ptr()) };
        x.issueCommand(command)
    }
    pub fn attack_position(&self, target: Position) -> bool {
        self.issue_command(UnitCommand::attack_move(self, target, false))
    }
    pub fn attack_unit(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::attack_unit(self, target, false))
    }
    pub fn build(&self, unit_type: UnitType, target: TilePosition) -> bool {
        self.issue_command(UnitCommand::build(self, target, unit_type))
    }
    pub fn build_addon(&self, unit_type: UnitType) -> bool {
        self.issue_command(UnitCommand::build_addon(&self, unit_type))
    }
    pub fn train(&self, unit_type: UnitType) -> bool {
        self.issue_command(UnitCommand::train(&self, unit_type))
    }
    pub fn morph(&self, unit_type: UnitType) -> bool {
        self.issue_command(UnitCommand::morph(&self, unit_type))
    }
    pub fn research(&self, tech_type: TechType) -> bool {
        self.issue_command(UnitCommand::research(&self, tech_type))
    }
    pub fn upgrade(&self, upgrade_type: UpgradeType) -> bool {
        self.issue_command(UnitCommand::upgrade(&self, upgrade_type))
    }
    pub fn set_rally_position(&self, target: Position) -> bool {
        self.issue_command(UnitCommand::set_rally_position(&self, target))
    }
    pub fn set_rally_unit(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::set_rally_unit(&self, target))
    }
    pub fn move_(&self, target: Position) -> bool {
        self.issue_command(UnitCommand::move_(&self, target, false))
    }
    pub fn patrol(&self, target: Position) -> bool {
        self.issue_command(UnitCommand::patrol(&self, target, false))
    }
    pub fn hold_position(&self) -> bool {
        self.issue_command(UnitCommand::hold_position(&self, false))
    }
    pub fn stop(&self) -> bool {
        self.issue_command(UnitCommand::stop(&self, false))
    }
    pub fn follow(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::follow(&self, target, false))
    }
    pub fn gather(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::gather(&self, target, false))
    }
    pub fn return_cargo(&self) -> bool {
        self.issue_command(UnitCommand::return_cargo(&self, false))
    }
    pub fn repair(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::repair(&self, target, false))
    }
    pub fn burrow(&self) -> bool {
        self.issue_command(UnitCommand::burrow(&self))
    }
    pub fn unburrow(&self) -> bool {
        self.issue_command(UnitCommand::unburrow(&self))
    }
    pub fn cloak(&self) -> bool {
        self.issue_command(UnitCommand::cloak(&self))
    }
    pub fn decloak(&self) -> bool {
        self.issue_command(UnitCommand::decloak(&self))
    }
    pub fn siege(&self) -> bool {
        self.issue_command(UnitCommand::siege(&self))
    }
    pub fn unsiege(&self) -> bool {
        self.issue_command(UnitCommand::unsiege(&self))
    }
    pub fn lift(&self) -> bool {
        self.issue_command(UnitCommand::lift(&self))
    }
    pub fn land(&self, target: TilePosition) -> bool {
        self.issue_command(UnitCommand::land(&self, target))
    }
    pub fn load(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::load(&self, target, false))
    }
    pub fn unload(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::unload(&self, target))
    }
    pub fn unload_all(&self) -> bool {
        self.issue_command(UnitCommand::unload_all(&self, false))
    }
    pub fn unload_all_position(&self, target: Position) -> bool {
        self.issue_command(UnitCommand::unload_all_position(&self, target, false))
    }
    pub fn right_click_position(&self, target: Position) -> bool {
        self.issue_command(UnitCommand::right_click_position(&self, target, false))
    }
    pub fn right_click_unit(&self, target: &Unit) -> bool {
        self.issue_command(UnitCommand::right_click_unit(&self, target, false))
    }
    pub fn halt_construction(&self) -> bool {
        self.issue_command(UnitCommand::halt_construction(&self))
    }
    pub fn cancel_construction(&self) -> bool {
        self.issue_command(UnitCommand::cancel_construction(&self))
    }
    pub fn cancel_addon(&self) -> bool {
        self.issue_command(UnitCommand::cancel_addon(&self))
    }
    pub fn cancel_train(&self) -> bool {
        self.issue_command(UnitCommand::cancel_train(&self))
    }
    pub fn cancel_morph(&self) -> bool {
        self.issue_command(UnitCommand::cancel_morph(&self))
    }
    pub fn cancel_research(&self) -> bool {
        self.issue_command(UnitCommand::cancel_research(&self))
    }
    pub fn cancel_upgrade(&self) -> bool {
        self.issue_command(UnitCommand::cancel_upgrade(&self))
    }
    pub fn use_tech_position(&self, tech_type: TechType, target: Position) -> bool {
        self.issue_command(UnitCommand::use_tech_position(&self, tech_type, target))
    }
    pub fn use_tech_unit(&self, tech_type: TechType, target: &Unit) -> bool {
        self.issue_command(UnitCommand::use_tech_unit(&self, tech_type, target))
    }
    pub fn place_cop(&self, target: TilePosition) -> bool {
        self.issue_command(UnitCommand::place_cop(&self, target))
    }

    pub fn can_issue_command(&self, command: UnitCommand) -> bool {
        self.can_issue_command_(command, CommandFlags::builder().build())
    }
    pub fn can_issue_command_(&self, command: UnitCommand, flags: CommandFlags) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.canIssueCommand(
            command,
            flags.check_can_use_tech_position_on_positions,
            flags.check_can_use_tech_unit_on_units,
            flags.check_can_build_unit_type,
            flags.check_can_target_unit,
            flags.check_can_issue_command_type,
            flags.check_commandibility,
        )
    }

    pub fn can_issue_command_grouped(&self, command: UnitCommand) -> bool {
        self.can_issue_command_grouped_(command, CommandGroupedFlags::builder().build())
    }
    pub fn can_issue_command_grouped_(&self, command: UnitCommand, flags: CommandGroupedFlags) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.canIssueCommandGrouped(
            command,
            flags.check_can_use_tech_position_on_positions,
            flags.check_can_use_tech_unit_on_units,
            flags.check_can_target_unit,
            flags.check_can_issue_command_type,
            flags.check_commandibility_grouped,
            flags.check_commandibility,
        )
    }

    pub fn can_command(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.canCommand()
    }
    pub fn can_command_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility: bool = true;
        x.canCommandGrouped(check_commandibility)
    }
    pub fn can_issue_command_type(&self, command_type: UnitCommandType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canIssueCommandType(command_type, check_commandibility)
    }
    pub fn can_issue_command_type_grouped(&self, command_type: UnitCommandType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canIssueCommandTypeGrouped(command_type, check_commandibility_grouped, check_commandibility)
    }
    pub fn can_target_unit(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        unsafe { x.canTargetUnit(target_unit.raw.as_ptr(), check_commandibility) }
    }
    pub fn can_attack(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canAttack_(check_commandibility)
    }
    pub fn can_attack_position(&self, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        x.canAttackP(
            target,
            flags.check_can_target_unit,
            flags.check_can_issue_command_type,
            flags.check_commandibility,
        )
    }
    pub fn can_attack_unit(&self, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canAttackU(
                target.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_attack_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canAttackGrouped_(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_attack_grouped_position(&self, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4igFlags::builder().build();
        x.canAttackGroupedP(
            target,
            flags.check_can_target_unit,
            flags.check_can_issue_command_type,
            flags.check_commandibility_grouped,
            flags.check_commandibility,
        )
    }
    pub fn can_attack_grouped_unit(&self, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4igFlags::builder().build();
        unsafe {
            x.canAttackGroupedU(
                target.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility_grouped,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_attack_move(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canAttackMove(check_commandibility)
    }
    pub fn can_attack_move_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canAttackMoveGrouped(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_attack_units(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canAttackUnits(check_commandibility)
    }
    pub fn can_attack_unit_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canAttackUnitU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_attack_unit_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canAttackUnitGrouped_(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_attack_unit_grouped_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4igFlags::builder().build();
        unsafe {
            x.canAttackUnitGroupedU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility_grouped,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_build_(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canBuild_(check_commandibility)
    }
    pub fn can_build_t(&self, unit_type: UnitType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canBuildT(unit_type, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_build_tp(&self, unit_type: UnitType, tile: TilePosition) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_target_unit_type = true;
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canBuildTP(
            unit_type,
            tile,
            check_target_unit_type,
            check_can_issue_command_type,
            check_commandibility,
        )
    }
    pub fn can_build_addon(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canBuildAddon_(check_commandibility)
    }
    pub fn can_build_addon_t(&self, unit_type: UnitType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canBuildAddonT(unit_type, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_train(&self, check_commandibility: bool) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.canTrain_(check_commandibility)
    }
    pub fn can_train_t(&self, unit_type: UnitType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canTrainT(unit_type, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_morph(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility: bool = true;
        x.canMorph_(check_commandibility)
    }
    pub fn can_morph_utype(&self, unit_type: UnitType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canMorphT(unit_type, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_research(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canResearch_(check_commandibility)
    }
    pub fn can_research_t(&self, tech_type: TechType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        x.canResearchT(tech_type, check_can_issue_command_type)
    }
    pub fn can_upgrade(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canUpgrade_(check_commandibility)
    }
    pub fn can_upgrade_t(&self, tech_type: UpgradeType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        x.canUpgradeT(tech_type, check_can_issue_command_type)
    }
    pub fn can_set_rally_point(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canSetRallyPoint_(check_commandibility)
    }
    pub fn can_set_rally_point_p(&self, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        x.canSetRallyPointP(
            target,
            flags.check_can_target_unit,
            flags.check_can_issue_command_type,
            flags.check_commandibility,
        )
    }
    pub fn can_set_rally_point_u(&self, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canSetRallyPointU(
                target.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_set_rally_position(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canSetRallyPosition(check_commandibility)
    }
    pub fn can_set_rally_unit(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canSetRallyUnit_(check_commandibility)
    }
    pub fn can_set_rally_unit_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canSetRallyUnitU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_move(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canMove(check_commandibility)
    }
    pub fn can_move_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canMoveGrouped(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_patrol(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canPatrol(check_commandibility)
    }
    pub fn can_patrol_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canPatrolGrouped(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_follow(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canFollow_(check_commandibility)
    }
    pub fn can_follow_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canFollowU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_gather(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canGather_(check_commandibility)
    }
    pub fn can_gather_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canGatherU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_return_cargo(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canReturnCargo(check_commandibility)
    }
    pub fn can_hold_position(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canHoldPosition(check_commandibility)
    }
    pub fn can_stop(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canStop(check_commandibility)
    }
    pub fn can_repair(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canRepair_(check_commandibility)
    }
    pub fn can_repair_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canRepairU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_burrow(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canBurrow(check_commandibility)
    }
    pub fn can_unburrow(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canUnburrow(check_commandibility)
    }
    pub fn can_cloak(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCloak(check_commandibility)
    }
    pub fn can_decloak(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canDecloak(check_commandibility)
    }
    pub fn can_siege(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canSiege(check_commandibility)
    }
    pub fn can_unsiege(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canUnsiege(check_commandibility)
    }
    pub fn can_lift(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canLift(check_commandibility)
    }
    pub fn can_land(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canLand_(check_commandibility)
    }
    pub fn can_land_p(&self, target: TilePosition) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canLandP(target, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_load(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canLoad_(check_commandibility)
    }
    pub fn can_load_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canLoadU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_unload_with_or_without_target(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canUnloadWithOrWithoutTarget(check_commandibility)
    }
    pub fn can_unload_at_position(&self, target_drop_pos: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canUnloadAtPosition(target_drop_pos, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_unload(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canUnload_(check_commandibility)
    }
    pub fn can_unload_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4piFlags::builder().build();
        unsafe {
            x.canUnloadU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_position,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_unload_all(&self, check_commandibility: bool) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.canUnloadAll(check_commandibility)
    }
    pub fn can_unload_all_position(&self, check_commandibility: bool) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        x.canUnloadAllPosition_(check_commandibility)
    }
    pub fn can_unload_all_position_p(&self, target_drop_pos: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canUnloadAllPositionP(target_drop_pos, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_right_click(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canRightClick_(check_commandibility)
    }
    pub fn can_right_click_p(&self, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        x.canRightClickP(
            target,
            flags.check_can_target_unit,
            flags.check_can_issue_command_type,
            flags.check_commandibility,
        )
    }
    pub fn can_right_click_u(&self, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canRightClickU(
                target.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_right_click_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canRightClickGrouped_(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_right_click_grouped_p(&self, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4igFlags::builder().build();
        x.canRightClickGroupedP(
            target,
            flags.check_can_target_unit,
            flags.check_can_issue_command_type,
            flags.check_commandibility_grouped,
            flags.check_commandibility,
        )
    }
    pub fn can_right_click_grouped_u(&self, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4igFlags::builder().build();
        unsafe {
            x.canRightClickGroupedU(
                target.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility_grouped,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_right_click_position(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canRightClickPosition(check_commandibility)
    }
    pub fn can_right_click_position_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canRightClickPositionGrouped(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_right_click_unit(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canRightClickUnit_(check_commandibility)
    }
    pub fn can_right_click_unit_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3uFlags::builder().build();
        unsafe {
            x.canRightClickUnitU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_right_click_unit_grouped(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility_grouped = true;
        let check_commandibility = true;
        x.canRightClickUnitGrouped_(check_commandibility_grouped, check_commandibility)
    }
    pub fn can_right_click_unit_grouped_u(&self, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4igFlags::builder().build();
        unsafe {
            x.canRightClickUnitGroupedU(
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_can_issue_command_type,
                flags.check_commandibility_grouped,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_halt_construction(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canHaltConstruction(check_commandibility)
    }
    pub fn can_cancel_construction(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCancelConstruction(check_commandibility)
    }
    pub fn can_cancel_addon(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCancelAddon(check_commandibility)
    }
    pub fn can_cancel_train(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCancelTrain(check_commandibility)
    }
    pub fn can_cancel_train_slot(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCancelTrainSlot_(check_commandibility)
    }
    pub fn can_cancel_train_slot_i(&self, slot: i32) -> bool {
        // default slot = -2
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canCancelTrainSlotI(slot, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_cancel_morph(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCancelMorph(check_commandibility)
    }
    pub fn can_cancel_research(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCancelResearch(check_commandibility)
    }
    pub fn can_cancel_upgrade(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canCancelUpgrade(check_commandibility)
    }
    pub fn can_use_tech_with_or_without_target(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canUseTechWithOrWithoutTarget_(check_commandibility)
    }
    pub fn can_use_tech_with_or_without_target_t(&self, tech: TechType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canUseTechWithOrWithoutTargetT(tech, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_use_tech_p(&self, tech: TechType, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4tiFlags::builder().build();
        x.canUseTechP(
            tech,
            target,
            flags.check_can_target_unit,
            flags.check_targets_type,
            flags.check_can_issue_command_type,
            flags.check_commandibility,
        )
    }
    pub fn can_use_tech_u(&self, tech: TechType, target: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4tiFlags::builder().build();
        unsafe {
            x.canUseTechU(
                tech,
                target.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_targets_type,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_use_tech_without_target(&self, tech: TechType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canUseTechWithoutTarget(tech, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_use_tech_unit(&self, tech: TechType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canUseTechUnit_(tech, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_use_tech_unit_t(&self, tech: TechType, target_unit: &Unit) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common4uiFlags::builder().build();
        unsafe {
            x.canUseTechUnitT(
                tech,
                target_unit.raw.as_ptr(),
                flags.check_can_target_unit,
                flags.check_targets_units,
                flags.check_can_issue_command_type,
                flags.check_commandibility,
            )
        }
    }
    pub fn can_use_tech_position(&self, tech: TechType) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canUseTechPosition_(tech, check_can_issue_command_type, check_commandibility)
    }
    pub fn can_use_tech_position_p(&self, tech: TechType, target: Position) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let flags = Common3pFlags::builder().build();
        x.canUseTechPositionP(
            tech,
            target,
            flags.check_targets_positions,
            flags.check_can_issue_command_type,
            flags.check_commandibility,
        )
    }
    pub fn can_place_cop(&self) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_commandibility = true;
        x.canPlaceCOP_(check_commandibility)
    }
    pub fn can_place_cop_p(&self, target: TilePosition) -> bool {
        let x: &ffi::UnitInterface = unsafe { self.raw.as_ref() };
        let check_can_issue_command_type = true;
        let check_commandibility = true;
        x.canPlaceCOPP(target, check_can_issue_command_type, check_commandibility)
    }
}
