use ndarray::{Array2, Axis, concatenate};

pub struct LCD {
    number: u32
}

impl LCD {
    pub fn new(number:u32) -> LCD {
        LCD{ number }
    }

    pub fn show(&self,s:usize) {
        let number = self.build_number(s);
        let mut result:String = String::new();
        number.rows().into_iter().for_each(|row| {
            row.iter().enumerate().for_each(|(index, col)| {
                if index == 0 { return }
                result += match col { 1 => "-", 2 => "|", _ => " " }
            });
            result += "\n";
        });
        println!("{}",result)
    }

    fn build_number(&self, s:usize) -> Array2<u32> {
        let mut n = self.number;
        let divider = Array2::zeros((2*s+3,1));
        let mut result = Array2::zeros((2*s+3,0));
        while n > 0 {
            let dig = n % 10;
            n = (n - dig) / 10;
            let digit = self.build_digit(dig as usize, s);
            result = concatenate(Axis(1),&[divider.view(),digit.view(),result.view()]).unwrap().to_owned();
        }
        return result;
    }

    fn build_digit(&self, n:usize, s:usize) -> Array2<u32> {
        let mut arr = vec![];
        for (row_index, row) in NUMBERS[n].iter().enumerate() {
            let mut res_row = vec![];
            for (col_index, col) in row.iter().enumerate() {
                if col_index == 1 && n != 1 {
                    (0..s).for_each(|_| { res_row.push(*col); })
                } else {
                    res_row.push(*col)
                }
            }
            if row_index == 1 || row_index == 3 {
                (0..s).for_each(|_| { arr.push(res_row.clone()) })
            } else {
                arr.push(res_row);
            }
        }
        let mut result = Array2::zeros((2 * s + 3, s + 2)).to_owned();
        if n == 1 { result = Array2::zeros((2 * s + 3, 1)).to_owned(); }
        for (row_idx, row) in arr.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                result[(row_idx, col_idx)] = *col;
            }
        };
        return result;
    }
}

const NUMBERS: &[&[&[u32]]] = &[
    &[
        &[0, 1, 0],
        &[2, 0, 2],
        &[0, 0, 0],
        &[2, 0, 2],
        &[0, 1, 0]
    ],
    &[
        &[0],
        &[2],
        &[0],
        &[2],
        &[0]
    ],
    &[
        &[0, 1, 0],
        &[0, 0, 2],
        &[0, 1, 0],
        &[2, 0, 0],
        &[0, 1, 0]
    ],
    &[
        &[0, 1, 0],
        &[0, 0, 2],
        &[0, 1, 0],
        &[0, 0, 2],
        &[0, 1, 0]
    ],
    &[
        &[0, 0, 0],
        &[2, 0, 2],
        &[0, 1, 0],
        &[0, 0, 2],
        &[0, 0, 0]
    ],
    &[
        &[0, 1, 0],
        &[2, 0, 0],
        &[0, 1, 0],
        &[0, 0, 2],
        &[0, 1, 0]
    ],
    &[
        &[0, 1, 0],
        &[2, 0, 0],
        &[0, 1, 0],
        &[2, 0, 2],
        &[0, 1, 0]
    ],
    &[
        &[0, 1, 0],
        &[0, 0, 2],
        &[0, 0, 0],
        &[0, 0, 2],
        &[0, 0, 0]
    ],
    &[
        &[0, 1, 0],
        &[2, 0, 2],
        &[0, 1, 0],
        &[2, 0, 2],
        &[0, 1, 0]
    ],
    &[
        &[0, 1, 0],
        &[2, 0, 2],
        &[0, 1, 0],
        &[0, 0, 2],
        &[0, 1, 0]
    ]
];