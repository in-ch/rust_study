fn main() {
    println!("Hello, world!");
}

// toml 파일은 Cargo의 환경설정 포맷이다. 
// cargo build를 통해 프로젝트를 빌드 -> target/debug/ 쪽에 실행 파일을 만들어 준다.
// 이 디렉토리로 이동해서 프로그램을 실행하면 된다. -> ./target/debug/hello_cargo
// 이게 귀찮으면 그냥 메인 디렉토리에서 cargo run을 통해 빌드와 실행을 동시에 실행할 수 있다. 

// cargo check을 통해 실행은 안하고 빌드가 되는지 안되는지만 확인할 수 있다. (바이너리 파일을 생성하지 않는다.)

// cargo build --release를 실행하면 target/release에 실행파일을 만들어 준다. 

