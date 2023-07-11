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