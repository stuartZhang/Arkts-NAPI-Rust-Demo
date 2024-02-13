# `Rust`编程开发`ArkTs NAPI`原生模块的例程

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

    |目标|`CPU`架构|设备类型|
    |----|---------|------|
    |模块根目录/libs/arm64-v8a|`64`位`ARM CPU`|真机|
    |模块根目录/libs/armeabi-v7a|`32`位`ARM CPU`|真机|
    |模块根目录/libs/x86_64|`64`位`AMD / Intel CPU`|模拟器|

4. 在`模块根目录/src/main/`目录下，创建`rust`文件夹。
5. 在`模块根目录/src/main/rust`文件夹内，使用`cargo init --lib`命令初始`Cargo Package`工程
6. 基于[ohos-node-bindgen](https://github.com/stuartZhang/node-bindgen)基建，开发【鸿蒙`ArkTs N-API`】原生模块。
7. 编写`build.rs`构建程序，将交叉编译输出的`*.so`文件分别复制到`模块根目录/libs/arm64-v8a`，`模块根目录/libs/armeabi-v7a`和`模块根目录/libs/x86_64`文件夹下。
8. 执行交叉编译指指令

    ```shell
    cargo +nightly build --release -Zbuild-std \
        --target=aarch64-unknown-linux-ohos \
        --target=armv7-unknown-linux-ohos \
        --target=x86_64-unknown-linux-ohos
    ```

9. 交叉编译输出的【链接库】文件名被自动命名为“`lib<Package_Name>.so`”。所以，若`Cargo.toml`定义`[package] name`为`calculator`，那么交叉编译输出的链接库名就是`libcalculator.so`。
10. 于是，在`ArkTs`代码中，就可直接以【链接库】文件名为【`ES Module`模块名】导入原生模块。比如，

    ```typescript
    import calculator from 'libcalculator.so';
    const result = calculator.add(2, 3);
    ```

总得来讲，除了`Rust + N-API`编程门槛着实有点高之外，剩余的工作就不难了！
