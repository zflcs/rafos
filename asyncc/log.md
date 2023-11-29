
### 异步运行时控制器开发日志


#### 20231125

- 定义 status 寄存器，记录控制流变化的原因
  - mode 字段
  - code 字段
  - cause 方法
  - set_cause 方法
- 定义 message 寄存器，记录内存中消息缓冲区的起始地址
  - 在标准库环境下测试 heapless 库中的 mpmc queue 队列，用它来表示内存中的缓冲区
  - set_msgbuf 方法
  - get_msgqueue 方法

#### 20231124

- 定义 eptr 寄存器，用来指向内存中 `Executor` 数据结构
- 定义 Asyncc 结构（目前只有 base_addr 字段），其成员函数
  - new
  - reset
  - get_executor
- 软件模拟硬件行为
  - 根据 eptr 寄存器，找到 `Executor` 结构
