use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

// 사용자 입력을 받고 결과값을 표시하기 위해서는 io(input/output) 라이브러리를 스코프로 가져와야한다. io 라이브러리는 std라고 불리는 표준 라이브러리에 있다.

// 러스트의 모든 변수는 기본적으로 불변이기 때문에 mut를 붙여서 가변적으로 만들어 준다. -> 따라서의 let mut guess는 guess라는 이름의 가변변수이다. 
// guess는 함수 String:new의 결과값인 새로운 String 인스턴스가 묶이는 대상이 된다.

// ::new의 ::은 new가 String 타입의 연관함수 임을 나타낸다. 
// new는 새로운 빈 String을 생성한다. 

// 요약하자면 let mut guess = String::new();라인은 새로운 빈 String 인스턴스와 연결된 가변변수를 생성한다는 의미이다. 


// 만약 코드 시작점에 use std:io;가 없다면 함수 호출 시 std::io::stdin 처럼 작성해야 한다. 
// stdin 함수는 터미널의 표준 입력의 handle의 타입인 std:io::stdin의 인스턴스를 되돌려 준다. 

// .read_line(&mut guess)는 사용자로부터 입력을 받기 위해 표준 입력 핸들에서 .read_line(&mut guess) 메소드를 호출한다. 
// 또한 read_line에 &mut guess를 인자로 하나 넘긴다. 

// read_line은 사용자가 표준 입력에 입력할 때마다 입력된 문자들을 하나의 문자열에 저장하므로 인자로 값을 저장할 문자열이 필요하다. -> 입력마다 변경되므로 가변적이여야 한다. 

// &는 코드의 여러 부분에서 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자이다. -> 참조자를 사용함으로써 안전성과 용이성이 생긴다. 

// .expect는 프로그램을 종료하고 메시지를 출력한다.. -> 없어도 컴파일은 되지만 경고가 나타난다. 

// placeholder를 이용한 값 출력 => println!("You guessed : {}", guess);