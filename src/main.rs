fn main() {
 let v = vec![1,4,3,6,2,7,8,5];
let answer = sort(v.clone());

    println!("{:?}", answer);
}

fn sort(v:Vec<i32>)-> Vec<i32>{
    let len = v.len();
    let mut answer: Vec<i32> = Vec::new(); 
    if len >= 2{
        let mut right: Vec<i32> =  Vec::new();
        let mut left: Vec<i32> = Vec::new();
        let mut i:usize  = 0;
        for a in v {
            if i < len/2{
                right.push(a);
            }else{
                left.push(a);
            }
            i+=1;
        }
        right = sort(right);
        left = sort(left);
        let mut a:usize = 0;
        let mut b:usize = 0;
        for i in 0..=(len -1){
            if right[a]>left[b]{
                answer.push(right[a]);
                a+=1;
            }else if left[b] > right[a]{
                answer.push(left[b]);
                b +=1;
            }else {
                answer.push(right[a]);
                a+=1;
                b+=1;
            }
            if a > (right.len()-1){
                for k in b ..=left.len()-1{
                    answer.push(left[k]);
                }
                break; 
            }
            if b > (left.len()-1){
                for k in a ..=right.len()-1{
                    answer.push(right[k]);
                }
                break;
            }
        }
    }else{
        answer = v;
    }
    return answer;
    
}
