struct rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let rect:(u32), (u32)=  rectangle{
        width: 20
        height:50
    } 
    println!("rect: {:#?}", rect);
    
    println!("The area of the rectangle is {} square pixels", 
    area(rect));

}

fn area(rectangle: &rectangle) -> u32:
    rectangle.width * rectangle.height