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
    // Gồm có scala và compound
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


    // Compound --- gồm tuple và array
    // tuple gồm nhiều kiểu dữ liệu
    let tup = ("hello", 100_000);
    println!("{:?}", tup);
    let (_string, _integer) = tup;
    let _integer = tup.1;
    println!("{}",_string);
    println!("{}",_integer);

    // array là 1 danh sách có kích thước cố định và cùng 1 kiểu dữ liệu
    let numbers = [100,200,300];
    let get_number = numbers[2];
    println!("{}",get_number);
    
    // tạo ra 1 mảng có 32 số 0, chú ý dấu phẩy và dấu chấm phẩy

    let _hashing = [0;32];

    println!("{:?}", _hashing);
    for i in _hashing.iter(){
        print!("{}",i);
    }
    println!("TAN DEP TRAI");
}
