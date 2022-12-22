https://rinthel.github.io/rust-lang-book-ko/

# 3.1 변수와 가변성

### 변수와 가변성

> Rust에서 기본 변수는 불변성이다. 변수가 불변성인 경우, 일단 값이 bound되면 해당 값을 변경할 수 없다.
> 만약에 불변성으로 선언한 것의 값을 변경하고자 하는 시도를 하면 컴파일 타임의 에러를 얻게 된다.

- mut를 사용해서 불변성 변수를 가변성 변수로 변경할 수 있다.

<code>
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
</code>

### 변수와 상수 간의 차이점들

> 불변성 변수와 마찬가지로 상수 또한 이름으로 bound된 후에는 값의 변경이 허용되지 않지만, 상수와 변수는 조금 다르다.

1. 상수는 const 키워드를 사용해야 하며 mut를 허용하지 않는다. -> 불변성 그 자체이다.
2. 상수는 전체 영역을 포함하여 어떤 영역에서도 선언될 수 있다. -> 코드의 많은 부분에서 사용될 필요가 있는 값을 다루는데 유용하다.
3. 상수는 오직 상수 표현식만 설정될 수 있지, 함수 호출의 결과값이나 그 외의 실행 시간에 결정되는 값이 설정될 수 없다.
   ex)
   <code>
   const MAX_POINTS: u32 = 100_000;
   </code>

### Shadowing

> 이전에 선언한 변수와 같은 새 변수를 선언할 수 있고, 새 변수는 이전 변수를 shadows를 하게 된다. -> 러스트인들은 이를 첫 변수가 두 번째에 의해 shadowed 됐다고 표현한다.
> let 키워드를 사용해서 반복하여 같은 변수명으로 변수를 shadow할 수 있다.

다음 예제는 x에 처음 5를 bind를 하고 shadow하여 원본에 1를 더해서 6더하고 이런 식으로 반복한다.

<code>
    let x = 5;
    let x = x + 1;
    let x = x + 2;

    println!("The value of x is: {}", x);

</code>

그런데 만약에 let를 사용하지 않고 mut로 선언하게 되면 컴파일 시 에러를 얻게 된다.

### mut과 let의 차이점

즉, 변수를 지속적으로 변하게 싶으면 let을 쓰고 변수를 새로 할당하고 싶으면 mut 키워드를 쓰면 될 것 같다.

# 3.2

> 기억해야할 것은 Rust에는 변수가 어떤 형태의 데이터인지 명시해줘야한다. -> 타입은 고정적이다.
> 데이터의 타입은 크게 스칼라와 컴파운드, 둘로 나뉜다.

1. 스칼라

스칼라는 하나의 값으로 표현되는 타입이다. (정수, 부동소수점 숫자, boolean, 문자)

- 정수형 : 소수점이 없는 숫자이다. 2장에서는 u32타입인 정수형을 사용했었다. -> 부호 없는 32비트 변수임을 나타냄. -> 만약 부호가 있다면 i32처럼 u대신 i로 시작함.

## Length Signed Unsigned

8-bit i8 u8
16-bit i16 u16
32-bit i32 u32
64-bit i64 u64
arch isize usize

여기서 부호, 미부호의 의미는 숫자가 양수 혹은 음수를 다룰 수 있는지 혹은 없는지를 나타낸다. -> 만약 오직 양수만을 가질 것이면 부호가 없이도 표현할 수 있기 때문에 미부호로 나타낸다.

각 부호 변수는 -(2n - 1) 부터 2n - 1 - 1 까지의 값을 포괄한다. 여기서 n은 사용되는 타입의 비트 수 입니다. 즉, i8은 -(27) 에서 27 - 1 까지의 값, 즉 -128 에서 127 사이의 값을 저장할 수 있다.
미부호 타입은 0 에서 2n - 1 까지의 값을 저장할 수 있다. 즉, u8 타입은 0 에서 28 - 1 다시 말해, 0 에서 255 까지의 값을 저장할 수 있습니다.

isize와 usize 타입은 프로그램이 동작하는 컴퓨터 환경이 64-bits인지 32-bit인지에 따라 쓰인다.

- 정수형 리터럴

## Number literals Example

Decimal 98_222
Hex 0xff
Octal 0o77
Binary 0b1111_0000
Byte (u8 only) b'A'

일반적인 경우에는 (심지어 64bit환경에서도) i32가 일반적으로 좋은 선택이다.

- 부동 소수점 타입
  32bit와 64bit의 크기를 가진 f32와 f64가 있다.

ex)
<code>
let x = 2.0; // f64
let y:f32 = 3.0; // f32
</code>
f32타입은 1배수의 정밀도인 부동소수점이고, f64는 2배수의 정밀도인 부동소수점이다.

- 수학적 연산들
  let을 사용하면 rust가 알아서 산출된 값을 변수로 bound한다.

ex)
<code>
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

</code>

- Boolean 타입

일반적인 다른 언어와 사용법이 같다. bool 키워드를 사용한다.

ex)
<code>
fn main() {
let t = true;

    let f: bool = false; // with explicit type annotation

}
</code>

- 문자 타입

''와 ""는 다르다.
''는 Unicode Scalar를 표현하는 값이고 이는 한국어/중국어/일본어, 이모티콘 등 모두 사용 가능한 char 타입이다. -> ""와 ''의 차이점에 대해서는 8장 String에서 더 자세히 다룬다.

- 복합 타입들

튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법이다.
괄호 안에 콤마로 구분되는 값들의 목록을 작성하여 튜플을 만든다. -> 다 달라도 된다.

ex)
<code>
let tup: (i32, f64, u8) = (500, 6.4, 1);
</code>

튜플은 단일 요소를 위한 복합계로 고려되었기에 전체가 bind된다. 개별적으로 끄내기 위해서는 구조분해를 해야한다.

ex)
<code>
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is : {}", y);
</code>

구조분해에 추가로 뒤에 마침표(.) 뒤에 접근하기 원하는 값의 색인을 넣는 것을 통해 튜플의 요소에 직접적으로 접근할 수 있다.

ex)
<code>
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
</code>

뭔 배열에서 x[0]쓰는 것과 비슷하냐 ㅋㅋㅋㅋ
당연히 튜플의 첫 번째 색인은 0이다.

- 배열

튜플과 다르게 배열의 모든 요소는 모두 같은 타입이여야 한다. 또한 고정된 길이를 갖게 되며 한번 선언되면 크기는 커지거나 작아지지 않는다. -> 가변적이게 사용하고 싶다면 8장에서 배우는 벡터를 사용해야 한다.

ex)
<code>
let a = [1, 2, 3, 4, 5];
</code>

- 배열의 요소에 접근하기

일반적인 방법과 같다.
<code>
let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

</code>

여기서 다른 언어와 다른 점은 만약 배열의 크기가 넘은 index을 호출할 경우 Rust는 프로그램이 오류와 함께 종료될 때 패닉한다. -> 메모리 접근을 허용하고 계속 진행하는 대신 즉시 종료한다. -> 실행 중에 에러가 나오는 것이다.

# 3.3 함수 동작 원리

> fn 키워드를 이용해서 함수를 만들 수 있다.
> Rust는 뱀 형태로 변수나 함수 이름의 규칙으로 사용한다. (ex some_function )

```
fn main() {
    println!("Hello world");
}

fn another_function() {
    println!("Another function.");
}
```

- 함수의 매개변수
  > 함수는 함수 고유한 부분인 특별한 변수 매개변수를 갖는 형식으로 선언될 수 있다.

```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

위의 예제는 x 매개변수를 선언하고 형식은 i32로 한 것이다.

- 함수의 표현식

```
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

여기서 y안에 x를 선언했는데, x + 1은 뒤에 세미클론이 없다는 게 특징이다.

- 반환 값을 갖는 함수
  > 함수는 그들을 호출한 코드에 값을 반환할 수 있다.
  > 그들의 타입은 화살표 (->) 뒤에 선언해야 한다.

```
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

여기서 five는 단지 5를 적어놨는데 Rust에서는 이게 함수로 허용된다.
근데 5뒤에 ;를 찍으면 콘솔에서 에러를 반환한다.

# 3.4 주석

> 러스트에서도 주석을 사용할 수 있다.

<code>
// Hello, world. 
</code>

# 3.5 제어문

> 여기서는 if표현식과 반복문에 대해서 공부해보자.

- if 표현식
  사용법

```
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

```
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

- let 구문에서 if문 사용 가능하다.

```
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

이렇게 하면 if 식에서 산추된 값이 bound되게 된다.
단, if식에 속한 각 return 값은

- loop 문
  > loop keyword는 Rust에게 그만두라고 명시하여 알려주기 전까지 코드 블럭을 반복 수행한다.
  > CTRL + C 를 통해 종료 시킬 수 있다.

```
fn main() {
    loop {
        println!("again!");
    }
}
```

- while 문
  > while 문을 통해 조건이 참일때까지만 실행하는 반복문을 만들 수 있다.

```
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

- for문 + while

```
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

근데 안정성을 높이기 위해서 다음과 같이 할 수 있다.

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

혹은 range을 역순하는 rev메소드를 사용해서 다음과 같이 할 수 있다.

```
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

# 4 소유권 이해하기

> Ownership이란 러스트가 가비지 컬렉션없이 메모리 안정성 보장을 하게 해준다. -> 러스트는 직접 메모리를 관리해주어야 한다.

## 4.1 스택과 힙 이해하기

> 스택은 기본적으로 접시와 같이 위로 쌓고 위에서부터 제거한다. 그리고 데이터를 추가하는 것을 pushing on the stack이라고 하고, 제거하는 것을 popping off the stack이라고 한다.
>
> > 이렇게 함으로써 데이터를 넣어두기 위한 공간과 데이터를 가져올 공간을 검색할 필요가 없기 때문에 (항상 스택의 꼭대기가 그 공간이기 때문에) 빠르게 동작한다.
> > 또한 스택은 모든 빠르기를 위해 스택에 담긴 모든 데이터가 결정되어 있는 고정된 크기를 가진다.

> 힙은 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터를 저장한다. -> 포인터를 써야하는데 운영체제가 충분히 커다란 힙 안의 빈 어떤 지점을 찾아서 이 곳을 사용중이라고 표시하고, 해당 지점의 포인터를 우리에게 돌려준다. 이 절차를 allocation on the heap이라고 한다. -> 포인터를 따라가야 하기 때문에 좀 느리다.

### 소유권의 규칙

1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 떄, 값은 버려진다. (dropped).

다음과 같은 변수가 있다고 하자.

```
let string = "hello";
```

여기서 기억할 것은

1. 스코프 안에서 string은 유효하다.
2. 이 유효기간은 스코프 밖으로 벗어날 때가지 지속된다.
   이것은 데이터 타입들이 스택에 저장되어있다가 스코프를 벗어날 때 스택으로부터 팝이 된다.
   그렇다면 힙에 저장되는 데이터들은 어떻게 될까??

다음과 같은 변수가 있다고 보자.

```
let string = String::from("hello");
```

여기서 더블 클론(::)는 String 타입 아래의 from 함수를 특정지을 수 있도록 해주는 네임스페이스 연산자이다.

다음과 같이하면 변수는 변할 수 있다.

```
let mut s = String::from("hello");

s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.

println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.
```

그런데 왜 스트링 리터널은 변하지 않을까??? 이거는 투 타입의 메모리를 쓰는 방식에 차이가 있다.

1. 스트링 리터널 = 컴파일 타임에 알 수 있도록 텍스트가 최종 실행 파일에 직접 하드코딩이 되었다. -> 효율적이고 빠르게 된다.
2. String타입 = String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어 졌고 힙에서 컴파일 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있다. -> 런타임에 운영체제로부터 메모리가 요청되어야 한다. String의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요하다.

여기서 2번의 경우 Rust는 GC가 없기 때문에 아주 중요한 예제이다.

```
{
    let s = String::from("hello");
}
```

String이 요구한 메모리를 운영체제에게 반납하는 자연스러운 지점이 있다. s가 스코프 밖으로 벗어날 때이다. 변수가 스코프 밖으로 벗어나면, 러스트는 우리를 위해 특별한 함수를 호출한다. 이 함수를 drop이라고 부르고, String의 개발자가 메모리를 반환하도록 하는 코드를 집어넣을 수 있다. 러스트는 } 괄호가 닫힐때 자동적으로 drop을 호출합니다.

### 변수와 데이터가 상호작용하는 방법: 이동(move)

다음과 같은 코드가 있다고 보자.

<code>
let s1 = String::from("hello");
let s2 = s1;
</code>

이렇게하면 s1는 무효화되기 때문에 얇은 복사와 비슷한 개념인 이동(move)된다. 즉, s1의 값은 s2로 옮겨지는 것이다.
만약 s1이 무효화되지 않았다면 drop됐을 때 s1, s2가 동시에 해제되기 때문에 double free라는 오류가 발생하여 메모리 안정성 버그가 생긴다.
![trpl04-04](https://user-images.githubusercontent.com/49556566/205492117-7cbf6079-ca5f-4e81-900d-7018844d960e.svg)

### 변수와 데이터가 상호작용하는 방법: 클론

만약 string의 스택 데이터 뿐 아니라 힙 데이터를 깊이 복사하기를 원한다면, clone이라는 공용 메소드를 사용해야 한다.

<code>
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
</code>

### 스택에만 있는 데이터 복사: 복사

<code>
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
</code>

- Copy가 가능한 정수형 타입들
  1. u32와 같은 모든 정수형 타입들
  2. true와 false값을 갖는 bool
  3. f64와 같은 모든 부동 소수점 타입들
  4. Copy가 가능한 타입만으로 구성된 튜플들 (i32, i32)는 copy가 되자만 (i32, String)은 안된다.

## 4.2

<code>
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
let length = s.len(); // len()함수는 문자열의 길이를 반환합니다.
(s, length)
}
</code>

여기서 String을 호출하는 함수 쪽으로 소우권을 줌으로써 calculate_length를 호출한 이후에도 여전히 String을 이용할 수 있다.
그런데 이 방법은 추천되지 않으므로 소유권을 넘기는 대신 개체에 대한 참조자(reference)를 인자로 사용되는 calculate_length 함수를 정의하고 이용할 수 있다.

<code>
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
s.len()
}
</code>

& 키워드를 사용하는 것이다.
이렇게함으로써 어떤 값을 소유권을 넘기지 않고 참조할 수 있도록 해준다. -> 참조자가 가리키는 값은 참조자가 스코프 밖으로 벗어났을 때도 메모리가 반납되지 않는다. -> 소우권을 되돌려주기 위해 값을 다시 반환할 필요도 없다.

### 빌림

함수의 파라미터로 참조자를 만드는 것을 빌림이라고 한다.

- 빌린 값을 바꾸려고 하면 오류가 리턴된다.
  <code>
  fn main() {
  let s = String::from("hello");
  change(&s);
  }
  fn change(some_string: &String) {
  some_string.push_str(", world");
  }
  </code>
  이건 오류가 리턴된다.

### 가변 참조자(Mutable References)

- mut를 사용하는 것이다.
- 단, 특정 스코프 내에서 가변 참조자는 딱 하나만 만들 수 있다.

<code>
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
some_string.push_str(", world");
}
</code>

### 댕글러 참조자(Dangling References)

댕글러 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말한다. 단, rust는 참조자들이 댕글링 참조자가 되지 않도록 보장해준다. -> 어떠한 데이터의 참조자를 만들었다면, 컴파일러는 그 참조자가 스코프 밖으로 벗어나기 전에는 데이터가 스코프 밖으로 벗어나지 않을 것임을 확인해 줄 것이다.

### 참조자의 규칙
1. 어떠한 경우이든 간, 둘 중 하나만 가질 수 있다.
    - 하나의 가변 참조자
    - 임의 개수의 불변 참조자들
2. 참조자는 항상 유효해야만 한다.

## 4.3 

### 슬라이스(Slices)
소유권을 갖지 않는 또다른 데이터 타입은 슬라이스이다. 
슬라이스는 컬렉션 전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게 한다.
[start ... end] 문법을 사용하고 start로 시작해 end는 포함하지 않는 연속 범위를 나타낸다.

```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

- 0으로 시작하면 start 생략 가능
- 끝까지 포함하면 end 생략 가능
- 전체 스트링의 슬라이스를 만든다면 양쪽 값 생략 가능 

```
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
```
let s = "Hello, world!";
```

- 배열의 슬라이스, 그리고 배열의 슬라이스는 &[i32] 타입을 갖는다.
```
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
```
struct User {
    username: String,
    email: String,
    sign: u64,
    active: bool,
}
```

> 구조체를 사용하려면 인스턴스(instance)를 생성해야 한다. key, value를 사용한다.
```
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

> 구조체는 .을 사용해서 불러오면 되고 =를 사용해서 값을 변경할 수 있다. (반드시 mutable이여야 한다.)
```
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

> 함수로 구조체를 만들 수 있다. 또한 축약법도 가능하다. 
```
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
```
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
```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```


