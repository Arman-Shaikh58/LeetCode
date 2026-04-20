
struct Arrays{
    nums: Vec<u32>,
    length: usize
}
impl Arrays{
    fn merge(v1: &mut Arrays,v2:&mut Arrays){
        let v1_cpy = v1.nums[..v1.length].to_vec(); // .clone does not work here because mutable reference dont contain that method
        //here if just did this let v1_cpy = v1.nums[..v1.length]; this would have create v1_cpy but it will hold reference to v1.num's memory location
        //by using .to_vec() we create a deep copy of the  vector
        let mut i :usize= 0;
        let mut j:usize = 0;
        let mut z:usize=0;

        while i< v1.length && j < v2.length{
            if v1_cpy[i]<v2.nums[j]{
                v1.nums[z] = v1_cpy[i];
                i+=1;
                z+=1;
            }else{
                v1.nums[z] = v2.nums[j];
                j+=1;
                z+=1;
            }
        }
        while i < v1.length{
             v1.nums[z] = v1_cpy[i];
                i+=1;
                z+=1;
        }
        while j < v2.length{
            v1.nums[z] = v2.nums[j];
                j+=1;
                z+=1;
        }
    }
}

fn main(){
    let mut nums1 = Arrays{
        nums:[1,2,3,0,0,0].to_vec(),
        length:3
    };
    let mut nums2 = Arrays{
        nums:[2,5,6].to_vec(),
        length:3
    };

    Arrays::merge(&mut nums1, &mut nums2);

    for i in nums1.nums{
        println!("{i}");
    }

}