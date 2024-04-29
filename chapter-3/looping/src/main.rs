fn main() {
    let mut x = 1;
    loop{
        println!("again!");
        x+=1; //Check to make sure the loop is not infinite
        if(x>3){
            break;
        }
    }
}
