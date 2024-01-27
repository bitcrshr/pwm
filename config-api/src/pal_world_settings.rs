#[derive(Debug)]
pub struct PalWorldSettings {
    pub difficulty: String,
    pub day_time_speed_rate: f32,
    pub night_time_speed_rate: f32,
    pub exp_rate: f32,
    pub pal_capture_rate: f32,
    pub pal_spawn_num_rate: f32,
    pub pal_damage_rate_attack: f32,
    pub pal_damage_rate_defense: f32,
    pub player_damage_rate_attack: f32,
    pub player_damage_rate_defense: f32,
    pub player_stomach_decrease_rate: f32,
    pub player_stamina_decrease_rate: f32,
    pub player_auto_hp_regen_rate: f32,
    pub player_auto_hp_regen_rate_in_sleep: f32,
    pub pal_stomach_decrease_rate: f32,
    pub pal_stamina_decrease_rate: f32,
    pub pal_auto_hp_regen_rate: f32,
    pub pal_auto_hp_regen_rate_in_sleep: f32,
    pub build_object_damage_rate: f32,
    pub build_object_deterioration_damage_rate: f32,
    pub collection_drop_rate: f32,
    pub collection_object_hp_rate: f32,
    pub collection_object_respawn_speed_rate: f32,
    pub enemy_drop_item_rate: f32,
    pub death_penalty: String,
    pub b_enable_player_to_player_damage: bool,
    pub b_enable_friendly_fire: bool,
    pub b_enable_invader_enemy: bool,
    pub b_active_unko: bool,
    pub b_enable_aim_assist_pad: bool,
    pub b_enable_aim_assist_keyboard: bool,
    pub drop_item_max_num: i32,
    pub drop_item_max_num_unko: i32,
    pub base_camp_max_num: i32,
    pub base_camp_worker_max_num: i32,
    pub drop_item_alive_max_hours: f32,
    pub b_auto_reset_guild_no_online_players: bool,
    pub auto_reset_guild_time_no_online_players: f32,
    pub guild_player_max_num: i32,
    pub pal_egg_default_hatching_time: f32,
    pub work_speed_rate: f32,
    pub b_is_multiplay: bool,
    pub b_is_pvp: bool,
    pub b_can_pickup_other_guild_death_penalty_drop: bool,
    pub b_enable_non_login_penalty: bool,
    pub b_enable_fast_travel: bool,
    pub b_is_start_location_select_by_map: bool,
    pub b_exist_player_after_logout: bool,
    pub b_enable_defense_other_guild_player: bool,
    pub coop_player_max_num: i32,
    pub server_player_max_num: i32,
    pub server_name: String,
    pub server_description: String,
    pub admin_password: String,
    pub server_password: String,
    pub public_port: i32,
    pub public_ip: String,
    pub rcon_enabled: bool,
    pub rcon_port: i32,
    pub region: String,
    pub b_use_auth: bool,
    pub ban_list_url: String,
}

impl Default for PalWorldSettings {
    fn default() -> Self {
        Self {
            difficulty: "None".to_string(),
            day_time_speed_rate: 1.0,
            night_time_speed_rate: 1.0,
            exp_rate: 1.0,
            pal_capture_rate: 1.0,
            pal_spawn_num_rate: 1.0,
            pal_damage_rate_attack: 1.0,
            pal_damage_rate_defense: 1.0,
            player_damage_rate_attack: 1.0,
            player_damage_rate_defense: 1.0,
            player_stomach_decrease_rate: 1.0,
            player_stamina_decrease_rate: 1.0,
            player_auto_hp_regen_rate: 1.0,
            player_auto_hp_regen_rate_in_sleep: 1.0,
            pal_stomach_decrease_rate: 1.0,
            pal_stamina_decrease_rate: 1.0,
            pal_auto_hp_regen_rate: 1.0,
            pal_auto_hp_regen_rate_in_sleep: 1.0,
            build_object_damage_rate: 1.0,
            build_object_deterioration_damage_rate: 1.0,
            collection_drop_rate: 1.0,
            collection_object_hp_rate: 1.0,
            collection_object_respawn_speed_rate: 1.0,
            enemy_drop_item_rate: 1.0,
            death_penalty: "All".to_string(),
            b_enable_player_to_player_damage: false,
            b_enable_friendly_fire: false,
            b_enable_invader_enemy: true,
            b_active_unko: false,
            b_enable_aim_assist_pad: true,
            b_enable_aim_assist_keyboard: false,
            drop_item_max_num: 3000,
            drop_item_max_num_unko: 100,
            base_camp_max_num: 128,
            base_camp_worker_max_num: 15,
            drop_item_alive_max_hours: 1.0,
            b_auto_reset_guild_no_online_players: false,
            auto_reset_guild_time_no_online_players: 72.0,
            guild_player_max_num: 20,
            pal_egg_default_hatching_time: 72.0,
            work_speed_rate: 1.0,
            b_is_multiplay: false,
            b_is_pvp: false,
            b_can_pickup_other_guild_death_penalty_drop: false,
            b_enable_non_login_penalty: true,
            b_enable_fast_travel: true,
            b_is_start_location_select_by_map: true,
            b_exist_player_after_logout: false,
            b_enable_defense_other_guild_player: false,
            coop_player_max_num: 4,
            server_player_max_num: 32,
            server_name: "Default Palworld Server".to_string(),
            server_description: "".to_string(),
            admin_password: "".to_string(),
            server_password: "".to_string(),
            public_port: 8211,
            public_ip: "".to_string(),
            rcon_enabled: false,
            rcon_port: 25575,
            region: "".to_string(),
            b_use_auth: true,
            ban_list_url: "https://api.palworldgame.com/api/banlist.txt".to_string(),
        }
    }
}

impl std::fmt::Display for PalWorldSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let capbool = |b: bool| if b { "True" } else { "False" };

        write!(f, "[/Script/Pal.PalGameWorldSettings]\nOptionSettings=(")?;
        write!(f, "Difficulty={},", self.difficulty)?;
        write!(f, "DayTimeSpeedRate={:.6},", self.day_time_speed_rate)?;
        write!(f, "NightTimeSpeedRate={:.6},", self.night_time_speed_rate)?;
        write!(f, "ExpRate={:.6},", self.exp_rate)?;
        write!(f, "PalCaptureRate={:.6},", self.pal_capture_rate)?;
        write!(f, "PalSpawnNumRate={:.6},", self.pal_spawn_num_rate)?;
        write!(f, "PalDamageRateAttack={:.6},", self.pal_damage_rate_attack)?;
        write!(
            f,
            "PalDamageRateDefense={:.6},",
            self.pal_damage_rate_defense
        )?;
        write!(
            f,
            "PlayerDamageRateAttack={:.6},",
            self.player_damage_rate_attack
        )?;
        write!(
            f,
            "PlayerDamageRateDefense={:.6},",
            self.player_damage_rate_defense
        )?;
        write!(
            f,
            "PlayerStomachDecreaceRate={:.6},",
            self.player_stomach_decrease_rate
        )?;
        write!(
            f,
            "PlayerStaminaDecreaceRate={:.6},",
            self.player_stamina_decrease_rate
        )?;
        write!(
            f,
            "PlayerAutoHPRegeneRate={:.6},",
            self.player_auto_hp_regen_rate
        )?;
        write!(
            f,
            "PlayerAutoHpRegeneRateInSleep={:.6},",
            self.player_auto_hp_regen_rate_in_sleep
        )?;
        write!(
            f,
            "PalStomachDecreaceRate={:.6},",
            self.pal_stomach_decrease_rate
        )?;
        write!(
            f,
            "PalStaminaDecreaceRate={:.6},",
            self.pal_stamina_decrease_rate
        )?;
        write!(f, "PalAutoHPRegeneRate={:.6},", self.pal_auto_hp_regen_rate)?;
        write!(
            f,
            "PalAutoHpRegeneRateInSleep={:.6},",
            self.pal_auto_hp_regen_rate_in_sleep
        )?;
        write!(
            f,
            "BuildObjectDamageRate={:.6},",
            self.build_object_damage_rate
        )?;
        write!(
            f,
            "BuildObjectDeteriorationDamageRate={:.6},",
            self.build_object_deterioration_damage_rate
        )?;
        write!(f, "CollectionDropRate={:.6},", self.collection_drop_rate)?;
        write!(
            f,
            "CollectionObjectHpRate={:.6},",
            self.collection_object_hp_rate
        )?;
        write!(
            f,
            "CollectionObjectRespawnSpeedRate={:.6},",
            self.collection_object_respawn_speed_rate
        )?;
        write!(f, "EnemyDropItemRate={:.6},", self.enemy_drop_item_rate)?;
        write!(f, "DeathPenalty={},", self.death_penalty)?;
        write!(
            f,
            "bEnablePlayerToPlayerDamage={},",
            capbool(self.b_enable_player_to_player_damage)
        )?;
        write!(
            f,
            "bEnableFriendlyFire={},",
            capbool(self.b_enable_friendly_fire)
        )?;
        write!(
            f,
            "bEnableInvaderEnemy={},",
            capbool(self.b_enable_invader_enemy)
        )?;
        write!(f, "bActiveUNKO={},", capbool(self.b_active_unko))?;
        write!(
            f,
            "bEnableAimAssistPad={},",
            capbool(self.b_enable_aim_assist_pad)
        )?;
        write!(
            f,
            "bEnableAimAssistKeyboard={},",
            capbool(self.b_enable_aim_assist_keyboard)
        )?;
        write!(f, "DropItemMaxNum={},", self.drop_item_max_num)?;
        write!(f, "DropItemMaxNum_UNKO={},", self.drop_item_max_num_unko)?;
        write!(f, "BaseCampMaxNum={},", self.base_camp_max_num)?;
        write!(f, "BaseCampWorkerMaxNum={},", self.base_camp_worker_max_num)?;
        write!(
            f,
            "DropItemAliveMaxHours={:.6},",
            self.drop_item_alive_max_hours
        )?;
        write!(
            f,
            "bAutoResetGuildNoOnlinePlayers={},",
            capbool(self.b_auto_reset_guild_no_online_players)
        )?;
        write!(
            f,
            "AutoResetGuildTimeNoOnlinePlayers={:.6},",
            self.auto_reset_guild_time_no_online_players
        )?;
        write!(f, "GuildPlayerMaxNum={},", self.guild_player_max_num)?;
        write!(
            f,
            "PalEggDefaultHatchingTime={:.6},",
            self.pal_egg_default_hatching_time
        )?;
        write!(f, "WorkSpeedRate={:.6},", self.work_speed_rate)?;
        write!(f, "bIsMultiplay={},", capbool(self.b_is_multiplay))?;
        write!(f, "bIsPvP={},", capbool(self.b_is_pvp))?;
        write!(
            f,
            "bCanPickupOtherGuildDeathPenaltyDrop={},",
            capbool(self.b_can_pickup_other_guild_death_penalty_drop)
        )?;
        write!(
            f,
            "bEnableNonLoginPenalty={},",
            capbool(self.b_enable_non_login_penalty)
        )?;
        write!(
            f,
            "bEnableFastTravel={},",
            capbool(self.b_enable_fast_travel)
        )?;
        write!(
            f,
            "bIsStartLocationSelectByMap={},",
            capbool(self.b_is_start_location_select_by_map)
        )?;
        write!(
            f,
            "bExistPlayerAfterLogout={},",
            capbool(self.b_exist_player_after_logout)
        )?;
        write!(
            f,
            "bEnableDefenseOtherGuildPlayer={},",
            capbool(self.b_enable_defense_other_guild_player)
        )?;
        write!(f, "CoopPlayerMaxNum={},", self.coop_player_max_num)?;
        write!(f, "ServerPlayerMaxNum={},", self.server_player_max_num)?;
        write!(f, "ServerName=\"{}\",", self.server_name)?;
        write!(f, "ServerDescription=\"{}\",", self.server_description)?;
        write!(f, "AdminPassword=\"{}\",", self.admin_password)?;
        write!(f, "ServerPassword=\"{}\",", self.server_password)?;
        write!(f, "PublicPort={},", self.public_port)?;
        write!(f, "PublicIP=\"{}\",", self.public_ip)?;
        write!(f, "RCONEnabled={},", capbool(self.rcon_enabled))?;
        write!(f, "RCONPort={},", self.rcon_port)?;
        write!(f, "Region=\"{}\",", self.region)?;
        write!(f, "bUseAuth={},", capbool(self.b_use_auth))?;
        write!(f, "BanListURL=\"{}\")", self.ban_list_url)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let settings = PalWorldSettings::default();
        let serialize = settings.to_string();

        let expected = "[/Script/Pal.PalGameWorldSettings]\nOptionSettings=(Difficulty=None,DayTimeSpeedRate=1.000000,NightTimeSpeedRate=1.000000,ExpRate=1.000000,PalCaptureRate=1.000000,PalSpawnNumRate=1.000000,PalDamageRateAttack=1.000000,PalDamageRateDefense=1.000000,PlayerDamageRateAttack=1.000000,PlayerDamageRateDefense=1.000000,PlayerStomachDecreaceRate=1.000000,PlayerStaminaDecreaceRate=1.000000,PlayerAutoHPRegeneRate=1.000000,PlayerAutoHpRegeneRateInSleep=1.000000,PalStomachDecreaceRate=1.000000,PalStaminaDecreaceRate=1.000000,PalAutoHPRegeneRate=1.000000,PalAutoHpRegeneRateInSleep=1.000000,BuildObjectDamageRate=1.000000,BuildObjectDeteriorationDamageRate=1.000000,CollectionDropRate=1.000000,CollectionObjectHpRate=1.000000,CollectionObjectRespawnSpeedRate=1.000000,EnemyDropItemRate=1.000000,DeathPenalty=All,bEnablePlayerToPlayerDamage=False,bEnableFriendlyFire=False,bEnableInvaderEnemy=True,bActiveUNKO=False,bEnableAimAssistPad=True,bEnableAimAssistKeyboard=False,DropItemMaxNum=3000,DropItemMaxNum_UNKO=100,BaseCampMaxNum=128,BaseCampWorkerMaxNum=15,DropItemAliveMaxHours=1.000000,bAutoResetGuildNoOnlinePlayers=False,AutoResetGuildTimeNoOnlinePlayers=72.000000,GuildPlayerMaxNum=20,PalEggDefaultHatchingTime=72.000000,WorkSpeedRate=1.000000,bIsMultiplay=False,bIsPvP=False,bCanPickupOtherGuildDeathPenaltyDrop=False,bEnableNonLoginPenalty=True,bEnableFastTravel=True,bIsStartLocationSelectByMap=True,bExistPlayerAfterLogout=False,bEnableDefenseOtherGuildPlayer=False,CoopPlayerMaxNum=4,ServerPlayerMaxNum=32,ServerName=\"Default Palworld Server\",ServerDescription=\"\",AdminPassword=\"\",ServerPassword=\"\",PublicPort=8211,PublicIP=\"\",RCONEnabled=False,RCONPort=25575,Region=\"\",bUseAuth=True,BanListURL=\"https://api.palworldgame.com/api/banlist.txt\")".to_string();

        assert_eq!(serialize, expected)
    }
}
