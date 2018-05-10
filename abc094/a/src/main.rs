
macro_rules! get {
      ($t:ty) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.trim().parse::<$t>().unwrap()
          }
      };
      ($($t:ty),*) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              let mut iter = line.split_whitespace();
              (
                  $(iter.next().unwrap().parse::<$t>().unwrap(),)*
              )
          }
      };
      ($t:ty; $n:expr) => {
          (0..$n).map(|_|
              get!($t)
          ).collect::<Vec<_>>()
      };
      ($($t:ty),*; $n:expr) => {
          (0..$n).map(|_|
              get!($($t),*)
          ).collect::<Vec<_>>()
      };
      ($t:ty ;;) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.split_whitespace()
                  .map(|t| t.parse::<$t>().unwrap())
                  .collect::<Vec<_>>()
          }
      };
      ($t:ty ;; $n:expr) => {
          (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
      };
}

#[test]
fn test_1(){
   let expect = solve(3, 5, 4);   
   assert_eq!(true, expect);
}

#[test]
fn test_2(){
   let expect = solve(2, 2, 6);   
   assert_eq!(false, expect);
}

#[test]
fn test_3(){
   let expect = solve(5, 3, 2);   
   assert_eq!(false, expect);
}

#[test]
fn test_4(){
   let expect = solve(100, 100, 200);   
   assert_eq!(true, expect);
}

fn solve(a: u32, b:u32, x:u32) -> bool{
   if (a + b) >= x  &&  a <= x{
        return true; 
    }

    false 
}

fn main() {

    let (a, b, x) = get!(u32, u32, u32);
    let ans = solve(a, b, x);

    println!("{}", if ans {"YES"} else {"NO"});

}
