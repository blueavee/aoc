fn main() {
    let file = std::fs::read_to_string("input").unwrap();

    let mut a_vec: Vec<u64> = vec![];
    let mut b_vec: Vec<u64> = vec![];
    let mut c_vec: Vec<u64> = vec![];
    let mut d_vec: Vec<u64> = vec![];

    file.lines().for_each(|line| {
        let mut items = line.split_whitespace();

        if let Some(a) = items.next() {
            if let Ok(a_num) = a.parse::<u64>() {
                a_vec.push(a_num);
            }
        }

        if let Some(b) = items.next() {
            if let Ok(b_num) = b.parse::<u64>() {
                b_vec.push(b_num);
            }
        }
    });

    println!("{:?}", a_vec);

    a_vec.sort();
    b_vec.sort();

    for i in 0..b_vec.len() {
        let difference = a_vec[i].abs_diff(b_vec[i]);
        c_vec.push(difference);
    }

    let _sum: u64 = c_vec.iter().sum();

    for i in 0..a_vec.len() {
        let mut occurence: u64 = 0;
        for k in 0..b_vec.len() {
            if a_vec[i] == b_vec[k] {
                occurence += 1
            }
        }

        d_vec.push(a_vec[i] * occurence);
    }

    let occure_sum: u64 = d_vec.iter().sum();
    println!("{}", occure_sum)
}
