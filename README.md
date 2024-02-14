# 【例程】`Rust`编程开发`ArkTs NAPI`原生模块

在该完整例程中，

* 既包含了`DevEco Studio`的`Empty Ability`工程
* 也包括了`Cargo (Library) Package`工程
* 更涵盖了[ohos-node-bindgen](https://github.com/stuartZhang/node-bindgen)的用例代码。
* 最后，`Cargo (Library) Package`工程根目录下的`build.rs`与`post_build.rs`构建程序也能直接在其它同类`Cargo Package`工程内复用。

## 如何使普通的`Empty Ability`工程支持`Rust`原生模块开发

1. 新建/打开`DevEco Studio`工程。
2. 修改**模块级**的`build-profile.json5`文件。比如，`entry/build-profile.json5`文件。在`buildOption`节点下，添加如下配置数组

    ```json
    "externalNativeOptions": {
      "abiFilters": [
        "arm64-v8a",
        "armeabi-v7a",
        "x86_64"
      ]
    }
    ```

3. 在模块根目录下，创建如下三个目录

    |目录|`CPU`架构|设备类型|
    |----|---------|------|
    |模块根目录/libs/arm64-v8a|`64`位`ARM CPU`|真机|
    |模块根目录/libs/armeabi-v7a|`32`位`ARM CPU`|真机|
    |模块根目录/libs/x86_64|`64`位`AMD / Intel CPU`|模拟器|

4. 在`模块根目录/src/main/`目录下，创建`rust`文件夹。
5. 在`模块根目录/src/main/rust`文件夹内，使用`cargo init --lib --name=<包名>`命令初始`Cargo Package`工程
   * 【注意】交叉编译输出链接库的`ABI`格式不是`cdylib`，而是`dylib`。在`Cargo.toml`中，该设置值有些反直觉

        ```toml
        [lib]
        crate-type = ["dylib"]
        ```

   * `DevEco Studio`工程的新目录结构变为

        ```shell
        DevEco Studio 工程根目录
        ├── entry — 模块根目录
        │   ├── libs — 交叉编译输出的 *.so 文件都被复制到下面的子文件夹内
        │   │   ├── arm64-v8a
        │   │   ├── armeabi-v7a
        │   │   └── x86_64
        │   ├── src
        │   │   ├── main
        │   │   │  ├── resources
        │   │   │  ├── ets  — 旧有的 ArkTs 源码目录
        │   │   │  ├── rust — 新建的、专门盛放 Cargo （Lib） Package 工程的目录
        │   │   │  │   ├── Cargo.toml
        │   │   │  │   ├── src — Rust 源码目录
        │   │   │  │   ├── target
        │   │   │  │   │  ├── aarch64-unknown-linux-ohos
        │   │   │  │   │  │  └── release
        │   │   │  │   │  ├── armv7-unknown-linux-ohos
        │   │   │  │   │  │  └── release
        │   │   │  │   │  ├── x86_64-unknown-linux-ohos
        │   │   │  │   │  │  └── release
        ```

6. 依赖[ohos-node-bindgen crate](https://github.com/stuartZhang/node-bindgen)基建，开发【鸿蒙`ArkTs N-API`】原生模块。因为由`ohos-node-bindgen crate`间接依赖的[socket2 crate](https://crates.io/crates/socket2)不兼容【华为-鸿蒙】操作系统，所以，需要
   1. 在`DevEco Studio`工程的平级目录，克隆[stuartZhang/socket2](https://github.com/stuartZhang/socket2)至本地，

        ```shell
        git clone git@github.com:stuartZhang/socket2.git
        ```

   2. 将其切分支至`v0.4.x`

        ```shell
        cd socket2
        git checkout -q v0.4.x
        ```

   3. 在`Cargo.toml`中，局部地重写`Override`依赖图

        ```toml
        [dependencies]
        socket2 = "0.4.10"

        [patch.crates-io]
        socket2 = { path = "../../../../../socket2" }
        ```

7. 在`src`目录下，编写`Rust`业务逻辑处理程序。
8. 安装`cargo-post`工具链增补项，以使用`cargo build`命令支持【后置处理】程序。

    ```shell
    cargo install cargo-post
    ```

9. 编写`build.rs`与`post_build.rs`构建程序，将交叉编译输出的`*.so`文件分别复制到`模块根目录/libs/arm64-v8a`，`模块根目录/libs/armeabi-v7a`和`模块根目录/libs/x86_64`文件夹内。其中，
   * `build.rs`作为编译【前置处理】程序
     * 收集`*.so`文件的位置信息
     * 生成【文件复制】指令
     * 将【文件复制】指令追加写入到指定的`*.cmd / *.sh`文件内。
   * `post_build.rs`作为编译【后置处理】程序
     * 执行【文件复制】脚本程序文件
     * 删除该脚本程序文件
10. 执行交叉编译指指令

    ```shell
    cargo +nightly post build --release -Zbuild-std \
        --target=aarch64-unknown-linux-ohos \
        --target=armv7-unknown-linux-ohos \
        --target=x86_64-unknown-linux-ohos
    ```

11. 交叉编译输出的【链接库】文件名被自动命名为“`lib<包名>.so`”。所以，若`Cargo.toml`定义`[package] name`为`calculator`，那么交叉编译输出的链接库文件名就是`libcalculator.so`。
12. 在`ArkTs`代码中，直接以【链接库】文件名为【`ES Module`模块名】导入原生模块，并执行它的成员方法。

    ```typescript
    import calculator from 'libcalculator.so';
    const result = calculator.add(2, 3);
    ```

总得来讲，除了`Rust + N-API`编程门槛着实有点高之外，剩余的工作就不难了！
