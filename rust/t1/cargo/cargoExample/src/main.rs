use std::io;

fn main() {
    println!("hello cargo!");

    //let peut etre redefini
    let x: i16 = 4;
    println!("x = {x}");

    let mut x: i8 = 6;
    println!("x : {x}");

    //const ne peut jamais changer
    const Y: u8 = 3;
    println!("y = {Y}");

    //shadow variable qui permet de modifier des truc que dans ce context
    {
        x = 2;
        println!("x = {x}");
    }
    //pas de x++
    x += 1;
    println!("x = {x}");

    //tuple permet de lier plusieur valeur
    let size: (u8, u8) = (4, 9);
    println!("sizeX = {} \nsizeY = {}", size.0, size.1);

    let mut tup: (f32, char, bool) = (4.42, 'r', true);
    println!("tup[0] = {}", tup.0);
    println!("tup[1] = {}", tup.1);
    println!("tup[2] = {}", tup.2);
    tup.0 = 53.9813;
    println!("tup[0] = {}", tup.0);

    //array
    let arr: [i8; 5] = [3; 5];
    println!("arr[0] = {}", arr[0]);
    println!("arr[1] = {}", arr[1]);
    println!("arr[2] = {}", arr[2]);
    println!("arr[3] = {}", arr[3]);
    println!("arr[4] = {}", arr[4]);

    //string
    let strr: &str = "jack mange";
    println!("{strr}");

    //cast
    println!("{}", x + (Y as i8));

    println!("Going to wait...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
