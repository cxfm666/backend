// 使用标准库中的HashMap
use std::collections::HashMap;

// 引入第三方库
use num_enum::TryFromPrimitive; // 用于将整数尝试转换为枚举
use serde::{Deserialize, Serialize}; // 用于序列化和反序列化
use validator::Validate; // 用于验证数据

// 引入项目内的模块或定义
use crate::{models::attachment::File, OverrideField};

/// 实用函数：检查布尔值是否为假
pub fn if_false(t: &bool) -> bool {
    !t
}

/// 代表服务器角色的结构体
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, OptionalStruct, Default)]
// 各种派生宏用于序列化、反序列化、调试打印等
#[optional_derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
// 为部分字段提供默认值或跳过序列化的特性
#[optional_name = "PartialRole"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct Role {
    // 角色名
    pub name: String,
    // 此角色拥有的权限
    pub permissions: OverrideField,
    // 角色的颜色（任何有效的CSS颜色）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    // 是否在成员侧边栏单独显示此角色
    #[serde(skip_serializing_if = "if_false", default)]
    pub hoist: bool,
    // 角色的排名
    #[serde(default)]
    pub rank: i64,
}

/// 频道类别
#[derive(Validate, Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct Category {
    // 此类别的唯一ID
    #[validate(length(min = 1, max = 32))]
    pub id: String,
    // 类别的标题
    #[validate(length(min = 1, max = 32))]
    pub title: String,
    // 此类别中的频道
    pub channels: Vec<String>,
}

/// 系统消息频道分配
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct SystemMessageChannels {
    // 用户加入消息发送的频道ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_joined: Option<String>,
    // 用户离开消息发送的频道ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_left: Option<String>,
    // 用户被踢消息发送的频道ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_kicked: Option<String>,
    // 用户被禁消息发送的频道ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_banned: Option<String>,
}

/// 服务器标志枚举
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum ServerFlags {
    Verified = 1, // 已验证
    Official = 2, // 官方
}

/// 代表Revolt上的服务器
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, OptionalStruct, Default)]
#[optional_derive(Serialize, Deserialize, JsonSchema, Debug, Default, Clone)]
#[optional_name = "PartialServer"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct Server {
    // 唯一Id
    #[serde(rename = "_id")]
    pub id: String,
    // 服务器拥有者的用户ID
    pub owner: String,

    // 服务器名称
    pub name: String,
    // 服务器描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // 服务器内的频道
    pub channels: Vec<String>,
    // 服务器的类别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    // 发送系统事件消息的配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<SystemMessageChannels>,

    // 服务器的角色
    #[serde(
        default = "HashMap::<String, Role>::new",
        skip_serializing_if = "HashMap::<String, Role>::is_empty"
    )]
    pub roles: HashMap<String, Role>,
    // 服务器和频道的默认权限集
    pub default_permissions: i64,

    // 图标附件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<File>,
    // 横幅附件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<File>,

    // 服务器标志的位字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    // 标记服务器是否不适合工作环境
    #[serde(skip_serializing_if = "if_false", default)]
    pub nsfw: bool,
    // 是否启用分析
    #[serde(skip_serializing_if = "if_false", default)]
    pub analytics: bool,
    // 服务器是否应公开可发现
    #[serde(skip_serializing_if = "if_false", default)]
    pub discoverable: bool,
}

/// 服务器对象上的可选字段
#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Eq, Clone)]
pub enum FieldsServer {
    Description,
    Categories,
    SystemMessages,
    Icon,
    Banner,
}

/// 角色对象上的可选字段
#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Eq, Clone)]
pub enum FieldsRole {
    Colour,
}
