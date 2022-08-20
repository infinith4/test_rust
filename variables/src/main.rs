fn main() {
    let mut x = 5;  //変数名の前にmutキーワードを付けることで、可変にできるわけです。
    println!("The value of x is: {}", x);;     // xの値は{}です
    x = 6;  //このエラーは、エラーの原因が不変変数xに2回代入できないであると示しています。不変なxという変数に別の値を代入しようとしたからです。
    
    println!("The value of x is: {}", x);
}