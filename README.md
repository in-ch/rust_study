https://rinthel.github.io/rust-lang-book-ko/

[3.1 변수와 가변성](#-3.1-변수와-가변성) 

[3.2 데이터 타입들](#32-데이터-타입들)

[3.3 함수 동작 원리](#33-함수-동작-원리)

[3.4 주석](#34-주석)

[3.5 제어문](#35-제어문)

[4.1 스택과 힙 이해하기](#41-스택과-힙-이해하기)

[4.2 참조자와 빌림](#42-참조자references와-빌림borrowing)

[4.3 슬라이스](#43-슬라이드slices)

[5.1 구조체를 정의하고 초기화하기](#51-구조체를-정의하고-초기화하기)

[5.2 구조체를 이용한 예제 프로그램](#52-구조체를-이용한-예제-프로그램)

[5.3 메소드 문법](#53-메소드-문법)

[6.1 열거형 정의하기](#61-열거형-정의하기)

[6.2 match 흐름 제어 연산자](#62-match-흐름-제어-연산자)

[6.3 if let을 사용한 간결한 흐름 제어](#63-if-let을-사용한-간결한-흐름-제어)

[7.1 mod와 파일 시스템](#71-mod와-파일-시스템)

[7.2 pub으로 가시성 제어하기](#72-pub으로-가시성-제어하기)

[7.3 use로 이름 가져오기](#73-use로-이름-가져오기)

[8.1 벡터](#81-벡터)

[8.2 스트링](#82-스트링)

[8.3 해쉬맵](#83-해쉬맵)

[9.1 panic!과 함께하는 복구 불가능한 에러](#91-panic과-함께하는-복구-불가능한-에러)

[9.2 Result와 함께하는 복구 가능한 에러](#92-result와-함께하는-복구-가능한-에러)

[9.3 panic! 이냐, panic!이 아니냐, 그것이 문제로소이다.](#93-panic이냐-panic이-아니냐-그것이-문제로다)

## 3.1 변수와 가변성
<details>
    <summary>자세히 보기</summary>
    
### 변수와 가변성

> Rust에서 기본 변수는 불변성이다. 변수가 불변성인 경우, 일단 값이 bound되면 해당 값을 변경할 수 없다.
> 만약에 불변성으로 선언한 것의 값을 변경하고자 하는 시도를 하면 컴파일 타임의 에러를 얻게 된다.

```rust
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
```
이렇게 하고 <code>cargo run</code>을 실행시켜보면 다음과 같은 에러를 볼 수 있다.
![스크린샷 2023-06-11 오후 10 05 32](https://github.com/in-ch/rust_study/assets/49556566/edd552db-b141-41e0-ae98-7bf4e9f042fd)
> 에러가 나타나는 이유는 <code>불변성 변수에 재할당</code>이 문제가 된 것이다. 따라서 불변성 변수 <code>x</code>에 두 번째로 값을 할당했기 떄문이다. 
  우리가 이전에 불변성으로 선언한 것의 값을 변경하고자 하는 시도를 하면 컴파일 타임의 에러를 얻게 되고 이로 인해 버그가 발생할 수 있기 때문에 중요하다. 

### mut를 활용해서 가변성 추가하기

> <code>mut</code> 접두어를 통해 가변성 변수를 선언할 수 있다. 이는 변수의 값이 변경을 허용하는 것에 추가로 향후 코드를 보는 사람에게 코드의 다른 부분에서 해당 변수의 값을 변경할 것이라는 의도를 주지시켜준다. 
```rust
    fn main() {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }
```
![스크린샷 2023-06-11 오후 10 19 53](https://github.com/in-ch/rust_study/assets/49556566/451a6e93-50f2-4e49-99e9-07c228802ea8)


### 변수와 상수 간의 차이점들

> 불변성 변수와 마찬가지로 상수 또한 이름으로 bound된 후에는 값의 변경이 허용되지 않지만, 상수와 변수는 조금 다르다.

1. 상수는 const 키워드를 사용해야 하며 mut를 허용하지 않는다. -> 불변성 그 자체이다.
2. 상수는 전체 영역을 포함하여 어떤 영역에서도 선언될 수 있다. -> 코드의 많은 부분에서 사용될 필요가 있는 값을 다루는데 유용하다.
3. 상수는 오직 상수 표현식만 설정될 수 있지, 함수 호출의 결과값이나 그 외의 실행 시간에 결정되는 값이 설정될 수 없다.
   ex)
```rust
   const MAX_POINTS: u32 = 100_000;
```

### Shadowing

> 이전에 선언한 변수와 같은 새 변수를 선언할 수 있고, 새 변수는 이전 변수를 shadows를 하게 된다. -> 러스트인들은 이를 첫 변수가 두 번째에 의해 shadowed 됐다고 표현한다.
> let 키워드를 사용해서 반복하여 같은 변수명으로 변수를 shadow할 수 있다.

다음 예제는 x에 처음 5를 bind를 하고 shadow하여 원본에 1를 더해서 6더하고 이런 식으로 반복한다.

```rust
    let x = 5;
    let x = x + 1;
    let x = x + 2;

    println!("The value of x is: {}", x);
```

그런데 만약에 <code>let</code>를 사용하지 않고 <code>mut</code>로 선언하게 되면 컴파일 시 에러를 얻게 된다.

### mut과 let의 차이점

- <code>mut</code>와 <code>shadowing</code>의 차이점은 <code>let</code> 키워드를 다시 사용하여 효과적으로 새 변수를 선언하고, 값의 유형을 변경할 수 있으면서도 동일 이름을 사용할 수 있다는 점이다. 
- 변수를 지속적으로 변하게 싶으면 <code>mut</code>을 쓰고 변수를 새로 할당하고 싶으면 <code>let</code> 키워드를 쓰면 된다.
    
```rust
let spaces = "   ";
let spaces = spaces.len();
```
- 이게 가능한 이유는 첫 변수가 <b>문자열 유형</b>이고 두 번째 변수도 첫 번째 것과 동일한 이름을 가진 새롭게 정의된 <b>숫자 유형</b>의 변수이기 때문이다. 즉, 두번째 번수가 첫번째 변수를 덮어쓰기 한 것이다.

```rust
let mut spaces = "   ";
spaces = spaces.len();
```
- 이건 문제가 되는데 <code>spaces</code>가 첫번째 줄에서 문자형이고 두번째 줄에서 숫자형이기 때문에 컴파일-시의 에러가 발생하게 된다. 

</details>

## 3.2 데이터 타입들

<details>
    <summary>자세히 보기</summary>
    
> 기억해야할 것은 Rust에는 변수가 어떤 형태의 데이터인지 명시해줘야한다. -> 타입은 고정적이다. (컴파일 시 반드시 모든 변수의 타입이 정해져 있어야 한다.)
```rust
let guess: u32 = "42".parse().expect("Not a number!");
```
> 데이터의 타입은 크게 스칼라와 컴파운드, 둘로 나뉜다.

### 스칼라

스칼라는 하나의 값으로 표현되는 타입이다. (정수, 부동소수점 숫자, boolean, 문자 -> 이렇게 네가지를 보유하고 있다.)

- 정수형 : 소수점이 없는 숫자이다. 2장에서는 u32타입인 정수형을 사용했었다. -> 부호 없는 32비트 변수임을 나타냄. -> 만약 부호가 있다면 i32처럼 u대신 i로 시작함.

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

- 여기서 <code>Signed</code>과 <code>Unsigned</code>의 차이점은 양수와 음수의 차이이다. 
- 각 부호 변수는 -2^(n - 1) 부터  2^(n - 1) - 1까지의 값을 포괄한다. 여기서 n은 사용되는 타입의 비트 수 이다. 즉, i8은 -(2^7) 에서 2^7 - 1 까지의 값, 즉 -128 에서 127 사이의 값을 저장할 수 있다. 미부호 타입은 0 에서 2^(n - 1) 까지의 값을 저장할 수 있다. 즉, u8 타입은 0 에서 2^(8 - 1) 다시 말해, 0 에서 255 까지의 값을 저장할 수 있습니다.

- isize와 usize 타입은 프로그램이 동작하는 컴퓨터 환경이 64-bits인지 32-bit인지에 따라 쓰인다.

### 정수형 리터럴

- Number literals Example

| Number literals | Example      |
|-----------------|--------------|
| Decimal         | 98_222       |
| Hex             | 0xff         |
| Octal           | 0o77         |
| Binary          | 0b1111_0000  |
| Byte (u8 only)  | b'A'         |

일반적인 경우에는 (심지어 64bit환경에서도) i32가 일반적으로 좋은 선택이다.

### 부동 소수점 타입
- Rust에는 소수점을 갖는 숫자인 부동소수점 숫자를 위한 두 가지 기본 타입이 있다. -> 32bit와 64bit의 크기를 가진 <code>f32</code>와 <code>f64</code>가 있다.
- 기본 타입은 <code>f64</code>인데 그 이유는 더 좋기 떄문이다.

ex)
```rust
let x = 2.0; // f64
let y:f32 = 3.0; // f32
```
- f32타입은 1배수의 정밀도인 부동소수점이고, f64는 2배수의 정밀도인 부동소수점이다.

### 수학적 연산들
  let을 사용하면 rust가 알아서 산출된 값을 변수로 bound한다.

ex)
```rust
// addition
let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
```

### Boolean 타입

일반적인 다른 언어와 사용법이 같다. <code>bool</code> 키워드를 사용한다.

ex)
```rust
fn main() {
let t = true;

    let f: bool = false; // with explicit type annotation

}
```

### 문자 타입
- Rust의 <code>char</code>는 이 언어의 가장 근본적인 알파벳 타입이다. 
- ''와 ""는 다르다. -> ""는 <code>string</code>이고 ''는 <code>char</code>이다.
- ''는 <code>Unicode Scalar</code>를 표현하는 값이고 이는 한국어/중국어/일본어, 이모티콘 등 모두 사용 가능한 char 타입이다. -> ""와 ''의 차이점에 대해서는 8장 String에서 더 자세히 다룬다.

### 복합 타입들

- 튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법이다.
- 괄호 안에 콤마로 구분되는 값들의 목록을 작성하여 튜플을 만든다. -> 다 달라도 된다.

ex)
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

> 튜플은 단일 요소를 위한 복합계로 고려되었기에 전체가 bind된다. 개별적으로 끄내기 위해서는 구조분해를 해야한다.

ex)
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is : {}", y);
```

> 구조분해에 추가로 뒤에 마침표(.) 뒤에 접근하기 원하는 값의 색인을 넣는 것을 통해 튜플의 요소에 직접적으로 접근할 수 있다.

ex)
```rust
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

뭔 배열에서 x[0]쓰는 것과 비슷하냐 ㅋㅋㅋㅋ
당연히 튜플의 첫 번째 색인은 0이다.

### 배열

튜플과 다르게 배열의 모든 요소는 모두 같은 타입이여야 한다. 또한 고정된 길이를 갖게 되며 한번 선언되면 크기는 <b>커지거나 작아지지 않는다.</b> -> 가변적이게 사용하고 싶다면 8장에서 배우는 벡터를 사용해야 한다.

ex)
```rust
let a = [1, 2, 3, 4, 5];
```

- 배열의 요소에 접근하기

일반적인 방법과 같다.
```rust
let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
```

- 여기서 다른 언어와 다른 점은 만약 배열의 크기가 넘은 index을 호출할 경우 Rust는 프로그램이 오류와 함께 종료될 때 패닉한다. -> 메모리 접근을 허용하고 계속 진행하는 대신 즉시 종료한다. -> 실행 중에 에러가 나오는 것이다.
- 배열이 유용할 때는 데이터를 heap보다 stack에 할당하는 것을 원하거나, 항상 고정된 숫자의 요소를 갖는다고 확신하고 싶을 때 사용하는 것이다. 

ex) 월 단위
```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

</details>

## 3.3 함수 동작 원리

<details>
    <summary>자세히 보기</summary>

> <code>fn</code> 키워드를 이용해서 함수를 만들 수 있다.
> Rust는 <code>뱀 형태</code>로 변수나 함수 이름의 규칙으로 사용한다. (ex some_function )
> <code>뱀 형태</code>에서는 모든 문자는 소문자를 사용하며 밑줄 표시로 단어를 구분한다. 

```rust
fn main() {
    println!("Hello world");
}

fn another_function() {
    println!("Another function.");
}
```

- <code>main()</code> 함수 앞에 <code>anotion_function()</code>를 써도 상관없다.

### 함수의 매개변수
  > 함수는 함수 고유한 부분인 특별한 변수 매개변수를 갖는 형식으로 선언될 수 있다.
  > 여기서 전달되는 상수를 전달인자라고 부른다. 여기서 <code>전달인자</code>와 <code>매개변수</code>를 혼용해서 사용하는 경향이 있다. 

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

> 위의 예제는 x 매개변수를 선언하고 형식은 i32로 한 것이다.

- 여기서 x는 <code>매개변수</code>이고 x값에 들어간 5가 <code>전달인자</code>이다.

### 함수의 표현식

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

> 여기서 y안에 x를 선언했는데, x + 1은 뒤에 세미클론이 없다는 게 특징이다.
  표현식은 종결을 나타내는 세미콜론을 사용하지 않는다. 만약 세미콜론을 표현식 마지막에 추가하면, 이는 구문으로 변경되고 반환 값이 아니게 된다.


### 반환 값을 갖는 함수
  > 함수는 그들을 호출한 코드에 값을 반환할 수 있다.
    그들의 타입은 화살표 (->) 뒤에 선언해야 한다.

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

> 여기서 five는 단지 5를 적어놨는데 Rust에서는 이게 함수로 허용된다.
  근데 5뒤에 ;를 찍으면 콘솔에서 에러를 반환한다. 왜냐하면 반환값이 없기 때문이다. 

</details>

## 3.4 주석

<details>
    <summary>자세히 보기</summary>

    > 러스트에서도 주석을 사용할 수 있다.

```rust
// Hello, world. 
```

</details>


## 3.5 제어문

<details>
    <summary>자세히 보기</summary>

> 여기서는 if표현식과 반복문에 대해서 공부해보자.

### if 표현식
  사용법

```rust
fn main() {
    let numver = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

일반적인 사용법이랑 같은데 다른 점은 ()가 빠져있다.
그리고 이번 코드의 조건은 반드시 bool이어야 한다.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### let 구문에서 if문 사용 가능하다.

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

> 이렇게 하면 if 식에서 산추된 값이 bound되게 된다.
  단, if식에 속한 각 return 값은 반드시 같은 타입이여야 한다. 

- 만약에 다음과 같이 한다면 오류가 나오게 된다. 

```rust
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

![스크린샷 2023-06-16 오후 11 45 07](https://github.com/in-ch/rust_study/assets/49556566/d8731b72-63bf-45a9-95e9-b0133fa254a0)

### loop 문
  > loop keyword는 Rust에게 그만두라고 명시하여 알려주기 전까지 코드 블럭을 반복 수행한다.
  > CTRL + C 를 통해 종료 시킬 수 있다.
  > 혹은 <code>break</code> keyword를 위치시켜 프로그램이 언제 루프를 멈춰야 하는지 알려줄 수 있다.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

### while 문
  > while 문을 통해 조건이 참일때까지만 실행하는 반복문을 만들 수 있다.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

### for문 + while

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

> 근데 안정성을 높이기 위해서 다음과 같이 할 수 있다.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

> 혹은 range을 역순하는 rev메소드를 사용해서 다음과 같이 할 수 있다.

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

</details>


# 4 소유권 이해하기

> <code>Ownership</code>이란 러스트가 가비지 컬렉션없이 메모리 안정성 보장을 하게 해준다. -> 러스트는 직접 메모리를 관리해주어야 한다.

## 4.1 스택과 힙 이해하기

<details>
    <summary>자세히 보기</summary>

### 스택과 힙 

- 스택과 힙 둘 다 코드상에서 런타임에 사용할 수 있는 메모리의 부분이다. 다만 둘은 다른 방식으로 구조화 되어 있다.

> 스택은 기본적으로 접시와 같이 위로 쌓고 위에서부터 제거한다. 그리고 데이터를 추가하는 것을 pushing on the stack이라고 하고, 제거하는 것을 popping off the stack이라고 한다.
  이렇게 함으로써 데이터를 넣어두기 위한 공간과 데이터를 가져올 공간을 검색할 필요가 없기 때문에 (항상 스택의 꼭대기가 그 공간이기 때문에) 빠르게 동작한다.
  또한 스택은 빠르기를 위해 스택에 담긴 모든 데이터가 결정되어 있는 고정된 크기를 가진다.

> 힙은 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터를 저장한다. -> 포인터를 써야하는데 운영체제가 충분히 커다란 힙 안의 빈 어떤 지점을 찾아서 이 곳을 사용중이라고 표시하고, 해당 지점의 포인터를 우리에게 돌려준다. 이 절차를 allocation on the heap이라고 한다. -> 포인터를 따라가야 하기 때문에 좀 느리다.

### 소유권의 규칙 

1. 러스트의 각각의 값은 해당값의 <code>오너(owner)</code>라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 떄, 값은 버려진다. (dropped).

- 예를 들어 다음과 같은 변수가 있다고 하자.

```rust
{
    let string = "hello";
} // 여기까지만 유효하다. 
```

여기서 기억할 것은

1. 스코프 안에서 <code>string</code>은 유효하다.
2. 여기서 변수 <code>string</code>은 <code>스트링 레터널</code>이라고 한다. 
3. 이 유효기간은 스코프 밖으로 벗어날 때가지 지속된다.
   이것은 데이터 타입들이 스택에 저장되어있다가 스코프를 벗어날 때 스택으로부터 팝(pop)이 된다.
   그렇다면 힙에 저장되는 데이터들은 어떻게 될까??

### <code>String</code> 타입 

- 다음과 같은 변수가 있다고 보자.

```rust
let string = String::from("hello");
```

- 더블 콜론(::)은 우리가 string_from과 같은 이름을 쓰기 보다는 String 타입 아래의 from 함수를 특정지을 수 있도록 해주는 네임스페이스 연산자이다.
- 다음과 같이하면 변수는 변할 수 있다.

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.

println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.
```
- 여기서 <code>String</code>으로 선언하면 변할 수 있는데, 왜 <code>스트링 레터널</code>은 변하지 않을까? (스코프 안에서만 유효할까?)
- 차이점은 두 타입이 메모리를 쓰는 방식에 있다. 
1. 스트링 리터널 = 컴파일 타임에 알 수 있도록 텍스트가 최종 실행 파일에 직접 하드코딩이 되었다. -> 효율적이고 빠르게 된다.
2. String타입 = <code>String</code> 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어 졌고 힙에서 컴파일 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있다. -> 런타임에 운영체제로부터 메모리가 요청되어야 한다. <code>String</code>의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요하다.
    - 여기서 2번의 경우 <code>Rust</code>는 <code>GC</code>가 없기 때문에 아주 중요한 예제이다.
    - 따라서 <code>GC</code>가 없을 경우, 할당받은 메모리가 더 필요없는 시점을 알아서 명시적으로 이를 반납하는 코드를 호출하는 것은 프로그래머의 책임이 된다. 

- <code>String</code>이 요구한 메모리를 운영체제에게 반납하는 자연스러운 지점이 있다. s가 스코프 밖으로 벗어날 때이다. 변수가 스코프 밖으로 벗어나면, 러스트는 우리를 위해 특별한 함수를 호출한다. 이 함수를 <code>drop</code>이라고 부르고, String의 메모리를 개발자가 반환하도록 하는 코드를 집어넣을 수 있다. 러스트는 } 괄호가 닫힐때 자동적으로 drop을 호출합니다.
- 요약하자면 <code>rust</code>는 괄호가 닫힐때 자동적으로 <code>drop</code>을 호출한다. 

- 이 패턴은 매우 단순해 보이지만, 우리가 <code>힙</code>에 할당시킨 데이터를 사용하는 여러 개의 변수를 사용하고자 할 경우와 같이 좀더 복잡한 상황에서, 코드의 동작은 예기치 못할 수 있다. 이제 그런 경우을 살펴보자.

### 변수와 데이터가 상호작용하는 방법: 이동(move)

다음과 같은 코드가 있다고 보자.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

> 이렇게하면 s1는 무효화되기 때문에 얇은 복사와 비슷한 개념인 이동(move)된다. 즉, s1의 값은 s2로 옮겨지는 것이다.
  만약 s1이 무효화되지 않았다면 drop됐을 때 s1, s2가 동시에 해제되기 때문에 <code>double free</code>라는 오류가 발생하여 메모리 안정성 버그가 생긴다.
![trpl04-04](https://user-images.githubusercontent.com/49556566/205492117-7cbf6079-ca5f-4e81-900d-7018844d960e.svg)

### 변수와 데이터가 상호작용하는 방법: 클론

- 만약 <code>string</code>의 <code>스택 데이터</code> 뿐 아니라 <code>힙 데이터</code>를 깊이 복사하기를 원한다면, <code>clone</code>이라는 공용 메소드를 사용해야 한다.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### 스택에만 있는 데이터 복사: 복사

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

> 이 코드는 <code>clone</code>을 호출하지 않았지만, x도 유효하고 y로 이동하지 않았다. 
  이유는 정수형과 같이 컴파일 타입에 결정되어 있는 크기의 타입은 <code>스택</code>에 모두 저장되기 때문에, 실제 값의 복사본이 빠르게 만들어질 수 있기 때문이다. 

- Copy가 가능한 정수형 타입들
  1. u32와 같은 모든 정수형 타입들
  2. true와 false값을 갖는 bool
  3. f64와 같은 모든 부동 소수점 타입들
  4. Copy가 가능한 타입만으로 구성된 튜플들 (i32, i32)는 copy가 되자만 (i32, String)은 안된다.

### 소유권과 함수 

```rust
fn main() {
    let s = String::from("hello");  // s가 스코프 안으로 들어옴

    takes_ownership(s);             // s의 값이 함수 안으로 이동
                                    // ... 그리고 이제 더이상 유효하지 않음
    let x = 5;                      // x가 스코프 안으로 들어옴

    makes_copy(x);                  // x가 함수 안으로 이동했지만
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용 가능

} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나간다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않음.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옴
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출, 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어옴.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않음.
```

- 여기서 만일 <code>s</code>를 <code>takes_ownership</code> 함수를 호출한 이후에 사용하려 한다면, 러스트는 컴파일 타입 오류를 낼 것이다. 

- 또다른 예제 

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
                                        // 이동

    let s2 = String::from("hello");     // s2가 스코프 안에 들어옴

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
                                        // 이동되었고, 이 함수가 반환값을 s3으로도
                                        // 이동

} // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출. s2는 스코프 밖으로
  // 벗어났지만 이동되었으므로 아무 일도 일어나지 않음. s1은 스코프 밖으로
  // 벗어나서 drop이 호출

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
                                             // 호출한 쪽으로 이동시킴  

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어옴

    some_string                              // some_string이 반환되고, 호출한 쪽의
                                             // 함수로 이동
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
                                                      // 안으로 들어옴

    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동
}
```

</details>



## 4.2 참조자(References)와 빌림(Borrowing) 

<details>
    <summary>자세히 보기</summary>

- 위의 예제에서 마지막에 등장한 튜플을 이용하는 이슈는 <code>String</code>을 호출하는 함수 쪽으로 반환함으로써 <code>calculate_length</code>를 호출한 이후에도 여전히 <code>String</code>을 이용할 수 있도록 하는 것인데, 그 이유는 <code>String</code>이 <code>calculate_length</code> 안쪽으로 이동되었기 떄문이다. 

- 또 다른 예제 

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- 변수 선언부와 함수 반환값에 있던 튜플 코드가 모두 없어졌다. 
- <code>calculate_length</code> 함수에 <code>&s1</code>를 넘기고, 함수의 정의 부분에는 <code>String</code>이 아니라 <code>&String</code>을 이용했다는 점을 주목

- 이 엠퍼센드(&) 기호가 <code>참조자</code>이며, 이는 어떤 값을 소유권을 넘기지 않고 참조할 수 있도록 해준다. 

> <code>&s1</code>문법은 우리가 <code>s1</code>의 값을 참조하지만 소유하지 않는 참조자를 생성하였으므로, 
   소유권을 갖고 있지 않기 떄문에, 이 참조자가 가리키는 값은 참조자가 스코프 밖으로 벗어났을 때도 메모리가 반납되지 않는다.

### 참조자를 통해 빌린 값을 고치려고 한다면 무슨 일이 일어날까? 

- 예제 
```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
> 이렇게 하면 오류가 발생한다. 왜냐하면 변수가 기본적으로 불변인 것처럼, 참조자도 불변이기 때문이다. 

### 가변 참조자(Mutable References)

> 이를 위해 <code>Mutable References</code>를 사용해야 한다. 

- 단, 특정 스코프 내에서 가변 참조자는 딱 하나만 만들 수 있다.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
some_string.push_str(", world");
}
```

- 또한 불변 참조자와 가변 참조자를 동시에 사용할 수 없다. 
```rust
let mut s = String::from("hello");

let r1 = &s; // 문제 없음
let r2 = &s; // 문제 없음
let r3 = &mut s; // 큰 문제
```

### 댕글러 참조자(Dangling References)

- <code>댕글러 포인터</code>란 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말한다. 
- 단, <code>rust</code>는 참조자들이 댕글링 참조자가 되지 않도록 보장해준다. -> 어떠한 데이터의 참조자를 만들었다면, 컴파일러는 그 참조자가 스코프 밖으로 벗어나기 전에는 데이터가 스코프 밖으로 벗어나지 않을 것임을 확인해 줄 것이다.

- 댕글링 참조자를 만드는 시도
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

> 위의 예제는 오류가 발생한다. 왜나하면 <code>&s</code>는 스코프를 벗어났을 때 자동으로 메모리에서 할당 해제되기 떄문이다. 
  다음과 같이 해야 한다. 
```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

### 참조자의 규칙
1. 어떠한 경우이든 간, 둘 중 하나만 가질 수 있다.
    - 하나의 가변 참조자
    - 임의 개수의 불변 참조자들
2. 참조자는 항상 유효해야만 한다.


</details>


## 4.3 슬라이드(Slices)

<details>
    <summary>자세히 보기</summary>

### 슬라이스(Slices)
> 소유권을 갖지 않는 또다른 데이터 타입은 슬라이스이다. 
  슬라이스는 컬렉션 전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게 한다.
  <code>[start ... end]</code> 문법을 사용하고 start로 시작해 end는 포함하지 않는 연속 범위를 나타낸다.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

- 0으로 시작하면 start 생략 가능
- 끝까지 포함하면 end 생략 가능
- 전체 스트링의 슬라이스를 만든다면 양쪽 값 생략 가능 

```rust
let s = String::from("hello world");

let slice1 = &s[..2]; // start 생략 가능
let slice2 = &s[3..]; // end 생략 가능 
let slice3 = &s[..]; // 둘다 생략 가능  
```

### 정리 

- 스트링 리터럴은 슬라이스이다.

- 배열의 슬라이스, 그리고 배열의 슬라이스는 &[i32] 타입을 갖는다.

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[0..4];
```

> 소유권, 빌림, 그리고 슬라이스의 개념은 러스트 프로그램의 메모리 안정성을 컴파일 타임에 보장하는 것이다. 러스트 언어는 다른 시스템 프로그래밍 언어와 같이 메모리 사용에 제어권을 주지만, 데이터의 소유자가 스코프 밖으로 벗아났을 때 소유자가 자동적으로 데이터를 버리도록 하는 것은 이러한 제어를 위해 추가적인 코드 작성이나 디버깅을 하지 않아도 된다는 것이다.

</details>


# 5. 연관된 데이터들을 구조체로 다루기
> <code>구조체(struct)</code>는 사용자들이 연관된 여러 값들을 묶어서 의미있는 데이터 단위를 정의할 수 있게 된다. -> 객체의 데이터 속성 같은 것으로 보면된다. 
> <code>튜플</code>과 비교된다.

## 5.1 구조체를 정의하고 초기화하기

<details>
    <summary>자세히 보기</summary>

- <code>구조체(Struct)</code>은 튜플과 비슷하다. 튜플과 비슷하게 구조체의 구성요소들은 각자 다른 타입을 지닐 수 있다. 
- 튜플과는 다르게 각 구성요소들은 명명할 수 있어 값이 의미하는 바를 명확하게 인지할 수 있다. (튜플보다 유연하다.)
- <code>struct</code> 키워드를 입력해서 구조체를 만들 수 있다.

```rust
struct User {
    username: String,
    email: String,
    sign: u64,
    active: bool,
}
```
> 구조체를 사용하려면 인스턴스(instance)를 생성해야 한다. key, value를 사용한다.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

> 구조체는 .을 사용해서 불러오면 되고 =를 사용해서 값을 변경할 수 있다. (반드시 <code>mutable</code>이여야 한다.)

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

### 변수명이 필드명과 같을 때 간단하게 필드 초기화하기 
- 변수명과 구조체의 필드명이 같다면, <code>필드 초기화 축약법</code>을 이용할 수 있다. 

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### 구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기 

- 존재하는 인스턴스에서 기존 값의 대부분은 재사용하고, 몇몇 값만 바꿔 새로운 인스턴스를 정의할 수 있다. 
- 밑의 예제에서는 <code>user2</code>에 <code>email</code>과 <code>username</code>은 새로 할당하고, 나머지들은 <code>user1</code>의 값들을 그대로 사용해서 새로운 인스턴스를 생성하는 법을 보여준다. 

```rust
let user1 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### 이름이 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체 

- <code>튜플 구조체</code>는 일반적인 구조체 정의방법과 똑같이 <code>struct</code>키워드를 통해 정의할 수 있고, 튜플의 타입 정의가 키워드 뒤에서 이루어지면 된다. 

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
</details>


## 5.2 구조체를 이용한 예제 프로그램

<details>
    <summary>자세히 보기</summary>

> 사각형의 넓이를 계산하는 프로그램을 작성한다. 

rectangles라는 사각형의 넓이를 계산하는 프로그램이다.
```rust
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width;
}
```

### 튜플을 이용한 리펙토링 
> 길이와 너비를 함께 묶는다면 더 읽기 쉽고 관리하기도 쉬울 것이다.

```rust
fn main() {
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)        
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

> 이제 하나의 인자만 넘기면 된다. length가 튜플 인덱스 0이고 width가 튜플 인덱스 1이라는 점을 기억해야 한다.


### 구조체를 이용한 리팩토링: 의미를 더 추가하기 
> 데이터에 이름표를 붙여서 의미를 부여하기 위해 구조체를 이용할 수 있다. 

```rust
struct Rectangle {
    length: u32,
    length: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
```

- 구조체의 소유권을 원하기 보다 빌리기를 원하기 떄문에 <code>&</code>을 사용하였다. 

### 파생 트레잇(derived trait)으로 유용한 기능 추가하기

> 디버깅하는 동안 구조체 내의 모든 값을 보기 위해서 <code>Rectangle</code>의 인스턴스를 출력할 수 있다면 도움이 될 것 이다.
  <code>구조체</code>를 사용하는 경우, <code>println!</code>이 출력을 형식화하는 방법은 덜 명확한데 이는 표시 방법의 가능성이 더 많기 때문이다. 
  따라서 <code>구조체</code>를 사용하기 위해 <code>파생 트레잇</code>을 사용해야 한다.

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
}
```

- 러스트는 우리를 위해 <code>derive 어노테이션</code>을 이용한 여러 트레잇을 제공하여 우리의 커스텀 타입에 대해 유용한 동작을 추가할 수 있도록 해준다.

</details>



## 5.3 메소드 문법

<details>
    <summary>자세히 보기</summary>

> 메소드는 함수와 유사하다. <code>fn 키워드</code>와 이름을 가지고 선언되고, 파라미터와 반환값을 가지고 있으며, 다른 어딘가로부터 호출되었을때 실행될 코드를 담고 있다.
> 다만 메소드는 함수와는 달리 구조체의 내용 안에 정의되며 첫번째 파라미터가 언제나 <code>self</code>인데, 이는 메소드가 호출되고 있는 <code>구조체의 인스턴스</code>를 나타낸다.

### 메서드 정의하기 

예제
```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

> <code>Rectangle</code>의 내용 안에 함수를 정의하기 위해서는, <code>impl(구현: implementation)</code> 블록을 시작한다. 그 다음 <code>area</code>함수를 <code>impl</code> 중괄호로 옮기고 시그니처 및 본체 내의 모든 곳에 있는 파라미터를 <code>self</code>로 변경시킨다. 
> <code>area</code>의 시그니처 내에서는, rectangle: &Rectangle 대신 <code>&self</code>가 사용되었는데 이는 메소드가 <code>impl Rectangle</code> 내용물 안에 위치하고 있어 러스트가 <code>self</code>의 타입이 Rectangle 라는 사실을 알 수 있기 때문이다.
> <code>&</code>을 썻기 때문에 소유권을 빌려오는 것을 알 수 있다.  

### 더 많은 파라미터를 가진 메소드

예제 2 
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

### 연관 함수

> <code>impl</code> 블록의 유용한 기능은 <code>self</code> 파라미터를 갖지 않는 함수도 <code>impl</code> 내에 정의하는 것이 허용된다는 것이다. 이걸 <code>연관 함수(associated functions)</code>라고 부른다. 
> 연관 함수는 새로운 구조체의 인스턴스를 반환해주는 생성자로서 자주 사용된다. 

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```
> 이 연관 함수를 호출하기 위해서는 <code>let sq = Rectangle::square(3);</code> 처럼, 구조체 이름과 함께 <code>::</code> 문법을 이용한다. 
  이 함수는 구조체의 이름공간 내에 있다.

### 정리 
> 구조체는 커스텀 타입을 만들 수 있게 해준다. 구조체를 이용함으로써, 연관된 데이터의 조각들을 서로 연결하여 유지할 수 있으며 각 데이터 조각에 이름을 붙여 코드를 더 명확하게 만들어 줄 수 있다. 

</details>

# 6. 열거형(enum)과 패턴 매칭 
> <code>열거형(enum)</code>은 하나의 타입이 가질 수 있는 값들을 열거 함으로써 타입을 정의할 수 있도록 한다.
  여기서는 <code>Option</code>이라고 하는 유용한 열거형을 자세히 살펴보고, 그 다음으로, 열거형의 값에 따라 쉽게 다른 코드를 실행하기 위해 <code>match</code>표현식에서 패턴 매칭을 사용하는 방법을 살펴본다.
  마지막으로, 코드에서 열거형을 편하고 간결하게 다루기 위한 관용 표현인 <code>if left</code>구문을 다룬다. 
> 열거형은 다른 언어들에서도 볼 수 있는 특징이지만, 각 언어마다 열거형으로 할 수 있는 것들이 다르다.
  러스트의 열거형은 <code>F#</code>, <code>OCaml</code>, <code>Haskell</code>과 같은 함수형 언어의 대수 데이터 타입과 가장 비슷하다. 

## 6.1 열거형 정의하기 

<details>
    <summary>자세히 보기</summary>

> 열거형은 다음과 같은 때에 유용할 것이다. 예를 들어 아이피같은 경우는 두 개의 주요한 표준이 있다. 바로 버전4와 버전6인데, 경우의 수가 이 두개가 전부이기 때문에 모든 가능한 값들을 나열(enumerate)하는 게 더 좋을 것이다. 

```rust
enum IpAddrKind {
    V4,
    V6,
}
```
이렇게 하면 <code>IpAddrKind</code>는 우리의 코드 어디에서나 쓸 수 있는 데이터 타입이 되었다.

### 열거형 값 

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
- <code>열거형의 variants</code>는 열거형을 정의한 식별자에 의해 이름 공간이 생기며, 두 개의 콜론을 사용하여 둘을 구분할 수 있다.  
- 다음과 같이 <code>IpAddrKind</code>타입을 인자로 받는 함수를 정의할 수 있다.
```rust
fn route(ip_type: IpAddrKind) { }
```

> 그리고 <code>variant</code> 중 하나를 사용해서 다음과 같이 함수를 호출할 수 있다.
```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

### 좀 더 긴 예제 
```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```
- 이렇게 <code>구조체에 넣을 수 있다.</code>

- 또다른 예제 
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```
- 각 열거형 <code>variant</code>에 데이터를 직접 넣는 방식을 사용해서 열거형을 구조체의 일부로 사용하는 방식보다 더 간결하게 동일한 개념을 표현할 수 있다.
- 각 <code>IpAddr</code>열거형의 새로운 정의에서는 두 개의 <code>V4</code>와 <code>V6</code>는 연관된 <code>String</code> 타입의 값을 갖게 된다.
- 이렇게 하니깐 각 <code>variant</code>에 직접 데이터를 붙임으로써, 구조체를 굳이 사용할 필요가 없어졌다. 

- 또다른 예제 
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```
- 이렇게 각 <code>enum</code>에 다른 구조를 넣을 수 있다. 
- 또 열거형에는 어떤 자료형이든지 넣을 수 있다. 


### Option 열거형과 Null 값 보다 좋은 점들.
> <code>Option</code> 타입은 많이 사용되는데, 값이 있거나 없을 수도 있는 아주 흔한 상황을 나타내기 때문이다.
  <code>null</code> 값으로 발생하는 문제는, <code>null</code>값을 <code>null</code>이 아닌 값처럼 사용하려고 할 때 여러 종류의 오류가 발생할 수 있다는 것이다. 
  그래서 <code>null</code>대신 <code>Option</code>을 쓰는 게 좋다. 

```rust
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

### 왜 Option<T> 가 null 을 갖는 것보다 나을까? 

- 만약 다음과 같이 하면 오류가 발생한다. 
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```
> 이렇게 하면 오류가 발생한다. 왜냐하면 <code>i8</code>과 <code>Option<i8></code>은 다른 타입이기 때문이다. 
  다르게 얘기하자면, <code>T</code> 에 대한 연산을 수행하기 전에 <code>Option<T></code> 를 <code>T</code> 로 변환해야 한다. 
  일반적으로, 이런 방식은 <code>null</code>과 관련된 가장 흔한 이슈 중 하나를 발견하는데 도움을 준다. -> 실제로 <code>null</code> 일 때, <code>null</code> 이 아니라고 가정하는 경우이다. 

</details>

## 6.2 match 흐름 제어 연산자

<details>
    <summary>자세히 보기</summary>

- 러스트는 <code>match</code>라고 불리는 극도로 강력한 흐름 제어 연산자를 가지고 있다. 이는 우리에게 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매치되었는지를 바탕으로 코드를 수행하게 해준다.
- 패턴은 리터럴 값, 변수명, 와일드카드, 그리고 많은 것들로 구성될 수 있다. (18장에서 더 다룰 것이다.)
- <code>match</code>의 힘은 패턴의 표현성으로부터 오며 컴파일러는 모든 가능한 경우가 다루어지는지를 검사한다. 
- <code>match</code>내의 각 패턴을 통과하고, 해당 값에 "맞는" 첫 번째 패턴에서, 그 값은 실행 중에 사용될 연관된 코드 블럭 안으로 떨어진다. 

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
- <code>if</code>와 비슷해보이지만 큰 차이점이 있다. 
    1. <code>if</code>를 사용할 때는, 해당 표현식은 부울린 값을 반환할 필요가 있다. 여기서는 어떤 타입이든 가능하다. 
    2. 다음은 <code>match</code> 갈래(arm)이다. 하나의 갈래는 두 부분을 갖고 있다: 패턴과 어떤 코드로. 여기서의 첫 번째 갈래는 값 <code>Coin::Penny</code>로 되어있는 패턴을 가지고 있고 그 후에 패턴과 실행되는 코드를 구분해주는 => 연산자가 있다. 위의 경우에서 코드는 그냥 값 <code>1</code>이다. 각 갈래는 그다음 갈래와 쉼표로 구분한다. 
    3. 각 갈래와 연관된 코드는 <code>표현식</code>이고, 이 매칭 갈래에서의 표현식의 결과 값은 전체 <code>match</code> 표현식에 대해 반환되는 값이다. 
    4. <code>중괄호({})</code>를 사용할 수도 있다. (짧다면 사용하지 않는다.)

```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 값들을 바인딩하는 패턴들 
- 매치 갈래의 또 다른 유용한 기능은 패턴과 매치된 값들의 부분을 바인딩할 수 있다는 것이다. 이것이 열거형 <code>variant</code>로 부터 어떤 값들을 추출할 수 있는 방법이다. 

```rust
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

value_in_cents(Coin::Quarter(UsState::Alaska))
```
> <code>value_in_cents(Coin::Quarter(UsState::Alaska))</code>를 호출하면, coin은 <code>Coin::Quarter(UsState::Alaska)</code>가 된다. 각각의 매치 갈래들과 이 값을 비교할 때, <code>Coin::Quarter(state)</code>에 도달할 때까지 아무것도 매치되지 않는다. 이 시점에서, state에 대한 바인딩은 값 <code>UsState::Alaska</code>가 된다. 그러면 이 바인딩을 <code>println!</code> 표현식 내에서 사용할 수 있고, 따라서 <code>Quarter</code>에 대한 <code>Coin</code> 열거형 variant로부터 내부의 주에 대한 값을 얻는다.

### Option<T>를 이용하는 매칭 

> 이전 절에서 <code>Option<T></code>을 사용할 때 Some 케이스로부터 내부의 T 값을 얻을 필요가 있었다; Coin 열거형을 가지고 했던 것처럼 <code>match</code>를 이용하여 <code>Option<T></code>를 다룰 수 있다. 동전들을 비교하는 대신, <code>Option<T></code>의 variant를 비교할 것이지만, <code>match</code> 표현식이 동작하는 방법은 동일하게 남아있다.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
- 처음 5는 아무것도 매칭되지 않기 때문에 <code>let five = 5</code>이고, <code>plus_one(five)</code>는 <code>five</code>가 Some(5)이므로 6이 된다. 

</details>

## 6.3 if let을 사용한 간결한 흐름 제어 

<details>
    <summary>자세히 보기</summary>

> <code>if let</code>문법은 <code>if</code>와 <code>let</code>을 조합하여 하나의 패턴만 매칭시키고 나머지 경우는 무시하는 값을 다루는 덜 수다스러운 방법을 제공한다. 
  
- 아래는 어떤 <code>Option<u8></code>값을 매칭하지만 그 값이 3일 경우에만 코드를 실행시키고 싶어하는 코드이다. 

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

- 여기서 <code>Some(3)</code>이 매칭되는 경우에만 뭔가를 하지만 다른 <code>Some<u8></code>값 혹은 <code>None</code>값인 경우에는 아무것도 하지 않고 싶다. 
  이러한 <code>match</code>표현식을 만족시키기 위해, <code>_ => ()</code>을 단 하나의 variant를 처리한 다음에 추가해야 하는데, 이는 추가하기에 너무 많은 보일러 플레이트 코드가 추가된다. 

  그 대신 <code>if let</code>을 이용하여 이 코드를 더 짧게 쓸 수 있다. 

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```
> <code>if let</code>은 =로 구분된 패턴과 표현식을 입력받는다.
  이는 <code>match</code>와 동일한 방식으로 작동하는데, 여기서 표현식은 <code>match</code>에 주어지는 것이고 패턴은 이 <code>match</code>의 첫 번째 갈래와 같다. 
  <code>if let</code>을 이용하는 것은 덜 타이핑하고, 덜 들여 쓰기 하고, 보일러 플레이트 코드를 덜 쓰게 된다는 뜻이다. 단, <code>match</code>가 강제했던 빠짐없는 검사를 잃게 되기는 한다. 

### else도 사용 가능하다.
> <code>if let</code>과 함께 <code>else</code>를 포함시킬 수 있다.
  <code>else</code> 뒤에 나오는 코드 블록은 <code>match</code>표현식에서 _ 케이스 뒤에 나오는 코드 블록과 동일하다. 

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```
혹은 
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

</details>

# 7. 모듈 
> 코드가 커짐에 따라서, 재사용 및 더 나은 조직화를 위해 결국 어떤 기능을 다른 함수로 이동시킬 것이다.
  코드를  더 작은 덩어리로 쪼갬으로서, 각각의 덩어리들은 개별적으로 이해하기 더 수월해진다. -> <code>module</code>시스템을 통해 할 수 있다.

> 코드 몇줄을 함수로 추출하는 것과 같은 방식으로, 함수 (혹은 구조체나 열거형 같은 다른 코드들)를 다른 모듈로 뽑아낼 수 있으며, 이것들의 정의가 모듈의 바깥쪽에서 볼 수 있도록 하거나(public) 혹은 보이지 않게 하도록 (private)하게 선택할 수 있다.
- <code>mod</code> 키워드는 새로운 모듈을 선언한다. 모듈 내의 코드는 이 선언 바로 뒤에 중괄호로 묶여서 따라오거나 다른 파일에 놓일 수 있다.
- 기본적으로 함수, 타입, 상수, 그리고 모듈은 <code>private</code>이다. <code>pub</code> 키워드가 어떤 아이템을 public하게 만들어줘서 이것의 네임스페이스 바깥쪽에서도 볼 수 있도록 한다.
- <code>use</code>키워드는 모듈이나 모듈 내의 정의들을 스코프 안으로 가져와서 이들을 더 쉽게 참조할 수 있도록 한다. 

## 7.1 mod와 파일 시스템 

<details>
    <summary>자세히 보기</summary>

- 새로운 프로젝트를 만드는 것으로 <code>바이너리 크레이트(crate)</code>을 만드는 것 대신에 <code>라이브러리 크레이트</code>를 만들 것이다. 
- <code>라이브러리 크레이트</code>란 다른 사람들이 자신들의 프로젝트에 디펜던시로 추가할 수 있는 프로젝트를 말한다 
- 라이브러리를 만들 때는 <code>--bin</code>대신에 <code>--lib</code> 옵션을 넘겨야 한다.

```cli
$ cargo new communicator --lib
$ cd communicator
```
- 이렇게 하면 기존과 다른 파일을 만들어 준다.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

### 모듈 정의
- <code>communicator</code> 네트워크 라이브러리를 만들기 위해, 먼저 <code>connect</code>라는 이름의 함수가 정의되어 있는 <code>network</code>라는 이름의 모듈을 정의한다. 
- 러스트 내의 모듈 정의는 모두 <code>mod</code>로 시작된다.

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```

- 이 함수를 <code>network</code> 모듈 바깥의 스크립트에서 호출하고자 한다면, 우리는 모듈을 특정할 필요가 있으므로 이름공간 문법 <code>::</code>를 이용해야 한다. ex) <code>network::connect</code>

- 만약에 다음과 같이 하면 어떻게 될까? 
```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```
- 이렇게 써야 한다. <code>network::client::connect</code>

- 그리고 구조는 다음과 같이 된다. 
```rust
communicator
 └── network
     └── client
```

### 모듈을 다른 파일로 옮기기 
```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}

communicator
 ├── client
 └── network
     └── server
```

> 이렇게 계속 하면 파일이 너무 길어진다. 그래서 각각 파일을 <code>src/lib.rs</code>로부터 떼어내어 각자를 위한 파일들에 위치시켜 분리할 수 있다. 

Filename: src/lib.rs
```rust
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

Filename: src/client.rs
```rust
fn connect() {
}
```

- 이미 <code>src/lib.rs</code>안에다가 <code>client</code>모듈을 <code>mod</code>를 이용하여 선언을 했기 때문에, 이 파일 안에는 <code>mod</code> 선언이 필요없다. 단지 <code>src/client.rs</code>에는 <code>client</code> 모듈의 내용물만 제공할 뿐이다. 
- 만약 또 여기에 mod를 만들면 <code>client</code> 모듈 내에 또다른 서브<code>client</code>모듈을 만들 뿐이다. 

### 모듈 파일 시스템의 규칙

- 만일 <code>foo</code>라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, <code>foo.rs</code>라는 이름의 파일 내에 <code>foo</code>에 대한 선언을 집어넣어야 한다.

- 만일 <code>foo</code>가 서브모듈을 가지고 있다면, <code>foo/mod.rs</code>라는 이름의 파일에 <code>foo</code>에 대한 선언을 집어넣어야 합니다.

> 이 규칙들은 재귀적으로 적용되므로, <code>foo</code>라는 이름의 모듈이 <code>bar</code>라는 이름의 서브모듈을 갖고 있고 <code>bar</code>는 서브모듈이 없다면, src 디렉토리 안에는 아래와 같은 파일들이 있어야 한다. 

```rust
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

> 이 모듈들은 부모 모듈의 파일에 <code>mod</code> 키워드를 사용하여 선언되어 있어야 한다. 

</details>


## 7.2 pub으로 가시성 제어하기

<details>
    <summary>자세히 보기</summary>

> 이번에는 <code>connect</code>라는 함수들을 또다른 프로젝트에 사용되게 만들게 시도해보자.

```rust
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

- <code>communicator</code> 라이브러리 크레이트를 가져오기 위해 <code>extern crate</code> 명령어를 사용한다. 

- <code>communicator</code> 라이브러리 밖의 크레이트가 안을 들여다 보는 시점에서, 우리가 만들어왔던 모든 모듈들은 <code>communicator</code>라는 이름을 갖는 모듈 내에 있다 크레이트의 최상위 모듈을 <code>루트 모듈 (root module)</code> 이라 부른다.

- 또한. 비록 프로젝트의 서브모듈 내에서 외부 크레이트를 이용하고 있을지라도, <code>extern crate</code>이 루트 모듈에 와 있어야 한다는 점(즉 <code>src/main.rs</code> 혹은 <code>src/lib.rs</code>)을 기억해야 한다. 그러면 서브모듈 안에서 마치 최상위 모듈의 아이템을 참조하듯 외부 크레이트로부터 아이템들을 참조할 수 있다. 

-  바이너리 크레이트는 고작 라이브러리의 <code>client</code> 모듈로부터 <code>connect</code> 함수를 호출할 뿐이다. 하지만 <code>cargo build</code>을 실행하면 경고들 이후에 에러를 표시할 것이다.

```rust
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
> 여기서 알 수 있는 것은 <code>client</code> 모듈이 비공개임을 알려주고 있다. (러스트의 내용 중에서 공개 그리고 비공개에 대한 개념에 대해 알 수 있는 첫번째 시간이답). 참고로 모든 러스트의 안에 코드의 기본 상태는 비공개이다. 즉, 다른 사람은 이 코드를 사용할 수 없다. 
  만약 프로그램 내에서 비공개 함수를 이용하지 않는다면, 루트 프로그램이 그 함수를 이용할 수 있는 유일한 곳이기 때문에, 러스트는 그 함수가 사용된 적이 없다고 경고를 낼 것이다. 

> <code>client::connect</code>와 같은 함수를 공개로 지정한 뒤에는 우리의 바이너리 크레이트 상에서 이 함수를 호출하는 것이 가능해질 뿐만 아니라, 그 함수가 사용된 적이 없다는 경고 또한 사라질 것이다. -> 함수를 공개로 표시하는 것은 러스트로 하여금 그 함수가 우리 프로그램 외부의 코드에 의해 사용될 것이라는 점을 알게끔 해준다. 

### 함수를 공개로 만들기 

- 러스트에게 어떤 것을 공개하기 위해서는, 공개하길 원하는 아이템의 선언 시작 부분에 <code>client::connect</code>가 사용된 적 없음을 알리는 경고와 바이너리 크레이트에서 나온 <code>module `client` is private</code>를 없애야 한다.

```rust
pub mod client;

pub mod network;
```

```rust
pub fn connect() {
}

mod server;
```

### 비공개 규칙(Privacy Rules)
> 다음과 같은 규칙이 있다.
- 만일 어떤 아이템이 공개라면, 이는 부모 모듈의 어디에서건 접근이 가능하다.
- 만일 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근 가능하다. 

### 비공개 예제(Privacy Examples)

```rust
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

</details>

## 7.3 use로 이름 가져오기

<details>
    <summary>자세히 보기</summary>

- 다음은 <code>nested_modules</code>함수를 호출하는 것처럼, 모듈 이름을 호출 구문의 일부분으로 사용하여 해당 모듈 내에 정의된 함수를 호출하는 방법을 다룬다. 

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

- 위에서 보다시피 완전하게 경로를 지정한 이름을 참조하는 것은 너무 길어질 수 있다. 다행히도 러스트는 이러한 호출을 더 간결하게 만들어주는 키워드를 가지고 있다. 

### use를 이용한 간결한 가져오기 

> 러스트의 <code>use</code> 키워드는 스코프 내에서 호출하고 싶어하는 함수의 모듈을 가져옴으로써 긴 함수 호출을 줄여준다. 

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

- <code>use a::series::of;</code> 줄은 <code>of</code> 모듈을 참조하고 싶은 곳마다 <code>a::series::of</code> 전부를 사용하기 보다는 of를 사용할 수 있다는 뜻.
- 단, 위의 코드에서 <code>use</code> 키워드는 우리가 명시한 것만 스코프 내로 가져온다: 즉 모듈의 자식들을 스코프 내로 가져오지는 않는다. 이는 <code>nested_modules</code> 함수를 호출하고자 할 때 여전히 <code>of::nested_modules</code>를 사용해야 하는 이유이다.  -> 다음과 같이 <code>use</code>구문 안에서 모듈 대신 함수를 명시하여 스코프 내에서 함수를 가져올 수 있다. 

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

- 열거형 또한 모듈과 비슷한 일종의 이름공간을 형성하고 있기 때문에, 열거형의 variant 또한 <code>use</code>를 이용하여 가져올 수 있다. 
- 어떠한 <code>use</code>구문이건 하나의 이름공간으로부터 여러 개의 아이템을 가져오려 한다면, 아래와 같이 중괄호와 쉼표를 구문의 마지막 위치에 사용하여 이 아이템들을 나열 할 수 있다.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```
> <code>Green variant</code>에 대해서는 여전히 <code>TrafficLight</code> 이름공간을 명시하고 있는데, 이는 <code>use</code> 구문 내에 <code>Green</code>를 포함하지 않았기 때문이다.

### *를 이용한 모두(glob) 가져오기 
- 이름공간 내의 모든 아이템을 가져오기 위해서는 <code>*</code>문법을 이용할 수 있다. 
- <code>*</code>은 <code>글론(glob)</code>이라고 부르며, 이는 이름공간 내에 공개된 모든 아이템을 가져온다. 
- 단, 글론은 편리하지만, 예상보다 많은 아이템을 끌어와서 이름 간의 <code>충돌(naming conflict)</code>의 원인이 될 수 있다.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

### super를 사용하여 부모 모듈에 접근하기 
- 다음과 같이 <code>super</code>를 사용하여 계층 구조 상에서 현재 모듈로부터 한 모듈 거슬러 올라갈 수 있다.
<code>super::client::connect()</code>

</details>

# 8. 일반적인 컬렉션
> 러스트의 표준 라이브러리에는 컬렉션이라 불리는 여러 개의 매우 유용한 데이터 구조들이 포함되어 있다. 대부분의 다른 데이터 타입들은 하나의 특정한 값을 나타내지만, 컬렉션은 다수의 값을 담을 수 있다. 

> 내장된 배열(build-in array)와 튜플 타입과는 달리, 이 컬렉션들이 가리키고 있는 데이터들은 힙에 저장되는데, 이는 즉 데이터량이 컴파일 타임에 결정되지 않아도 되며 프로그램이 실행될 때 늘어나거나 줄어들 수 있다는 의미이다. 

> 각각의 컬렉션 종류는 서로 다른 용량과 비용을 가지고 있으며, 여러분의 현재 상황에 따라 적절한 컬렉션을 선택하는 것은 시간이 지남에 따라 발전시켜야 할 기술이다. 

- 벡터(vector)는 여러 개의 값을 서로 붙어 있게 저장할 수 있도록 해준다.
- 스트링(string)은 문자의 모음이다. <code>String</code>타입은 이전에 다루었지만, 이번 장에서는 더 깊이 있게 이야기 해본다.
- 해쉬맵(hash map)은 어떤 값을 특정한 키와 연관지어 주도록 해줍니다. 이는 맵(map)이라 일컫는 좀더 일반적인 데이터 구조의 특정한 구현 형태이다. 

- 더 [자세한 내용](https://doc.rust-lang.org/std/collections/index.html)

## 8.1 벡터

<details>
    <summary>자세히 보기</summary>

- 벡터(<code>Vec(T)</code>)는 메모리 상에 서로 이웃하도록 모든 값을 집어 넣는 단일 데이터 구조 안에 하나 이상의 값을 저장하도록 해준다. 벡터는 같은 타입의 값만을 저장할 수 있다.

### 새 벡터 만들기 

```rust
let v: Vec<i32> = Vec::new();
```

- 이 벡터에 어떠한 값도 집어넣지 않았기 때문에, 러스트는 우리가 저장하고자 하는 요소의 종류가 어떤 것인지 알지 못한다. -> 대신에 저장하고자 하는 값의 타입을 대부분 유추할 수는 있다. 
- 벡터는 제네릭(generic)을 이용하여 구현되었다. (여기서 <code>Vec</code>은 러스트에서 제공해주는 표준 라이브러리이며, <code>제네릭(generic)</code>은 10장에서 배울 것이다. 

- 아니면 다음과 같이 초기값들을 갖고 있게끔 생성할 수 있다. 이는 가장 일반적인 생성 방법이다. 

```rust
let v = vec![1, 2, 3];
```

### 벡터 갱신하기 

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

- 어떤 변수에 대해 그 변수가 담고 있는 값이 변경될 수 있도록 하려면, <code>mut</code> 키워드를 사용하여 해당 변수를 가변으로 만들어 줄 필요가 있다. 

### 벡터를 드롭하는 것은 벡터의 요소들을 드롭시킨다.
- <code>struct</code>와 마찬가지로, 벡터도 스코프 밖으로 벗어났을 때 해제된다.

```rust
{
    let v = vec![1, 2, 3, 4];

    // v를 가지고 뭔가 합니다

} // <- v가 스코프 밖으로 벗어났고, 여기서 해제
```

> 벡터가 드롭될 때 벡터의 내용물 또한 전부 드롭되는데, 이는 벡터가 가지고 있는 정수들이 모두 제거된다는 의미이다.
  이는 직관적인 것처럼 보일 수도 있겠지만 벡터의 요소들에 대한 참조자를 만들때는 좀 더 복잡해질 수 있다. 


### 벡터의 요소들 읽기 

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

- 각각 2가지의 방법이 설명되어 있다. 
    1. <code>&</code>와 <code>[]</code>를 이용하여 참조자를 얻은 것
    2. <code>get</code> 함수에 인덱스를 파라미터로 넘겨서 <code>Option<&T></code>를 얻은 것

- 첫번째 방법은 벡터의 끝을 넘어서는 요소에 접근하는 시도를 하면 프로그램이 죽게끔하는 치명적인 에러를 발생하도록 하기를 고려하는 경우 가장 좋다. 
- <code>get</code>함수에 벡터 범위를 벗어난 인덱스가 주어졌을 때는 패닉 없이 <code>None</code>이 반환된다. 

### 벡터 내의 값들에 대한 반복처리

- 만일 벡터 내의 각 요소들을 차례대로 접근하고 싶다면, 하나의 값에 접근하기 위해 인덱스를 사용하는 것보다는, 모든 요소들에 대해 반복처리를 할 수 있다. 

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

- 만일 모든 요소들을 변형시키길 원한다면 가변 벡터 내의 각 요소에 대한 가변 참조자로 반복작업을 할 수도 있다.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

### 열거형을 사용하여 여러 타입을 저장하기 

- 벡터는 같은 값만 담을 수 있는데, 다행히도, 열거형의 variant는 같은 열거형 타입 내에 정의가 되므로, 벡터 내에 다른 타입의 값들을 저장할 필요가 있다면 열거형을 정의하여 사용할 수 있다. 

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

</details>

## 8.2 스트링

<details>
    <summary>자세히 보기</summary>

### 개요
- 스트링이 컬렉션 장에 있는 이유는 스트링이 바이트의 컬렉션 및 이 바이트들을 텍스트로 통역할 때 유용한 기능을 제공하는 몇며 메소드로 구현되어 있기 떄문이다.
- 이번 절에서는 생성, 갱신, 값 읽기와 같은 모든 컬렉션 타입이 가지고 있는, <code>String</code>에서의 연산에 대해 이야기 할 것이다. 또한 <code>String</code>을 다른 컬렉션들과 다르게 만드는 부분, 즉 사람과 컴퓨터가 <code>String</code> 데이터를 통역하는 방식 간의 차이로 인해 생기는 <code>String</code> 인덱싱의 복잡함을 논의해 볼 것이다.

### 스트링이란? 

- <code>&str</code>에서 봤듯이, 러스트는 핵심 언어 기능 내에서 딱 한가지 스트링 타입만 제공하는데, 이는 바로 스트링 슬라이스인 <code>str</code>이다. 
- <code>String</code> 타입은 핵심 언어 기능 내에 구현된 것이 아니고 러스트의 표준 라이브러리를 통해 제공되며, 커질 수 있고, 가변적이며, 소유권을 갖고 있고, UTF-8로 인코딩된 스트링 타입이다. 
- 러스트인들이 "스트링"에 대해 이야기할 때, 보통 <code>String</code>과 스트링 슬라이스 <code>&str</code> 타입 둘 모두를 이야기한 것이다. -> 두 타입 모두 러스트 표준 라이브러리에서 매우 많이 사용되며 <code>String</code>과 <code>스트링 슬라이스</code> 모두 UTF-8로 인코딩되어 있다. 

### 다양한 러스트의 String 표준 라이브러리 
- 러스트 표준 라이브러리는 <code>OsString</code>, <code>OsStr</code>, <code>CString</code>, <code>CStr</code>과 같은 몇가지 다른 스트링 타입도 제공한다. 심지어 어떤 라이브러리 크레이트들은 스트링 데이터를 저장하기 위해 더 많은 옵션을 제공한다. 
- <code>*String / *Str</code>이라는 작명과 유사하게, 이들은 종종 소유권이 있는 타입과 이를 빌린 변형 타입을 제공하는데, 이는 <code>String / &Str</code>과 비슷합니다. 이러한 스트링 타입들은, 예를 들면 다른 종류의 인코딩이 된 텍스트를 저장하거나 다른 방식으로 메모리에 저장될 수 있다. 

### 새로운 스트링 생성하기 

```rust
let mut s = String::new();
```
- 다음과 같이 생성할 수 있다. -> s라는 변형 가능한 빈 스트링을 만들어 준다. 

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```
- 다음과 같이 초기값을 갖게 생성할 수도 있다. 

```rust
let s = String::from("initial contents");
```

- 혹은 다음과 같이 스트링 리터럴로부터 <code>String</code>을 생성하기 위해 <code>String:from</code>함수를 이용할 수도 있다. 

> 참고로 스트링이 UTF-8로 인코딩되어 있어서 어떠한 텍스트 데이터라도 포함할 수 있다. (한국어도 당연히 포함되어 있다.) 

### 스트링 갱신하기 

1. <code>push_str</code>과 <code>push</code>을 이용하여 스트링 추가하기 

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(&s2);
println!("s2 is {}", s2); // foo bar
```

```rust
let mut s = String::from("lo");
s.push('l'); /// lol
```

2. <code>+</code> 연산자나 <code>format!</code> 매크로를 이용한 접합

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음, Hello, world!
```

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3; // tic-tac-toe
```

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3); // tic-tac-toe
                                         // 이렇게 하면 읽기도 쉽고,  어떠한 파라미터들의 소유권도 가져가지 않으므로 추천하는 방식이다.
```

</details>

## 8.3 해쉬맵

<details>
    <summary>자세히 보기</summary>

- <code>HashMap<K, V></code> 타입은 K 타입의 키에 V 타입의 값을 매핑한 것을 저장한다. 이 매핍은 해쉬 함수을 통해 동작하는데, 해쉬 함수는 이 키와 값을 메모리 어디에 저장할지 결정한다. 
- 해쉬맵은 벡터를 이용하듯 인덱스를 이용하는 것이 아니라 임의의 타입으로 된 키를 이용하여 데이터를 찾기를 원할때 유용하다. 

### 새로운 해쉬맵 생성하기 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- 먼저 표준 라이브러리의 컬렉션 부분으로부터 <code>HashMap</code>을 <code>use</code>로 가져와야 한다. 
- 이 방법은 덜 자주 사용된다. 
- 벡터와 마찬가지로, 해쉬맵도 데이터를 힙에 저장한다. 이 <code>HsashMap</code>은 String 타입의 키와 i32 타입의 값을 갖는다. -> 벡터와 비슷하게 해쉬맵도 동질적이다: 모든 키는 같은 타입이어야 하고, 모든 값도 같은 타입이여야 한다.

- 해쉬맵을 생성하는 또다른 방법은 튜플의 벡터에 대해 collect 메소드를 사용하는 것인데, 이 벡터의 각 튜플은 키와 키에 대한 값으로 구성되어 있다. 
- <code>collect</code> 메소드는 데이터를 모아서 <code>HashMap</code>을 포함한 여러 컬렉션 타입으로 만들어 준다.

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```
- 타입 명시 <code>HashMap<_, _></code>이 필요한데 이는 <code>collect</code>가 다른 많은 데이터 구조로 바뀔 수 있고, 러스트는 특정하지 않으면 어떤 것을 원하는지 모르기 때문이다. 

### 해쉬맵과 소유권
- <code>i32</code>와 같이 <code>Copy</code> 트레잇을 구현한 타입에 대하여, 그 값들은 해쉬맵 안으로 복사된다. <code>String</code>과 같이 소유권 값들에 대해서는, 아래의 같이 값들이 이동되어 해쉬맵이 그 값들에 대한 소유자가 될 것이다. 

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name과 field_value은 이 지점부터 유효하지 않는다.
// 이들을 이용하는 시도를 해보고 어떤 컴파일러 에러가 나오는지 확인.
```

- <code>insert</code>를 호출하여 <code>field_name</code>과 <code>field_value</code>를 해쉬맵으로 이동시킨 후에는 더 이상 이 둘을 사용할 수 없다. 
- 만일 우리가 해쉬맵에 값들의 참조자들을 삽입한다면, 이 값들은 해쉬맵으로 이동되지 않을 것이다. 하지만 참조자가 가리키고 있는 값은 해쉬맵이 유효할 때까지 계속 유효해야 한다.

### 해쉬맵 내의 값 접근하기 

- <code>get</code>메서드에 키를 제공하여 해쉬맵으로부터 값을 얻어올 수 있다.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

- 여기서 <code>score</code>는 블루 팀과 연관된 값을 가지고 있을 것이고, 결과값은 <code>Some(&10)</code>일 것이다. 결과값은 <code>Some</code>으로 감싸져 있는데 왜냐하면 <code>get</code>이 <code>Option<&V></code>를 반환하기 때문이다. 만일 해쉬맵 내에 해당 키에 대한 값이 없다면 <code>get</code>은 <code>None</code>을 반환한다. 

- 벡터에서 했던 방법과 유사한 식으로 <code>for</code> 루프를 이용하여 해쉬맵에서도 각각의 키/값 쌍에 대한 반복작업을 할 수 있다.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

이 코드의 결과값은 다음과 같다. 

```rust
Yellow: 50
Blue: 10
```

### 해쉬맵 갱신하기 

- 키와 값의 개수가 증가할 수 있을지라도, 각각의 개별적인 키는 한번에 연관된 값 하나만을 가질 수 있다. 
- 해쉬맵 내의 데이터를 변경하길 원한다면, 키에 이미 값이 할당되어 있을 경우에 대한 처리를 어떻게 할지 결정해야 한다. 
- 값을 덮어쓸 수도 있고 새 값을 추가할 수도 있고, 두 개를 조합할 수도 있다. 

1. 값을 덮어쓰기 

- 만일 해쉬맵에 키와 값을 삽입하고, 그 후 똑같은 키에 다른 값을 삽입하면, 키에 연관지어진 값은 새 값으로 대신될 것이다. 
- 아래 예제에서는 <code>insert</code>를 두 번 호출함에도, 해쉬맵은 딱 하나의 키/값 쌍을 담게 될 것인데 그 이유는 두 번 모두 블루 팀의 키에 대한 값을 삽입하고 있기 떄문이다. 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);

// 결과값, 원래 값 10은 덮여쓰여 진다. 
{"Blue": 25}
```

2. 키에 할당된 값이 없을 경우에만 삽입하기 

- 특정 키가 값을 가지고 있는지 검사하고, 만일 가지고 있지 않다면 이 키에 대한 값을 삽입하고자 하는 경우 <code>entry</code>라고 하는 것을 사용해야 한다. 이는 우리가 검사하고자 하는 키를 파라미터로 받는다. <code>entry</code>함수의 리턴값은 열거형 <code>Entry</code>인데, 해당 키가 있는지 혹은 없는지를 나타낸다. 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

/// 결과값 
{"Yellow": 50, "Blue": 10} 
```

3. 예전 값을 기초로 값을 갱신하기 

- 해쉬맵에 대한 또다른 흔한 사용 방식은 키에 대한 값을 찾아서 예전 값에 기초하여 값을 갱신하는 것이다. 
- 아래 코드는 해쉬맵을 이용하여 해당 단어가 몇번이나 나왔는지를 유지하기 위해 값을 증사키니는 코드이다. 만일 처음 본 것이라면 0을 넣을 것이다.

```rust
use std::collections:HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count - map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);

/// 결과 
{"world": 2, "hello": 1, "wonderful": 1}
```

- 여기서 <code>or_insert</code> 메소드는 실제로는 해당 키에 대한 값의 가변 참조자 <code>(&mut V)</code>를 반환한다. 여기서는 <code>count</code> 변수에 가변 참조자를 저장하였고, 여기에 값을 할당하기 위해 먼저 애스터리스크 <code>(*)</code>를 사용하여 <code>count</code>를 역참조해야 한다. 가변 참조자는 <code>for</code> 루프의 끝에서 스코프 밖으로 벗어나고, 따라서 모든 값들의 변경은 안전하며 빌림 규칙에 위배되지 않는다. 

### 해쉬 함수 

- 기본적으로 <code>HashMap</code>은 서비스 거부 공격(Dos)에 저항 기능을 제공할 수 있는 암호학적으로 보안되는 해쉬 함수를 사용한다.
  이는 사용 가능한 가장 빠른 해쉬 알고리즘은 아니지만, 성능을 떨어트리면서 더 나은 보안을 취하는 거래는 가치가 있다.
- 만일 코드를 프로파일하여 기본 해쉬 함수가 목표에 관해서는 느리다면, 다른 <code>hasher</code>를 특정하여 다른 함수로 바꿀 수 있다. 
- 해쉬어는 <code>BuildHasher</code> 트레잇을 구현한 타입을 말한다. (10장에서 더 말한다.)
  근데 어짜피 직접 구현할 필요없고 [creates.io](https://crates.io/)에서 많은 범용적인 해쉬 알고리즘을 구현한 해쉬어를 제공하는 공유 라이브러리를 제공한다.

### 정리 

- 백터, 스트링, 그리고 해쉬맵은 프로그램 내에서 데이터를 저장하고, 접근하고, 수정하고 싶어하는 곳마다 필요한 수많은 기능들을 제공한다.

</details>


# 9. 에러 처리 

> 러스트의 신뢰성에 대한 약속은 에러 처리에도 확장되어 있다. 
  러스트는 보통 많은 경우 에러가 발생할 가능성을 발견하고 코드가 컴파일되기 전에 수정을 요청한다. 이러한 요구사항은 코드를 제품으로서 배포하기 전에 에러를 발견하고 조치할 것이라고 보장함으로써 프로그램을 더 강건하게 해준다.

> 러스트는 에러를 두 가지 범주로 묶는다.
    1. 복구 가능한 에러: 사용자에게 문제를 보고하고 연산을 재시도하는 것이 보통 합리적인 경우인데, (이를테면 파일을 찾지 못하는 에러) 복구 불가능한 에러는 언제나 버그의 증상이 나타난다. 예를 들면 배열의 끝을 넘어선 위치의 값에 접근하려고 하는 등의 경우이다.
    2. 복구 불가능한 에러: 말그대로 복구 불가능한 에러

  러스트는 예외 처리 기능이 없다. 대신 복구 가능한 에러를 위한 <code>Result<T, E></code>값과 복구 불가능한 에러가 발생했을 때 실행을 멈추는 <code>panic!</code> 매크로를 가지고 있다. 
    이번 장에서는 <code>panic!</code>을 호출하는 것을 먼저 다룬 뒤, <code>Result<T, E></code>값을 반환하는 것에 대해 이야기 한다. 추가로, 에러로부터 복구를 시도할지 아니면 실행을 멈출지를 결정할 때 고려할 것에 대해 탐구해 보겠다. 

## 9.1 panic!과 함께하는 복구 불가능한 에러

<details>
    <summary>자세히 보기</summary>

> 에러 통제를 위해 rust는 <code>panic!</code> 매크로를 가지고 있다. 이 매크로가 실행되면, 프로그램은 실패 메세지를 출력하고, 스택을 되감고 청소하고, 그 후 종료된다. 
  이런 일이 발생하는 가장 흔한 상황은 어떤 종류의 버그가 발견되었고 프로그래머가 이 에러를 어떻게 처리할지가 명확하지 않을 때이다. 

### panic!엥 응하여 스택을 되감거나 그만두기

> 기본적으로, panic!이 발생하면, 프로그램은 되감기(unwinding) 를 시작하는데, 이는 러스트가 패닉을 마주친 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를 제거한다는 뜻이지만, 이 훑어가기 및 제거는 일이 많다.  
  다른 대안으로는 즉시 <code>그만두기(abort)</code> 가 있는데, 이는 데이터 제거 없이 프로그램을 끝내는 것이다. 프로그램이 사용하고 있던 메모리는 운영체제에 의해 청소될 필요가 있을 것이다. 프로젝트 내에서 결과 바이너리가 가능한 작아지기를 원한다면, Cargo.toml 내에서 적합한 [profile] 섹션에 <code>panic = 'abort'</code>를 추가함으로써 되감기를 그만두기로 바꿀 수 있다. 
  예를 들면, 릴리즈 모드 내에서는 패닉 상에서 그만두기를 쓰고 싶다면, 다음을 추가하면 된다.

  ```rust
  [profile.release]
  panic = 'abort'
  ```

ex) 단순한 프로그램 내에서 <code>panic!</code>호출하기

```rust
fn main() {
    panic!("crash and burn");
}
```

- 이 프로그램을 실행하면, 다음과 같은 것을 보게 된다. 

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

### panic! 백트레이스 사용하기 
> 다음을 통해 직접 매크로를 호출하는 대신 코드의 버그 때문에 <code>panic!</code>호출이 라이브러리로부터 발생될 때는 어떻게 되는지 본다. 

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```
> 여기서 벡터의 100번째 요소에 접근하기를 시도하지만 벡터는 3개의 요소만 가지고 있다. 이러면 러스트는 패닉을 일으킬 것이다. 
> 여기서 C같은 경우는 백터 내에 해당 요소와 상응하는 위치의 메모리에 들어 있는 무언가를 얻을 것이다. (설령 그 메모리 영역이 벡터 소유가 아닐지라도) -> 이것을 <code>오버리드(buffer overread)</code>라고 부른다. 
  러스트는 이러한 종류의 취약점으로 보호하기 위해 실행을 멈추고 계속하기를 거부할 것이다. 

```rust
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', /stable-dist-rustc/build/src/libcollections/vec.rs:1362
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```
- 위 에러는 우리가 작성하지 않은 파일인 libcollections/vec.ts를 가리키고 있다. 이는 표준 라이브러리 내에 있는 <code>Vec<T></code>의 구현 부분이다. 우리가 벡터 v에 []를 사용할 때 실행되는 코드는 libcollections/vec.rs안에 있으며, 그곳이 바로 <code>panic!</code>이 실제 발생한 곳이다. 

- 그 다음 노트는 <code>RUST_BACKTRACE</code>환경 변수를 설정하여 에러의 원인이 된 것이 무엇인지 정확하게 백트레이스할 수 있다고 말해주고 있다. <code>백트레이스(backtrace)</code>란 어떤 지점에 도달하기까지 호출해온 모든 함수의 리스트를 말한다. 
- <code>백트레이스(backtrace)</code>를 읽는 요령은 위에서부터 시작하여 우리가 작성한 파일이 보일 때까지 읽는 것이다. 그곳이 바로 문제를 일으킨 지점이다. 

</details>

## 9.2 Result와 함께하는 복구 가능한 에러

<details>
    <summary>자세히 보기</summary>

- 우리가 어떤 파일을 여는데 해당 파일이 존재하지 않아서 연산에 실패했다면, 프로세스를 멈추는 대신 파일을 새로 만드는 것을 원할지도 모른다. 

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- <code>T</code>와 <code>E</code>는 제네릭 타입 파라미터이다. ->  <code>T</code>는 성공한 경우에 <code>Ok variant</code> 내에 반환될 값의 타입을 나타내고 <code>E</code>는 실패한 경우에 <code>Err variant</code> 내에 반환될 에러의 타입을 나타낸다. 
- <code>Result</code>가 이러한 제네릭 타입 파라미터를 갖기 때문에, 우리가 반환하고자 하는 성공적인 값과 에러 값이 다를 수 있는 다양한 상황 내에서 표준 라이브러리에 정의된 <code>Result</code> 타입과 함수들을 사용할 수 있다.

- 아래 예제에서 실패할 수도 있기 때문에 <code>Result</code>값을 반환하는 함수를 호출해 본다. 

```rust
use std::fs::File;

fn main() {
    let f:u32 = File::open("hello.txt");
}
```
> 다음과 같이하고 컴파일을 시도한다면 다음 메세지가 나타난다.

```rust
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  = note:    found type `std::result::Result<std::fs::File, std::io::Error>`
```
> 위의 에러는 <code>File::open</code> 함수의 반환 타입이 <code>Result<T,E></code>라는 것을 알려준다. 여기서 제네릭 파라미터 <code>T</code>는 성공값의 타입인 <code>std::fs::File</code>로 채워져 있는데, 이것은 파일 핸들이다. 에러에 사용되는 <code>E</code>의 타입은 <code>std::io::Error</code> -> 이 반환 타입은 <code>File::open</code>을 호출하는 것이 성공하여 우리가 읽거나 쓸 수 있는 파일 핸들을 반환해 줄 수도 있다는 뜻이다. -> 함수 호출은 또한 실패할 수도 있다.

> <code>File::open</code>이 성공한 경우, 변수 <code>f</code>가 가지게 될 값은 파일 핸들을 담고 있는 <code>Ok</code> 인스턴스가 될 것이다. 실패한 경우, <code>f</code>의 값은 발생한 에러의 종류에 대한 더 많은 정보를 가지고 있는 <code>Err</code>의 인스턴스가 될 것이다. 

- 다음 예제는 <code>File::open</code>이 반환하는 값에 따라 다른 행동을 취하는 코드를 추가할 필요가 있다. -> <code>match</code>표현식을 이용하여 <code>Result</code>를 처리하는 한 가지 방법을 보여준다. 

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

- <code>Option</code> 열거형과 같이 <code>Result</code> 열거형과 <code>variant</code>들은 <code>프렐루드(prelude)</code>로부터 가져와진다는 점을 기억해야 한다. 따라서 <code>match</code>의 각 경우에 대해서 <code>Ok</code>와 <code>Err</code> 앞에 <code>Result::</code>를 특정하지 않아도 된다.

- 여기서 우리는 러스트에게 결과가 <code>Ok</code>일 때에는 <code>Ok variant</code>로부터 내부의 <code>file</code> 값을 반환하고, 이 파일 핸들 값을 변수 <code>f</code>에 대입한다고 말해주고 있다. <code>match</code> 이후에는 읽거나 쓰기 위해 이 파일 핸들을 사용할 수 있다.

- <code>match</code>의 다른 경우는 <code>File::open</code>으로부터 <code>Err</code>를 얻은 경우를 처리한다. 

- 이 예제에서 hello.txt라는 이름의 파일이 없는데 이 코드를 실행하게 되면, <code>panic!</code> 매크로로부터 다음과 같은 호출을 보게 될 것이다 .

```rust
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```
- 늘 그렇듯 에러가 발생했다...

### 서로 다른 에러에 대해 매칭하기 

- <code>File:open</code>이 실패한 이유가 무엇이든 간에 <code>panic!</code>을 일으킬 것이다. 대신 우리가 원하는 것은 실패 이유에 따라 다른 행동을 취해야 한다. <code>File:open</code>이 실패한 것이라면, 새로운 파일을 만들어서 핸들을 반환해보자. 만일 그 밖의 이유로 <code>File:open</code>이 실패한 거라면, 예를 들어 파일을 열 권한이 없어서라면 마찬가지로 <code>panic!</code>을 일으키고 한다. <code>match</code>에 새로운 경우를 추가한 예제를 작성한다. 

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

- <code>Err variant</code> 내에 있는 <code>File::open</code>이 반환하는 값의 타입은 <code>io::Error</code>인데, 이는 표준 라이브러리에서 제공하는 구조체이다. 이 구조체는 <code>kind</code> 메소드를 제공하는데 이를 호출하여 <code>io:ErrorKind</code> 값을 얻을 수 있다. 

- <code>io::ErrorKind</code>는 io 연산으로부터 발생할 수 있는 여러 종류의 에러를 표현하는 <code>variant</code>를 가진, 표준 라이브러리에서 제공하는 열거형이다. 우리가 사용하고자 하는 <code>variant</code>는 <code>ErrorKind::NotFound</code>인데, 이는 열고자 하는 파일이 아직 존재하지 않음을 나타낸다.

- 조건문 <code>if error.kind() == ErrorKind::NotFound</code>는 매치 가드(match guard) 라고 부른다: 이는 <code>match</code> 줄기 상에서 줄기의 패턴을 좀 더 정제해주는 추가 조건문이다.

- 그 줄기의 코드가 실행되기 위해서는 이 조건문이 참이어야 한다; 그렇지 않다면, 패턴 매칭은 <code>match</code>의 다음 줄기에 맞춰보기 위해 이동할 것이다. 패턴에는 <code>ref</code>가 필요하며 그러므로써 <code>error</code>가 가드 조건문으로 소유권 이동이 되지 않고 그저 참조만 된다. 패턴 내에서 참조자를 얻기 위해 <code>&</code> 대신 <code>ref</code>가 사용되는 이유는 <code>&</code>는 참조자를 매치하고 그 값을 제공하지만, <code>ref</code>는 값을 매치하여 그 참조자를 제공한다. 

- 매치 가드 내에서 확인하고자 하는 조건문은 <code>error.kind()</code>에 의해 반환된 값이 <code>ErrorKind</code> 열거형의 <code>NotFound</code> variant인가 하는 것이다. 만일 그렇다면 <code>File:create</code>로 파일 생성을 시도한다. 그러나 <code>File::create</code> 또한 실패할 수 있기 때문에, 안쪽에 <code>match</code> 구문을 바깥쪽과 마찬가지로 추가할 필요가 있다. 파일이 열 수 없을 때, 다른 에러 메세지가 출력될 것이다. 바깥쪽 <code>match</code>의 마지막 갈래는 똑같이 남아서, 파일을 못 찾는 에러 외에 다른 어떤 에러에 대해서도 패닉을 일으킨다. 

### 에가 났을 때 패닉을 위한 숏컷: unwrap과 expect
- <code>match</code>의 사용은 충분히 잘 동작하지만, 의도를 항상 잘 전달하는 게 아니다. <code>Result<T, E></code>타입은 다양한 작업을 하기 위해 정의된 수많은 헬퍼 메소드를 가지고 있다. 그 중 하나인 <code>unwrap</code>이라 부르는 메소드는 <code>match</code>구문과 비슷한 구현을 한 숏컷 메소드이다. 
- 만일 <code>Result</code>값이 Ok variant라면, <code>unwrap</code>은 Ok 내의 값을 반환할 것이다. 만일 <code>Result</code>가 Err variant라면, <code>unwrap</code>은 우리를 위해 <code>panic!</code> 매크로를 호출할 것이다. -> 아래 예제는 <code>unwrap</code>이 작동하는 예이다.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
> 여기서 hello.txt 파일이 없는 상태에서 이 코드를 실행시키면, <code>unwrap</code>메소드에 의한 <code>panic!</code> 호출로부터의 에러 메세지를 보게 될 것이다. 

```rust
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
/stable-dist-rustc/build/src/libcore/result.rs:868
```
> 또 다른 메소드인 <code>expect</code>는 <code>unwrap</code>과 유사한데, 우리가 <code>panic!</code> 에러 메시지를 선택할 수 있게 해준다. 
  <code>unwrap</code> 대신 <code>expect</code>를 이용하고 좋은 에러 메세지를 제공하는 것은 의도를 전달해주고 패닉의 근원을 추적하는 걸 쉽게 해줄 수 있을 것이다. 

- 아래는 <code>expect</code>의 예제이다. 

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```
> <code>expect</code>는 <code>unwrap</code>과 같은 식으로 사용한다.
  <code>expect</code>가 <code>panic!</code> 호출에 사용하는 에러 메세지는 <code>unwrap</code>이 사용하는 기본 <code>panic!</code> 메세지보다는 <code>expect</code>에 넘기는 파라미터로 설정될 것이다.

```rust
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }',
/stable-dist-rustc/build/src/libcore/result.rs:868
```

- 이 에러 메세지는 우리가 특정한 텍스트인 <code>Failed to open hello.txt</code>로 시작하기 때문에, 이 에러 메세지가 어디서부터 왔는지를 코드 내에서 찾기가 더 수월해질 것이다. 
  만일 우리가 여러 군대에 <code>unwrap</code>을 사용하면, 정확히 어떤 <code>unwrap</code>이 패닉을 일으켰는지 찾기에 더 많은 시간이 걸릴 수 있는데, 그 이유는 패닉을 호출하는 모든 <code>unwrap</code>이 동일한 메세지를 출력하기 때문이다.

### 에러 전파하기 
- 실패할지도 모르는 무언가를 호출하는 구현을 가진 함수를 작성할 때, 이 함수 내에서 에러를 처리하는 대신, 에러를 호출하는 코드 쪽으로 반환하여 그쪽에서 어떻게 할지 결정하도록 할 수 있다. 
  이는 <code>에러 전파하기</code>로 알려져 있으며, 에러가 어떻게 처리해야 좋을지 좌우해야 할 상황에서, 우리의 코드 내용 내에서 이용 가능한 것들보다 더 많은 정보와 로직을 가지고 있을 수도 있는 호출하는 코드 쪽에 더 많은 제어권을 준다.
- 예시 -> 파일로부터 사용자 이름을 읽는 함수를 작성한 것이다. 만일 파일이 존재하지 않거나 읽을 수 없다면, 이 함수는 호출하는 코드 쪽으로 해당 에러를 반환할 것이다.

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

1. 함수의 반환 타입부터 먼저 살펴본다.: <code>Result<String, io::Error>.</code> 이는 함수가 <code>Result<T, E></code> 타입의 값을 반환하는데 제네릭 파라미터 T는 구체적 타입(concrete type)인 <code>String</code>로 채워져 있고, <code>제네릭 타입 E</code>는 구체적 타입인 <code>io::Error</code>로 채워져 있다. 
2. 만일 이 함수가 어떤 문제 없이 성공하면, 함수를 호출한 코드는 String을 담은 값을 받을 것이다.-> 이 함수가 파일로부터 읽어들인 사용자 이름이다. 만일 어떤 문제가 발생한다면, 이 함수를 호출한 코드는 문제가 무엇이었는지에 대한 더 많은 정보를 담고 있는 <code>io::Error</code>의 인스턴스를 담은 <code>Err</code> 값을 받을 것이다.
3. 이 함수의 반환 타입으로서 <code>io::Error</code>를 선택했는데, 그 이유는 우리가 이 함수 내부에서 호출하고 있는 실패 가능한 연산 두 가지가 모두 이 타입의 에러 값을 반환하기 때문 -> <code>File::open</code> 함수와 <code>read_to_string</code> 메소드

- 러스트에서 에러를 전파하는 패턴은 너무 흔하여 러스트에서는 이를 더 쉽게 해주는 물음표 연산자 <code>?</code>를 제공한다. 

### 에러를 전파하기 위한 숏컷: ? 

> 위에 연산자를 알아보기 위해 예제를 준비했음. 
  <code>read_username_from_file</code>의 구현을 보여주는데, 다만 이 구현은 물음표 연산자를 이용하고 있다.
  <code>?</code>는 많은 수의 보일러플레이트(boilerplate)를 제거해주고 함수의 구현을 더 단순하게 만들어 준다.

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

- <code>f</code> 변수를 만드는 대신, <code>File::open("hello.txt")?</code>의 결과 바로 뒤에 <code>read_to_string</code>의 호출을 연결시켰다.
- <code>read_to_string</code> 호출의 끝에는 여전히 <code>?</code>가 남아있고, <code>File::open</code>과 <code>read_to_string</code>이 모두 에러를 반환하지 않고 성공할 때 <code>s</code> 안의 사용자 이름을 담은 <code>Ok</code>를 반환한다.

### ?는 Result를 반환하는 함수에서만 사용될 수 있다. 

- <code>?</code>는 <code>Result</code> 타입을 반환하는 함수에서만 사용이 가능한데, <code>match</code> 표현식과 동일한 방식으로 동작하도록 정의되어 있기 때문이다.
- <code>Result</code> 반환 타입을 요구하는 <code>match</code> 부분은 <code>return Err(e)</code>이며, 따라서 함수의 반환 타입은 반드시 이 <code>return</code>과 호환 가능한 <code>Result</code>가 되어야 한다. 

> <code>main</code>의 반환 타입이 ()라는 것을 상기하면서, 만약 <code>main</code> 함수 내에서 <code>?</code>를 사용하면 어떤 일이 생길까?

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```
- 다음과 같은 에러 메세지가 호출된다. 

```rust
error[E0277]: the `?` operator can only be used in a function that returns
`Result` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ------------------------
  |             |
  |             cannot use the `?` operator in a function that returns `()`
  |             in this macro invocation
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
```
- 이 에러는 오직 <code>Result</code>를 반환하는 함수 내에서만 물음표 연산자를 사용할 수 있음을 지적한다. 
- <code>Result</code>를 반환하지 않는 함수 내에서, <code>Result</code>를 반환하는 다른 함수를 호출했을 때, <code>?</code>를 사용하여 호출하는 코드에게 잠재적으로 에러를 전파하는 대신 <code>match</code>나 <code>Result</code>에서 제공하는 메소드들 중 하나를 사용하여 이를 처리할 필요가 있다.

</details>

## 9.3 panic!이냐, panic!이 아니냐, 그것이 문제로다.

<details>
    <summary>자세히 보기</summary>

- <code>panic!</code>은 에러가 복구 불가능하다고 결정을 내리는 것이라 보통 <code>Result</code>를 반환하는 것이 일반적이고 더 적합하다. 
- 하지만 가끔씩 써야할 때도 있는데 언제 써야하는 지 알아보자. 

1. 예제, 프로토타입 코드, 그리고 테스트는 전부 패닉을 일으켜도 완전 괜찮은 곳이다.
   오히려 패닉을 써야하기도 한다.

2. 컴파일러보다 우리가 더 많은 정보를 가지고 있을 때
   <code>Result</code>가 Ok 값을 가지고 있을 거라 확신할 다른 논리를 가지고 있지만, 그 논리가 컴파일러에 의해 이해라 수 있는 것이 아닐 때라면, <code>unwrap</code>을 호출하는 것이 또한 적절할 수 있다. 

3. 에러 처리를 위한 가이드라인 
    > 나쁜 상태란 어떤 가정, 보장, 계약, 혹은 불변성이 깨질 때를 뜻하는 것으로 이를테면 유효하지 않은 값이나 모순되는 값, 혹은 찾을 수 없는 값이 코드를 통과할 경우를 말한다. 
    1. 나쁜 상태란 것이 가끔 벌어질 것으로 예상되는 무언가가 아니다.
    2. 그 시점 이후의 코드는 이 나쁜 상태에 있지 않아야만 할 필요가 있다
    3. 사용하고 있는 타입 내에 이 정보를 집어 넣을만한 뽀족한 수가 없다.

### 유효성을 위한 커스텀 타입 생성하기 
- 러스트의 타입 시스템을 이용하여 유효한 값을 보장하는 아이디어에서 한 발 더 나가서, 유효성을 위한 커스텀 타입을 생성하는 것을 살펴본다. 

- 만약 1 ~ 100까지 입력을 해야 하는데 사용자가 음수를 입력하는 것을 예제로 살펴본다. 

```rust
loop {
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // snip
}
```
- 음수를 입력할 수 있게 <code>i32</code>로 반환값을 지정 
>  우리는 새로운 타입을 만들어서, 유효성 확인을 모든 곳에서 반복하는 것보다는 차라리 그 타입의 인스턴스를 생성하는 함수 내에 유효성 확인을 넣을 수 있다. 
   이 방식에서, 함수가 그 시그니처 내에서 새로운 타입을 이용하고 받은 값을 자신 있게 사용하는 것은 안전하다. 
   <code>Guess</code>타입을 정의한다.

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```

1. <code>u32</code>를 갖는 <code>value</code>라는 이름의 항목을 가진 <code>Guess</code>라는 이름의 구조체를 선언하였다. 
2. 그런 뒤 <code>Guess</code> 값의 인스턴스를 생성하는 <code>new</code>라는 이름의 연관 함수를 구현한다.
3. 만일 <code>value</code>가 이 테스트에 통과하지 못하면 <code>panic!</code>을 호출하며, 이는 이 코드를 호출하는 프로그래머에게 고쳐야 할 버그가 있음을 알려주는데, 범위 밖의 <code>value</code>를 가지고 <code>Guess</code>를 생성하는 것은 <code>Guess::new</code>가 필요로 하는 계약을 위반하기 때문이다.

4. 다음으로 <code>self</code>를 빌리고, 파라미터를 갖지 않으며, <code>u32</code>를 반환하는 <code>value</code>라는 이름의 메소드를 구현한다. 이러한 종류 메서드를 종종 <code>게터(getter)</code>라고 부르는데, 그 이유는 이런 함수의 목적이 객체의 항목으로부터 어떤 데이터를 가져와서 이를 반환하는 것이기 때문이다. 
5. 모듈 밖의 코드는 반드시 <code>Guess::new</code> 함수를 이용하여 새로운 <code>Guess</code>의 인스턴스를 만들어야 하는데, 이는 <code>Guess</code>가 <code>Guess::new</code> 함수의 조건들을 확인한 적이 없는 <code>value</code>를 갖는 방법이 없음을 보장한다.

</details>

# 10. 제네릭 타입, 트레잇, 그리고 라이프타임

> 모든 프로그래밍 언어는 컨셉의 복제를 효율적으로 다루기 위한 도구를 가지고 있다. 그러한 도구 중 하나가 바로 <code>제네릭</code>이다. 
제네릭은 구체화된 타입이나 다른 속성들에 대하여 추상화된 대리인이다. 코드를 작성하고 컴파일할 때, 제네릭들이 실제로 어떻게 완성되는지 알 필요 없이, 제네릭의 동작 혹은 다른 제네릭과 어떻게 연관되는지와 같은 제네릭에 대한 속성을 표현할 수 있다. 
  여러 개의 구체화된 값들에 대해 실행될 코드를 작성하기 위해서 함수가 어떤 값을 담을지 알 수 없는 파라미터를 갖는 것과 동일한 방식으로, <code>i32</code>나 <code>String</code>과 같은 구체화된 타입 대신 몇몇 제네릭 타입의 파라미터를 갖는 함수를 작성할 수 있다. ex) <code>Option<T></code>, <code>Vec<T></code> 등 등 
  <code>트레잇</code>에 대해서도 논의할 것인데, 이는 동작을 제네릭 한 방식으로 정의하는 방법을 말한다.
  <code>트레잇</code>은 제네릭 타입과 결합되어 제네릭 타입에 대해 아무 타입이나 허용하지 않고, 특정 동작을 하는 타입으로 제한할 수 있다.
  마지막으로 <code>라이프타임</code>에 대해서도 다룰 것인데, 이는 제네릭의 일종으로서 우리가 컴파일러에게 참조자들이 서로에게 어떤 연관이 있는지에 대한 정보를 줄 수 있도록 해준다. 라이프타임은 수많은 상황에서 값을 빌릴 수 있도록 허용해 주고도 여전히 참조자들이 유요할지를 컴파일러가 검증하도록 해주는 러스트의 지능이다. 
