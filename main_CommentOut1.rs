fn main() {

/* types   
    let x: u64 = 45; 
    let f = 6.7; //f32
    let b:bool = false;

    println!("x");
    println!("f");
    println!("b");
    println!("Hello, world!");
*/

/* loop
    let mut n = 0;

    loop {
        n+=1;

        if n > 100 {
            break; //continue also works
        }
        println!("The value of n is {}",n); //print all the number for n
    }
*/

/* Vectors and loop range
    let numbers = 11..51;
    let animals = vec!["Rabbit","Dog","Cat"];
    for i in numbers {
        println! ("The number is {}",i);
        for (index,j) in animals.iter().enumerate() {  //iter for the vectors. and the enumerate for the index of the vectors
            println! ("The animals inside is {}, and his number is {}",j,index);
        }
    }
*/
}
