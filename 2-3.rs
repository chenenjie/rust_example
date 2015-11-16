use std::mem;
//打印数组第一个元素和数组长度的方法
//获取数组不变的引用
fn analyze_slice(slice: &[i32]){
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main(){
    //初始化数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    //数组的随机访问
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    //数组的长度
    println!("array size: {}", xs.len());

    //xs数组在栈上分配的内存大小
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    //将整个数组引用作为参数
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    //将数组的某个片段作为参数
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    //数组只传了4个元素 访问第五个会yue界
    println!("{}", xs[5]);
}
