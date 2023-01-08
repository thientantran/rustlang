fn main() {
    // Mặc định của biến trong rust là bất biến
    let mut x =10;
    println!("{}",x);
    x = 20;
    println!("{}",x);

    const HANG_SO:u128 = 100_000_000_000;
    println!("{}",HANG_SO);


    // SHADOWING
    let outer = 10;
    {
        let outer = 200;
        println!("outer: {}", outer);
    }
    println!("outer: {}", outer);
    println!("Khi khai báo biến trong phạm vi khác thì biến cũ vấn ở đó, chứ ko bị biến mất, bị che phủ thôi");

    // DATA TYPEs

    // Scalar
        //Integer
    let x = 100;
    let y:i32; // bắt buộc phải khai báo kiểu dữ liệu khi không khai báo giá trị    
    println!("Có thể khai báo ở dạng số decimal, hex, octal, binary, và byte");
        //Character
    let s: char = 'z';
    let c: &str = "abc";
    println!("{}",s);
    println!("{}",c);
    println!("Phân biệt cả ngoặc đơn và ngoặc kép đối với character và string");

        //Boolean

    let t=true;
    let f = false;    

        //Float
    let f= 2.0;
    let g: f32= 3.0;
    println!("Chia lấy dư");
    let remainder = 43%4;
    println!("remainder: {}", remainder);

}
