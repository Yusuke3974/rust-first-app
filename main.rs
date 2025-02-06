use std::io;

// 数値の合計を計算する関数
fn sum_numbers(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

// 文字列を反転する関数
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// フィボナッチ数列のn番目の値を計算する関数
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    println!("Rustアプリへようこそ！");
    println!("1: 数値の合計を計算\n2: 文字列を反転\n3: フィボナッチ数列の値を取得");
    println!("オプションを選択してください：");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let choice: u32 = input.trim().parse().unwrap_or(0);
    
    match choice {
        1 => {
            println!("カンマ区切りで数値を入力してください (例: 1,2,3,4):");
            let mut numbers_input = String::new();
            io::stdin().read_line(&mut numbers_input).expect("入力エラー");
            let numbers: Vec<i32> = numbers_input.trim()
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            
            let sum = sum_numbers(numbers);
            println!("合計値: {}", sum);
        }
        2 => {
            println!("反転する文字列を入力してください:");
            let mut text_input = String::new();
            io::stdin().read_line(&mut text_input).expect("入力エラー");
            let reversed = reverse_string(text_input.trim());
            println!("反転後の文字列: {}", reversed);
        }
        3 => {
            println!("フィボナッチ数列の何番目を計算しますか？:");
            let mut fib_input = String::new();
            io::stdin().read_line(&mut fib_input).expect("入力エラー");
            let n: u32 = fib_input.trim().parse().unwrap_or(0);
            let result = fibonacci(n);
            println!("フィボナッチ({}) = {}", n, result);
        }
        _ => {
            println!("無効な選択です。");
        }
    }
}
