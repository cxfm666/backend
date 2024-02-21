// 引入标准库中的HashSet
use std::collections::HashSet;

// 引入revolt_quark库中的各种模块和类型
use revolt_quark::{
    models::{
        // 服务器相关的模型，例如分类、服务器的部分信息、系统消息通道等
        server::{Category, FieldsServer, PartialServer, SystemMessageChannels},
        File, Server, User, // 文件、服务器、用户模型
    },
    perms, Db, Error, Permission, Ref, Result, // 权限、数据库、错误、权限、引用、结果类型
};

use rocket::serde::json::Json; // 引入Rocket框架的JSON支持
use serde::{Deserialize, Serialize}; // 引入Serde的序列化与反序列化支持
use validator::Validate; // 引入validator库支持数据验证

/// # 服务器数据
#[derive(Validate, Serialize, Deserialize, JsonSchema)]
pub struct DataEditServer {
    /// 服务器名称
    #[validate(length(min = 1, max = 32))]
    name: Option<String>,
    /// 服务器描述
    #[validate(length(min = 0, max = 1024))]
    description: Option<String>,

    /// 用于图标的附件ID
    icon: Option<String>,
    /// 用于横幅的附件ID
    banner: Option<String>,

    /// 服务器的分类结构
    #[validate]
    categories: Option<Vec<Category>>,
    /// 系统消息配置
    system_messages: Option<SystemMessageChannels>,

    /// 服务器标志的位字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    // 是否这个服务器是限制年龄的
    // nsfw: Option<bool>,
    /// 是否这个服务器是公开的，并且应该出现在[Revolt Discover](https://rvlt.gg)上
    discoverable: Option<bool>,
    /// 是否应该为这个服务器收集分析数据
    ///
    /// 必须启用以便在[Revolt Discover](https://rvlt.gg)上显示。
    analytics: Option<bool>,

    /// 从服务器对象中移除的字段
    #[validate(length(min = 1))]
    remove: Option<Vec<FieldsServer>>,
}

/// # 编辑服务器
///
/// 通过其ID编辑服务器。
#[openapi(tag = "Server Information")]
#[patch("/<target>", data = "<data>")]
pub async fn req(
    db: &Db,
    user: User,
    target: Ref,
    data: Json<DataEditServer>,
) -> Result<Json<Server>> {
    let data = data.into_inner();
    // 验证数据
    data.validate()
        .map_err(|error| Error::FailedValidation { error })?;

    let mut server = target.as_server(db).await?;
    let mut permissions = perms(&user).server(&server);
    // 计算权限
    permissions.calc(db).await?;

    // 检查权限
    if data.name.is_none()
        && data.description.is_none()
        && data.icon.is_none()
        && data.banner.is_none()
        && data.system_messages.is_none()
        && data.categories.is_none()
        // && data.nsfw.is_none()
        && data.flags.is_none()
        && data.analytics.is_none()
        && data.discoverable.is_none()
        && data.remove.is_none()
    {
        return Ok(Json(server));
    } else if data.name.is_some()
        || data.description.is_some()
        || data.icon.is_some()
        || data.banner.is_some()
        || data.system_messages.is_some()
        || data.analytics.is_some()
        || data.remove.is_some()
    {
        // 检查是否有管理服务器的权限
        permissions
            .throw_permission(db, Permission::ManageServer)
            .await?;
    }

    // 如果更改敏感字段，检查我们是否有权限
    if (data.flags.is_some() /*|| data.nsfw.is_some()*/ || data.discoverable.is_some())
        && !user.privileged
    {
        return Err(Error::NotPrivileged);
    }

    // 更改分类需要管理频道的权限
    if data.categories.is_some() {
        permissions
            .throw_permission(db, Permission::ManageChannel)
            .await?;
    }

    let DataEditServer {
        name,
        description,
        icon,
        banner,
        categories,
        system_messages,
        flags,
        // nsfw,
        discoverable,
        analytics,
        remove,
    } = data;

    let mut partial = PartialServer {
        name,
        description,
        categories,
        system_messages,
        flags,
        // nsfw,
        discoverable,
        analytics,
        ..Default::default()
    };

    // 1. 从对象中移除字段
    if let Some(fields) = &remove {
        if fields.contains(&FieldsServer::Banner) {
            if let Some(banner) = &server.banner {
                db.mark_attachment_as_deleted(&banner.id).await?;
            }
        }

        if fields.contains(&FieldsServer::Icon) {
            if let Some(icon) = &server.icon {
                db.mark_attachment_as_deleted(&icon.id).await?;
            }
        }
    }

    // 2. 验证更改
    if let Some(system_messages) = &partial.system_messages {
        for id in system_messages.clone().into_channel_ids() {
            if !server.channels.contains(&id) {
                return Err(Error::NotFound);
            }
        }
    }

    if let Some(categories) = &mut partial.categories {
        let mut channel_ids = HashSet::new();
        for category in categories {
            for channel in &category.channels {
                if channel_ids.contains(channel) {
                    return Err(Error::InvalidOperation);
                }

                channel_ids.insert(channel.to_string());
            }

            category
                .channels
                .retain(|item| server.channels.contains(item));
        }
    }

    // 3. 应用新图标
    if let Some(icon) = icon {
        partial.icon = Some(File::use_server_icon(db, &icon, &server.id).await?);
        server.icon = partial.icon.clone();
    }

    // 4. 应用新横幅
    if let Some(banner) = banner {
        partial.banner = Some(File::use_banner(db, &banner, &server.id).await?);
        server.banner = partial.banner.clone();
    }

    // 应用更改到服务器
    server
        .update(db, partial, remove.unwrap_or_default())
        .await?;

    Ok(Json(server))
}
