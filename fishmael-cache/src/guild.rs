
use std::borrow::Cow;

use fishmael_cache_core::{Cacheable, RedisKeyProvider};
use fishmael_cache_derive::RedisFieldProvider;

use twilight_model::guild::{Guild, PartialGuild};

#[derive(RedisFieldProvider, Clone, Debug)]
pub struct CacheableGuild {
    pub afk_timeout: u16,
    pub application_id: Option<u64>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub banner: Option<Vec<u8>>,
    pub channels: Vec<u64>,
    pub default_message_notifications: u8,
    pub description: Option<String>,
    pub discovery_splash: Option<Vec<u8>>,
    pub emojis: Vec<u64>,
    pub explicit_content_filter: u8,
    pub features: Vec<String>,
    // pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub icon: Option<Vec<u8>>,
    pub id: u64,
    pub joined_at: Option<i64>,
    pub large: Option<bool>,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    // pub max_stage_video_channel_users: Option<u64>,
    pub max_video_channel_users: Option<u64>,
    pub member_count: Option<u64>,
    pub members: Vec<u64>,
    pub mfa_level: u8,
    pub name: String,
    pub nsfw_level: u8,
    pub owner_id: u64,
    pub owner: Option<bool>,
    pub permissions: Option<u64>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: u8,
    // pub presences: Vec<Presence>,  // TODO: `guild_id:user_id`` compound id
    pub public_updates_channel_id: Option<u64>,
    pub roles: Vec<u64>,
    pub rules_channel_id: Option<u64>,
    pub safety_alerts_channel_id: Option<u64>,
    pub splash: Option<Vec<u8>>,
    // pub stage_instances: Vec<StageInstance>,
    pub stickers: Vec<u64>,
    pub system_channel_flags: u64,
    pub system_channel_id: Option<u64>,
    pub threads: Vec<u64>,
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: u8,
    // pub voice_states: Vec<VoiceState>,
    pub widget_channel_id: Option<u64>,
    pub widget_enabled: Option<bool>,
}

impl RedisKeyProvider for CacheableGuild {
    fn get_key(&self) -> String {
        format!("guild:{}", self.id)
    }
}

impl Cacheable for CacheableGuild {}

impl From<Guild> for CacheableGuild {
    fn from(value: Guild) -> Self {
        Self {
            afk_timeout: value.afk_timeout.get(),
            application_id: value.application_id.map(Into::into),
            approximate_member_count: value.approximate_member_count,
            approximate_presence_count: value.approximate_presence_count,
            banner: value.banner.map(|b| b.bytes().to_vec()),
            channels: value.channels.iter().map(|c| c.id.into()).collect(),
            default_message_notifications: value.default_message_notifications.into(),
            description: value.description,
            discovery_splash: value.discovery_splash.map(|d| d.bytes().to_vec()),
            emojis: value.emojis.iter().map(|e| e.id.into()).collect(),
            explicit_content_filter: value.explicit_content_filter.into(),
            features: value.features.into_iter().map(|f| Cow::from(f).into_owned()).collect(),
            // guild_scheduled_events: value.guild_scheduled_events,
            icon: value.icon.map(|i| i.bytes().to_vec()),
            id: value.id.into(),
            joined_at: value.joined_at.map(|t| t.as_micros()),
            large: Some(value.large),
            max_members: value.max_members,
            max_presences: value.max_presences,
            // max_stage_video_channel_users: value.max_stage_video_channel_users,
            max_video_channel_users: value.max_video_channel_users,
            member_count: value.member_count,
            members: value.members.iter().map(|m| m.user.id.into()).collect(),
            mfa_level: value.mfa_level.into(),
            name: value.name,
            nsfw_level: value.nsfw_level.into(),
            owner: value.owner,
            owner_id: value.owner_id.into(),
            permissions: value.permissions.map(|flag| flag.bits()),
            preferred_locale: value.preferred_locale,
            premium_progress_bar_enabled: value.premium_progress_bar_enabled,
            premium_subscription_count: value.premium_subscription_count,
            premium_tier: value.premium_tier.into(),
            public_updates_channel_id: value.public_updates_channel_id.map(Into::into),
            roles: value.roles.iter().map(|r| r.id.into()).collect(),
            rules_channel_id: value.rules_channel_id.map(Into::into),
            safety_alerts_channel_id: value.safety_alerts_channel_id.map(Into::into),
            splash: value.splash.map(|s| s.bytes().to_vec()),
            system_channel_flags: value.system_channel_flags.bits(),
            stickers: value.stickers.iter().map(|s| s.id.into()).collect(),
            system_channel_id: value.system_channel_id.map(Into::into),
            threads: value.threads.iter().map(|t| t.id.into()).collect(),
            unavailable: value.unavailable,
            vanity_url_code: value.vanity_url_code,
            verification_level: value.verification_level.into(),
            widget_channel_id: value.widget_channel_id.map(Into::into),
            widget_enabled: value.widget_enabled,
        }
    }
}


impl From<PartialGuild> for CacheableGuild {
    fn from(value: PartialGuild) -> Self {
        Self {
            afk_timeout: value.afk_timeout.get(),
            application_id: value.application_id.map(Into::into),
            approximate_member_count: None,
            approximate_presence_count: None,
            banner: value.banner.map(|b| b.bytes().to_vec()),
            channels: Vec::new(),
            default_message_notifications: value.default_message_notifications.into(),
            description: value.description,
            discovery_splash: value.discovery_splash.map(|d| d.bytes().to_vec()),
            emojis: Vec::new(),
            explicit_content_filter: value.explicit_content_filter.into(),
            features: Vec::new(),
            icon: value.icon.map(|i| i.bytes().to_vec()),
            id: value.id.into(),
            joined_at: None,
            large: None,
            max_members: value.max_members,
            max_presences: value.max_presences,
            max_video_channel_users: None,
            member_count: value.member_count,
            members: Vec::new(),            
            mfa_level: value.mfa_level.into(),
            name: value.name,
            nsfw_level: value.nsfw_level.into(),
            owner: value.owner,
            owner_id: value.owner_id.into(),
            permissions: value.permissions.map(|flag| flag.bits()),
            preferred_locale: value.preferred_locale,
            premium_progress_bar_enabled: value.premium_progress_bar_enabled,
            premium_subscription_count: value.premium_subscription_count,
            premium_tier: value.premium_tier.into(),
            public_updates_channel_id: value.public_updates_channel_id.map(Into::into),
            roles: value.roles.iter().map(|r| r.id.into()).collect(),
            rules_channel_id: value.rules_channel_id.map(Into::into),
            safety_alerts_channel_id: None,
            splash: value.splash.map(|s| s.bytes().to_vec()),
            stickers: Vec::new(),
            system_channel_flags: value.system_channel_flags.bits(),
            system_channel_id: value.system_channel_id.map(Into::into),
            threads: Vec::new(),
            unavailable: false,
            vanity_url_code: value.vanity_url_code,
            verification_level: value.verification_level.into(),
            widget_channel_id: value.widget_channel_id.map(Into::into),
            widget_enabled: value.widget_enabled,
        }
    }
}
