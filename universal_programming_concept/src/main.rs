// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// 이렇게 할 경우 불변성의 문제 떄문에 오류가 발생한다.

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// mut 접두어를 통해 x를 가변성으로 만들어준다.

fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

// shadowing이다. 여기서는 mut 키워드를 안 붙여도 된다.
