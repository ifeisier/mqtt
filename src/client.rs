//! 创建 v3.x 或 v5.0 客户端

use crate::Client;
use anyhow::Result;
use rumqttc::Error;
use std::time::Duration;

/// MQTT v3.x 客户端默认实现
pub struct DefaultClient {
    pub(crate) client: rumqttc::AsyncClient,
}

impl DefaultClient {
    /// 创建新的 v3.x 客户端
    ///
    /// # 参数
    /// - id: 客户端 ID
    /// - host: MQTT 服务器地址
    /// - port: MQTT 服务器端口
    /// - user_name: 用户名
    /// - pass_word: 密码
    ///
    /// # 返回
    /// - Ok: v3.x 客户端 和 EventLoop(发送和接收消息).
    /// - Err: 返回错误信息
    pub fn new(id: &str, host: &str, port: u16) -> (Self, rumqttc::EventLoop) {
        let options = mqtt_options(id, host, port);

        // cap 是异步通道的容量, 数据发送太快可以增加这个数值.
        let (client, event_loop) = rumqttc::AsyncClient::new(options, 30);
        (DefaultClient { client }, event_loop)
    }

    /// 创建新的 v3.x 客户端
    ///
    /// 可以指定用户名和密码.
    ///
    /// # 参数
    /// - id: 客户端 ID
    /// - host: MQTT 服务器地址
    /// - port: MQTT 服务器端口
    /// - user_name: 用户名
    /// - pass_word: 密码
    ///
    /// # 返回
    /// - Ok: v3.x 客户端 和 EventLoop(发送和接收消息).
    /// - Err: 返回错误信息
    pub fn new_username_password(
        id: &str,
        host: &str,
        port: u16,
        user_name: &str,
        pass_word: &str,
    ) -> (Self, rumqttc::EventLoop) {
        let mut options = mqtt_options(id, host, port);
        options.set_credentials(user_name, pass_word);
        let (client, event_loop) = rumqttc::AsyncClient::new(options, 30);
        (DefaultClient { client }, event_loop)
    }
}

impl Client for DefaultClient {
    type Client = rumqttc::AsyncClient;

    fn publish(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()> {
        let qos = rumqttc::mqttbytes::qos(qos)?;

        // publish 和 try_publish 的区别
        // publish: 如果有界队列满了, 那么就会阻塞
        // try_publish: 如果有界队列满了, 那么就会返回错误
        self.client.try_publish(topic, qos, false, payload)?;
        Ok(())
    }

    fn publish_retain(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()> {
        let qos = rumqttc::mqttbytes::qos(qos)?;
        self.client.try_publish(topic, qos, true, payload)?;
        Ok(())
    }

    fn subscribe(&self, topic: &str, qos: u8) -> Result<()> {
        let qos = rumqttc::mqttbytes::qos(qos)?;
        self.client.try_subscribe(topic, qos)?;
        Ok(())
    }

    fn unsubscribe(&self, topic: &str) -> Result<()> {
        self.client.try_unsubscribe(topic)?;
        Ok(())
    }

    fn get_client(&self) -> &Self::Client {
        &self.client
    }
}

/// MQTT v5.0 客户端默认实现
pub struct DefaultV5Client {
    pub(crate) client: rumqttc::v5::AsyncClient,
}

impl DefaultV5Client {
    /// 创建新的 v5.0 客户端
    ///
    /// # 参数
    /// - id: 客户端 ID
    /// - host: MQTT 服务器地址
    /// - port: MQTT 服务器端口
    /// - user_name: 用户名
    /// - pass_word: 密码
    ///
    /// # 返回
    /// - Ok: v5.0 客户端 和 EventLoop(发送和接收消息).
    /// - Err: 返回错误信息
    pub fn new(id: &str, host: &str, port: u16) -> (Self, rumqttc::v5::EventLoop) {
        let options = mqtt_options_v5(id, host, port);
        let (client, event_loop) = rumqttc::v5::AsyncClient::new(options, 30);
        (DefaultV5Client { client }, event_loop)
    }

    /// 创建新的 v5.0 客户端
    ///
    /// # 参数
    /// - id: 客户端 ID
    /// - host: MQTT 服务器地址
    /// - port: MQTT 服务器端口
    /// - user_name: 用户名
    /// - pass_word: 密码
    ///
    /// # 返回
    /// - Ok: v5.0 客户端 和 EventLoop(发送和接收消息).
    /// - Err: 返回错误信息
    pub fn new_username_password(
        id: &str,
        host: &str,
        port: u16,
        user_name: &str,
        pass_word: &str,
    ) -> (Self, rumqttc::v5::EventLoop) {
        let mut options = mqtt_options_v5(id, host, port);
        options.set_credentials(user_name, pass_word);
        let (client, event_loop) = rumqttc::v5::AsyncClient::new(options, 30);
        (DefaultV5Client { client }, event_loop)
    }
}

impl Client for DefaultV5Client {
    type Client = rumqttc::v5::AsyncClient;

    fn publish(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()> {
        let qos = qos_v5(qos)?;
        self.client.try_publish(topic, qos, false, payload)?;
        Ok(())
    }

    fn publish_retain(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()> {
        let qos = qos_v5(qos)?;
        self.client.try_publish(topic, qos, true, payload)?;
        Ok(())
    }

    fn subscribe(&self, topic: &str, qos: u8) -> Result<()> {
        let qos = qos_v5(qos)?;
        self.client.try_subscribe(topic, qos)?;
        Ok(())
    }

    fn unsubscribe(&self, topic: &str) -> Result<()> {
        self.client.try_unsubscribe(topic)?;
        Ok(())
    }

    fn get_client(&self) -> &Self::Client {
        &self.client
    }
}

/// 创建 v3 的 mqtt_options
///
/// # 参数
/// - id: 客户端 ID
/// - host: MQTT 服务器地址
/// - port: MQTT 服务器端口
fn mqtt_options(id: &str, host: &str, port: u16) -> rumqttc::MqttOptions {
    let mut options = rumqttc::MqttOptions::new(id, host, port);
    options.set_keep_alive(Duration::from_secs(15));
    options.set_clean_session(true);
    options.set_max_packet_size(1048576, 1048576); // 1048576Byte = 1MBßß
    options
}

/// 创建 v5.0 的 mqtt_options
///
/// # 参数
/// - id: 客户端 ID
/// - host: MQTT 服务器地址
/// - port: MQTT 服务器端口
fn mqtt_options_v5(id: &str, host: &str, port: u16) -> rumqttc::v5::MqttOptions {
    let mut options = rumqttc::v5::MqttOptions::new(id, host, port);
    options.set_keep_alive(Duration::from_secs(15));
    options.set_clean_start(true);
    options.set_connection_timeout(30);
    options.set_max_packet_size(Some(1048576));
    options
}

/// 判断和返回 v5.0 的 qos
fn qos_v5(qos: u8) -> Result<rumqttc::v5::mqttbytes::QoS> {
    Ok(match qos {
        0 => Ok(rumqttc::v5::mqttbytes::QoS::AtMostOnce),
        1 => Ok(rumqttc::v5::mqttbytes::QoS::AtLeastOnce),
        2 => Ok(rumqttc::v5::mqttbytes::QoS::ExactlyOnce),
        qos => Err(Error::InvalidQoS(qos)),
    }?)
}
