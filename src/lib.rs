use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        let mut v = vec![];

        for i in values{
        	v.push(*i);
        }
        let new_vector = Matrix{data:v,row:row,col:col};      
        return new_vector;
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        let v = vec![];
        let any_struct = Matrix{data:v,row:row,col:col};
        return any_struct;

    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row,self.col)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = Vec::new();
        let length = self.row*self.col;
        if self.row != rhs.row || self.col != rhs.col{
        	panic!();
        }
        for i in 0..length{
        	v.push(self.data[i]+rhs.data[i]);
        }
        Matrix{data:v,row:self.row,col:self.col}
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = Vec::new();
        let length = self.row*self.col;
        if self.row != rhs.row || self.col != rhs.col{
        	panic!();
        }
        for i in 0..length{
        	v.push(self.data[i]-rhs.data[i]);
        }
        Matrix{data:v,row:self.row,col:self.col}
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Default> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        let mut v = Vec::new();
        
        let m1 = self.row;
        let n1 = self.col;
        //let m2 = rhs.row;
        let n2 = rhs.col;
        if self.col != rhs.row{
        	panic!();
        }
        for i in 0..m1{
        	for j in 0..n2{
        		let mut sum: T = Default::default();
        		for k in 0..n1{
        			sum = sum+self.data[n1*i+k]*rhs.data[j+n2*k];
        		}
        		v.push(sum);

        	}
        	

        }


        Matrix{data:v,row:self.row,col:rhs.col}
        ///////////////////////////////////////////////////////////////////////
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_string();
        
        for k in 0..self.row*self.col{
        	string.push_str(&format!("{}",self.data[k]));
        	if k != self.col-1{
        		string.push_str(&format!(" "));
        	}
        	if k == self.col-1{
        		string.push_str(&format!("\n"));
        	}
        	
        }
        string.pop();
        string.push_str(&format!("\n"));

        write!(f,"{}",string)

        

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
//use Matrix;
use super::*;
    #[test]
    fn test_new() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        println!("{:?}", x.row);
        println!("{:?}", x.col);
        println!("{:?}", x.data);
        assert_eq!(x.row, 2 as usize);
        assert_eq!(x.col, 3 as usize);
        assert_eq!(x.data[0], -2);
    }
    #[test]
    fn test_new_empty() {
        let x:Matrix<i32> = Matrix::new_empty(2,3);
        assert_eq!(x.row, 2);
        assert_eq!(x.col, 3);
    }

    #[test]
    fn test_data() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); 
        let shared_ref = x.data();
        assert_eq!(x.data[0], shared_ref[0]);
    }
    #[test]
    fn test_mut_data() {
        let mut x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let data = x.mut_data();
        data[0] = 5;
        assert_eq!(data[0], 5);
    }
    #[test] 
    fn test_size() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = x.size();
        assert_eq!(y.0, 2);
        assert_eq!(y.1, 3);
    }
     #[test]
    fn test_add() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let z = x + y;
        assert_eq!(z.data, [-1,0,1,2,3,4]);
        //assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    }
    #[test]
    fn test_sub() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let z = x - y;
        assert_eq!(z.data, [-3,-2,-1,0,1,2]);
    }
    #[test]
    #[should_panic]
    //#[ignore]
    fn test_add_panic() {
        let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let _z = x + y;
    }
    
    #[test]
    #[should_panic]
    //#[ignore]
    fn test_sub_panic() {
        let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let _z = x - y;
    }
    #[test]
    fn test_fmt() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    }
    #[test]
    fn test_mul() {
        let x = Matrix::new(3, 3, &[1,2,3,4,5,6,7,8,9]);
        let y = Matrix::new(3, 3, &[9,8,7,2,2,2,1,1,1]);
        let z = x * y;
        assert_eq!(z.data, [16,15,14,52,48,44,88,81,74]);
    }
    #[test]
    fn test_mul2() {
        let x = Matrix::new(2, 2, &[2,1,3,1]);
        let y = Matrix::new(2, 3, &[1,2,3,3,2,1]);
        let z = x * y;
        assert_eq!(z.data, [5,6,7,6,8,10]);
    }
    #[test]
    fn test_mul3() {
        let x = Matrix::new(1, 1, &[2]);
        let y = Matrix::new(1, 1, &[1]);
        let z = x * y;
        assert_eq!(z.data, [2]);
    }
}


// #[cfg(test)]
// mod tests {
// 	use super::*;
//     #[test]
// fn test() {
//     let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
//     assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
// }
// #[test]
//     fn test_mul() {
//         let x = Matrix::new(3, 3, &[1,2,3,4,5,6,7,8,9]);
//         let y = Matrix::new(3, 3, &[9,8,7,2,2,2,1,1,1]);
//         let z = x * y;
//         assert_eq!(z.data, [16,15,14,52,48,44,88,81,74]);
//     }

// #[test]
//     fn test_mul2() {
//         let x = Matrix::new(2, 2, &[1,2,3,4]);
//         let y = Matrix::new(2, 2, &[9,8,7,2]);
//         let z = x * y;
//         assert_eq!(z.data, [23,12,55,32]);
//     }
// }
