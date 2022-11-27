use std::io;

fn main() {
    loop{
        println!("Input the number");

        let mut fibonnaci_number = String::new();
    
        io::stdin().read_line(&mut fibonnaci_number)
            .expect("Failed to read line");
    
        let fibonnaci_number: u32 = match fibonnaci_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        let mut index = 1;
        let mut value = vec![0];
    
        while index < fibonnaci_number {
            println!("index: {}" , index);
            println!("fibonnaci_number: {}" , fibonnaci_number);
            
            if fibonnaci_number == 2 {
                value.push(1);
                break;
            }
            let a = index - 1;
            let b = index - 2;

            value.push(&value[a as usize] + &value[b as usize]);
            index = index + 1;
        }
    
        println!("{:?}" , value);
        break;
    }
}


// 배열은 크기가 불변이라 8장 벡터 미리 찾아봐서 써봤다
// ㄹㅇ 어렵네 ㅋㅋㅋㅋㅋㅋㅋ 
// 빌드는 되는데 코드가 틀림... 추후에 더 공부해보고 고쳐보자,,