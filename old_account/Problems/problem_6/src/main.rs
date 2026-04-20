
fn convert(s:&String,num_rows:usize)->String{
    if num_rows<s.len() || num_rows==1 {
        return s.to_string();
    }
    let mut res = String::new();
    let s_chars: Vec<char> = s.chars().collect();
    let len = s.len();
    let jump_distance= (num_rows-1)*2;

    for rows in 0..num_rows {
        
        let mut j = rows;
        while j < len{
            res.push(s_chars[j]);
            let dig_element= j+jump_distance- 2*rows;
            if rows!=0 && rows!=num_rows-1 && j>=len {
                res.push(s_chars[dig_element]);
            }
            j+=jump_distance;
        }
    }
    res
}

fn main() {
    let s: String = String::from("PAYPALISHIRING");
    let num_rows:usize = 4;
    let res = convert(&s, num_rows);
    println!("The Output is: {res}");
    println!("The Expected Output is: {s}");
}
//im very happy because i got very less errors while compiling this program and i didn't even use GPT :) ooo hoo