//Program to find the nth term of the fibonacci series
fn main(){
    let x=10;
    let mut a=1i128;
    let mut b=1i128;
    let mut c=0i128;
    if x==1 || x==2 {
        println!("{}",a);
    }
    else{

    
    for _i in 1..x-1 {
        c=a+b;
        a=b;
        b=c;

    }
    println!("{}",c)
    }
}
