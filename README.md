https://rinthel.github.io/rust-lang-book-ko/

[3.1 변수와 가변성](#-3.1-변수와-가변성) 

[3.2 데이터 타입들](#32-데이터-타입들)

[3.3 함수 동작 원리](#33-함수-동작-원리)

[3.4 주석](#34-주석)

[3.5 제어문](#35-제어문)

[4.1 스택과 힙 이해하기](#41-스택과-힙-이해하기)

[4.2 참조자와 빌림](#42-참조자references와-빌림borrowing)

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


## 4.3 

### 슬라이스(Slices)
소유권을 갖지 않는 또다른 데이터 타입은 슬라이스이다. 
슬라이스는 컬렉션 전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게 한다.
[start ... end] 문법을 사용하고 start로 시작해 end는 포함하지 않는 연속 범위를 나타낸다.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

- 0으로 시작하면 start 생략 가능
- 끝까지 포함하면 end 생략 가능
- 전체 스트링의 슬라이스를 만든다면 양쪽 값 생략 가능 

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

let slice = &s[0..len];
let slice = &s[..];
```

- 스트링 리터럴은 슬라이스이다.
```rust
let s = "Hello, world!";
```

- 배열의 슬라이스, 그리고 배열의 슬라이스는 &[i32] 타입을 갖는다.
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

이런 식을 타입 스크립트처럼 param과 return의 형을 정의해줄 수 있다.

소유권, 빌림, 그리고 슬라이스의 개념은 러스트 프로그램의 메모리 안정성을 컴파일 타임에 보장하는 것이다. 러스트 언어는 다른 시스템 프로그래밍 언어와 같이 메모리 사용에 제어권을 주지만, 데이터의 소유자가 스코프 밖으로 벗아났을 때 소유자가 자동적으로 데이터를 버리도록 하는 것은 이러한 제어를 위해 추가적인 코드 작성이나 디버깅을 하지 않아도 된다는 것이다.

# 5. 연관된 데이터들을 구조체로 다루기
> Struct은 사용자들이 연관된 여러 값들을 묶어서 의미있는 데이터 단위를 정의할 수 있게 된다. (튜플과 비교된다.)
> 메소드와 구조체 데이터의 동작과 관련된 연관함수의 정의 방법에 대해 알아보도록 한다. 

## 5.1 구조체를 정의하고 초기화하기
> 구조체는 튜플과 비슷하다. 그러나 튜플과는 다르게 각 구성요소들은 명명할 수 있어 값이 의미하는 바를 명확하게 인지할 수 있다. (튜플보다 유연하다.)
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

> 구조체는 .을 사용해서 불러오면 되고 =를 사용해서 값을 변경할 수 있다. (반드시 mutable이여야 한다.)
```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

> 함수로 구조체를 만들 수 있다. 또한 축약법도 가능하다. 
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

- 구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기 
```rust
let user2 = User {
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

- 이름이 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체 
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## 5.2 구조체를 이용한 예제 프로그램
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

- 튜플을 이용한 리펙토링 
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
이제 하나의 인자만 넘기면 된다. length가 튜플 인덱스 0이고 width가 튜플 인덱스 1이라는 점을 기억해야 한다.

- 구조체를 이용한 리팩토링: 의미를 더 추가하기 
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

- 파생 트레잇(derived trait)으로 유용한 기능 추가하기
> 디버깅하는 동안 구조체 내의 모든 값을 보기 위해서 Rectangle의 인스턴스를 출력할 수 있다면 도움이 될 것 같다.
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

## 5.3 메소드
> 메소드는 함수와 유사하다. fn 키워드와 이름을 가지고 선언되고, 파라미터와 반환값을 가지고 있으며, 다른 어딘가로부터 호출되었을때 실행될 코드를 담고 있다.
> 다만 메소드는 함수와는 달리 구조체의 내용 안에 정의되며 첫번째 파라미터가 언제나 self인데, 이는 메소드가 호출되고 있는 구조체의 인스턴스를 나타낸다.

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

> Rectangle의 내용 안에 함수를 정의하기 위해서는 ,impl(구현: implementation) 블록을 시작한다. 그 다음 area함수를 impl 중괄호로 옮기고 시그니처 및 본체 내의 모든 곳에 있는 파라미터를 self로 변경시킨다. 

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
> impl 블록의 유용한 기능은 self 파라미터를 갖지 않는 함수도 impl 내에 정의하는 것이 허용된다는 것이다. 이걸 연관 함수(associated functions)라고 부른다. 
> 연관 함수는 새로운 구조체의 인스턴스를 반환해주는 생성자로서 자주 사용된다. 
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```
> 이 연관 함수를 호출하기 위해서는 <code>let sq = Rectangle::square(3);</code> 처럼, 구조체 이름과 함께 :: 문법을 이용한다. 

### 정리 
> 구조체는 커스텀 타입을 만들 수 있게 해준다. 구조체를 이용함으로써, 연관된 데이터의 조각들을 서로 연결하여 유지할 수 있으며 각 데이터 조각에 이름을 붙여 코드를 더 명확하게 만들어 줄 수 있다. 


# 열거형(enum) 정의학
> 
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
이렇게 하면 <code>IpAddrKind</code>는 우리의 코드 어디에서나 쓸 수 있는 데이터 타입이 되었다.

> 이걸 활용해서 인스턴스도 만들 수 있다.

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_type: IpAddrKind) { }

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

> 다음과 같이 처리할 수도 있다.
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

### Option 열거형과 Null 값 보다 좋은 점들.
> Option 타입은 많이 사용되는데, 값이 있거나 없을 수도 있는 아주 흔한 상황을 나타내기 때문이다.

