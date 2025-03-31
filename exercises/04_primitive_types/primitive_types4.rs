fn main() {
    // You can optionally experiment here.
    let s  = String::from("Yash Raj");
    let surname = get_last_name(&s);
    println!("{}",surname);
}

fn get_last_name(s: &String)-> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
           if item == b' '{
            return &s[i+1..];
           }
    }
     &s[..]
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..a.len()-1];
        // println!("{nice_slice}");
        assert_eq!([2, 3, 4], nice_slice);
    }
}
