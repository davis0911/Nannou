//Code to find perfect numbers less than 10000
fn main() {
    let mut sum=0.0;
    for i in 1..10001 {
        for j in 1..i+1 {
            if i%j==0 {
            sum=sum+j as f64;//calculating the sum of the factors of a number
       }
    }
        sum=sum / i as f64;
        //If the sum is twice the number then the condition for perfect number is met and the number is printed
        if sum ==2.0 {
            println!("{i}");
    }
        sum=0.0;
    }
}