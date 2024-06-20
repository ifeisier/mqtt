# MQTT

快速创建 rumqttc 客户端.


## 使用

```rust
use std::time::Duration;
use mqtt::Client;
use mqtt::client::DefaultClient;
use tokio::time;

fn main() {
  let runtime = new_current_thread().unwrap();
  runtime.block_on(async {
    let (client, mut event_loop) = DefaultClient::new("id", "192.168.200.217", 1883);
    client.subscribe("hello/rumqtt", 2).unwrap();

    tokio::spawn(async move {
      for i in 0..10 {
        client
        .publish("hello/rumqtt", 2, vec![i; i as usize])
        .unwrap();
        time::sleep(Duration::from_millis(100)).await;
      }
    });

    loop {
      let notification = event_loop.poll().await.unwrap();
      println!("Received = {:?}", notification);
    }
  });
}
```


## 免责声明

本开源项目（以下简称“项目”）按“现状”提供，不附带任何明示或暗示的担保。使用者应自行承担使用本项目的风险。在任何情况下，项目的贡献者和维护者均不对因使用本项目而产生的任何直接、间接、偶然、特殊、惩罚性或后果性损害负责，即使已被告知可能发生此类损害。

具体包括但不限于以下内容：

1. **适用性和可靠性**：
    - 项目不保证其适用于特定目的或适销性。
    - 项目不保证其不会中断、无错误或完全安全。
2. **责任限制**：
    - 项目贡献者不对任何因使用或无法使用项目而导致的损失负责，包括但不限于数据丢失、业务中断或经济损失。
3. **第三方库**：
    - 本项目可能包含第三方库，这些库按各自的许可协议提供。
    - 对于第三方库的使用和由此引起的任何问题，项目贡献者不承担任何责任。
    - 用户有责任阅读并遵守第三方库的许可协议。
4. **更新和维护**：
    - 项目贡献者没有义务更新、维护或支持本项目。项目的更新或功能的添加完全由贡献者自主决定。
5. **用户责任**：
    - 使用本项目的用户有责任确保其使用符合所有适用法律法规。
    - 用户应对其使用本项目产生的任何后果负责，包括对第三方造成的任何损害。

通过使用本项目，您确认已阅读并同意本免责声明的全部条款。如果您不同意这些条款，请不要使用本项目。
