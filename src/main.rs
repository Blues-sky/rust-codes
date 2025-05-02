struct Rectangle{w:u32,h:u32,}
impl Rectangle{fn area(&self)->u32{self.w*self.h}}
impl Rectangle{
fn can_hold(&self,o:&Rectangle)->bool{self.area()>o.area()}}


fn main(){
    let r1=Rectangle{w:30,h:50};
    let r2=Rectangle{w:10,h:40};
    let r3=Rectangle{w:60,h:45};
    println!("{}", r1.can_hold(&r2));
    println!("{}", r1.can_hold(&r3));
}