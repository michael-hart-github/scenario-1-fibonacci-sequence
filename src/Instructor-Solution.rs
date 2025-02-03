fn main() {
/* Here is the terrible solution the teacher provided. */

    /* 
        SCENARIO 1 : Fibonacci Series (Print first 10 numbers)

        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
     */
    
    let mut count = 0;
    let mut a = 0;
    let mut b = 1;

    loop {
        if count == 10 {
            break;
        }
        if count == 0 {
            println!("{}",a);
        } else if count == 1 {
            println!("{}", b);
        }else {
            let next = a+b;
            println!("{}",next); //0+1=1; 1+1=2; 2+1=3
            a = b;
            b = next; 
        }
        count += 1;
    }

}
