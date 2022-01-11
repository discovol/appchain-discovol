

# Discovol  Pallet 的描述


Discovol 区块链运行时使用以下自定义 Pallet 来处理其业务逻辑

## 内容发现 Pallet 

内容发现 Pallet 处理创建“内容发现”登记和转账的逻辑。

此 Pallet 公开以下外部调用：

### 初始化内容发现 Pallet 设置
```rust
pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
``
### 内容发现创建和转账
```rust
pub fn create_register(origin: OriginFor<T>, hash: Vec<u8>, url: Vec<u8>) -> DispatchResultWithPostInfo {
``


## 内容引用 Pallet 

内容引用 Pallet 处理创建“内容发现”引用和转帐及分成的逻辑。

此 Pallet 公开以下外部调用：

### 初始化内容引用 Pallet 设置
```rust
pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
``
### 更新内容创建和转账及分成
```rust
pub fn create_spread(origin: OriginFor<T>, hash: Vec<u8>, url: Vec<u8>, relation: T::AccountId, score: u8) -> DispatchResultWithPostInfo {
``


## 影响因子 Pallet 

影响因子 Pallet 处理创建、更新 Curator “影响因子”的逻辑。

此 Pallet 公开以下外部调用：

### 初始化影响因子 Pallet 设置
```rust
pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
``
### 影响因子创建和更新
```rust
pub fn transfer(_origin: OriginFor<T>, to: T::AccountId, value: u64) -> DispatchResultWithPostInfo {
``
