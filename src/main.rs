
use rand::Rng;

fn main() {
    println!("Selection Sort");

    //配列を用意する
    const MIN : i32 = -1000;
    const MAX : i32 = 1000;
    const SIZE : usize = 100;
    const ARRAY_NUM : usize = SIZE;
    let mut a: [i32; ARRAY_NUM] = [1; ARRAY_NUM];

    for n in 0..ARRAY_NUM{
        let rand = rand::thread_rng().gen_range(MIN, MAX + 1);
        a[n] = rand;
        println!("n{} = {}", n, a[n]);
    }

    //選択ソート
    for i in 0..ARRAY_NUM -1{
        let mut min = i;
            for j in i + 1{
                j < ARRAY_NUM; j+;
                if (a[j] < a[min]){
                    min = j;
                }
            }
    }

    //結果
    for n in 0..ARRAY_NUM{
        println!("Result a_{} = {}", n, a[n]);
    }
}
