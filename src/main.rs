fn main() {
    /* 
        SCENARIO 1 : Fibonacci Series (Print first 10 numbers)

        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
        You are building a program to generate and print the first 10 numbers in the Fibonacci sequence.
        The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones, usually starting with 0 and 1.
     */
    
    /*  There will be at least one mutable int
        There should be 3 variables
        If I wanted to cheat, I would use a for loop.
        0+0=0
        0+1=1
        0+1=1...?
        1+1=2
        1+2=3
        2+3=5
        3+5=8
        5+8=13
        8+13=21
        13+21=34

        Let's see...

        */

    let mut num1 = 0;
    let mut num2 = 1;
    let mut sum = 0;
    let mut count = 0;
    while count<10 {
        println!("Fibonacci sequence is: {}", sum);
        sum=num1+num2;
        num1=num2;
        num2=sum;
            /* I hate this if statement with a passion!
            There's probably a better way, but I'm too stupid to see it. It's also 0330, so I'll cut myself a break. :)
            */
        if count==0{
            num1=0;
            num2=1;
        }
        count+=1;
    } 
}