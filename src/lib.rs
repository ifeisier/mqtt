#![warn(missing_docs)]
//! MQTT (Message Queuing Telemetry Transport) 是一种轻量级的消息传输协议, 专为资源受限的设备和低带宽、高延迟或不稳定网络条件而设计.
//!
//! 质量服务等级 (QoS): MQTT 支持三种不同的服务质量等级, 以确保消息的可靠传递:
//! - QoS 0: 最多一次传输 (消息发送一次, 不保证到达).
//! - QoS 1: 至少一次传输 (确保消息至少到达一次, 但可能重复).
//! - QoS 2: 仅一次传输 (确保消息到达一次且仅一次).
//!
//! 保留消息 (Retain): 它用于指定消息是否应在消息代理 (broker) 上保留, 一个主题只能有一个保留消息:
//! - true: 将这条消息保留到主题内, 让有客户端订阅这个主题的时候, 服务器会立即发送这条消息到客户端.
//! - false: 不需要保留消息到主题内.

pub mod client;

use anyhow::Result;

/// 一个简单的客户端接口
///
/// 想要获取完整的客户端功能, 可以使用 `get_client` 方法获取.
pub trait Client {
    /// 完整的客户端类型
    type Client;

    /// 发送 MQTT 消息
    ///
    /// # 参数
    /// - topic: 消息主题
    /// - qos: 服务质量 (QoS) 等级
    /// - payload: 消息内容
    ///
    /// # 返回
    /// - Ok: 返回空
    /// - Err: 如果操作失败, 返回错误信息.
    fn publish(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()>;

    /// 发送 MQTT 保留消息
    ///
    /// # 参数
    /// - topic: 消息主题
    /// - qos: 服务质量 (QoS) 等级
    /// - payload: 消息内容
    ///
    /// # 返回
    /// - Ok: 返回空
    /// - Err: 返回错误信息
    fn publish_retain(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()>;

    /// 订阅指定的 MQTT 主题
    ///
    /// # 参数
    /// - topic: 要订阅的主题
    /// - qos: 服务质量 (QoS) 等级
    ///
    /// # 返回
    /// - Ok: 返回空
    /// - Err: 返回错误信息
    fn subscribe(&self, topic: &str, qos: u8) -> Result<()>;

    /// 取消订阅指定的 MQTT 主题
    ///
    /// # 参数
    /// - topic: 要取消订阅的主题
    ///
    /// # 返回
    /// - Ok: 返回空
    /// - Err: 返回错误信息
    fn unsubscribe(&self, topic: &str) -> Result<()>;

    /// 获取完整的 MQTT 客户端
    fn get_client(&self) -> &Self::Client;
}
