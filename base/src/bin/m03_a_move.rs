fn main() {
    let s1 = String::from("bonjour");
    let s2 = s1;          // move : s1 n'est plus utilisable
    // println!("{s1}");  // erreur E0382 : borrow of moved value
    let s3 = s2.clone();  // copie profonde, explicite
    println!("{s2} {s3}");

    let a = 5;
    let b = a;            // i32 est Copy : a reste valide
    println!("{a} {b}");
}
