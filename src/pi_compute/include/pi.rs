
 /*
  *pi.rs is  translate from pi.cpp which is from pi.d
  * see https://pypi.org/project/pi-compute/ 
  all rights reserved 
  rustc pi.rs 
  ./pi 1000
  * */
type MYTYPE = i8;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut n = 100;
    if args.len() == 2 {
        n = args[1].parse().expect("prefix length of pi");
        println!("pi {}", n);
    } else {
        println!("pi 100");
    }
    let ans = compute(n);

    println!("{:?}", ans);
}
fn tiszero(t: &Vec<MYTYPE>) -> bool {
    for x in t {
        if *x != 0 {
            return false;
        }
    }
    return true;
}

fn mul4(p: &mut Vec<MYTYPE>) {
    let mut c: MYTYPE = 0;
    for x in p.iter_mut().rev() {
        let s = *x;
        let d = (s * 4 + c) % 10;
        c = (s * 4 + c) / 10;
        *x = d;
    }
}

/*
fn div4( p:&mut Vec<MYTYPE>) {
   let mut d:MYTYPE=0;
   for x in p.iter_mut() {
       let s =*x;
       let c = (10*d+s)/4;
       d = (10*d + s)%4;
       *x =c;
   }
}
*/

fn div(t: &mut Vec<MYTYPE>, divisor: i32) {
    let mut r = 0;
    for x in t.iter_mut() {
        let s = *x as i32;
        let b = 10 * r + s;
        let q = b / divisor;
        r = b % divisor;
        *x = q as MYTYPE;
    }
}

fn mul(t: &mut Vec<MYTYPE>, multiplier: i32) {
    let mut carry: i32 = 0;
    for x in t.iter_mut().rev() {
        let s = *x as i32;
        let b = s * multiplier + carry;
        carry = b / 10;
        let d = b % 10;
        *x = d as MYTYPE;
    }
}

fn sub(p: &mut Vec<MYTYPE>, t: &Vec<MYTYPE>) {
    let n = p.len();
    for i in (0..n).rev() {
        if p[i] < t[i] {
            p[i] = p[i] + 10 - t[i];
            if i >= 1 {
                p[i - 1] -= 1;
            }
        } else {
            p[i] -= t[i];
        }
    }
}

fn add(p: &mut Vec<MYTYPE>, t: &Vec<MYTYPE>) {
    let n = p.len();
    for i in (0..n).rev() {
        if p[i] + t[i] > 9 {
            p[i] += t[i] - 10;
            if i >= 1 {
                p[i - 1] += 1;
            }
        } else {
            p[i] += t[i];
        }
    }
}

fn arctan(p: &mut Vec<MYTYPE>, t: &mut Vec<MYTYPE>, s: i32, flag: bool) {
    let mut n = 1;
    t[0] = 1;
    div(t, s);
    if flag {
        sub(p, t);
    } else {
        add(p, t);
    }
    loop {
        mul(t, n);
        div(t, s * s);
        n += 2;
        div(t, n);
        if ((n - 1) / 2) % 2 == 0 {
            if flag {
                sub(p, t);
            } else {
                add(p, t);
            }
        } else {
            if flag {
                add(p, t);
            } else {
                sub(p, t);
            }
        }
        if tiszero(t) {
            break;
        }
    }
}

fn compute(n: i32) -> String {
    let n = n as usize;
    let mut p: Vec<MYTYPE> = vec![0; n + 1];
    let mut q: Vec<MYTYPE> = vec![0; n + 1];
    let mut p1: Vec<MYTYPE> = vec![0; n + 1];
    let mut q1: Vec<MYTYPE> = vec![0; n + 1];
    let th1 = std::thread::spawn(move || {
        arctan(&mut p, &mut q, 5, false);
        mul4(&mut p);
        return p;
    });
    let th2 = std::thread::spawn(move || {
        arctan(&mut p1, &mut q1, 239, true);
        return p1;
    });
    let mut p = th1.join().unwrap();
    let p1 = th2.join().unwrap();
    add(&mut p, &p1);
    mul4(&mut p);
    let n = p.len();
    let mut ans: String = String::new();
    ans.push_str("3");
    ans.push_str(".");
    for i in 2..n + 1 {
        ans.push_str(&p[i - 1].to_string());
    }
    return ans;
}
