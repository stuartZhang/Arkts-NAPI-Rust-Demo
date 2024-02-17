import calculator from 'libcalculator.so';

@Entry
@Component
struct Index {
  @State message: string = '';
  aboutToAppear(){
    setTimeout(() => {
      this.message = String(calculator.add(2, 3));
    }, 250);
  }
  build() {
    Row() {
      Column() {
        Text('ArkTs N-API + Rust 代码示例')
          .fontSize(30)
          .fontWeight(FontWeight.Bold)
        Text(`由原生模块计算 2 + 3 = ${this.message}`)
          .fontSize(25)
      }
      .width('100%')
    }
    .height('100%')
  }
}