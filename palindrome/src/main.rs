//Code to find the palindrome of numbers from 10 to 110 and the number of iteration it takes to find the palindrome of a number

//Function to reverse the digits of a number
fn reverse(mut x:i64)->i64{
    let mut a=0;
    while x!=0 {
        a=a*10+ x%10;
        x=x/10;
    }
    a
}
//Function to check whether a number is a palindrome or not
fn palindrome(x:i64)->bool{
    if x==reverse(x) {
        true
    }else{
        false
    }
}
fn main() {
    let mut cnt=0;
    let mut a;
    let mut b=10;
    for i in 10..111 {
        a=i;
        while !palindrome(b) {
            if palindrome(a){
                b=a;
                break;
            }
            b=a+reverse(a); 
            a=b;
            cnt=cnt+1;
        }
        println!("The palindrome of {} is {}",i,b);
        println!("No. of times in loop {}",cnt);
        println!(" ");
        b=10;//setting b to a non palindome number to keep going in a loop to find the palindrome number
        cnt=0;//setting count to zero for every new number 
    }
    
}

