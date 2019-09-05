
fn main(){
  let name_of_pakistan = String::from("PAKISTAN ZINDABAD");
  let length_of_name = name_of_pakistan.len();
    
  println!("{}",name_of_pakistan);  

    println!("LENGTH OF {} IS {}",name_of_pakistan,length_of_name);
    
    let number1: i64 =  85;
   println!("{}",number1);

    let number2: i32 =  -550;
   println!("{}",number2);

   let float:f32 =56.6;
   println!("{}",float);
   addition(76,23);
   substraction(76,23);
   multiplication(76,23);
   division(76,23);
   modulos(76,23);

   let array = [100,150,200,250,300];
   println!("{:?}",array);
   println!("{:?}",array[1]);
   println!("{:?}",array[3]);


   let tuple = ("IoT","AI","Cloud",500.65,8645,65.4);
   println!("{:?}",tuple);
   println!("{:?}",tuple.3);
   println!("{:?}",tuple.4);
   println!("{:?}",tuple.5);


    fnsum(10,20,30);
   
    
   let result_flat_multiply =  fnmultiply(5.6,2.4,10.2);
    println!("{}",result_flat_multiply);

    marks_grade(95);


    let year = 2019 % 4 == 0;
    println!("This Year is Leap {}",year);

// My Logic Is Here But second idea is good 
   for x in (0..10).step_by(2) {
        println!("Even Number is {}", x);
    }



    let mut x = 0;
       while(x<10){
           if x % 2 == 1 {
    println!("\n"); 
    println!("The number {} is an Odd number.",x);
} 
          x=x+1;
       }
// dont change 0 and 1 only change 10 and 20



let table_number = 10;
for x in (1..11){
    println!("{} * {} = {}",table_number , x , x*table_number);
}



}    fn addition(x:i32, y:i32){
        let result = x+y;
        println!("x+y = {}",result)
    }
    fn substraction(x:i32, y:i32){
        let result = x-y;
        println!("x-y = {}",result)
    }
    fn multiplication(x:i32, y:i32){
        let result = x*y;
        println!("x*y = {}",result)
    }
    fn division(x:i32, y:i32){
        let result = x/y;
        println!("x/y = {}",result)
    }
    fn modulos(x:i32, y:i32){
        let result = x%y;
        println!("x%y = {}",result)
    }


    fn fnsum(x:i32, y:i32, z:i32)
    {
        // let res = x+y+z extral line use 
        println!("{}",x+y+z);
        
    }

    fn fnmultiply(a:f32 , b:f32 , c:f32)->f32{
            a*b*c
       
    }

    fn marks_grade(mark:i32){
        
        if(mark > 80 )
        {
            println!("Greater than 80 - Grade A+");
        }
        else if(mark >= 70 && mark <= 80){
            println!("Between 70 and 80 - Grade A")
        }
                else if(mark >= 60 && mark < 70)
        {
            println!("Between 60 and 70 - Grade B")
        }
                else if(mark >= 50 && mark < 60)
        {
            println!("Between 50 and 60 - Grade C")
        }
                else if(mark >= 40 && mark < 50)
        {
            println!("Between 40 and 50 - Grade D")
        }
                else if(mark < 40)
        {
            println!("Below 40 - Grade F")
        }
    }

// fn factorial(number:i32)->i32{
   
//     let mut result = 1;
//   for n in 1..=number {
//        result=n*result;
//     }


//     return result

  

// }
   // println!("Marksheet");
    //array();
    //variable();
    //emoji();
    //compound();
   //let mut x = String::new();
    //io::stdin().read_line(&mut x)
      //  .expect("Failed to read line");
   // x.int32();    

// println!("Your number is {}",x);

// if x >= 80 {
//     println!("A Grade");
// }
// else if x <= 79 && x >= 60
// {
//     println!("B grade {}",x);
// }
// else if x <= 59 && x >= 40
// {
//     println!("C grade {}",x);
// }
// else if x < 39 {
//     println!("You are Fail");
// }
// }





// fn array(){
//     let mut names = ["javed","hassan","mumtaz"];
//     println!("{:?}",names);
//     names[2]="umar";
//     println!("{:?}",names);
// }
// fn variable (){
//     let decimal_value = 50;
//     let hex_value = 0x50;
//     let octal_value = 0o50;
//     let binary_value= 0b1100;
//     let character_value = b'B';
//     println!("{}{}{}{}{}",decimal_value,hex_value,octal_value,binary_value,character_value);
// }
// fn emoji(){
// let emoj = '\u{1F601}';
// println!("Emoji{}",emoj);
// infinityloop();
// }

// fn compound (){
//     let truple = (1.5,88,"pakistan");
//     println!("Truple{:?}",truple);
//     let (x,y,z)=truple;
//     println!("Truple value get {}{}{}",x,y,z);

//     let x =[4;20];
//     println!("{:?}",x);
// }

// fn infinityloop(){
//     let mut x = 10;
//     loop{
//         println!("Hello Pakistan{}",x);
//         x=x+1;
//     }
// }
