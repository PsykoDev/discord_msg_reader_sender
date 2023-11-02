use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub t: String,
    pub s: i64,
    pub op: i64,
    pub d: D,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct D {
    pub guilds: Vec<Guild>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    #[serde(rename = "preferred_locale")]
    pub preferred_locale: String,
    #[serde(rename = "max_stage_video_channel_users")]
    pub max_stage_video_channel_users: i64,
    pub icon: Option<String>,
    #[serde(rename = "system_channel_flags")]
    pub system_channel_flags: i64,
    #[serde(rename = "application_id")]
    pub application_id: Value,
    #[serde(rename = "hub_type")]
    pub hub_type: Value,
    #[serde(rename = "max_members")]
    pub max_members: i64,
    #[serde(rename = "verification_level")]
    pub verification_level: i64,
    #[serde(rename = "afk_timeout")]
    pub afk_timeout: i64,
    pub channels: Vec<Channel>,
    pub id: String,
    #[serde(rename = "premium_tier")]
    pub premium_tier: i64,
    #[serde(rename = "home_header")]
    pub home_header: Option<String>,
    pub roles: Vec<Role>,
    pub name: String,
    pub stickers: Vec<Sticker>,
    #[serde(rename = "guild_scheduled_events")]
    pub guild_scheduled_events: Vec<Value>,
    #[serde(rename = "premium_subscription_count")]
    pub premium_subscription_count: i64,
    pub threads: Vec<Thread>,
    #[serde(rename = "embedded_activities")]
    pub embedded_activities: Vec<Value>,
    #[serde(rename = "nsfw_level")]
    pub nsfw_level: i64,
    #[serde(rename = "safety_alerts_channel_id")]
    pub safety_alerts_channel_id: Option<String>,
    pub emojis: Vec<Emoji>,
    #[serde(rename = "rules_channel_id")]
    pub rules_channel_id: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "max_video_channel_users")]
    pub max_video_channel_users: i64,
    pub lazy: bool,
    #[serde(rename = "afk_channel_id")]
    pub afk_channel_id: Option<String>,
    #[serde(rename = "vanity_url_code")]
    pub vanity_url_code: Option<String>,
    #[serde(rename = "inventory_settings")]
    pub inventory_settings: Option<InventorySettings>,
    pub features: Vec<String>,
    #[serde(rename = "soundboard_sounds")]
    pub soundboard_sounds: Vec<SoundboardSound>,
    pub large: bool,
    #[serde(rename = "discovery_splash")]
    pub discovery_splash: Option<String>,
    #[serde(rename = "voice_states")]
    pub voice_states: Vec<VoiceState>,
    #[serde(rename = "default_message_notifications")]
    pub default_message_notifications: i64,
    #[serde(rename = "latest_onboarding_question_id")]
    pub latest_onboarding_question_id: Option<String>,
    #[serde(rename = "mfa_level")]
    pub mfa_level: i64,
    #[serde(rename = "premium_progress_bar_enabled")]
    pub premium_progress_bar_enabled: bool,
    pub region: String,
    pub members: Vec<Member2>,
    #[serde(rename = "public_updates_channel_id")]
    pub public_updates_channel_id: Option<String>,
    #[serde(rename = "stage_instances")]
    pub stage_instances: Vec<Value>,
    pub nsfw: bool,
    #[serde(rename = "joined_at")]
    pub joined_at: String,
    pub splash: Option<String>,
    pub banner: Option<String>,
    #[serde(rename = "application_command_counts")]
    pub application_command_counts: ApplicationCommandCounts,
    pub version: i64,
    #[serde(rename = "incidents_data")]
    pub incidents_data: Option<IncidentsData>,
    #[serde(rename = "system_channel_id")]
    pub system_channel_id: Option<String>,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    #[serde(rename = "explicit_content_filter")]
    pub explicit_content_filter: i64,
    pub presences: Vec<Presence>,
    #[serde(rename = "member_count")]
    pub member_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub version: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub topic: Option<String>,
    #[serde(rename = "theme_color")]
    pub theme_color: Option<i64>,
    pub status: Option<Value>,
    #[serde(rename = "rate_limit_per_user")]
    pub rate_limit_per_user: Option<i64>,
    pub position: i64,
    #[serde(rename = "permission_overwrites")]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,
    pub nsfw: Option<bool>,
    pub name: String,
    #[serde(rename = "last_pin_timestamp")]
    pub last_pin_timestamp: Option<String>,
    #[serde(rename = "last_message_id")]
    pub last_message_id: Option<String>,
    pub id: String,
    #[serde(rename = "icon_emoji")]
    pub icon_emoji: Option<IconEmoji>,
    pub flags: i64,
    #[serde(rename = "user_limit")]
    pub user_limit: Option<i64>,
    #[serde(rename = "rtc_region")]
    pub rtc_region: Option<String>,
    pub bitrate: Option<i64>,
    #[serde(rename = "default_auto_archive_duration")]
    pub default_auto_archive_duration: Option<i64>,
    #[serde(rename = "default_thread_rate_limit_per_user")]
    pub default_thread_rate_limit_per_user: Option<i64>,
    pub template: Option<String>,
    #[serde(rename = "default_sort_order")]
    pub default_sort_order: Option<i64>,
    #[serde(rename = "default_reaction_emoji")]
    pub default_reaction_emoji: Option<DefaultReactionEmoji>,
    #[serde(rename = "default_forum_layout")]
    pub default_forum_layout: Option<i64>,
    #[serde(rename = "available_tags")]
    #[serde(default)]
    pub available_tags: Vec<AvailableTag>,
    #[serde(rename = "video_quality_mode")]
    pub video_quality_mode: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionOverwrite {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: String,
    pub deny: String,
    pub allow: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconEmoji {
    pub name: String,
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultReactionEmoji {
    #[serde(rename = "emoji_name")]
    pub emoji_name: Option<String>,
    #[serde(rename = "emoji_id")]
    pub emoji_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTag {
    pub name: String,
    pub moderated: bool,
    pub id: String,
    #[serde(rename = "emoji_name")]
    pub emoji_name: Option<String>,
    #[serde(rename = "emoji_id")]
    pub emoji_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub version: i64,
    #[serde(rename = "unicode_emoji")]
    pub unicode_emoji: Option<String>,
    pub tags: Tags,
    pub position: i64,
    pub permissions: String,
    pub name: String,
    pub mentionable: bool,
    pub managed: bool,
    pub id: String,
    pub icon: Option<String>,
    pub hoist: bool,
    pub flags: i64,
    pub color: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    #[serde(rename = "bot_id")]
    pub bot_id: Option<String>,
    #[serde(rename = "premium_subscriber")]
    pub premium_subscriber: Option<Value>,
    #[serde(rename = "guild_connections")]
    pub guild_connections: Option<Value>,
    #[serde(rename = "subscription_listing_id")]
    pub subscription_listing_id: Option<String>,
    #[serde(rename = "integration_id")]
    pub integration_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sticker {
    pub version: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub tags: String,
    pub name: String,
    pub id: String,
    #[serde(rename = "guild_id")]
    pub guild_id: String,
    #[serde(rename = "format_type")]
    pub format_type: i64,
    pub description: Option<String>,
    pub available: bool,
    pub asset: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "total_message_sent")]
    pub total_message_sent: i64,
    #[serde(rename = "thread_metadata")]
    pub thread_metadata: ThreadMetadata,
    #[serde(rename = "rate_limit_per_user")]
    pub rate_limit_per_user: i64,
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    pub name: String,
    #[serde(rename = "message_count")]
    pub message_count: i64,
    #[serde(rename = "member_ids_preview")]
    pub member_ids_preview: Vec<String>,
    #[serde(rename = "member_count")]
    pub member_count: i64,
    pub member: Member,
    #[serde(rename = "last_message_id")]
    pub last_message_id: String,
    pub id: String,
    #[serde(rename = "guild_id")]
    pub guild_id: String,
    pub flags: i64,
    #[serde(rename = "applied_tags")]
    pub applied_tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadMetadata {
    pub locked: bool,
    #[serde(rename = "create_timestamp")]
    pub create_timestamp: String,
    #[serde(rename = "auto_archive_duration")]
    pub auto_archive_duration: i64,
    pub archived: bool,
    #[serde(rename = "archive_timestamp")]
    pub archive_timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub muted: bool,
    #[serde(rename = "mute_config")]
    pub mute_config: Value,
    #[serde(rename = "join_timestamp")]
    pub join_timestamp: String,
    pub flags: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub version: i64,
    pub roles: Vec<String>,
    #[serde(rename = "require_colons")]
    pub require_colons: bool,
    pub name: String,
    pub managed: bool,
    pub id: String,
    pub available: bool,
    pub animated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventorySettings {
    #[serde(rename = "is_emoji_pack_collectible")]
    pub is_emoji_pack_collectible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundboardSound {
    pub volume: f64,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "sound_id")]
    pub sound_id: String,
    pub name: String,
    #[serde(rename = "guild_id")]
    pub guild_id: String,
    #[serde(rename = "emoji_name")]
    pub emoji_name: Option<String>,
    #[serde(rename = "emoji_id")]
    pub emoji_id: Option<String>,
    pub available: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceState {
    #[serde(rename = "user_id")]
    pub user_id: String,
    pub suppress: bool,
    #[serde(rename = "session_id")]
    pub session_id: String,
    #[serde(rename = "self_video")]
    pub self_video: bool,
    #[serde(rename = "self_mute")]
    pub self_mute: bool,
    #[serde(rename = "self_deaf")]
    pub self_deaf: bool,
    #[serde(rename = "request_to_speak_timestamp")]
    pub request_to_speak_timestamp: Value,
    pub mute: bool,
    pub deaf: bool,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "self_stream")]
    pub self_stream: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member2 {
    pub user: User,
    pub roles: Vec<String>,
    #[serde(rename = "premium_since")]
    pub premium_since: Option<String>,
    pub pending: bool,
    pub nick: Option<String>,
    pub mute: bool,
    #[serde(rename = "joined_at")]
    pub joined_at: String,
    pub flags: i64,
    pub deaf: bool,
    #[serde(rename = "communication_disabled_until")]
    pub communication_disabled_until: Option<String>,
    pub avatar: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    #[serde(rename = "public_flags")]
    pub public_flags: i64,
    pub id: String,
    #[serde(rename = "global_name")]
    pub global_name: Option<String>,
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    pub discriminator: String,
    pub bot: bool,
    #[serde(rename = "avatar_decoration_data")]
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    pub avatar: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarDecorationData {
    #[serde(rename = "sku_id")]
    pub sku_id: String,
    pub asset: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationCommandCounts {
    #[serde(rename = "3")]
    pub n3: Option<i64>,
    #[serde(rename = "1")]
    pub n1: Option<i64>,
    #[serde(rename = "2")]
    pub n2: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentsData {
    #[serde(rename = "raid_detected_at")]
    pub raid_detected_at: Value,
    #[serde(rename = "invites_disabled_until")]
    pub invites_disabled_until: Value,
    #[serde(rename = "dms_disabled_until")]
    pub dms_disabled_until: Value,
    #[serde(rename = "dm_spam_detected_at")]
    pub dm_spam_detected_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presence {
    pub user: User2,
    pub status: Option<String>,
    #[serde(rename = "client_status")]
    pub client_status: ClientStatus,
    pub broadcast: Value,
    pub activities: Vec<Activity>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User2 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub web: Option<String>,
    pub mobile: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub state: Option<String>,
    pub name: String,
    pub id: String,
    pub details: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    pub assets: Option<Assets>,
    #[serde(rename = "application_id")]
    pub application_id: Option<String>,
    pub timestamps: Option<Timestamps>,
    pub emoji: Option<Emoji2>,
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    pub buttons: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    #[serde(rename = "large_image")]
    pub large_image: String,
    #[serde(rename = "large_text")]
    pub large_text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamps {
    pub start: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji2 {
    pub name: String,
    pub id: Option<String>,
    pub animated: Option<bool>,
}
