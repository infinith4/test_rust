fn main() {
    let mut x = 5;  //変数名の前にmutキーワードを付けることで、可変にできるわけです。
    const MAX_POINTS: u32 = 100_000;  //定数はletキーワードの代わりに、constキーワードで宣言し、値の型は必ず注釈しなければなりません。
    println!("The value of x is: {}. const: {}", x, MAX_POINTS);    // xの値は{}です
    x = 6;  //このエラーは、エラーの原因が不変変数xに2回代入できないであると示しています。不変なxという変数に別の値を代入しようとしたからです。
    
    println!("The value of x is: {}", x);

    {
        let y = 5;

        let y = y + 1;

        {
            let y = y * 2;
            println!("The value of y in the inner scope is: {}", y);
        }

        println!("The value of y is: {}", y);
    }

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}",spaces);
    //この文法要素は、容認されます。というのも、最初のspaces変数は文字列型であり、2番目のspaces変数は、 たまたま最初の変数と同じ名前になったまっさらな変数のわけですが、数値型になるからです。故に、シャドーイングのおかげで、 異なる名前を思いつく必要がなくなるわけです。spaces_strとspaces_numなどですね; 代わりに、 よりシンプルなspacesという名前を再利用できるわけです。

    // let mut spaces = "   ";
    // spaces = spaces.len();
    let tup: (i32, f64, u8) = (500, 6.4, 1);


    let p = Point { x: 1, y: 2 };
    //println!("{}", p);    // => (1, 2)
    println!("{:?}", tup);
    //{} では fmt::Display の実装が使われ
    //{:?} では fmt::Debug の実装が使われる

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

}

// 構造体の宣言
struct Point {
    x: i32,
    y: i32,
}

// // fmt::Display トレイトの実装
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "({}, {})", self.x, self.y)
//     }
// }