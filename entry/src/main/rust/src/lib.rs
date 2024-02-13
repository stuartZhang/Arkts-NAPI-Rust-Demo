use ::ohos_node_bindgen::derive::ohos_node_bindgen;

#[ohos_node_bindgen]
fn add(first: i32, second: i32) -> i32 {
    return first + second
}
