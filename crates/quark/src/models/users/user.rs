// 尝试从原始值转换的数字枚举
use num_enum::TryFromPrimitive;
// 序列化和反序列化支持
use serde::{Deserialize, Serialize};
// 验证支持
use validator::Validate;

// 引入文件附件模型
use crate::models::attachment::File;

/// 检查布尔值是否为假的实用函数
pub fn if_false(t: &bool) -> bool {
    !t
}

/// 用户与另一个用户（或他们自己）的关系状态
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub enum RelationshipStatus {
    None,
    User,
    Friend,
    Outgoing,
    Incoming,
    Blocked,
    BlockedOther,
}

/// 表示与其他用户当前状态的关系条目
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct Relationship {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: RelationshipStatus,
}

/// 在线状态
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub enum Presence {
    /// 用户在线
    Online,
    /// 用户当前不可用
    Idle,
    /// 用户正在专注/只会收到提及
    Focus,
    /// 用户忙碌/不会收到任何通知
    Busy,
    /// 用户看起来离线
    Invisible,
}

/// 用户的活跃状态
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Validate, Default)]
pub struct UserStatus {
    /// 自定义状态文本
    #[validate(length(min = 1, max = 128))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 当前在线状态选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

/// 用户的个人资料
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
pub struct UserProfile {
    /// 用户资料上的文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 用户资料上可见的背景
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<File>,
}

/// 用户徽章位
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum Badges {
    /// Revolt 开发者
    Developer = 1,
    /// 帮助翻译 Revolt
    Translator = 2,
    /// 财务支持 Revolt
    Supporter = 4,
    /// 梦乡皇帝
    Adelaide = 6,
    /// 负责任地披露了一个安全问题
    ResponsibleDisclosure = 8,
    /// Revolt 创始人
    Founder = 16,
    /// 平台管理员
    PlatformModeration = 32,
    /// 活跃的财务支持者
    ActiveSupporter = 64,
    /// 🦊🦝
    Paw = 128,
    /// 作为2021年前1000名用户之一加入
    EarlyAdopter = 256,
    /// Amogus
    ReservedRelevantJokeBadge1 = 512,
    /// 低分辨率的恶搞脸
    ReservedRelevantJokeBadge2 = 1024,
}

/// 用户标志枚举
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum Flags {
    /// 用户已从平台中暂停
    Suspended = 1,
    /// 用户已删除他们的账户
    Deleted = 2,
    /// 用户已被平台禁止
    Banned = 4,
    /// 用户被标记为垃圾邮件并从平台中移除
    Spam = 8,
}

/// 如果用户是机器人的机器人信息
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct BotInformation {
    /// 该机器人所有者的 Id
    pub owner: String,
}

/// Revolt 上的用户表示。
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, OptionalStruct, Default)]
#[optional_derive(Serialize, Deserialize, Debug, Default, Clone)]
#[optional_name = "PartialUser"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct User {
    /// 唯一 Id
    #[serde(rename = "_id")]
    pub id: String,
    /// 用户名
    pub username: String,
    /// 分隔符
    pub discriminator: String,
    /// 显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 头像附件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<File>,
    /// 与其他用户的关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Relationship>>,

    /// 用户徽章的位域
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: Option<i32>,
    /// 用户当前状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 用户的个人资料页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,

    /// 用户标志枚举
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    /// 此用户是否享有特权
    #[serde(skip_serializing_if = "if_false", default)]
    pub privileged: bool,
    /// 机器人信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<BotInformation>,

    // ? 下面的条目永远不应推送到数据库
    /// 当前会话用户与此用户的关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<RelationshipStatus>,
    /// 此用户当前是否在线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}

/// 用户对象上的可选字段
#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Eq, Clone)]
pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
    DisplayName,
}

/// 提供关于我们正在处理的用户类型提示的枚举
pub enum UserHint {
    /// 可能是用户或机器人
    Any,
    /// 仅匹配机器人
    Bot,
    /// 仅匹配用户
    User,
}
