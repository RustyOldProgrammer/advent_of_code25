#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let rect: Rectangle =  Rectangle{
        width: 20, 
        height:50
    };

    println!("rect: {:#?}", rect);
    
    println!("The area of the Rectangle is {} square pixels", 
    area(&rect));

}

fn area(Rectangle: &Rectangle) -> u32{
    Rectangle.width * Rectangle.height
}